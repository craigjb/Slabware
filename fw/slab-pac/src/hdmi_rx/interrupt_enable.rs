#[doc = "Register `InterruptEnable` reader"]
pub type R = crate::R<InterruptEnableSpec>;
#[doc = "Register `InterruptEnable` writer"]
pub type W = crate::W<InterruptEnableSpec>;
#[doc = "Field `cableDetectChangedEnable` reader - Enable cable detect changed interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type CableDetectChangedEnableR = crate::BitReader;
#[doc = "Field `cableDetectChangedEnable` writer - Enable cable detect changed interrupt"]
pub type CableDetectChangedEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pllLockEnable` reader - Enable PLL lock interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type PllLockEnableR = crate::BitReader;
#[doc = "Field `pllLockEnable` writer - Enable PLL lock interrupt"]
pub type PllLockEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `freqChangedEnable` reader - Enable frequency change interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type FreqChangedEnableR = crate::BitReader;
#[doc = "Field `freqChangedEnable` writer - Enable frequency change interrupt"]
pub type FreqChangedEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `allGtpResetsDoneEnable` reader - Enable all channel GTP resets done interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type AllGtpResetsDoneEnableR = crate::BitReader;
#[doc = "Field `allGtpResetsDoneEnable` writer - Enable all channel GTP resets done interrupt"]
pub type AllGtpResetsDoneEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `allHdmiDataValidEnable` reader - Enable all channel data is valid interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type AllHdmiDataValidEnableR = crate::BitReader;
#[doc = "Field `allHdmiDataValidEnable` writer - Enable all channel data is valid interrupt"]
pub type AllHdmiDataValidEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable cable detect changed interrupt"]
    #[inline(always)]
    pub fn cable_detect_changed_enable(&self) -> CableDetectChangedEnableR {
        CableDetectChangedEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable PLL lock interrupt"]
    #[inline(always)]
    pub fn pll_lock_enable(&self) -> PllLockEnableR {
        PllLockEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable frequency change interrupt"]
    #[inline(always)]
    pub fn freq_changed_enable(&self) -> FreqChangedEnableR {
        FreqChangedEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable all channel GTP resets done interrupt"]
    #[inline(always)]
    pub fn all_gtp_resets_done_enable(&self) -> AllGtpResetsDoneEnableR {
        AllGtpResetsDoneEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable all channel data is valid interrupt"]
    #[inline(always)]
    pub fn all_hdmi_data_valid_enable(&self) -> AllHdmiDataValidEnableR {
        AllHdmiDataValidEnableR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable cable detect changed interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cable_detect_changed_enable(
        &mut self,
    ) -> CableDetectChangedEnableW<InterruptEnableSpec> {
        CableDetectChangedEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable PLL lock interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock_enable(&mut self) -> PllLockEnableW<InterruptEnableSpec> {
        PllLockEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable frequency change interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn freq_changed_enable(&mut self) -> FreqChangedEnableW<InterruptEnableSpec> {
        FreqChangedEnableW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable all channel GTP resets done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn all_gtp_resets_done_enable(&mut self) -> AllGtpResetsDoneEnableW<InterruptEnableSpec> {
        AllGtpResetsDoneEnableW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable all channel data is valid interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn all_hdmi_data_valid_enable(&mut self) -> AllHdmiDataValidEnableW<InterruptEnableSpec> {
        AllHdmiDataValidEnableW::new(self, 4)
    }
}
#[doc = "Interrupt enables\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptEnableSpec;
impl crate::RegisterSpec for InterruptEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enable::R`](R) reader structure"]
impl crate::Readable for InterruptEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enable::W`](W) writer structure"]
impl crate::Writable for InterruptEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets InterruptEnable to value 0"]
impl crate::Resettable for InterruptEnableSpec {
    const RESET_VALUE: u32 = 0;
}
