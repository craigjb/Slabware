#[doc = "Register `interruptMask` reader"]
pub type R = crate::R<InterruptMaskSpec>;
#[doc = "Register `interruptMask` writer"]
pub type W = crate::W<InterruptMaskSpec>;
#[doc = "Field `overflowMask` reader - Mask overflow interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type OverflowMaskR = crate::BitReader;
#[doc = "Field `overflowMask` writer - Mask overflow interrupt"]
pub type OverflowMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `compare0Mask` reader - Mask compare0 interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Compare0maskR = crate::BitReader;
#[doc = "Field `compare0Mask` writer - Mask compare0 interrupt"]
pub type Compare0maskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `compare1Mask` reader - Mask compare1 interrupt\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Compare1maskR = crate::BitReader;
#[doc = "Field `compare1Mask` writer - Mask compare1 interrupt"]
pub type Compare1maskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask overflow interrupt"]
    #[inline(always)]
    pub fn overflow_mask(&self) -> OverflowMaskR {
        OverflowMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask compare0 interrupt"]
    #[inline(always)]
    pub fn compare0mask(&self) -> Compare0maskR {
        Compare0maskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask compare1 interrupt"]
    #[inline(always)]
    pub fn compare1mask(&self) -> Compare1maskR {
        Compare1maskR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask overflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn overflow_mask(&mut self) -> OverflowMaskW<InterruptMaskSpec> {
        OverflowMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask compare0 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn compare0mask(&mut self) -> Compare0maskW<InterruptMaskSpec> {
        Compare0maskW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask compare1 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn compare1mask(&mut self) -> Compare1maskW<InterruptMaskSpec> {
        Compare1maskW::new(self, 2)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptMaskSpec;
impl crate::RegisterSpec for InterruptMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_mask::R`](R) reader structure"]
impl crate::Readable for InterruptMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_mask::W`](W) writer structure"]
impl crate::Writable for InterruptMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets interruptMask to value 0"]
impl crate::Resettable for InterruptMaskSpec {
    const RESET_VALUE: u32 = 0;
}
