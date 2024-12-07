#[doc = "Register `counter` reader"]
pub type R = crate::R<CounterSpec>;
#[doc = "Register `counter` writer"]
pub type W = crate::W<CounterSpec>;
#[doc = "Field `counter` reader - Counter value\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type CounterR = crate::FieldReader<u16>;
#[doc = "Field `counter` writer - Counter value"]
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> CounterW<CounterSpec> {
        CounterW::new(self, 0)
    }
}
#[doc = "Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`counter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CounterSpec;
impl crate::RegisterSpec for CounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`counter::R`](R) reader structure"]
impl crate::Readable for CounterSpec {}
#[doc = "`write(|w| ..)` method takes [`counter::W`](W) writer structure"]
impl crate::Writable for CounterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets counter to value 0"]
impl crate::Resettable for CounterSpec {
    const RESET_VALUE: u32 = 0;
}
