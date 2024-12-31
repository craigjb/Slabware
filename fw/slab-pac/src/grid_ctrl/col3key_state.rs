#[doc = "Register `Col3KeyState` reader"]
pub type R = crate::R<Col3keyStateSpec>;
#[doc = "Field `state` reader - Colum 3 key state (row 0 is bit 0)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type StateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Colum 3 key state (row 0 is bit 0)"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Column 3 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col3key_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Col3keyStateSpec;
impl crate::RegisterSpec for Col3keyStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`col3key_state::R`](R) reader structure"]
impl crate::Readable for Col3keyStateSpec {}
#[doc = "`reset()` method sets Col3KeyState to value 0"]
impl crate::Resettable for Col3keyStateSpec {
    const RESET_VALUE: u32 = 0;
}
