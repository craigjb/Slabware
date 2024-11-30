#[doc = "Register `rxData` reader"]
pub type R = crate::R<RxDataSpec>;
#[doc = "Register `rxData` writer"]
pub type W = crate::W<RxDataSpec>;
#[doc = "Field `value` reader - Receive data value\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `valid` reader - Receive data valid (set to clear)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValidR = crate::BitReader;
#[doc = "Field `valid` writer - Receive data valid (set to clear)"]
pub type ValidW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `listen` writer - Listen for receive data"]
pub type ListenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Receive data value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Receive data valid (set to clear)"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Receive data valid (set to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<RxDataSpec> {
        ValidW::new(self, 8)
    }
    #[doc = "Bit 9 - Listen for receive data"]
    #[inline(always)]
    #[must_use]
    pub fn listen(&mut self) -> ListenW<RxDataSpec> {
        ListenW::new(self, 9)
    }
}
#[doc = "Receive data\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxDataSpec;
impl crate::RegisterSpec for RxDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data::R`](R) reader structure"]
impl crate::Readable for RxDataSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_data::W`](W) writer structure"]
impl crate::Writable for RxDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0100;
}
#[doc = "`reset()` method sets rxData to value 0"]
impl crate::Resettable for RxDataSpec {
    const RESET_VALUE: u32 = 0;
}
