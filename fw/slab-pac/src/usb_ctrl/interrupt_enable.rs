#[doc = "Register `InterruptEnable` reader"]
pub type R = crate::R<InterruptEnableSpec>;
#[doc = "Register `InterruptEnable` writer"]
pub type W = crate::W<InterruptEnableSpec>;
#[doc = "Field `enableEndpoints` reader - Enable endpoint interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableEndpointsR = crate::FieldReader<u16>;
#[doc = "Field `enableEndpoints` writer - Enable endpoint interrupt"]
pub type EnableEndpointsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `enableReset` reader - Enable USB reset interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableResetR = crate::BitReader;
#[doc = "Field `enableReset` writer - Enable USB reset interrupt"]
pub type EnableResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enableEp0Setup` reader - Enable endpoint 0 setup interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableEp0setupR = crate::BitReader;
#[doc = "Field `enableEp0Setup` writer - Enable endpoint 0 setup interrupt"]
pub type EnableEp0setupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enableSuspend` reader - Enable USB suspend interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableSuspendR = crate::BitReader;
#[doc = "Field `enableSuspend` writer - Enable USB suspend interrupt"]
pub type EnableSuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enableResume` reader - Enable USB resume interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableResumeR = crate::BitReader;
#[doc = "Field `enableResume` writer - Enable USB resume interrupt"]
pub type EnableResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enableConnect` reader - Enable USB connect interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableConnectR = crate::BitReader;
#[doc = "Field `enableConnect` writer - Enable USB connect interrupt"]
pub type EnableConnectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enableDisconnect` reader - Enable USB disconnect interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableDisconnectR = crate::BitReader;
#[doc = "Field `enableDisconnect` writer - Enable USB disconnect interrupt"]
pub type EnableDisconnectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Enable endpoint interrupt"]
    #[inline(always)]
    pub fn enable_endpoints(&self) -> EnableEndpointsR {
        EnableEndpointsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable USB reset interrupt"]
    #[inline(always)]
    pub fn enable_reset(&self) -> EnableResetR {
        EnableResetR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable endpoint 0 setup interrupt"]
    #[inline(always)]
    pub fn enable_ep0setup(&self) -> EnableEp0setupR {
        EnableEp0setupR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable USB suspend interrupt"]
    #[inline(always)]
    pub fn enable_suspend(&self) -> EnableSuspendR {
        EnableSuspendR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable USB resume interrupt"]
    #[inline(always)]
    pub fn enable_resume(&self) -> EnableResumeR {
        EnableResumeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable USB connect interrupt"]
    #[inline(always)]
    pub fn enable_connect(&self) -> EnableConnectR {
        EnableConnectR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable USB disconnect interrupt"]
    #[inline(always)]
    pub fn enable_disconnect(&self) -> EnableDisconnectR {
        EnableDisconnectR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Enable endpoint interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_endpoints(&mut self) -> EnableEndpointsW<InterruptEnableSpec> {
        EnableEndpointsW::new(self, 0)
    }
    #[doc = "Bit 16 - Enable USB reset interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_reset(&mut self) -> EnableResetW<InterruptEnableSpec> {
        EnableResetW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable endpoint 0 setup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ep0setup(&mut self) -> EnableEp0setupW<InterruptEnableSpec> {
        EnableEp0setupW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable USB suspend interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_suspend(&mut self) -> EnableSuspendW<InterruptEnableSpec> {
        EnableSuspendW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable USB resume interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_resume(&mut self) -> EnableResumeW<InterruptEnableSpec> {
        EnableResumeW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable USB connect interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_connect(&mut self) -> EnableConnectW<InterruptEnableSpec> {
        EnableConnectW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable USB disconnect interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_disconnect(&mut self) -> EnableDisconnectW<InterruptEnableSpec> {
        EnableDisconnectW::new(self, 21)
    }
}
#[doc = "Interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
