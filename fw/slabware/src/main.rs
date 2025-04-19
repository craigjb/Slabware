#![no_std]
#![no_main]

mod custom_int;
mod mi2c;
mod time_driver;
mod usb_driver;
mod video;

use defmt::unwrap;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
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
        peripherals.grid_ctrl
    )));

    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("craigjb.com");
    config.product = Some("USB-serial test");
    config.serial_number = Some("12345678");
    config.max_packet_size_0 = 64;

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut state = embassy_usb::class::cdc_acm::State::new();

    let mut builder = embassy_usb::Builder::new(
        usb_driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );

    let mut class = embassy_usb::class::cdc_acm::CdcAcmClass::new(&mut builder, &mut state, 64);
    let mut usb = builder.build();
    let usb_fut = usb.run();

    let hello_fut = async {
        class.wait_connection().await;
        defmt::info!("[CDC] Connected");
        let mut buf = [0; 64];
        loop {
            class.read_packet(&mut buf).await;
            class.write_packet(&buf).await;
            // while class.write_packet("Hello!\n\r".as_bytes()).await.is_ok() {
            // defmt::info!("[CDC] Sent hello");
            // Timer::after_secs(1).await;
            // }
        }
        // defmt::info!("[CDC] Disconnected");
    };

    join(usb_fut, hello_fut).await;
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
