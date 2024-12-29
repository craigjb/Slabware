#[doc = "Register `control` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `control` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `hpdEnable` reader - Hot plug detect enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type HpdEnableR = crate::BitReader;
#[doc = "Field `hpdEnable` writer - Hot plug detect enable"]
pub type HpdEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pllPowerDown` reader - PLL power down\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type PllPowerDownR = crate::BitReader;
#[doc = "Field `pllPowerDown` writer - PLL power down"]
pub type PllPowerDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pllReset` reader - PLL reset\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type PllResetR = crate::BitReader;
#[doc = "Field `pllReset` writer - PLL reset"]
pub type PllResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gtpReset` reader - GTP transceiver reset\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type GtpResetR = crate::BitReader;
#[doc = "Field `gtpReset` writer - GTP transceiver reset"]
pub type GtpResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Hot plug detect enable"]
    #[inline(always)]
    pub fn hpd_enable(&self) -> HpdEnableR {
        HpdEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL power down"]
    #[inline(always)]
    pub fn pll_power_down(&self) -> PllPowerDownR {
        PllPowerDownR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PLL reset"]
    #[inline(always)]
    pub fn pll_reset(&self) -> PllResetR {
        PllResetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTP transceiver reset"]
    #[inline(always)]
    pub fn gtp_reset(&self) -> GtpResetR {
        GtpResetR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hot plug detect enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpd_enable(&mut self) -> HpdEnableW<ControlSpec> {
        HpdEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - PLL power down"]
    #[inline(always)]
    #[must_use]
    pub fn pll_power_down(&mut self) -> PllPowerDownW<ControlSpec> {
        PllPowerDownW::new(self, 1)
    }
    #[doc = "Bit 2 - PLL reset"]
    #[inline(always)]
    #[must_use]
    pub fn pll_reset(&mut self) -> PllResetW<ControlSpec> {
        PllResetW::new(self, 2)
    }
    #[doc = "Bit 3 - GTP transceiver reset"]
    #[inline(always)]
    #[must_use]
    pub fn gtp_reset(&mut self) -> GtpResetW<ControlSpec> {
        GtpResetW::new(self, 3)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets control to value 0x02"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0x02;
}
