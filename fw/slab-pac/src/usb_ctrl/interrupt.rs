#[doc = "Register `Interrupt` reader"]
pub type R = crate::R<InterruptSpec>;
#[doc = "Register `Interrupt` writer"]
pub type W = crate::W<InterruptSpec>;
#[doc = "Field `endpoints` reader - Raised when an endpoint generates an interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EndpointsR = crate::FieldReader<u16>;
#[doc = "Field `endpoints` writer - Raised when an endpoint generates an interrupt"]
pub type EndpointsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reset` reader - Raised when a USB reset occurs\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ResetR = crate::BitReader;
#[doc = "Field `reset` writer - Raised when a USB reset occurs"]
pub type ResetW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ep0Setup` reader - Raised when endpoint 0 receives a setup transaction\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Ep0setupR = crate::BitReader;
#[doc = "Field `ep0Setup` writer - Raised when endpoint 0 receives a setup transaction"]
pub type Ep0setupW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `suspend` reader - Raised when a USB suspend occurs\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SuspendR = crate::BitReader;
#[doc = "Field `suspend` writer - Raised when a USB suspend occurs"]
pub type SuspendW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `resume` reader - Raised when a USB resume occurs\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `resume` writer - Raised when a USB resume occurs"]
pub type ResumeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `disconnect` reader - Raised when a USB disconnect occurs\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DisconnectR = crate::BitReader;
#[doc = "Field `disconnect` writer - Raised when a USB disconnect occurs"]
pub type DisconnectW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Raised when an endpoint generates an interrupt"]
    #[inline(always)]
    pub fn endpoints(&self) -> EndpointsR {
        EndpointsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Raised when a USB reset occurs"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Raised when endpoint 0 receives a setup transaction"]
    #[inline(always)]
    pub fn ep0setup(&self) -> Ep0setupR {
        Ep0setupR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Raised when a USB suspend occurs"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raised when a USB resume occurs"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Raised when a USB disconnect occurs"]
    #[inline(always)]
    pub fn disconnect(&self) -> DisconnectR {
        DisconnectR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Raised when an endpoint generates an interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn endpoints(&mut self) -> EndpointsW<InterruptSpec> {
        EndpointsW::new(self, 0)
    }
    #[doc = "Bit 16 - Raised when a USB reset occurs"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<InterruptSpec> {
        ResetW::new(self, 16)
    }
    #[doc = "Bit 17 - Raised when endpoint 0 receives a setup transaction"]
    #[inline(always)]
    #[must_use]
    pub fn ep0setup(&mut self) -> Ep0setupW<InterruptSpec> {
        Ep0setupW::new(self, 17)
    }
    #[doc = "Bit 18 - Raised when a USB suspend occurs"]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SuspendW<InterruptSpec> {
        SuspendW::new(self, 18)
    }
    #[doc = "Bit 19 - Raised when a USB resume occurs"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> ResumeW<InterruptSpec> {
        ResumeW::new(self, 19)
    }
    #[doc = "Bit 20 - Raised when a USB disconnect occurs"]
    #[inline(always)]
    #[must_use]
    pub fn disconnect(&mut self) -> DisconnectW<InterruptSpec> {
        DisconnectW::new(self, 20)
    }
}
#[doc = "Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptSpec;
impl crate::RegisterSpec for InterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt::R`](R) reader structure"]
impl crate::Readable for InterruptSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt::W`](W) writer structure"]
impl crate::Writable for InterruptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x001f_ffff;
}
#[doc = "`reset()` method sets Interrupt to value 0"]
impl crate::Resettable for InterruptSpec {
    const RESET_VALUE: u32 = 0;
}
