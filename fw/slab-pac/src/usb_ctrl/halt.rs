#[doc = "Register `Halt` reader"]
pub type R = crate::R<HaltSpec>;
#[doc = "Register `Halt` writer"]
pub type W = crate::W<HaltSpec>;
#[doc = "Field `endpointId` reader - The endpoint you want to put in sleep\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EndpointIdR = crate::FieldReader;
#[doc = "Field `endpointId` writer - The endpoint you want to put in sleep"]
pub type EndpointIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `enable` reader - Halt is active when set, endpoint is unhalted when cleared.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Halt is active when set, endpoint is unhalted when cleared."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `effective` reader - After setting the enable, wait for this bit to be set by the hardware to ensure atomicity\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EffectiveR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - The endpoint you want to put in sleep"]
    #[inline(always)]
    pub fn endpoint_id(&self) -> EndpointIdR {
        EndpointIdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Halt is active when set, endpoint is unhalted when cleared."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - After setting the enable, wait for this bit to be set by the hardware to ensure atomicity"]
    #[inline(always)]
    pub fn effective(&self) -> EffectiveR {
        EffectiveR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The endpoint you want to put in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn endpoint_id(&mut self) -> EndpointIdW<HaltSpec> {
        EndpointIdW::new(self, 0)
    }
    #[doc = "Bit 4 - Halt is active when set, endpoint is unhalted when cleared."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<HaltSpec> {
        EnableW::new(self, 4)
    }
}
#[doc = "Halt endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`halt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`halt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HaltSpec;
impl crate::RegisterSpec for HaltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`halt::R`](R) reader structure"]
impl crate::Readable for HaltSpec {}
#[doc = "`write(|w| ..)` method takes [`halt::W`](W) writer structure"]
impl crate::Writable for HaltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Halt to value 0"]
impl crate::Resettable for HaltSpec {
    const RESET_VALUE: u32 = 0;
}
