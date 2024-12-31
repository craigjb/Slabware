#[doc = "Register `Address` reader"]
pub type R = crate::R<AddressSpec>;
#[doc = "Register `Address` writer"]
pub type W = crate::W<AddressSpec>;
#[doc = "Field `value` reader - The device will only listen at tokens with the specified address. This field is automatically cleared on usb reset events\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `value` writer - The device will only listen at tokens with the specified address. This field is automatically cleared on usb reset events"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `enable` reader - Enable the USB address filtering if set\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Enable the USB address filtering if set"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trigger` reader - Set the enable (see above) on the next EP0 IN token completion Cleared by the hardware after any EP0 completion\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TriggerR = crate::BitReader;
#[doc = "Field `trigger` writer - Set the enable (see above) on the next EP0 IN token completion Cleared by the hardware after any EP0 completion"]
pub type TriggerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - The device will only listen at tokens with the specified address. This field is automatically cleared on usb reset events"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Enable the USB address filtering if set"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set the enable (see above) on the next EP0 IN token completion Cleared by the hardware after any EP0 completion"]
    #[inline(always)]
    pub fn trigger(&self) -> TriggerR {
        TriggerR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The device will only listen at tokens with the specified address. This field is automatically cleared on usb reset events"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<AddressSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bit 7 - Enable the USB address filtering if set"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<AddressSpec> {
        EnableW::new(self, 7)
    }
    #[doc = "Bit 8 - Set the enable (see above) on the next EP0 IN token completion Cleared by the hardware after any EP0 completion"]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TriggerW<AddressSpec> {
        TriggerW::new(self, 8)
    }
}
#[doc = "USB address\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddressSpec;
impl crate::RegisterSpec for AddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address::R`](R) reader structure"]
impl crate::Readable for AddressSpec {}
#[doc = "`write(|w| ..)` method takes [`address::W`](W) writer structure"]
impl crate::Writable for AddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Address to value 0"]
impl crate::Resettable for AddressSpec {
    const RESET_VALUE: u32 = 0;
}
