mod hdmi;
mod tmds181;

pub use hdmi::{handle_hdmi_interrupt, HDMI_RX_INTERRUPT_CODE};

use crate::mi2c::I2cMaster;
use defmt::unwrap;
use embassy_time::Duration;
use hdmi::Hdmi;
use slab_pac::{GridCtrl, HdmiRx, LcdDim};

struct Video {
    hdmi_rx: Hdmi,
    lcd_dim: LcdDim,
}

impl Video {
    fn new(hdmi_peripheral: HdmiRx, lcd_dim: LcdDim) -> Self {
        let hdmi_rx = hdmi::Hdmi::new(hdmi_peripheral);
        defmt::info!("[HDMI] Asserting HPD");
        hdmi_rx.enable_hpd();

        Self { hdmi_rx, lcd_dim }
    }

    fn enable_output(&self) {
        self.lcd_dim.control().write(|w| w.enable().set_bit());
        unsafe {
            let grid_ctrl = GridCtrl::steal();
            grid_ctrl.control().write(|w| w.enable().set_bit());
        }
    }

    fn disable_output(&self) {
        self.lcd_dim.control().write(|w| w.enable().clear_bit());
        unsafe {
            let grid_ctrl = GridCtrl::steal();
            grid_ctrl.control().write(|w| w.enable().clear_bit());
        }
    }

    async fn task_loop(&self) {
        loop {
            self.disable_output();
            self.hdmi_rx.reset_transceivers();

            self.hdmi_rx.wait_for_cable_status(true).await;
            defmt::info!("[HDMI] Cable plugged in");

            match self.hdmi_rx.detect_clock().await {
                Ok(clock_hz) => {
                    defmt::info!("[HDMI] Clock detected: {} Hz", clock_hz);
                }
                Err(err) => {
                    err.warn();
                    continue;
                }
            }

            self.hdmi_rx.reset_pll();
            defmt::info!("[HDMI] PLL reset");

            defmt::info!("[HDMI] Waiting for PLL lock");
            match self
                .hdmi_rx
                .wait_for_pll_lock(Duration::from_millis(1))
                .await
            {
                Ok(_) => {
                    defmt::info!("[HDMI] PLL locked")
                }
                Err(err) => {
                    err.warn();
                    continue;
                }
            }

            defmt::info!("[HDMI] Waiting for transceivers to reset");
            match self
                .hdmi_rx
                .wait_for_transceivers_reset(Duration::from_millis(5))
                .await
            {
                Ok(_) => {
                    defmt::info!("[HDMI] Transceivers reset done")
                }
                Err(err) => {
                    err.warn();
                    continue;
                }
            }

            match self
                .hdmi_rx
                .wait_for_valid_data(Duration::from_millis(2))
                .await
            {
                Ok(_) => {
                    defmt::info!("[HDMI] Data valid")
                }
                Err(err) => {
                    err.warn();
                    continue;
                }
            }

            self.enable_output();
            defmt::info!("[HDMI] Output enabled");

            self.hdmi_rx.wait_for_lost_signal().await;
            defmt::warn!("[HDMI] Signal lost")
        }
    }
}

#[embassy_executor::task]
pub async fn video_task(i2cm: I2cMaster, hdmi_peripheral: HdmiRx, lcd_dim: LcdDim) {
    let _retimer = unwrap!(
        tmds181::Tmds181Config::new()
            .address(0x5C)
            .lane_swap(true)
            .polarity_swap(true)
            .redriver_auto_standby(true)
            .build(i2cm)
            .await
    );
    defmt::info!("[HDMI] Retimer configured");

    let video = Video::new(hdmi_peripheral, lcd_dim);
    video.task_loop().await
}
