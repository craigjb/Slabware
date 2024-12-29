#[doc = "Register `clkDetTolerance` reader"]
pub type R = crate::R<ClkDetToleranceSpec>;
#[doc = "Register `clkDetTolerance` writer"]
pub type W = crate::W<ClkDetToleranceSpec>;
#[doc = "Field `value` reader - Tolerance for frequency change detection\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `value` writer - Tolerance for frequency change detection"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Tolerance for frequency change detection"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Tolerance for frequency change detection"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<ClkDetToleranceSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Clock detector tolerance\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_det_tolerance::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_det_tolerance::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkDetToleranceSpec;
impl crate::RegisterSpec for ClkDetToleranceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_det_tolerance::R`](R) reader structure"]
impl crate::Readable for ClkDetToleranceSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_det_tolerance::W`](W) writer structure"]
impl crate::Writable for ClkDetToleranceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clkDetTolerance to value 0x01"]
impl crate::Resettable for ClkDetToleranceSpec {
    const RESET_VALUE: u32 = 0x01;
}
