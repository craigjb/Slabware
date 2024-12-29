#[doc = "Register `clkDetDivisor` reader"]
pub type R = crate::R<ClkDetDivisorSpec>;
#[doc = "Field `value` reader - Divisor value\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Divisor value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Clock detector divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_det_divisor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkDetDivisorSpec;
impl crate::RegisterSpec for ClkDetDivisorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_det_divisor::R`](R) reader structure"]
impl crate::Readable for ClkDetDivisorSpec {}
#[doc = "`reset()` method sets clkDetDivisor to value 0"]
impl crate::Resettable for ClkDetDivisorSpec {
    const RESET_VALUE: u32 = 0;
}
