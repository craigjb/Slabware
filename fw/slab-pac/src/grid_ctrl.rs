#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
}
impl RegisterBlock {
    #[doc = "0x00 - SlabGrid control"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
}
#[doc = "Control (rw) register accessor: SlabGrid control\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "SlabGrid control"]
pub mod control;
