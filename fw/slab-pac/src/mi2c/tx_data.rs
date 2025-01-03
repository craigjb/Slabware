#[doc = "Register `txData` reader"]
pub type R = crate::R<TxDataSpec>;
#[doc = "Register `txData` writer"]
pub type W = crate::W<TxDataSpec>;
#[doc = "Field `value` writer - Transmit data value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `valid` reader - Transmit data valid\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValidR = crate::BitReader;
#[doc = "Field `valid` writer - Transmit data valid"]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enable` reader - Transmit data enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Transmit data enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `repeat` writer - Transmit data repeat"]
pub type RepeatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disableOnDataConflict` writer - Disable on data conflict"]
pub type DisableOnDataConflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Transmit data valid"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit data enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<TxDataSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bit 8 - Transmit data valid"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<TxDataSpec> {
        ValidW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmit data enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<TxDataSpec> {
        EnableW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmit data repeat"]
    #[inline(always)]
    #[must_use]
    pub fn repeat(&mut self) -> RepeatW<TxDataSpec> {
        RepeatW::new(self, 10)
    }
    #[doc = "Bit 11 - Disable on data conflict"]
    #[inline(always)]
    #[must_use]
    pub fn disable_on_data_conflict(&mut self) -> DisableOnDataConflictW<TxDataSpec> {
        DisableOnDataConflictW::new(self, 11)
    }
}
#[doc = "Transmit data\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxDataSpec;
impl crate::RegisterSpec for TxDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_data::R`](R) reader structure"]
impl crate::Readable for TxDataSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_data::W`](W) writer structure"]
impl crate::Writable for TxDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets txData to value 0x0500"]
impl crate::Resettable for TxDataSpec {
    const RESET_VALUE: u32 = 0x0500;
}
