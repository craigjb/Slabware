#[doc = "Register `compare1` reader"]
pub type R = crate::R<Compare1Spec>;
#[doc = "Register `compare1` writer"]
pub type W = crate::W<Compare1Spec>;
#[doc = "Field `value` reader - Compare 1 value\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `value` writer - Compare 1 value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 1 value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 1 value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<Compare1Spec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Compare 1\n\nYou can [`read`](crate::Reg::read) this register and get [`compare1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compare1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Compare1Spec;
impl crate::RegisterSpec for Compare1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compare1::R`](R) reader structure"]
impl crate::Readable for Compare1Spec {}
#[doc = "`write(|w| ..)` method takes [`compare1::W`](W) writer structure"]
impl crate::Writable for Compare1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets compare1 to value 0"]
impl crate::Resettable for Compare1Spec {
    const RESET_VALUE: u32 = 0;
}
