#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;
use riscv::asm::delay;
use riscv_rt::entry;
use slab_pac::I2c;

fn i2c_start_blocking(i2c: &I2c) {
    i2c.master_status().modify(|_, w| w.start().set_bit());
    while i2c.master_status().read().start().bit_is_set() {}
}

fn i2c_stop_blocking(i2c: &I2c) {
    i2c.master_status().modify(|_, w| w.stop().set_bit());
    while i2c.master_status().read().is_busy().bit_is_set() {}
}

fn i2c_tx(i2c: &I2c, data: u8) {
    i2c.tx_data().write(|w| unsafe {
        w.valid()
            .set_bit()
            .enable()
            .set_bit()
            .disable_on_data_conflict()
            .clear_bit()
            .value()
            .bits(data)
            .repeat()
            .clear_bit()
    });
}

fn i2c_tx_nack_blocking(i2c: &I2c) {
    i2c.tx_ack().write(|w| {
        w.value()
            .set_bit()
            .valid()
            .set_bit()
            .enable()
            .set_bit()
            .repeat()
            .clear_bit()
            .disable_on_data_conflict()
            .clear_bit()
    });
    while i2c.tx_ack().read().valid().bit_is_set() {}
}

fn i2c_tx_ack_blocking(i2c: &I2c) {
    i2c.tx_ack().write(|w| {
        w.value()
            .clear_bit()
            .valid()
            .set_bit()
            .enable()
            .set_bit()
            .repeat()
            .clear_bit()
            .disable_on_data_conflict()
            .clear_bit()
    });
    while i2c.tx_ack().read().valid().bit_is_set() {}
}

fn i2c_rx_ack(i2c: &I2c) -> bool {
    i2c.rx_ack().read().value().bit_is_clear()
}

fn i2c_rx(i2c: &I2c) -> u8 {
    i2c.rx_data().read().value().bits()
}

#[entry]
fn main() -> ! {
    let peripherals = slab_pac::Peripherals::take().unwrap();
    let i2c = peripherals.i2c;

    defmt::println!("Configuring I2C clocks");
    let sample_clock_divider = 10; // sample at 10 MHz
    i2c.sampling_clock_divider()
        .write(|w| unsafe { w.bits(sample_clock_divider) });
    defmt::println!("Sample clock divider set: {}", sample_clock_divider);
    let timeout = 0xFFFFF; // timeout after ~10 ms
    i2c.timeout().write(|w| unsafe { w.bits(timeout) });
    defmt::println!("Timeout set: {}", timeout);
    let half_cycle_time = 43;
    i2c.t_high().write(|w| unsafe { w.bits(half_cycle_time) });
    i2c.t_low().write(|w| unsafe { w.bits(half_cycle_time) });
    i2c.t_buf().write(|w| unsafe { w.bits(half_cycle_time) });
    defmt::println!("tHi, tLow, tBuf set: {}", half_cycle_time);

    i2c_start_blocking(&i2c);
    let address = 0xB8;
    i2c_tx(&i2c, address);
    i2c_tx_nack_blocking(&i2c);
    let data = 0x0;
    i2c_tx(&i2c, data);
    i2c_tx_nack_blocking(&i2c);
    defmt::println!("Sent write to {:X}", address);
    defmt::println!("Sent data {:X}", data);

    let mut data: [u8; 8] = [0; 8];
    i2c_start_blocking(&i2c);
    let address = 0xB9;
    i2c_tx(&i2c, address);
    i2c_tx_nack_blocking(&i2c);
    for i in 0..7 {
        i2c_tx(&i2c, 0xFF);
        i2c_tx_ack_blocking(&i2c);
        data[i] = i2c_rx(&i2c);
    }
    i2c_tx(&i2c, 0xFF);
    i2c_tx_nack_blocking(&i2c);
    data[7] = i2c_rx(&i2c);
    i2c_stop_blocking(&i2c);
    defmt::println!("Sent read to {:X}", address);
    defmt::println!("Read: {:X}", data);

    let mut mask = 0x80;
    loop {
        peripherals.leds.data().write(|w| unsafe { w.bits(mask) });
        mask >>= 1;
        if mask == 0 {
            mask = 0x80;
        }
        delay(600000);
    }
}
