#[doc = "Register `clkDetSampleRate` reader"]
pub type R = crate::R<ClkDetSampleRateSpec>;
#[doc = "Field `value` reader - Sample rate in Hz\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValueR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Sample rate in Hz"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Clock detector sample rate\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_det_sample_rate::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkDetSampleRateSpec;
impl crate::RegisterSpec for ClkDetSampleRateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_det_sample_rate::R`](R) reader structure"]
impl crate::Readable for ClkDetSampleRateSpec {}
#[doc = "`reset()` method sets clkDetSampleRate to value 0"]
impl crate::Resettable for ClkDetSampleRateSpec {
    const RESET_VALUE: u32 = 0;
}
