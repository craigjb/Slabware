#![no_std]
#![no_main]

mod mi2c;
mod time_driver;

use defmt::unwrap;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_halt as _;
use riscv::interrupt;
use slab_pac::Leds;

use mi2c::{I2c, I2cMaster};

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

    let mut i2cm = I2cMaster::new(peripherals.mi2c);
    let mut data: [u8; 8] = [0; 8];
    i2cm.write_read(0x5C, &[0x0], &mut data).await.unwrap();
    defmt::println!("Read at 0x0: {:#02X}", data);

    i2cm.write_read(0x5C, &[0x8], &mut data[0..1])
        .await
        .unwrap();
    defmt::println!("Read at 0x8: {:#02X}", data[0]);

    i2cm.write_read(0x5C, &[0x9], &mut data[0..4])
        .await
        .unwrap();
    defmt::println!("Read at 0x9: {:08b}", data[0..4]);

    i2cm.write(0x5C, &[0x9, 0x0]).await.unwrap();

    i2cm.write_read(0x5C, &[0x9], &mut data[0..4])
        .await
        .unwrap();
    defmt::println!("Read at 0x9: {:08b}", data[0..4]);

    // loop {
    //     Timer::after_secs(10).await;
    // }
}
