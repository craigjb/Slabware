#[doc = "Register `tsuData` writer"]
pub type W = crate::W<TsuDataSpec>;
#[doc = "Field `tsuData` writer - "]
pub type TsuDataW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn tsu_data(&mut self) -> TsuDataW<TsuDataSpec> {
        TsuDataW::new(self, 0)
    }
}
#[doc = "TSU Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsu_data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsuDataSpec;
impl crate::RegisterSpec for TsuDataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tsu_data::W`](W) writer structure"]
impl crate::Writable for TsuDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tsuData to value 0"]
impl crate::Resettable for TsuDataSpec {
    const RESET_VALUE: u32 = 0;
}
