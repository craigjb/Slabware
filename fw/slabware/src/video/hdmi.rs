use core::future::{poll_fn, Future};
use core::task::Poll;

use crate::custom_int::*;
use crate::sealed_instance::SealedInstance;
use embassy_futures::select::{select, Either};
use embassy_sync::waitqueue::AtomicWaker;
use embassy_time::{Duration, Timer};
use slab_pac::HdmiRx;

pub const HDMI_RX_INTERRUPT_CODE: usize = 21;

pub const CLOCK_DETECT_MILLIS: usize = 5_000;

pub enum HdmiError {
    Timeout,
    CableUnplugged,
    NoClockDetected,
    ClockFreqChanged,
}

impl HdmiError {
    pub fn warn(&self) {
        match self {
            HdmiError::Timeout => defmt::warn!("[HDMI] Timeout"),
            HdmiError::CableUnplugged => defmt::warn!("[HDMI] Cable unplugged"),
            HdmiError::NoClockDetected => defmt::warn!("[HDMI] No clock detected"),
            HdmiError::ClockFreqChanged => defmt::warn!("[HDMI] Clock frequency changed"),
        }
    }
}

pub struct Hdmi {
    regs: HdmiRx,
    waker: &'static AtomicWaker,
    clock_det_divisor: usize,
    clock_det_sample_rate: usize,
}

impl Hdmi {
    pub fn new(hdmi: HdmiRx) -> Self {
        let clock_det_divisor = hdmi.clk_det_divisor().read().value().bits() as usize;
        defmt::debug!("[HDMI] clk det divisor: {}", clock_det_divisor);

        let clock_det_sample_rate = hdmi.clk_det_sample_rate().read().value().bits() as usize;
        defmt::debug!("[HDMI] clk det sample rate: {}", clock_det_sample_rate);

        defmt::debug!("[HDMI] Enabling interrupts");
        enable_custom_interrupt(HDMI_RX_INTERRUPT_CODE);

        Self {
            regs: hdmi,
            waker: HdmiRx::waker(),
            clock_det_divisor,
            clock_det_sample_rate,
        }
    }

    pub fn enable_hpd(&self) {
        self.regs.control().modify(|_, w| w.hpd_enable().set_bit());
    }

    fn disable_all_interrupts(&self) {
        unsafe {
            self.regs.interrupt_enable().write_with_zero(|w| w);
        }
    }

