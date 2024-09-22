#[doc = "Register `rxAck` reader"]
pub type R = crate::R<RxAckSpec>;
#[doc = "Register `rxAck` writer"]
pub type W = crate::W<RxAckSpec>;
#[doc = "Field `value` reader - Receive acknowledge value\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValueR = crate::BitReader;
#[doc = "Field `valid` reader - Receive acknowledge valid (cleared on read)\n\n<div class=\"warning\">The field is <b>cleared</b> (set to zero) following a read operation.</div>"]
pub type ValidR = crate::BitReader;
#[doc = "Field `listen` writer - Listen for receive acknowledge"]
pub type ListenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive acknowledge value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Receive acknowledge valid (cleared on read)"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Listen for receive acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn listen(&mut self) -> ListenW<RxAckSpec> {
        ListenW::new(self, 9)
    }
}
#[doc = "Receive acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxAckSpec;
impl crate::RegisterSpec for RxAckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ack::R`](R) reader structure"]
impl crate::Readable for RxAckSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_ack::W`](W) writer structure"]
impl crate::Writable for RxAckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rxAck to value 0"]
impl crate::Resettable for RxAckSpec {
    const RESET_VALUE: u32 = 0;
}
