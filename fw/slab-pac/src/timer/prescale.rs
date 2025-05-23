#[doc = "Register `prescale` writer"]
pub type W = crate::W<PrescaleSpec>;
#[doc = "Field `value` writer - Timer prescale divisor"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Timer prescale divisor"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<PrescaleSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Prescale\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescale::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrescaleSpec;
impl crate::RegisterSpec for PrescaleSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prescale::W`](W) writer structure"]
impl crate::Writable for PrescaleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets prescale to value 0"]
impl crate::Resettable for PrescaleSpec {
    const RESET_VALUE: u32 = 0;
}