    pub async fn wait_for_cable_status(&self, plugged: bool) {
        self.regs
            .interrupt_enable()
            .modify(|_, w| w.cable_detect_changed_enable().set_bit());

        poll_fn(|cx| {
            self.waker.register(cx.waker());

            let status = self.regs.status().read().cable_detect().bit();
            self.regs
                .status()
                .write(|w| w.cable_detect_changed().clear_bit_by_one());
            defmt::debug!("[HDMI] cable status: {}", status);
            if status == plugged {
                self.disable_all_interrupts();
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
    }

    pub async fn detect_clock(&self) -> Result<usize, HdmiError> {
        let mut prev_count = 0;
        for _ in 0..CLOCK_DETECT_MILLIS {
            let count = self.regs.clk_det_count().read().value().bits();
            if self.regs.status().read().cable_detect().bit_is_clear() {
                self.regs
                    .status()
                    .write(|w| w.cable_detect_changed().clear_bit_by_one());
                return Err(HdmiError::CableUnplugged);
            } else if count != 0 {
                if prev_count != 0 && count >= prev_count - 1 && count <= prev_count + 1 {
                    let clock =
                        count as usize * self.clock_det_divisor * self.clock_det_sample_rate;
                    defmt::debug!("[HDMI] clock detected ({} Hz)", clock);
                    self.regs
                        .status()
                        .write(|w| w.freq_changed().clear_bit_by_one());
                    return Ok(clock);
                }
                prev_count = count;
            }
            Timer::after_millis(1).await;
        }
        Err(HdmiError::NoClockDetected)
    }

    pub fn reset_pll(&self) {
        self.regs.control().modify(|_, w| {
            w.pll_power_down()
                .clear_bit()
                .pll_reset()
                .set_bit()
                .gtp_reset()
                .set_bit()
        });
    }

    fn poll_status<'a, F>(&'a self, check: F) -> impl Future<Output = Result<(), HdmiError>> + 'a
    where
        F: (Fn(slab_pac::hdmi_rx::status::R) -> bool) + 'a,
    {
        poll_fn(move |cx| {
            self.waker.register(cx.waker());

            let status = self.regs.status().read();
            if status.cable_detect().bit_is_clear() {
                self.regs
                    .status()
                    .write(|w| w.cable_detect_changed().clear_bit_by_one());
                Poll::Ready(Err(HdmiError::CableUnplugged))
            } else if status.freq_changed().bit_is_set() {
                self.regs
                    .status()
                    .write(|w| w.freq_changed().clear_bit_by_one());
                Poll::Ready(Err(HdmiError::ClockFreqChanged))
            } else if check(status) {
                Poll::Ready(Ok(()))
            } else {
                Poll::Pending
            }
        })
    }

    async fn poll_timeout<F>(&self, poll: F, timeout: Duration) -> Result<(), HdmiError>
    where
        F: Future<Output = Result<(), HdmiError>>,
    {
        match select(poll, Timer::after(timeout)).await {
            Either::First(res) => {
                self.disable_all_interrupts();
                res
            }
            Either::Second(_) => {
                self.disable_all_interrupts();
                Err(HdmiError::Timeout)
            }
        }
    }

    pub async fn wait_for_pll_lock(&self, timeout: Duration) -> Result<(), HdmiError> {
        self.regs.interrupt_enable().modify(|_, w| {
            w.cable_detect_changed_enable()
                .set_bit()
                .freq_changed_enable()
                .set_bit()
                .pll_lock_enable()
                .set_bit()
        });
        self.regs.control().modify(|_, w| w.pll_reset().clear_bit());

        let poll = self.poll_status(|status| status.pll_lock().bit_is_set());
        self.poll_timeout(poll, timeout).await
    }

    pub fn reset_transceivers(&self) {
        self.regs.control().modify(|_, w| w.gtp_reset().set_bit());
    }

    pub async fn wait_for_transceivers_reset(&self, timeout: Duration) -> Result<(), HdmiError> {
        self.regs.interrupt_enable().modify(|_, w| {
            w.cable_detect_changed_enable()
                .set_bit()
                .freq_changed_enable()
                .set_bit()
                .all_gtp_resets_done_enable()
                .set_bit()
        });
        self.regs.control().modify(|_, w| w.gtp_reset().clear_bit());

        let poll = self.poll_status(|status| status.all_gtp_resets_done().bit_is_set());
        self.poll_timeout(poll, timeout).await
    }

    pub async fn wait_for_valid_data(&self, timeout: Duration) -> Result<(), HdmiError> {
        self.regs.interrupt_enable().modify(|_, w| {
            w.cable_detect_changed_enable()
                .set_bit()
                .freq_changed_enable()
                .set_bit()
                .all_hdmi_data_valid_enable()
                .set_bit()
        });

        let poll = self.poll_status(|status| status.all_hdmi_data_valid().bit_is_set());
        self.poll_timeout(poll, timeout).await
    }

    pub async fn wait_for_lost_signal(&self) -> HdmiError {
        self.regs.interrupt_enable().modify(|_, w| {
            w.cable_detect_changed_enable()
                .set_bit()
                .freq_changed_enable()
                .set_bit()
        });
        match self.poll_status(|_| false).await {
            Ok(_) => unreachable!(),
            Err(err) => err,
        }
    }
}

impl Drop for Hdmi {
    fn drop(&mut self) {
        disable_custom_interrupt(HDMI_RX_INTERRUPT_CODE);
    }
}

impl SealedInstance for HdmiRx {
    fn waker() -> &'static AtomicWaker {
        static HDMI_RX_WAKER: AtomicWaker = AtomicWaker::new();
        &HDMI_RX_WAKER
    }
}
pub fn handle_hdmi_interrupt() {
    HdmiRx::waker().wake();

    let regs = unsafe { HdmiRx::steal() };
    critical_section::with(|_| unsafe { regs.interrupt_enable().write_with_zero(|w| w) })
}
