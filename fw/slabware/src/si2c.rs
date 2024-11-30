// use embassy_sync::waitqueue::AtomicWaker;

use slab_pac::Si2c;

pub fn init(si2c: &Si2c) {
    defmt::debug!("Configuring SI2C clocks");
    let sample_clock_divider = 1; // sample at 10 MHz
    si2c.sampling_clock_divider()
        .write(|w| unsafe { w.bits(sample_clock_divider) });
    defmt::debug!("SI2C sample clock divider set: {}", sample_clock_divider);
    let half_cycle_time = 0;
    si2c.tsu_data()
        .write(|w| unsafe { w.bits(half_cycle_time) });
    defmt::debug!("SI2C tsuData set: {}", half_cycle_time);
}

fn wait_for_start(si2c: &Si2c) {
    si2c.interrupt()
        .modify(|_, w| w.start_enable().set_bit().end_enable().clear_bit());
    while si2c.interrupt().read().bits() == 0 {}
    si2c.interrupt()
        .modify(|_, w| w.start_enable().clear_bit().end_enable().set_bit());
}

fn rx(si2c: &Si2c) -> Result<u8, ()> {
    si2c.rx_data()
        .write(|w| w.valid().clear_bit_by_one().listen().set_bit());
    loop {
        let rx = si2c.rx_data().read();
        if rx.valid().bit_is_set() {
            si2c.rx_data()
                .write(|w| w.valid().clear_bit_by_one().listen().clear_bit());
            return Ok(rx.value().bits());
        } else if si2c.interrupt().read().end_flag().bit_is_set() {
            return Err(());
        }
    }
}

fn ack(si2c: &Si2c) {
    si2c.tx_ack().write(|w| {
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
    while si2c.tx_ack().read().valid().bit_is_set() {}
}

fn tx(si2c: &Si2c, data: u8) -> bool {
    si2c.tx_data().write(|w| unsafe {
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
    si2c.tx_ack().write(|w| {
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
    si2c.rx_ack().write(|w| w.listen().set_bit());
    while si2c.tx_ack().read().valid().bit_is_set() {}
    let response = si2c.rx_ack().read().value().bit_is_clear();
    si2c.rx_ack().write(|w| w.listen().clear_bit());
    response
}

const EDID_DATA: &[u8] = include_bytes!("SoundSlab.edid");

pub fn ddc_test(si2c: &Si2c) {
    let mut offset: u8 = 0;
    loop {
        wait_for_start(si2c);
        if let Ok(addr) = rx(si2c) {
            match addr {
                0xA0 => {
                    ack(si2c);
                    if let Ok(new_offset) = rx(si2c) {
                        ack(si2c);
                        offset = new_offset;
                        defmt::debug!("EDID offset set: {:#02X}", offset);
                    } else {
                        break;
                    }
                }
                0xA1 => {
                    ack(si2c);
                    let mut count: usize = 0;
                    loop {
                        let ack = tx(si2c, EDID_DATA[offset as usize]);
                        offset = offset.wrapping_add(1);
                        count += 1;
                        if !ack {
                            defmt::debug!("EDID read: {} byte(s)", count);
                            break;
                        }
                    }
                }
                _ => {
                    defmt::println!("EDID ignored address: {}", addr);
                }
            }
        }
    }
}

pub fn si2c_test(si2c: &Si2c) {
    loop {
        defmt::println!("Waiting for SI2C start");

        si2c.interrupt()
            .modify(|_, w| w.start_enable().set_bit().end_enable().clear_bit());
        while si2c.interrupt().read().bits() == 0 {}
        si2c.interrupt()
            .modify(|_, w| w.start_enable().clear_bit().end_enable().set_bit());
        si2c.rx_data()
            .write(|w| w.valid().clear_bit_by_one().listen().set_bit());
        si2c.tx_ack().write(|w| {
            w.value()
                .clear_bit()
                .valid()
                .set_bit()
                .enable()
                .set_bit()
                .repeat()
                .set_bit()
                .disable_on_data_conflict()
                .clear_bit()
        });

        let mut data = [0; 11];
        let mut len = 0;

        for i in 0..11 {
            if let Some(rx_byte) = loop {
                let rx = si2c.rx_data().read();
                if rx.valid().bit_is_set() {
                    si2c.rx_data()
                        .write(|w| w.valid().clear_bit_by_one().listen().set_bit());

                    break Some(rx.value().bits());
                } else if si2c.interrupt().read().end_flag().bit_is_set() {
                    break None;
                }
            } {
                data[i] = rx_byte;
                len += 1;
            } else {
                break;
            }
        }

        si2c.rx_data()
            .write(|w| w.valid().clear_bit_by_one().listen().clear_bit());

        defmt::println!("Received: {0:#02X}", data[0..len]);
    }
}

// * Slave initialisation :
// * 1) Configure samplingClockDivider/timeout/tsuDat at x28/x2C/x30
// *
// * Slave write without hardware address filtering :
// * 1) Wait the start interrupt, clear start flag
// * 2) Disable the rxAck register (! real time !)
// * 3) Wait the rxData interrupt and check the rxData value.
// *    - Address KO, put the rxAck in an NACK repeat mode and disable the rxData listen, then return to 1)
// *    - Address OK, set the rxAck to ACK
// * 4) wait for rxData interrupt, then set rxAck to NACK to end the transfer
// pub const SI2C_INTERRUPT_CODE: usize = 21;

// fn enable_si2c_interrupts() {
//     let bits: usize = 1 << SI2C_INTERRUPT_CODE;
//     unsafe {
//         core::arch::asm!(concat!("csrrs x0, ", stringify!(0x304), ", {0}"), in(reg) bits);
//     }
// }

// fn disable_si2c_interrupts() {
//     let bits: usize = 1 << SI2C_INTERRUPT_CODE;
//     unsafe {
//         core::arch::asm!(concat!("csrrc x0, ", stringify!(0x304), ", {0}"), in(reg) bits);
//     }
// }

// trait SealedInstance {
//     fn waker() -> &'static AtomicWaker;
// }
// impl SealedInstance for Si2c {
//     fn waker() -> &'static AtomicWaker {
//         static SI2C_WAKER: AtomicWaker = AtomicWaker::new();
//         &SI2C_WAKER
//     }
// }

// pub fn handle_si2c_interrupt() {
//     Si2c::waker().wake();

//     let regs = unsafe { Si2c::steal() };
//     critical_section::with(|_| {
//         regs.interrupt().modify(|_, w| w);
//     })
// }
