#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0xff00],
    frame: Frame,
    address: Address,
    interrupt: Interrupt,
    halt: Halt,
    config: Config,
    _reserved5: [u8; 0x0c],
    info_reg: InfoReg,
}
impl RegisterBlock {
    #[doc = "0xff00 - USB frame id"]
    #[inline(always)]
    pub const fn frame(&self) -> &Frame {
        &self.frame
    }
    #[doc = "0xff04 - USB address"]
    #[inline(always)]
    pub const fn address(&self) -> &Address {
        &self.address
    }
    #[doc = "0xff08 - Interrupt status"]
    #[inline(always)]
    pub const fn interrupt(&self) -> &Interrupt {
        &self.interrupt
    }
    #[doc = "0xff0c - Halt endpoint"]
    #[inline(always)]
    pub const fn halt(&self) -> &Halt {
        &self.halt
    }
    #[doc = "0xff10 - Configuration"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0xff20 - Info"]
    #[inline(always)]
    pub const fn info_reg(&self) -> &InfoReg {
        &self.info_reg
    }
}
#[doc = "Frame (r) register accessor: USB frame id\n\nYou can [`read`](crate::Reg::read) this register and get [`frame::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame`]
module"]
pub type Frame = crate::Reg<frame::FrameSpec>;
#[doc = "USB frame id"]
pub mod frame;
#[doc = "Address (rw) register accessor: USB address\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`]
module"]
pub type Address = crate::Reg<address::AddressSpec>;
#[doc = "USB address"]
pub mod address;
#[doc = "Interrupt (rw) register accessor: Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt`]
module"]
pub type Interrupt = crate::Reg<interrupt::InterruptSpec>;
#[doc = "Interrupt status"]
pub mod interrupt;
#[doc = "Halt (rw) register accessor: Halt endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`halt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`halt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@halt`]
module"]
pub type Halt = crate::Reg<halt::HaltSpec>;
#[doc = "Halt endpoint"]
pub mod halt;
#[doc = "Config (rw) register accessor: Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration"]
pub mod config;
#[doc = "infoReg (r) register accessor: Info\n\nYou can [`read`](crate::Reg::read) this register and get [`info_reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info_reg`]
module"]
#[doc(alias = "infoReg")]
pub type InfoReg = crate::Reg<info_reg::InfoRegSpec>;
#[doc = "Info"]
pub mod info_reg;
