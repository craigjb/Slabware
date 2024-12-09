#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `cableDetect` reader - Cable detect\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type CableDetectR = crate::BitReader;
#[doc = "Field `pllLock` reader - PLL is locked\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type PllLockR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Cable detect"]
    #[inline(always)]
    pub fn cable_detect(&self) -> CableDetectR {
        CableDetectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL is locked"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PllLockR {
        PllLockR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
