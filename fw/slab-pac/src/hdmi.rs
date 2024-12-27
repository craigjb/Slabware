#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    status: Status,
    clk_det_divisor: ClkDetDivisor,
    clk_det_sample_rate: ClkDetSampleRate,
    clk_det_count: ClkDetCount,
    clk_det_tolerance: ClkDetTolerance,
    channel0: Channel0,
    channel1: Channel1,
    channel2: Channel2,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Clock detector divisor"]
    #[inline(always)]
    pub const fn clk_det_divisor(&self) -> &ClkDetDivisor {
        &self.clk_det_divisor
    }
    #[doc = "0x0c - Clock detector sample rate"]
    #[inline(always)]
    pub const fn clk_det_sample_rate(&self) -> &ClkDetSampleRate {
        &self.clk_det_sample_rate
    }
    #[doc = "0x10 - Clock detector frequency count"]
    #[inline(always)]
    pub const fn clk_det_count(&self) -> &ClkDetCount {
        &self.clk_det_count
    }
    #[doc = "0x14 - Clock detector tolerance"]
    #[inline(always)]
    pub const fn clk_det_tolerance(&self) -> &ClkDetTolerance {
        &self.clk_det_tolerance
    }
    #[doc = "0x18 - Channel 0 status"]
    #[inline(always)]
    pub const fn channel0(&self) -> &Channel0 {
        &self.channel0
    }
    #[doc = "0x1c - Channel 1 status"]
    #[inline(always)]
    pub const fn channel1(&self) -> &Channel1 {
        &self.channel1
    }
    #[doc = "0x20 - Channel 2 status"]
    #[inline(always)]
    pub const fn channel2(&self) -> &Channel2 {
        &self.channel2
    }
}
#[doc = "control (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "control")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Control"]
pub mod control;
#[doc = "status (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "clkDetDivisor (r) register accessor: Clock detector divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_det_divisor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_det_divisor`]
module"]
#[doc(alias = "clkDetDivisor")]
pub type ClkDetDivisor = crate::Reg<clk_det_divisor::ClkDetDivisorSpec>;
#[doc = "Clock detector divisor"]
pub mod clk_det_divisor;
#[doc = "clkDetSampleRate (r) register accessor: Clock detector sample rate\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_det_sample_rate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_det_sample_rate`]
module"]
#[doc(alias = "clkDetSampleRate")]
pub type ClkDetSampleRate = crate::Reg<clk_det_sample_rate::ClkDetSampleRateSpec>;
#[doc = "Clock detector sample rate"]
pub mod clk_det_sample_rate;
#[doc = "clkDetCount (r) register accessor: Clock detector frequency count\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_det_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_det_count`]
module"]
#[doc(alias = "clkDetCount")]
pub type ClkDetCount = crate::Reg<clk_det_count::ClkDetCountSpec>;
#[doc = "Clock detector frequency count"]
pub mod clk_det_count;
#[doc = "clkDetTolerance (rw) register accessor: Clock detector tolerance\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_det_tolerance::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_det_tolerance::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_det_tolerance`]
module"]
#[doc(alias = "clkDetTolerance")]
pub type ClkDetTolerance = crate::Reg<clk_det_tolerance::ClkDetToleranceSpec>;
#[doc = "Clock detector tolerance"]
pub mod clk_det_tolerance;
#[doc = "channel0 (r) register accessor: Channel 0 status\n\nYou can [`read`](crate::Reg::read) this register and get [`channel0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel0`]
module"]
#[doc(alias = "channel0")]
pub type Channel0 = crate::Reg<channel0::Channel0Spec>;
#[doc = "Channel 0 status"]
pub mod channel0;
#[doc = "channel1 (r) register accessor: Channel 1 status\n\nYou can [`read`](crate::Reg::read) this register and get [`channel1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel1`]
module"]
#[doc(alias = "channel1")]
pub type Channel1 = crate::Reg<channel1::Channel1Spec>;
#[doc = "Channel 1 status"]
pub mod channel1;
#[doc = "channel2 (r) register accessor: Channel 2 status\n\nYou can [`read`](crate::Reg::read) this register and get [`channel2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel2`]
module"]
#[doc(alias = "channel2")]
pub type Channel2 = crate::Reg<channel2::Channel2Spec>;
#[doc = "Channel 2 status"]
pub mod channel2;
