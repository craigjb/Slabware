#![no_std]
#![no_main]

mod custom_int;
mod i2c;
mod mi2c;
mod sealed_instance;
mod time_driver;
mod tmds181;

use defmt::unwrap;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_halt as _;
use riscv::interrupt;
use slab_pac::Leds;

fn init() -> slab_pac::Peripherals {
    critical_section::with(|cs| {
        let peripherals = slab_pac::Peripherals::take().unwrap();
        time_driver::init(cs);

        unsafe {
            interrupt::enable();
        }

        peripherals
    })
}

#[embassy_executor::task]
async fn blink(leds: Leds) {
    let mut mask = 0x80;
    loop {
        leds.ctrl().write(|w| unsafe { w.value().bits(mask) });
        if mask == 0 {
            mask = 0x80;
        }
        mask >>= 1;
        Timer::after_millis(100).await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = init();

    // unwrap!(spawner.spawn(blink(peripherals.leds)));

    let i2cm = mi2c::I2cMaster::new(peripherals.mi2c);
    let _retimer = unwrap!(
        tmds181::Tmds181Config::new()
            .address(0x5C)
            .lane_swap(true)
            .polarity_swap(true)
            .redriver_auto_standby(true)
            .build(i2cm)
            .await
    );

    let hdmi = peripherals.hdmi;
    let divisor = hdmi.clk_det_divisor().read().value().bits();
    defmt::debug!("HDMI clk det divisor: {}", divisor);
    let sample_rate = hdmi.clk_det_sample_rate().read().value().bits();
    defmt::debug!("HDMI clk det sample rate: {}", sample_rate);

    loop {
        let cable_detect = hdmi.status().read().cable_detect().bit_is_set();
        defmt::debug!("HDMI cable detect: {}", cable_detect);
        let count = hdmi.clk_det_count().read().value().bits();
        defmt::debug!("HDMI clk count: {}", count);

        hdmi.control().write(|w| w.hpd_enable().bit(cable_detect));
        Timer::after_secs(1).await;
    }
}

#[export_name = "ExceptionHandler"]
fn exception_handler(_trap_frame: &riscv_rt::TrapFrame) -> ! {
    use riscv::register::{mcause, mcause::Trap};
    let cause = mcause::read();
    match cause.cause() {
        Trap::Exception(e) => {
            defmt::panic!("Exception trap, cause: {}", e as usize)
        }
        _ => unreachable!(),
    }
}

#[export_name = "DefaultHandler"]
fn interrupt_handler() {
    let cause = riscv::register::mcause::read().code();
    match cause {
        mi2c::MI2C_INTERRUPT_CODE => mi2c::handle_mi2c_interrupt(),
        _ => {}
    }
}
