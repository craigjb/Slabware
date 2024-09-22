#[doc = "Register `TBUF` writer"]
pub type W = crate::W<TbufSpec>;
#[doc = "Field `tBuf` writer - # of cycles idle"]
pub type TBufW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - # of cycles idle"]
    #[inline(always)]
    #[must_use]
    pub fn t_buf(&mut self) -> TBufW<TbufSpec> {
        TBufW::new(self, 0)
    }
}
#[doc = "I2C idle timing\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbuf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbufSpec;
impl crate::RegisterSpec for TbufSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tbuf::W`](W) writer structure"]
impl crate::Writable for TbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBUF to value 0"]
impl crate::Resettable for TbufSpec {
    const RESET_VALUE: u32 = 0;
}
