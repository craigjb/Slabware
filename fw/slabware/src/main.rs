#![no_std]
#![no_main]

mod time_driver;

use defmt::unwrap;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_halt as _;
use riscv::interrupt;
use slab_pac::Leds;

#[export_name = "ExceptionHandler"]
fn exception_handler(_trap_frame: &riscv_rt::TrapFrame) -> ! {
    use riscv::register::{mcause, mcause::Trap};
    let cause = mcause::read();
    match cause.cause() {
        Trap::Exception(e) => {
            defmt::error!("Exception: {}", e as usize);
        }
        _ => {}
    }
    loop {}
}

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

    defmt::println!("Starting...");
    loop {
        Timer::after_millis(1000).await;
        defmt::println!("TICK!");
    }
}
