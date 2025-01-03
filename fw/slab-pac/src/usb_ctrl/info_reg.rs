#[doc = "Register `infoReg` reader"]
pub type R = crate::R<InfoRegSpec>;
#[doc = "Field `ramSize` reader - Internal ram address width (bits)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type RamSizeR = crate::FieldReader;
#[doc = "Field `powerDetected` reader - USB bus power detected\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type PowerDetectedR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Internal ram address width (bits)"]
    #[inline(always)]
    pub fn ram_size(&self) -> RamSizeR {
        RamSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - USB bus power detected"]
    #[inline(always)]
    pub fn power_detected(&self) -> PowerDetectedR {
        PowerDetectedR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Info\n\nYou can [`read`](crate::Reg::read) this register and get [`info_reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfoRegSpec;
impl crate::RegisterSpec for InfoRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info_reg::R`](R) reader structure"]
impl crate::Readable for InfoRegSpec {}
#[doc = "`reset()` method sets infoReg to value 0"]
impl crate::Resettable for InfoRegSpec {
    const RESET_VALUE: u32 = 0;
}
