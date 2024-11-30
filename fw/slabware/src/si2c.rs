use core::future::poll_fn;
use core::task::Poll;

use crate::custom_int::*;
use crate::i2c::AckKind;
use crate::sealed_instance::SealedInstance;
use embassy_sync::waitqueue::AtomicWaker;
use slab_pac::Si2c;

pub const SI2C_INTERRUPT_CODE: usize = 21;

pub enum I2cSlaveError {
    FrameEnded,
}

pub struct I2cSlave {
    regs: Si2c,
    waker: &'static AtomicWaker,
}

impl I2cSlave {
    pub fn new(i2c: Si2c, address: u8) -> I2cSlave {
        defmt::debug!("Configuring SI2C clocks");
        let sample_clock_divider = 1;
        i2c.sampling_clock_divider()
            .write(|w| unsafe { w.bits(sample_clock_divider) });
        defmt::debug!("SI2C sample clock divider set: {}", sample_clock_divider);
        let tsu_data_time = 0;
        i2c.tsu_data().write(|w| unsafe { w.bits(tsu_data_time) });
        defmt::debug!("SI2C tsuData set: {}", tsu_data_time);

        i2c.address_filter0()
            .write(|w| unsafe { w.enable().set_bit().address().bits(address) });
        defmt::debug!("SI2C address filter set: {:#02x}", address);

        defmt::debug!("Enabling SI2C interrupts");
        enable_custom_interrupt(SI2C_INTERRUPT_CODE);

        Self {
            regs: i2c,
            waker: Si2c::waker(),
        }
    }

    pub async fn wait_for_start(&self) {
        self.regs
            .interrupt()
            .modify(|_, w| w.start_enable().set_bit());

        poll_fn(|cx| {
            self.waker.register(cx.waker());

            let status = self.regs.slave_status().read();
            if status.in_frame().bit_is_set() {
                self.regs
                    .interrupt()
                    .modify(|_, w| w.start_enable().clear_bit());
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
    }

    pub async fn read(&self, ack: AckKind) -> Result<u8, I2cSlaveError> {
        self.regs.rx_data().modify(|_, w| w.listen().set_bit());
        self.regs
            .interrupt()
            .modify(|_, w| w.rx_data_enable().set_bit().end_enable().set_bit());

        self.regs.tx_ack().write(|w| {
            w.value()
                .bit(ack.value())
                .valid()
                .set_bit()
                .enable()
                .set_bit()
                .repeat()
                .clear_bit()
                .disable_on_data_conflict()
                .clear_bit()
        });

        poll_fn(|cx| {
            self.waker.register(cx.waker());

            let rx = self.regs.rx_data().read();
            if rx.valid().bit_is_set() {
                self.regs.rx_data().write(|w| w.listen().clear_bit());
                self.regs
                    .interrupt()
                    .modify(|_, w| w.rx_data_enable().clear_bit().end_enable().clear_bit());
                Poll::Ready(Ok(rx.value().bits()))
            } else if self.regs.slave_status().read().in_frame().bit_is_clear() {
                self.regs.rx_data().write(|w| w.listen().clear_bit());
                self.regs
                    .interrupt()
                    .modify(|_, w| w.rx_data_enable().clear_bit().end_enable().clear_bit());
                Poll::Ready(Err(I2cSlaveError::FrameEnded))
            } else {
                Poll::Pending
            }
        })
        .await
    }

    pub async fn write(&self, data: u8) -> AckKind {
        self.regs.tx_data().write(|w| unsafe {
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
        self.regs.tx_ack().write(|w| {
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
        self.regs.rx_ack().write(|w| w.listen().set_bit());
        self.regs
            .interrupt()
            .modify(|_, w| w.tx_ack_enable().set_bit());

        poll_fn(|cx| {
            self.waker.register(cx.waker());

            let tx_ack = self.regs.tx_ack().read();
            if tx_ack.valid().bit_is_clear() {
                let response = self.regs.rx_ack().read().value().bit();
                self.regs.rx_ack().write(|w| w.listen().clear_bit());
                self.regs
                    .interrupt()
                    .modify(|_, w| w.tx_ack_enable().clear_bit());
                Poll::Ready(AckKind::from_value(response))
            } else {
                Poll::Pending
            }
        })
        .await
    }
}

impl Drop for I2cSlave {
    fn drop(&mut self) {
        disable_custom_interrupt(SI2C_INTERRUPT_CODE);
    }
}

impl SealedInstance for Si2c {
    fn waker() -> &'static AtomicWaker {
        static SI2C_WAKER: AtomicWaker = AtomicWaker::new();
        &SI2C_WAKER
    }
}

pub fn handle_si2c_interrupt() {
    Si2c::waker().wake();

    let regs = unsafe { Si2c::steal() };
    critical_section::with(|_| {
        regs.interrupt().modify(|_, w| {
            w.start_enable()
                .clear_bit()
                .end_enable()
                .clear_bit()
                .rx_data_enable()
                .clear_bit()
                .tx_ack_enable()
                .clear_bit()
                .rx_ack_enable()
                .clear_bit()
        });
    })
}
