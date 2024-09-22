#[doc = "Register `THIGH` writer"]
pub type W = crate::W<ThighSpec>;
#[doc = "Field `tHigh` writer - # of cycles high"]
pub type THighW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - # of cycles high"]
    #[inline(always)]
    #[must_use]
    pub fn t_high(&mut self) -> THighW<ThighSpec> {
        THighW::new(self, 0)
    }
}
#[doc = "I2C high timing\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thigh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThighSpec;
impl crate::RegisterSpec for ThighSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`thigh::W`](W) writer structure"]
impl crate::Writable for ThighSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THIGH to value 0"]
impl crate::Resettable for ThighSpec {
    const RESET_VALUE: u32 = 0;
}
