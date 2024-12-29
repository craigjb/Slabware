#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `status` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `cableDetect` reader - Cable detect\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type CableDetectR = crate::BitReader;
#[doc = "Field `cableDetectChanged` reader - Cable detect status changed\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type CableDetectChangedR = crate::BitReader;
#[doc = "Field `cableDetectChanged` writer - Cable detect status changed"]
pub type CableDetectChangedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `pllLock` reader - PLL is locked\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type PllLockR = crate::BitReader;
#[doc = "Field `freqChanged` reader - Clock detector frequency changed\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type FreqChangedR = crate::BitReader;
#[doc = "Field `freqChanged` writer - Clock detector frequency changed"]
pub type FreqChangedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `allGtpResetsDone` reader - All channel GTP resets done\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type AllGtpResetsDoneR = crate::BitReader;
#[doc = "Field `allHdmiDataValid` reader - All channel data is valid\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type AllHdmiDataValidR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Cable detect"]
    #[inline(always)]
    pub fn cable_detect(&self) -> CableDetectR {
        CableDetectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cable detect status changed"]
    #[inline(always)]
    pub fn cable_detect_changed(&self) -> CableDetectChangedR {
        CableDetectChangedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PLL is locked"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PllLockR {
        PllLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock detector frequency changed"]
    #[inline(always)]
    pub fn freq_changed(&self) -> FreqChangedR {
        FreqChangedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - All channel GTP resets done"]
    #[inline(always)]
    pub fn all_gtp_resets_done(&self) -> AllGtpResetsDoneR {
        AllGtpResetsDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - All channel data is valid"]
    #[inline(always)]
    pub fn all_hdmi_data_valid(&self) -> AllHdmiDataValidR {
        AllHdmiDataValidR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Cable detect status changed"]
    #[inline(always)]
    #[must_use]
    pub fn cable_detect_changed(&mut self) -> CableDetectChangedW<StatusSpec> {
        CableDetectChangedW::new(self, 1)
    }
    #[doc = "Bit 3 - Clock detector frequency changed"]
    #[inline(always)]
    #[must_use]
    pub fn freq_changed(&mut self) -> FreqChangedW<StatusSpec> {
        FreqChangedW::new(self, 3)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0a;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
