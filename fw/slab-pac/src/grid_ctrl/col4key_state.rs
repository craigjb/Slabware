#[doc = "Register `Col4KeyState` reader"]
pub type R = crate::R<Col4keyStateSpec>;
#[doc = "Field `state` reader - Colum 4 key state (row 0 is bit 0)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type StateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Colum 4 key state (row 0 is bit 0)"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Column 4 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col4key_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Col4keyStateSpec;
impl crate::RegisterSpec for Col4keyStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`col4key_state::R`](R) reader structure"]
impl crate::Readable for Col4keyStateSpec {}
#[doc = "`reset()` method sets Col4KeyState to value 0"]
impl crate::Resettable for Col4keyStateSpec {
    const RESET_VALUE: u32 = 0;
}
