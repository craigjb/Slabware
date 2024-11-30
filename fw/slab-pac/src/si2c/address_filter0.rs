#[doc = "Register `addressFilter0` reader"]
pub type R = crate::R<AddressFilter0Spec>;
#[doc = "Register `addressFilter0` writer"]
pub type W = crate::W<AddressFilter0Spec>;
#[doc = "Field `address` reader - Address\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type AddressR = crate::FieldReader;
#[doc = "Field `address` writer - Address"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `enable` reader - Enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Address"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Address"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<AddressFilter0Spec> {
        AddressW::new(self, 0)
    }
    #[doc = "Bit 7 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<AddressFilter0Spec> {
        EnableW::new(self, 7)
    }
}
#[doc = "Address filter 0\n\nYou can [`read`](crate::Reg::read) this register and get [`address_filter0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address_filter0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddressFilter0Spec;
impl crate::RegisterSpec for AddressFilter0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address_filter0::R`](R) reader structure"]
impl crate::Readable for AddressFilter0Spec {}
#[doc = "`write(|w| ..)` method takes [`address_filter0::W`](W) writer structure"]
impl crate::Writable for AddressFilter0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets addressFilter0 to value 0"]
impl crate::Resettable for AddressFilter0Spec {
    const RESET_VALUE: u32 = 0;
}
