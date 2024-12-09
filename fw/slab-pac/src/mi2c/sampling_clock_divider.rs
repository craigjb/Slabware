#[doc = "Register `samplingClockDivider` writer"]
pub type W = crate::W<SamplingClockDividerSpec>;
#[doc = "Field `samplingClockDivider` writer - Sampling clock divider"]
pub type SamplingClockDividerW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl W {
    #[doc = "Bits 0:9 - Sampling clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn sampling_clock_divider(&mut self) -> SamplingClockDividerW<SamplingClockDividerSpec> {
        SamplingClockDividerW::new(self, 0)
    }
}
#[doc = "Sampling clock\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampling_clock_divider::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SamplingClockDividerSpec;
impl crate::RegisterSpec for SamplingClockDividerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sampling_clock_divider::W`](W) writer structure"]
impl crate::Writable for SamplingClockDividerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets samplingClockDivider to value 0"]
impl crate::Resettable for SamplingClockDividerSpec {
    const RESET_VALUE: u32 = 0;
}
