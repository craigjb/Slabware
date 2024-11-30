#[doc = "Register `TLOW` writer"]
pub type W = crate::W<TlowSpec>;
#[doc = "Field `tLow` writer - # of cycles low"]
pub type TLowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - # of cycles low"]
    #[inline(always)]
    #[must_use]
    pub fn t_low(&mut self) -> TLowW<TlowSpec> {
        TLowW::new(self, 0)
    }
}
#[doc = "I2C low timing\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlow::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TlowSpec;
impl crate::RegisterSpec for TlowSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tlow::W`](W) writer structure"]
impl crate::Writable for TlowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TLOW to value 0"]
impl crate::Resettable for TlowSpec {
    const RESET_VALUE: u32 = 0;
}
