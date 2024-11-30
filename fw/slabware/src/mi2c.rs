use core::future::poll_fn;
use core::task::Poll;

use crate::custom_int::*;
use crate::i2c::AckKind;
use crate::sealed_instance::SealedInstance;
use embassy_sync::waitqueue::AtomicWaker;
pub use embedded_hal_async::i2c::I2c;
use embedded_hal_async::i2c::{ErrorKind, ErrorType, Operation, SevenBitAddress};
use slab_pac::Mi2c;

pub const MI2C_INTERRUPT_CODE: usize = 20;

pub struct I2cMaster {
    regs: Mi2c,
    waker: &'static AtomicWaker,
}

impl I2cMaster {
    pub fn new(i2c: Mi2c) -> I2cMaster {
        defmt::debug!("Configuring MI2C clocks");
        let sample_clock_divider = 10; // sample at 10 MHz
        i2c.sampling_clock_divider()
            .write(|w| unsafe { w.bits(sample_clock_divider) });
        defmt::debug!("MI2C sample clock divider set: {}", sample_clock_divider);
        let half_cycle_time = 43;
        i2c.thigh().write(|w| unsafe { w.bits(half_cycle_time) });
        i2c.tlow().write(|w| unsafe { w.bits(half_cycle_time) });
        i2c.tbuf().write(|w| unsafe { w.bits(half_cycle_time) });
        i2c.tsu_data().write(|w| unsafe { w.bits(half_cycle_time) });
        defmt::debug!("MI2C tHi, tLow, tBuf, tsuData set: {}", half_cycle_time);

        defmt::debug!("Enabling MI2C interrupts");
        enable_custom_interrupt(MI2C_INTERRUPT_CODE);

        Self {
            regs: i2c,
            waker: Mi2c::waker(),
        }
    }

    async fn master_start(&mut self) {
        self.regs
            .interrupt()
            .modify(|_, w| w.start_enable().set_bit().restart_enable().set_bit());
        self.regs.master_status().modify(|_, w| w.start().set_bit());

        poll_fn(|cx| {
            self.waker.register(cx.waker());

            let status = self.regs.master_status().read();
            if status.start().bit_is_clear() {
                self.regs
                    .interrupt()
                    .modify(|_, w| w.start_enable().clear_bit().restart_enable().clear_bit());
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
    }

    async fn master_stop(&mut self) {
        self.regs
            .interrupt()
            .modify(|_, w| w.end_enable().set_bit());
        self.regs.master_status().modify(|_, w| w.stop().set_bit());

        poll_fn(|cx| {
            self.waker.register(cx.waker());

            let status = self.regs.master_status().read();
            if status.stop().bit_is_clear() {
                self.regs
                    .interrupt()
                    .modify(|_, w| w.end_enable().clear_bit());
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
    }

    async fn master_write(&mut self, data: u8, ack: AckKind) {
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
        self.regs
            .interrupt()
            .modify(|_, w| w.tx_ack_enable().set_bit());

        poll_fn(|cx| {
            self.waker.register(cx.waker());

            let tx_ack = self.regs.tx_ack().read();
            if tx_ack.valid().bit_is_clear() {
                self.regs
                    .interrupt()
                    .modify(|_, w| w.tx_ack_enable().clear_bit());
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
    }

    fn master_read(&mut self) -> u8 {
        self.regs.rx_data().read().value().bits()
    }
}

impl ErrorType for I2cMaster {
    type Error = ErrorKind;
}

#[derive(Clone, Copy, PartialEq)]
enum LastOperation {
    None,
    Read,
    Write,
}

impl I2c for I2cMaster {
    async fn transaction<'a>(
        &mut self,
        address: SevenBitAddress,
        operations: &mut [Operation<'a>],
    ) -> Result<(), Self::Error> {
        let mut last_op = LastOperation::None;
        let mut op_iter = operations.iter_mut().peekable();
        let write_addr = address << 1;

        while let Some(op) = op_iter.next() {
            match op {
                Operation::Write(data) => {
                    if last_op != LastOperation::Write {
                        self.master_start().await;
                        last_op = LastOperation::Write;
                        self.master_write(write_addr, AckKind::Nack).await;
                    }
                    for byte in data.iter() {
                        self.master_write(*byte, AckKind::Nack).await;
                    }
                }
                Operation::Read(data) => {
                    if last_op != LastOperation::Read {
                        self.master_start().await;
                        last_op = LastOperation::Read;
                        self.master_write(write_addr | 0x1, AckKind::Nack).await;
                    }
                    let mut data_iter = data.iter_mut().peekable();
                    while let Some(byte) = data_iter.next() {
                        let ack = if op_iter.peek().is_none() && data_iter.peek().is_none() {
                            AckKind::Nack
                        } else {
                            AckKind::Ack
                        };
                        self.master_write(0xFF, ack).await;
                        *byte = self.master_read();
                    }
                }
            }
        }
        self.master_stop().await;

        Ok(())
    }
}

impl Drop for I2cMaster {
    fn drop(&mut self) {
        disable_custom_interrupt(MI2C_INTERRUPT_CODE);
    }
}

impl SealedInstance for Mi2c {
    fn waker() -> &'static AtomicWaker {
        static MI2C_WAKER: AtomicWaker = AtomicWaker::new();
        &MI2C_WAKER
    }
}

pub fn handle_mi2c_interrupt() {
    Mi2c::waker().wake();

    let regs = unsafe { Mi2c::steal() };
    critical_section::with(|_| {
        regs.interrupt().modify(|_, w| {
            w.start_enable()
                .clear_bit()
                .restart_enable()
                .clear_bit()
                .end_enable()
                .clear_bit()
                .tx_ack_enable()
                .clear_bit()
        });
    })
}
