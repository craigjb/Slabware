#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;
use riscv::asm::delay;
use riscv_rt::entry;

fn set_leds(mask: u32) {
    unsafe {
        *(0xf0000000 as *mut u32) = mask;
    }
}

#[entry]
fn main() -> ! {
    defmt::println!("print from VexRiscv!");
    defmt::info!("info from VexRiscv!");
    defmt::warn!("warn from VexRiscv!");
    defmt::error!("error from VexRiscv!");
    defmt::debug!("debug from VexRiscv!");
    defmt::trace!("trace from VexRiscv!");

    let mut mask = 0x80;
    loop {
        set_leds(mask);
        mask >>= 1;
        if mask == 0 {
            mask = 0x80;
        }
        delay(1000000);
    }
}
