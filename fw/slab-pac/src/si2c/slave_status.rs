#[doc = "Register `slaveStatus` reader"]
pub type R = crate::R<SlaveStatusSpec>;
#[doc = "Field `inFrame` reader - In Frame\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type InFrameR = crate::BitReader;
#[doc = "Field `sdaRead` reader - SDA read\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SdaReadR = crate::BitReader;
#[doc = "Field `sclRead` reader - SCL read\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SclReadR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - In Frame"]
    #[inline(always)]
    pub fn in_frame(&self) -> InFrameR {
        InFrameR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDA read"]
    #[inline(always)]
    pub fn sda_read(&self) -> SdaReadR {
        SdaReadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCL read"]
    #[inline(always)]
    pub fn scl_read(&self) -> SclReadR {
        SclReadR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Slave status\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlaveStatusSpec;
impl crate::RegisterSpec for SlaveStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave_status::R`](R) reader structure"]
impl crate::Readable for SlaveStatusSpec {}
#[doc = "`reset()` method sets slaveStatus to value 0"]
impl crate::Resettable for SlaveStatusSpec {
    const RESET_VALUE: u32 = 0;
}
