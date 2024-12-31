#[doc = "Register `Config` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `Config` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `pullUpEnable` reader - Enable USB device pullup on dp pin\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type PullUpEnableR = crate::BitReader;
#[doc = "Field `pullUpEnable` writer - Enable USB device pullup on dp pin"]
pub type PullUpEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `interruptEnable` reader - Enable interrupts\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type InterruptEnableR = crate::BitReader;
#[doc = "Field `interruptEnable` writer - Enable interrupts"]
pub type InterruptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable USB device pullup on dp pin"]
    #[inline(always)]
    pub fn pull_up_enable(&self) -> PullUpEnableR {
        PullUpEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable interrupts"]
    #[inline(always)]
    pub fn interrupt_enable(&self) -> InterruptEnableR {
        InterruptEnableR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable USB device pullup on dp pin"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_enable(&mut self) -> PullUpEnableW<ConfigSpec> {
        PullUpEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable(&mut self) -> InterruptEnableW<ConfigSpec> {
        InterruptEnableW::new(self, 1)
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Config to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
