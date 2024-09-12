#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;
use riscv::asm::delay;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    loop {
        defmt::println!("print from VexRiscv!");
        delay(1000000);
        defmt::info!("info from VexRiscv!");
        delay(1000000);
        defmt::warn!("warn from VexRiscv!");
        delay(1000000);
        defmt::error!("error from VexRiscv!");
        delay(1000000);
        defmt::debug!("debug from VexRiscv!");
        delay(1000000);
        defmt::trace!("trace from VexRiscv!");
        delay(1000000);
    }
}
