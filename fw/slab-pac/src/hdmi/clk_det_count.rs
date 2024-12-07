#[doc = "Register `clkDetCount` reader"]
pub type R = crate::R<ClkDetCountSpec>;
#[doc = "Field `value` reader - Frequency counter value\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - Frequency counter value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "Clock detector frequency count\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_det_count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkDetCountSpec;
impl crate::RegisterSpec for ClkDetCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_det_count::R`](R) reader structure"]
impl crate::Readable for ClkDetCountSpec {}
#[doc = "`reset()` method sets clkDetCount to value 0"]
impl crate::Resettable for ClkDetCountSpec {
    const RESET_VALUE: u32 = 0;
}
