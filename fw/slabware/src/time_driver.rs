use core::cell::Cell;
use core::sync::atomic::{compiler_fence, AtomicU32, AtomicU8, Ordering};
use core::{mem, ptr, u32};
use critical_section::CriticalSection;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time_driver::{AlarmHandle, Driver, TICK_HZ};
use slab_pac::Timer;

pub const CPU_FREQ: u32 = 50_000_000;

// NOTE regarding ALARM_COUNT:
//
// Timer compare0 is used as the halfway roll-over
// Extra compares can be used as alarms.
// Requires timer numCompares >= 2

const ALARM_COUNT: usize = 1;

#[export_name = "MachineTimer"]
fn timer_interrupt() {
    DRIVER.on_interrupt()
}

// Clock timekeeping works with something we call "periods", which are time intervals
// of 2^15 ticks. The Clock counter value is 16 bits, so one "overflow cycle" is 2 periods.
//
// A `period` count is maintained in parallel to the Timer hardware `counter`, like this:
// - `period` and `counter` start at 0
// - `period` is incremented on overflow (at counter value 0)
// - `period` is incremented "midway" between overflows (at counter value 0x8000)
//
// Therefore, when `period` is even, counter is in 0..0x7FFF. When odd, counter is in 0x8000..0xFFFF
// This allows for now() to return the correct value even if it races an overflow.
//
// To get `now()`, `period` is read first, then `counter` is read. If the counter value matches
// the expected range for the `period` parity, we're done. If it doesn't, this means that
// a new period start has raced us between reading `period` and `counter`, so we assume the `counter` value
// corresponds to the next period.
//
// `period` is a 32bit integer, so It overflows on 2^32 * 2^15 / 32768 seconds of uptime, which is 136 years.
fn calc_now(period: u32, counter: u16) -> u64 {
    ((period as u64) << 15) + ((counter as u32 ^ ((period & 1) << 15)) as u64)
}

struct AlarmState {
    timestamp: Cell<u64>,

    // This is really a Option<(fn(*mut ()), *mut ())>
    // but fn pointers aren't allowed in const yet
    callback: Cell<*const ()>,
    ctx: Cell<*mut ()>,
}

unsafe impl Send for AlarmState {}

impl AlarmState {
    const fn new() -> Self {
        Self {
            timestamp: Cell::new(u64::MAX),
            callback: Cell::new(ptr::null()),
            ctx: Cell::new(ptr::null_mut()),
        }
    }
}

pub(crate) struct RtcDriver {
    /// Number of 2^15 periods elapsed since boot.
    period: AtomicU32,
    alarm_count: AtomicU8,
    /// Timestamp at which to fire alarm. u64::MAX if no alarm is scheduled.
    alarms: Mutex<CriticalSectionRawMutex, [AlarmState; ALARM_COUNT]>,
}

#[allow(clippy::declare_interior_mutable_const)]
const ALARM_STATE_NEW: AlarmState = AlarmState::new();

embassy_time_driver::time_driver_impl!(static DRIVER: RtcDriver = RtcDriver {
    period: AtomicU32::new(0),
    alarm_count: AtomicU8::new(0),
    alarms: Mutex::const_new(CriticalSectionRawMutex::new(), [ALARM_STATE_NEW; ALARM_COUNT]),
});

impl RtcDriver {
    fn init(&'static self, _cs: critical_section::CriticalSection) {
        let timer = unsafe { Timer::steal() };

        let psc = CPU_FREQ / TICK_HZ as u32 - 1;
        let psc: u16 = match psc.try_into() {
            Err(_) => panic!("psc division overflow: {}", psc),
            Ok(n) => n,
        };

        timer.prescale().write(|w| unsafe { w.bits(psc as u32) });

        // Mid-way point
        timer.compare0().write(|w| unsafe { w.bits(0x8000) });

        // Enable overflow & compare0 (halfway) interrupts
        timer.interrupt_mask().write(|w| {
            w.overflow_mask()
                .clear_bit()
                .compare0mask()
                .clear_bit()
                .compare1mask()
                .set_bit()
        });

        // Clear any interrupts
        timer.interrupt_status().write(|w| {
            w.overflow_status()
                .clear_bit_by_one()
                .compare0status()
                .clear_bit_by_one()
                .compare1status()
                .clear_bit_by_one()
        });

        // Enable timer and interrupts
        timer
            .control()
            .write(|w| w.enable().set_bit().interrupt_enable().set_bit());

        unsafe {
            riscv::register::mie::set_mtimer();
        }
    }

