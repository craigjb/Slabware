#[doc = "Register `channel1` reader"]
pub type R = crate::R<Channel1Spec>;
#[doc = "Field `gtpResetDone` reader - GTP transceiver reset is done\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type GtpResetDoneR = crate::BitReader;
#[doc = "Field `hdmiDataOut0Valid` reader - HDMI data out 0 is valid\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type HdmiDataOut0validR = crate::BitReader;
#[doc = "Field `hdmiDataOut1Valid` reader - HDMI data out 1 is valid\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type HdmiDataOut1validR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - GTP transceiver reset is done"]
    #[inline(always)]
    pub fn gtp_reset_done(&self) -> GtpResetDoneR {
        GtpResetDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HDMI data out 0 is valid"]
    #[inline(always)]
    pub fn hdmi_data_out0valid(&self) -> HdmiDataOut0validR {
        HdmiDataOut0validR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HDMI data out 1 is valid"]
    #[inline(always)]
    pub fn hdmi_data_out1valid(&self) -> HdmiDataOut1validR {
        HdmiDataOut1validR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Channel 1 status\n\nYou can [`read`](crate::Reg::read) this register and get [`channel1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Channel1Spec;
impl crate::RegisterSpec for Channel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel1::R`](R) reader structure"]
impl crate::Readable for Channel1Spec {}
#[doc = "`reset()` method sets channel1 to value 0"]
impl crate::Resettable for Channel1Spec {
    const RESET_VALUE: u32 = 0;
}
