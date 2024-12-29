#[doc = "Register `Duty` reader"]
pub type R = crate::R<DutySpec>;
#[doc = "Register `Duty` writer"]
pub type W = crate::W<DutySpec>;
#[doc = "Field `value` reader - PWM duty value\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `value` writer - PWM duty value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PWM duty value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PWM duty value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<DutySpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "PWM duty\n\nYou can [`read`](crate::Reg::read) this register and get [`duty::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duty::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DutySpec;
impl crate::RegisterSpec for DutySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`duty::R`](R) reader structure"]
impl crate::Readable for DutySpec {}
#[doc = "`write(|w| ..)` method takes [`duty::W`](W) writer structure"]
impl crate::Writable for DutySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Duty to value 0x7f"]
impl crate::Resettable for DutySpec {
    const RESET_VALUE: u32 = 0x7f;
}
