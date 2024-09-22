#[doc = "Register `masterStatus` reader"]
pub type R = crate::R<MasterStatusSpec>;
#[doc = "Register `masterStatus` writer"]
pub type W = crate::W<MasterStatusSpec>;
#[doc = "Field `busy` reader - Is busy?\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type BusyR = crate::BitReader;
#[doc = "Field `start` reader - Order a start (set on set)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type StartR = crate::BitReader;
#[doc = "Field `start` writer - Order a start (set on set)"]
pub type StartW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `stop` reader - Order a stop (set on set)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type StopR = crate::BitReader;
#[doc = "Field `stop` writer - Order a stop (set on set)"]
pub type StopW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `drop` reader - Order a drop (set on set)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DropR = crate::BitReader;
#[doc = "Field `drop` writer - Order a drop (set on set)"]
pub type DropW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `recover` reader - Order a recover (set on set)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type RecoverR = crate::BitReader;
#[doc = "Field `recover` writer - Order a recover (set on set)"]
pub type RecoverW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `startDropped` reader - Timeout during start\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type StartDroppedR = crate::BitReader;
#[doc = "Field `startDropped` writer - Timeout during start"]
pub type StartDroppedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `stopDropped` reader - Timeout during stop\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type StopDroppedR = crate::BitReader;
#[doc = "Field `stopDropped` writer - Timeout during stop"]
pub type StopDroppedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `recoverDropped` reader - Timeout during recover\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type RecoverDroppedR = crate::BitReader;
#[doc = "Field `recoverDropped` writer - Timeout during recover"]
pub type RecoverDroppedW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Is busy?"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Order a start (set on set)"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Order a stop (set on set)"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Order a drop (set on set)"]
    #[inline(always)]
    pub fn drop(&self) -> DropR {
        DropR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Order a recover (set on set)"]
    #[inline(always)]
    pub fn recover(&self) -> RecoverR {
        RecoverR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Timeout during start"]
    #[inline(always)]
    pub fn start_dropped(&self) -> StartDroppedR {
        StartDroppedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timeout during stop"]
    #[inline(always)]
    pub fn stop_dropped(&self) -> StopDroppedR {
        StopDroppedR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timeout during recover"]
    #[inline(always)]
    pub fn recover_dropped(&self) -> RecoverDroppedR {
        RecoverDroppedR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Order a start (set on set)"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<MasterStatusSpec> {
        StartW::new(self, 4)
    }
    #[doc = "Bit 5 - Order a stop (set on set)"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<MasterStatusSpec> {
        StopW::new(self, 5)
    }
    #[doc = "Bit 6 - Order a drop (set on set)"]
    #[inline(always)]
    #[must_use]
    pub fn drop(&mut self) -> DropW<MasterStatusSpec> {
        DropW::new(self, 6)
    }
    #[doc = "Bit 7 - Order a recover (set on set)"]
    #[inline(always)]
    #[must_use]
    pub fn recover(&mut self) -> RecoverW<MasterStatusSpec> {
        RecoverW::new(self, 7)
    }
    #[doc = "Bit 9 - Timeout during start"]
    #[inline(always)]
    #[must_use]
    pub fn start_dropped(&mut self) -> StartDroppedW<MasterStatusSpec> {
        StartDroppedW::new(self, 9)
    }
    #[doc = "Bit 10 - Timeout during stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop_dropped(&mut self) -> StopDroppedW<MasterStatusSpec> {
        StopDroppedW::new(self, 10)
    }
    #[doc = "Bit 11 - Timeout during recover"]
    #[inline(always)]
    #[must_use]
    pub fn recover_dropped(&mut self) -> RecoverDroppedW<MasterStatusSpec> {
        RecoverDroppedW::new(self, 11)
    }
}
#[doc = "Master status\n\nYou can [`read`](crate::Reg::read) this register and get [`master_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasterStatusSpec;
impl crate::RegisterSpec for MasterStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`master_status::R`](R) reader structure"]
impl crate::Readable for MasterStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`master_status::W`](W) writer structure"]
impl crate::Writable for MasterStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0ef0;
}
#[doc = "`reset()` method sets masterStatus to value 0"]
impl crate::Resettable for MasterStatusSpec {
    const RESET_VALUE: u32 = 0;
}
