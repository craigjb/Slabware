use core::sync::atomic::{AtomicBool, Ordering};

use crate::usb_driver::Driver;
use embassy_futures::join::join3;
use embassy_time::Timer;
use embassy_usb::{class::hid, control::OutResponse, Handler};
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

const KEYBOARD_POLL_MS: u64 = 40;
static KEYBOARD_READY: AtomicBool = AtomicBool::new(false);

pub async fn run_usb_keys(usb_driver: Driver<'_>) {
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("craigjb.com");
    config.product = Some("SoundSlab");
    config.serial_number = Some("00000001");
    config.max_packet_size_0 = 64;

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut state = hid::State::new();

    let mut device_handler = DeviceHandler {};
    let mut request_handler = HidRequestHandler {};

    let mut builder = embassy_usb::Builder::new(
        usb_driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );
    builder.handler(&mut device_handler);

    let hid_config = hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: KEYBOARD_POLL_MS as u8,
        max_packet_size: 8,
    };

    let hid = hid::HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut state, hid_config);
    let mut usb = builder.build();
    let usb_fut = usb.run();

    let (hid_reader, mut hid_writer) = hid.split();

    let in_fut = async {
        let msg: [u8; 16] = [
            0x16, 0x12, 0x18, 0x11, 0x07, 0x16, 0x0f, 0x04, 0x05, 0x2c, 0x15, 0x18, 0x0f, 0x08,
            0x16, 0x28,
        ];
        let modifiers: [u8; 16] = [0x02, 0, 0, 0, 0, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        loop {
            for (code, modifier) in msg.iter().zip(modifiers.iter()) {
                Timer::after_millis(500).await;
                if !KEYBOARD_READY.load(Ordering::Relaxed) {
                    continue;
                }
                defmt::info!("Writing keyboard report");
                let report = KeyboardReport {
                    keycodes: [*code, 0, 0, 0, 0, 0],
                    leds: 0,
                    modifier: *modifier,
                    reserved: 0,
                };
                match hid_writer.write_serialize(&report).await {
                    Ok(()) => {}
                    Err(e) => defmt::warn!("[USB Keys] failed to send report: {:?}", e),
                };
                let report = KeyboardReport {
                    keycodes: [0, 0, 0, 0, 0, 0],
                    leds: 0,
                    modifier: 0,
                    reserved: 0,
                };
                match hid_writer.write_serialize(&report).await {
                    Ok(()) => {}
                    Err(e) => defmt::warn!("[USB Keys] failed to send report: {:?}", e),
                };
            }
        }
    };

    let out_fut = async {
        hid_reader.run(false, &mut request_handler).await;
    };

    join3(usb_fut, in_fut, out_fut).await;
}

struct DeviceHandler {}

impl Handler for DeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        KEYBOARD_READY.store(false, Ordering::Relaxed);
        if enabled {
            defmt::info!("[USB Keys] device enabled");
        } else {
            defmt::info!("[USB Keys] device disabled");
        }
    }

    fn reset(&mut self) {
        KEYBOARD_READY.store(false, Ordering::Relaxed);
        defmt::info!("[USB Keys] bus reset");
    }

    fn addressed(&mut self, _addr: u8) {
        KEYBOARD_READY.store(false, Ordering::Relaxed);
        defmt::info!("[USB Keys] address set");
    }

    fn configured(&mut self, configured: bool) {
        KEYBOARD_READY.store(configured, Ordering::Relaxed);
        if configured {
            defmt::info!("[USB Keys] device configured");
        } else {
            defmt::info!("[USB Keys] device not configured");
        }
    }
}

struct HidRequestHandler {}

impl hid::RequestHandler for HidRequestHandler {
    fn get_report(&mut self, id: hid::ReportId, _buf: &mut [u8]) -> Option<usize> {
        defmt::debug!("[USB Keys] Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: hid::ReportId, data: &[u8]) -> OutResponse {
        defmt::debug!("[USB Keys] Set report to {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, id: Option<hid::ReportId>, dur: u32) {
        defmt::debug!("[USB Keys] Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&mut self, id: Option<hid::ReportId>) -> Option<u32> {
        defmt::debug!("[USB Keys] Get idle rate for {:?}", id);
        None
    }
}
