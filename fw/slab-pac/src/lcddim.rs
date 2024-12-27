#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    period: Period,
    duty: Duty,
}
impl RegisterBlock {
    #[doc = "0x00 - PWM control"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - PWM period"]
    #[inline(always)]
    pub const fn period(&self) -> &Period {
        &self.period
    }
    #[doc = "0x08 - PWM duty"]
    #[inline(always)]
    pub const fn duty(&self) -> &Duty {
        &self.duty
    }
}
#[doc = "Control (rw) register accessor: PWM control\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "PWM control"]
pub mod control;
#[doc = "Period (rw) register accessor: PWM period\n\nYou can [`read`](crate::Reg::read) this register and get [`period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period`]
module"]
pub type Period = crate::Reg<period::PeriodSpec>;
#[doc = "PWM period"]
pub mod period;
#[doc = "Duty (rw) register accessor: PWM duty\n\nYou can [`read`](crate::Reg::read) this register and get [`duty::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duty::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@duty`]
module"]
pub type Duty = crate::Reg<duty::DutySpec>;
#[doc = "PWM duty"]
pub mod duty;
