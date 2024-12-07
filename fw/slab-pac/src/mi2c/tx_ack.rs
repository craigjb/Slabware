#[doc = "Register `txAck` reader"]
pub type R = crate::R<TxAckSpec>;
#[doc = "Register `txAck` writer"]
pub type W = crate::W<TxAckSpec>;
#[doc = "Field `value` writer - Transmit acknowledge value"]
pub type ValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `valid` reader - Transmit acknowledge valid\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValidR = crate::BitReader;
#[doc = "Field `valid` writer - Transmit acknowledge valid"]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enable` reader - Transmit acknowledge enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Transmit acknowledge enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `repeat` writer - Transmit acknowledge repeat"]
pub type RepeatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disableOnDataConflict` writer - Disable on data conflict"]
pub type DisableOnDataConflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Transmit acknowledge valid"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit acknowledge enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit acknowledge value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<TxAckSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit acknowledge valid"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<TxAckSpec> {
        ValidW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<TxAckSpec> {
        EnableW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit acknowledge repeat"]
    #[inline(always)]
    #[must_use]
    pub fn repeat(&mut self) -> RepeatW<TxAckSpec> {
        RepeatW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable on data conflict"]
    #[inline(always)]
    #[must_use]
    pub fn disable_on_data_conflict(&mut self) -> DisableOnDataConflictW<TxAckSpec> {
        DisableOnDataConflictW::new(self, 4)
    }
}
#[doc = "Transmit acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxAckSpec;
impl crate::RegisterSpec for TxAckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_ack::R`](R) reader structure"]
impl crate::Readable for TxAckSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_ack::W`](W) writer structure"]
impl crate::Writable for TxAckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets txAck to value 0"]
impl crate::Resettable for TxAckSpec {
    const RESET_VALUE: u32 = 0;
}
