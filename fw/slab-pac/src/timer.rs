#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    prescale: Prescale,
    control: Control,
    counter: Counter,
    interrupt_status: InterruptStatus,
    interrupt_mask: InterruptMask,
    compare0: Compare0,
    compare1: Compare1,
}
impl RegisterBlock {
    #[doc = "0x04 - Prescale"]
    #[inline(always)]
    pub const fn prescale(&self) -> &Prescale {
        &self.prescale
    }
    #[doc = "0x08 - Control"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x0c - Counter"]
    #[inline(always)]
    pub const fn counter(&self) -> &Counter {
        &self.counter
    }
    #[doc = "0x10 - Interrupt status"]
    #[inline(always)]
    pub const fn interrupt_status(&self) -> &InterruptStatus {
        &self.interrupt_status
    }
    #[doc = "0x14 - Interrupt mask"]
    #[inline(always)]
    pub const fn interrupt_mask(&self) -> &InterruptMask {
        &self.interrupt_mask
    }
    #[doc = "0x18 - Compare 0"]
    #[inline(always)]
    pub const fn compare0(&self) -> &Compare0 {
        &self.compare0
    }
    #[doc = "0x1c - Compare 1"]
    #[inline(always)]
    pub const fn compare1(&self) -> &Compare1 {
        &self.compare1
    }
}
#[doc = "prescale (w) register accessor: Prescale\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescale::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prescale`]
module"]
#[doc(alias = "prescale")]
pub type Prescale = crate::Reg<prescale::PrescaleSpec>;
#[doc = "Prescale"]
pub mod prescale;
#[doc = "control (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "control")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Control"]
pub mod control;
#[doc = "counter (rw) register accessor: Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`counter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter`]
module"]
#[doc(alias = "counter")]
pub type Counter = crate::Reg<counter::CounterSpec>;
#[doc = "Counter"]
pub mod counter;
#[doc = "interruptStatus (rw) register accessor: Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_status`]
module"]
#[doc(alias = "interruptStatus")]
pub type InterruptStatus = crate::Reg<interrupt_status::InterruptStatusSpec>;
#[doc = "Interrupt status"]
pub mod interrupt_status;
#[doc = "interruptMask (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_mask`]
module"]
#[doc(alias = "interruptMask")]
pub type InterruptMask = crate::Reg<interrupt_mask::InterruptMaskSpec>;
#[doc = "Interrupt mask"]
pub mod interrupt_mask;
#[doc = "compare0 (rw) register accessor: Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`compare0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compare0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compare0`]
module"]
#[doc(alias = "compare0")]
pub type Compare0 = crate::Reg<compare0::Compare0Spec>;
#[doc = "Compare 0"]
pub mod compare0;
#[doc = "compare1 (rw) register accessor: Compare 1\n\nYou can [`read`](crate::Reg::read) this register and get [`compare1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compare1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compare1`]
module"]
#[doc(alias = "compare1")]
pub type Compare1 = crate::Reg<compare1::Compare1Spec>;
#[doc = "Compare 1"]
pub mod compare1;