    fn on_interrupt(&self) {
        let timer = unsafe { Timer::steal() };

        critical_section::with(|cs| {
            let status = timer.interrupt_status().read();
            let masks = timer.interrupt_mask().read();

            // Clear all interrupts
            timer.interrupt_status().write(|w| {
                w.overflow_status()
                    .clear_bit_by_one()
                    .compare0status()
                    .clear_bit_by_one()
                    .compare1status()
                    .clear_bit_by_one()
            });

            // Overflow
            if status.overflow_status().bit_is_set() {
                self.next_period();
            }

            // Half overflow (compare 0)
            if status.compare0status().bit_is_set() {
                self.next_period();
            }

            // Alarm 0 (compare 1)
            if status.compare1status().bit_is_set() && masks.compare1mask().bit_is_clear() {
                self.trigger_alarm(0, cs);
            }
        })
    }

    fn next_period(&self) {
        let timer = unsafe { Timer::steal() };

        // We only modify the period from the timer interrupt, so we know this can't race.
        let period = self.period.load(Ordering::Relaxed) + 1;
        self.period.store(period, Ordering::Relaxed);
        let t = (period as u64) << 15;

        critical_section::with(move |cs| {
            timer.interrupt_mask().modify(move |_, w| {
                let alarm = &self.alarms.borrow(cs)[0];
                let at = alarm.timestamp.get();

                if at < t + 0xc000 {
                    // just enable it. `set_alarm` has already set the correct CCR val.
                    w.compare1mask().clear_bit()
                } else {
                    w.compare1mask().set_bit()
                }
            })
        })
    }

    fn get_alarm<'a>(&'a self, cs: CriticalSection<'a>, alarm: AlarmHandle) -> &'a AlarmState {
        // safety: we're allowed to assume the AlarmState is created by us, and
        // we never create one that's out of bounds.
        unsafe { self.alarms.borrow(cs).get_unchecked(alarm.id() as usize) }
    }

    fn trigger_alarm(&self, n: usize, cs: CriticalSection) {
        let alarm = &self.alarms.borrow(cs)[n];
        alarm.timestamp.set(u64::MAX);

        // Call after clearing alarm, so the callback can set another alarm.

        // safety:
        // - we can ignore the possibility of `f` being unset (null) because of the safety contract of `allocate_alarm`.
        // - other than that we only store valid function pointers into alarm.callback
        let f: fn(*mut ()) = unsafe { mem::transmute(alarm.callback.get()) };
        f(alarm.ctx.get());
    }
}

impl Driver for RtcDriver {
    fn now(&self) -> u64 {
        let timer = unsafe { Timer::steal() };

        let period = self.period.load(Ordering::Relaxed);
        compiler_fence(Ordering::Acquire);
        let counter = timer.counter().read().counter().bits();
        calc_now(period, counter)
    }

    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
        critical_section::with(|_| {
            let id = self.alarm_count.load(Ordering::Relaxed);
            if id < ALARM_COUNT as u8 {
                self.alarm_count.store(id + 1, Ordering::Relaxed);
                Some(AlarmHandle::new(id))
            } else {
                None
            }
        })
    }

    fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
        critical_section::with(|cs| {
            let alarm = self.get_alarm(cs, alarm);

            alarm.callback.set(callback as *const ());
            alarm.ctx.set(ctx);
        })
    }

    fn set_alarm(&self, alarm: AlarmHandle, timestamp: u64) -> bool {
        critical_section::with(|cs| {
            let timer = unsafe { Timer::steal() };

            let n = alarm.id() as usize;
            assert!(n == 0, "Only Alarm 0 is implemented");
            let alarm = self.get_alarm(cs, alarm);
            alarm.timestamp.set(timestamp);

            let t = self.now();
            if timestamp <= t {
                // If alarm timestamp has passed the alarm will not fire.
                // Disarm the alarm and return `false` to indicate that.

                // There's only alarm 0
                timer
                    .interrupt_mask()
                    .modify(|_, w| w.compare1mask().set_bit());

                alarm.timestamp.set(u64::MAX);

                return false;
            }

            // Write the compare value regardless of whether we're going to enable it now or not.
            // This way, when we enable it later, the right value is already set.
            timer
                .compare1()
                .write(|w| unsafe { w.value().bits(timestamp as u16) });

            // Enable it if it'll happen soon. Otherwise, `next_period` will enable it.
            let diff = timestamp - t;
            timer
                .interrupt_mask()
                .modify(|_, w| w.compare1mask().bit(!(diff < 0xc000)));

            // Reevaluate if the alarm timestamp is still in the future
            let t = self.now();
            if timestamp <= t {
                // If alarm timestamp has passed since we set it, we have a race condition and
                // the alarm may or may not have fired.
                // Disarm the alarm and return `false` to indicate that.
                // It is the caller's responsibility to handle this ambiguity.
                //
                timer
                    .interrupt_mask()
                    .modify(|_, w| w.compare1mask().set_bit());

                alarm.timestamp.set(u64::MAX);

                return false;
            }

            // We're confident the alarm will ring in the future.
            true
        })
    }
}

pub(crate) fn init(cs: CriticalSection) {
    DRIVER.init(cs)
}
