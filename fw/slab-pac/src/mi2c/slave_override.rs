#[doc = "Register `slaveOverride` reader"]
pub type R = crate::R<SlaveOverrideSpec>;
#[doc = "Register `slaveOverride` writer"]
pub type W = crate::W<SlaveOverrideSpec>;
#[doc = "Field `sda` reader - Force the SDA pin low when cleared\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SdaR = crate::BitReader;
#[doc = "Field `sda` writer - Force the SDA pin low when cleared"]
pub type SdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scl` reader - Force the SCL pin low when cleared\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SclR = crate::BitReader;
#[doc = "Field `scl` writer - Force the SCL pin low when cleared"]
pub type SclW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force the SDA pin low when cleared"]
    #[inline(always)]
    pub fn sda(&self) -> SdaR {
        SdaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force the SCL pin low when cleared"]
    #[inline(always)]
    pub fn scl(&self) -> SclR {
        SclR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force the SDA pin low when cleared"]
    #[inline(always)]
    #[must_use]
    pub fn sda(&mut self) -> SdaW<SlaveOverrideSpec> {
        SdaW::new(self, 0)
    }
    #[doc = "Bit 1 - Force the SCL pin low when cleared"]
    #[inline(always)]
    #[must_use]
    pub fn scl(&mut self) -> SclW<SlaveOverrideSpec> {
        SclW::new(self, 1)
    }
}
#[doc = "Slave override\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_override::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave_override::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlaveOverrideSpec;
impl crate::RegisterSpec for SlaveOverrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave_override::R`](R) reader structure"]
impl crate::Readable for SlaveOverrideSpec {}
#[doc = "`write(|w| ..)` method takes [`slave_override::W`](W) writer structure"]
impl crate::Writable for SlaveOverrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets slaveOverride to value 0x03"]
impl crate::Resettable for SlaveOverrideSpec {
    const RESET_VALUE: u32 = 0x03;
}
