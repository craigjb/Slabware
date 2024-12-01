#![no_std]
#![no_main]

mod custom_int;
mod edid;
mod i2c;
mod mi2c;
mod sealed_instance;
mod si2c;
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
async fn main(spawner: Spawner) {
    let peripherals = init();

    unwrap!(spawner.spawn(blink(peripherals.leds)));

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

    let i2cs = si2c::I2cSlave::new(peripherals.si2c, 0x50);
    unwrap!(spawner.spawn(edid::ddc_edid(i2cs)));
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
        si2c::SI2C_INTERRUPT_CODE => si2c::handle_si2c_interrupt(),
        _ => {}
    }
}
