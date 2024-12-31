#[doc = "Register `Col5KeyState` reader"]
pub type R = crate::R<Col5keyStateSpec>;
#[doc = "Field `state` reader - Colum 5 key state (row 0 is bit 0)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type StateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Colum 5 key state (row 0 is bit 0)"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Column 5 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col5key_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Col5keyStateSpec;
impl crate::RegisterSpec for Col5keyStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`col5key_state::R`](R) reader structure"]
impl crate::Readable for Col5keyStateSpec {}
#[doc = "`reset()` method sets Col5KeyState to value 0"]
impl crate::Resettable for Col5keyStateSpec {
    const RESET_VALUE: u32 = 0;
}
