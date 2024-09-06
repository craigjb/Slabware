#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;
use riscv::asm::delay;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = slab_pac::Peripherals::take().unwrap();

    defmt::println!("Hello from RISC-V!");
    let mut mask = 0x80;
    loop {
        peripherals.leds.data().write(|w| unsafe { w.bits(mask) });
        mask >>= 1;
        if mask == 0 {
            mask = 0x80;
        }
        delay(600000);
        defmt::println!("Hello from RISC-V!");
    }
}
