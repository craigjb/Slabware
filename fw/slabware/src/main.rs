#![no_std]
#![no_main]

mod custom_int;
mod mi2c;
mod time_driver;
mod usb_driver;
mod usb_keys;
mod video;

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
    let i2cm = mi2c::I2cMaster::new(peripherals.mi2c);
    let usb_driver = usb_driver::Driver::new(peripherals.usb_ctrl);

    unwrap!(spawner.spawn(blink(peripherals.leds)));
    unwrap!(spawner.spawn(video::video_task(
        i2cm,
        peripherals.hdmi_rx,
        peripherals.lcd_dim,
    )));

    usb_keys::run_usb_keys(usb_driver, peripherals.grid_ctrl).await;
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
        video::HDMI_RX_INTERRUPT_CODE => video::handle_hdmi_interrupt(),
        usb_driver::USB_INTERRUPT_CODE => usb_driver::handle_usb_interrupt(),
        _ => {}
    }
}
