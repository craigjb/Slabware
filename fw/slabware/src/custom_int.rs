pub fn enable_custom_interrupt(code: usize) {
    let bits: usize = 1 << code;
    unsafe {
        core::arch::asm!(concat!("csrrs x0, ", stringify!(0x304), ", {0}"), in(reg) bits);
    }
}

pub fn disable_custom_interrupt(code: usize) {
    let bits: usize = 1 << code;
    unsafe {
        core::arch::asm!(concat!("csrrc x0, ", stringify!(0x304), ", {0}"), in(reg) bits);
    }
}
