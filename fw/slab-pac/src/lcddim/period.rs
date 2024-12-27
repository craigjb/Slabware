#[doc = "Register `Period` reader"]
pub type R = crate::R<PeriodSpec>;
#[doc = "Register `Period` writer"]
pub type W = crate::W<PeriodSpec>;
#[doc = "Field `value` reader - PWM period value\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `value` writer - PWM period value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PWM period value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PWM period value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<PeriodSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "PWM period\n\nYou can [`read`](crate::Reg::read) this register and get [`period::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`period::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriodSpec;
impl crate::RegisterSpec for PeriodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`period::R`](R) reader structure"]
impl crate::Readable for PeriodSpec {}
#[doc = "`write(|w| ..)` method takes [`period::W`](W) writer structure"]
impl crate::Writable for PeriodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Period to value 0xff"]
impl crate::Resettable for PeriodSpec {
    const RESET_VALUE: u32 = 0xff;
}
