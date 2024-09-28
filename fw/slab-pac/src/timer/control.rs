#[doc = "Register `control` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `control` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `enable` reader - Timer enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Timer enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clear` writer - Clear prescaler and counter"]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `interruptEnable` reader - Interrupt enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type InterruptEnableR = crate::BitReader;
#[doc = "Field `interruptEnable` writer - Interrupt enable"]
pub type InterruptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn interrupt_enable(&self) -> InterruptEnableR {
        InterruptEnableR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ControlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear prescaler and counter"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<ControlSpec> {
        ClearW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable(&mut self) -> InterruptEnableW<ControlSpec> {
        InterruptEnableW::new(self, 2)
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
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0;
}
