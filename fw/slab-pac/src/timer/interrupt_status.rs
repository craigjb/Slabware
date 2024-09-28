#[doc = "Register `interruptStatus` reader"]
pub type R = crate::R<InterruptStatusSpec>;
#[doc = "Register `interruptStatus` writer"]
pub type W = crate::W<InterruptStatusSpec>;
#[doc = "Field `overflowStatus` reader - Overflow interrupt status (set to clear)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type OverflowStatusR = crate::BitReader;
#[doc = "Field `overflowStatus` writer - Overflow interrupt status (set to clear)"]
pub type OverflowStatusW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `compare0Status` reader - Compare0 interrupt status (set to clear)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Compare0statusR = crate::BitReader;
#[doc = "Field `compare0Status` writer - Compare0 interrupt status (set to clear)"]
pub type Compare0statusW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `compare1Status` reader - Compare1 interrupt status (set to clear)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Compare1statusR = crate::BitReader;
#[doc = "Field `compare1Status` writer - Compare1 interrupt status (set to clear)"]
pub type Compare1statusW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt status (set to clear)"]
    #[inline(always)]
    pub fn overflow_status(&self) -> OverflowStatusR {
        OverflowStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare0 interrupt status (set to clear)"]
    #[inline(always)]
    pub fn compare0status(&self) -> Compare0statusR {
        Compare0statusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare1 interrupt status (set to clear)"]
    #[inline(always)]
    pub fn compare1status(&self) -> Compare1statusR {
        Compare1statusR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt status (set to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn overflow_status(&mut self) -> OverflowStatusW<InterruptStatusSpec> {
        OverflowStatusW::new(self, 0)
    }
    #[doc = "Bit 1 - Compare0 interrupt status (set to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn compare0status(&mut self) -> Compare0statusW<InterruptStatusSpec> {
        Compare0statusW::new(self, 1)
    }
    #[doc = "Bit 2 - Compare1 interrupt status (set to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn compare1status(&mut self) -> Compare1statusW<InterruptStatusSpec> {
        Compare1statusW::new(self, 2)
    }
}
#[doc = "Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptStatusSpec;
impl crate::RegisterSpec for InterruptStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_status::R`](R) reader structure"]
impl crate::Readable for InterruptStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_status::W`](W) writer structure"]
impl crate::Writable for InterruptStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets interruptStatus to value 0"]
impl crate::Resettable for InterruptStatusSpec {
    const RESET_VALUE: u32 = 0;
}
