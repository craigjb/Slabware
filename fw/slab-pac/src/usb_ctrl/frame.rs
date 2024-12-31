#[doc = "Register `Frame` reader"]
pub type R = crate::R<FrameSpec>;
#[doc = "Field `usbFrameId` reader - Current USB frame ID\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type UsbFrameIdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Current USB frame ID"]
    #[inline(always)]
    pub fn usb_frame_id(&self) -> UsbFrameIdR {
        UsbFrameIdR::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "USB frame id\n\nYou can [`read`](crate::Reg::read) this register and get [`frame::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameSpec;
impl crate::RegisterSpec for FrameSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame::R`](R) reader structure"]
impl crate::Readable for FrameSpec {}
#[doc = "`reset()` method sets Frame to value 0"]
impl crate::Resettable for FrameSpec {
    const RESET_VALUE: u32 = 0;
}
