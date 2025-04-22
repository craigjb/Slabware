use core::sync::atomic::{AtomicBool, Ordering};

use crate::usb_driver::Driver;
use embassy_futures::join::join3;
use embassy_time::Timer;
use embassy_usb::{class::hid, control::OutResponse, Handler};
use slab_pac::GridCtrl;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

const KEYBOARD_POLL_MS: u64 = 25;
static KEYBOARD_READY: AtomicBool = AtomicBool::new(false);

// organized by column, then row from bottom to top
const SCAN_CODE_MAP: [[u8; 8]; 18] = [
    [95, 94, 93, 92, 91, 90, 89, 98],
    [96, 117, 118, 123, 124, 125, 126, 120],
    [97, 121, 116, 53, 43, 41, 225, 224],
    [146, 104, 58, 30, 20, 4, 29, 227],
    [103, 105, 59, 31, 26, 22, 27, 226],
    [127, 106, 60, 32, 8, 7, 6, 135],
    [129, 107, 61, 33, 21, 9, 25, 136],
    [128, 108, 62, 34, 23, 10, 5, 44],
    [148, 109, 63, 35, 28, 11, 17, 140],
    [147, 110, 64, 36, 24, 13, 16, 137],
    [122, 111, 65, 37, 12, 14, 54, 138],
    [83, 112, 66, 38, 18, 15, 55, 230],
    [72, 113, 67, 39, 19, 51, 56, 231],
    [119, 114, 68, 45, 47, 52, 229, 228],
    [71, 115, 69, 46, 48, 40, 145, 80],
    [74, 77, 139, 42, 50, 88, 82, 81],
    [75, 78, 144, 76, 101, 100, 70, 79],
    [182, 183, 84, 85, 86, 87, 99, 133],
];

// Reads each column of keys, 1 byte per column.
// Byte 0 (col 0) is farthest left.
// Bit 0 of each byte == row 0.
fn scan_grid(grid_ctrl: &GridCtrl) -> [u8; 18] {
    let column_base_ptr = grid_ctrl.col0key_state().as_ptr();
    let mut values = [0; 18];
    for (col, value) in values.iter_mut().enumerate() {
        unsafe {
            *value = column_base_ptr.add(col).read_volatile() as u8;
        }
    }
    values
}

#[derive(Clone, PartialEq)]
struct ScanReport {
    key_codes: [u8; 6],
    modifiers: u8,
}

impl ScanReport {
    fn new() -> Self {
        Self {
            key_codes: [0; 6],
            modifiers: 0,
        }
    }
}

fn report_scan_codes(scan: &[u8; 18]) -> ScanReport {
    let mut report = ScanReport::new();
    let mut key_index = 0;
    for (col_scan, col_map) in scan.iter().zip(SCAN_CODE_MAP.iter()) {
        for (bit, scan_code) in col_map.iter().enumerate() {
            if col_scan & (1 << bit) != 0 && key_index <= 6 {
                match scan_code {
                    // left ctrl
                    224 => report.modifiers |= 1 << 0,
                    // left shift
                    225 => report.modifiers |= 1 << 1,
                    // left alt
                    226 => report.modifiers |= 1 << 2,
                    // left meta
                    227 => report.modifiers |= 1 << 3,
                    // right ctrl
                    228 => report.modifiers |= 1 << 4,
                    // right shift
                    229 => report.modifiers |= 1 << 5,
                    // right alt
                    230 => report.modifiers |= 1 << 6,
                    // right meta
                    231 => report.modifiers |= 1 << 7,
                    _ => {
                        // non-modifier key
                        report.key_codes[key_index] = *scan_code;
                        key_index += 1;
                    }
                }
            }
        }
    }
    report
}

pub async fn run_usb_keys(usb_driver: Driver<'_>, grid_ctrl: GridCtrl) {
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
        let mut prev_report = ScanReport::new();
        loop {
            Timer::after_millis(KEYBOARD_POLL_MS).await;
            if !KEYBOARD_READY.load(Ordering::Relaxed) {
                continue;
            }

            let report = report_scan_codes(&scan_grid(&grid_ctrl));
            if report == prev_report {
                defmt::trace!(
                    "[USB Keys] No key state change modifiers={:#02x} keycodes={}",
                    report.modifiers,
                    report.key_codes,
                );
                continue;
            } else {
                prev_report = report.clone();
            }

            defmt::trace!(
                "[USB Keys] No key state change modifiers={:#02x} keycodes={}",
                report.modifiers,
                report.key_codes
            );
            let report = KeyboardReport {
                keycodes: report.key_codes,
                leds: 0,
                modifier: report.modifiers,
                reserved: 0,
            };
            match hid_writer.write_serialize(&report).await {
                Ok(()) => {}
                Err(e) => defmt::warn!("[USB Keys] failed to send report: {:?}", e),
            };
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
