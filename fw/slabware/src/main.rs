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
use slab_pac::{Hdmi, Leds};

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

    hdmi.control().write(|w| w.hpd_enable().set_bit());
    loop {
        wait_for_hdmi_cable(&hdmi).await;
        if !hdmi_clock_detect(&hdmi).await {
            continue;
        }
        hdmi_pll_reset(&hdmi);
        if !wait_for_hdmi_pll_lock(&hdmi).await {
            continue;
        }
        if !hdmi_gtp_reset(&hdmi).await {
            continue;
        }
        if !hdmi_wait_valid_data(&hdmi).await {
            continue;
        }
        wait_for_hdmi_lost(&hdmi).await;
    }
}

async fn wait_for_hdmi_cable(hdmi: &Hdmi) {
    loop {
        if hdmi.status().read().cable_detect().bit_is_set() {
            defmt::info!("HDMI cable detected");
            return;
        } else {
            Timer::after_millis(1).await;
        }
    }
}

async fn hdmi_clock_detect(hdmi: &Hdmi) -> bool {
    let divisor = hdmi.clk_det_divisor().read().value().bits() as usize;
    defmt::debug!("HDMI clk det divisor: {}", divisor);
    let sample_rate = hdmi.clk_det_sample_rate().read().value().bits() as usize;
    defmt::debug!("HDMI clk det sample rate: {}", sample_rate);

    let mut prev_count = 0;
    for _ in 0..2000 {
        let count = hdmi.clk_det_count().read().value().bits();
        if count != 0 {
            if prev_count != 0 && count >= prev_count - 1 && count <= prev_count + 1 {
                let clock = count as usize * divisor * sample_rate;
                defmt::info!("HDMI clock detected ({} Hz)", clock);
                return true;
            }
            prev_count = count;
        }
        Timer::after_millis(1).await;
    }
    false
}

fn hdmi_pll_reset(hdmi: &Hdmi) {
    defmt::info!("Resetting HDMI PLL");
    hdmi.control().modify(|_, w| {
        w.pll_power_down()
            .clear_bit()
            .pll_reset()
            .set_bit()
            .gtp_reset()
            .set_bit()
    });
    hdmi.control().modify(|_, w| w.pll_reset().clear_bit());
}

async fn hdmi_gtp_reset(hdmi: &Hdmi) -> bool {
    defmt::info!("Resetting GTP transceiver");
    hdmi.control().modify(|_, w| w.gtp_reset().clear_bit());

    defmt::info!("Waiting for GTP reset");
    let mut done = [false, false, false];
    for _ in 0..200 {
        if !done[0] && hdmi.channel0().read().gtp_reset_done().bit_is_set() {
            defmt::info!("GTP reset 0 done");
            done[0] = true;
        }
        if !done[1] && hdmi.channel1().read().gtp_reset_done().bit_is_set() {
            defmt::info!("GTP reset 1 done");
            done[1] = true;
        }
        if !done[2] && hdmi.channel2().read().gtp_reset_done().bit_is_set() {
            defmt::info!("GTP reset 2 done");
            done[2] = true;
        }

        if done.iter().all(|b| *b) {
            defmt::info!("GTP resets done");
            return true;
        } else {
            Timer::after_micros(10).await;
        }
    }
    false
}

async fn hdmi_wait_valid_data(hdmi: &Hdmi) -> bool {
    defmt::info!("Waiting for valid HDMI data");
    let mut valid = [false, false, false];
    for _ in 0..200 {
        if !valid[0]
            && hdmi.channel0().read().hdmi_data_out0valid().bit_is_set()
            && hdmi.channel0().read().hdmi_data_out1valid().bit_is_set()
        {
            defmt::info!("\tHDMI data 0 is valid");
            valid[0] = true;
        }
        if !valid[1]
            && hdmi.channel1().read().hdmi_data_out0valid().bit_is_set()
            && hdmi.channel1().read().hdmi_data_out1valid().bit_is_set()
        {
            defmt::info!("\tHDMI data 1 is valid");
            valid[1] = true;
        }
        if !valid[2]
            && hdmi.channel2().read().hdmi_data_out0valid().bit_is_set()
            && hdmi.channel2().read().hdmi_data_out1valid().bit_is_set()
        {
            defmt::info!("\tHDMI data 2 is valid");
            valid[2] = true;
        }

        if valid.iter().all(|b| *b) {
            defmt::info!("HDMI data is valid");
            return true;
        } else {
            Timer::after_micros(10).await;
        }
    }
    false
}

async fn wait_for_hdmi_pll_lock(hdmi: &Hdmi) -> bool {
    defmt::info!("Waiting for HDMI PLL lock");
    for _ in 0..200 {
        if hdmi.status().read().pll_lock().bit_is_set() {
            defmt::info!("HDMI PLL locked");
            return true;
        } else {
            Timer::after_micros(5).await;
        }
    }
    false
}

async fn wait_for_hdmi_lost(hdmi: &Hdmi) {
    loop {
        let status = hdmi.status().read();
        if status.cable_detect().bit_is_clear() {
            defmt::info!("HDMI cable unplugged");
            return;
        }
        if status.pll_lock().bit_is_clear() {
            defmt::info!("HDMI PLL lock lost");
            return;
        }
        if hdmi.clk_det_count().read().value().bits() == 0 {
            defmt::info!("HDMI clock lost");
            return;
        }
        Timer::after_millis(1).await;
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
