#![no_std]
#![no_main]

use core::arch::asm;
use defmt_rtt as _;
use panic_halt as _;
use riscv_rt::entry;

fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe {
            asm!("nop");
        }
    }
}

fn set_leds(mask: u32) {
    unsafe {
        *(0xf0000000 as *mut u32) = mask;
    }
}

#[entry]
fn main() -> ! {
    defmt::println!("Hello from RISC-V!");
    let mut mask = 0x80;
    loop {
        set_leds(mask);
        mask >>= 1;
        if mask == 0 {
            mask = 0x80;
        }
        delay(600000);
        defmt::println!("Hello from RISC-V!");
    }
}
