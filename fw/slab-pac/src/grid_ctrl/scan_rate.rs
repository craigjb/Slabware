#[doc = "Register `ScanRate` reader"]
pub type R = crate::R<ScanRateSpec>;
#[doc = "Field `rate` reader - Key scan rate (25 Hz)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type RateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Key scan rate (25 Hz)"]
    #[inline(always)]
    pub fn rate(&self) -> RateR {
        RateR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Grid key scan rate\n\nYou can [`read`](crate::Reg::read) this register and get [`scan_rate::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanRateSpec;
impl crate::RegisterSpec for ScanRateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scan_rate::R`](R) reader structure"]
impl crate::Readable for ScanRateSpec {}
#[doc = "`reset()` method sets ScanRate to value 0"]
impl crate::Resettable for ScanRateSpec {
    const RESET_VALUE: u32 = 0;
}
