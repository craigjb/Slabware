#[doc = "Register `interrupt` reader"]
pub type R = crate::R<InterruptSpec>;
#[doc = "Register `interrupt` writer"]
pub type W = crate::W<InterruptSpec>;
#[doc = "Field `rxDataEnable` reader - RX data interrupt enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type RxDataEnableR = crate::BitReader;
#[doc = "Field `rxDataEnable` writer - RX data interrupt enable"]
pub type RxDataEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxAckEnable` reader - RX ack interrupt enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type RxAckEnableR = crate::BitReader;
#[doc = "Field `rxAckEnable` writer - RX ack interrupt enable"]
pub type RxAckEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txDataEnable` reader - TX data interrupt enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TxDataEnableR = crate::BitReader;
#[doc = "Field `txDataEnable` writer - TX data interrupt enable"]
pub type TxDataEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txAckEnable` reader - TX ack interrupt enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TxAckEnableR = crate::BitReader;
#[doc = "Field `txAckEnable` writer - TX ack interrupt enable"]
pub type TxAckEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `startEnable` reader - I2C Start enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type StartEnableR = crate::BitReader;
#[doc = "Field `startEnable` writer - I2C Start enable"]
pub type StartEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `startFlag` reader - I2C Start flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type StartFlagR = crate::BitReader;
#[doc = "Field `startFlag` writer - I2C Start flag"]
pub type StartFlagW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `restartEnable` reader - I2C Restart enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type RestartEnableR = crate::BitReader;
#[doc = "Field `restartEnable` writer - I2C Restart enable"]
pub type RestartEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `restartFlag` reader - I2C Restart flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type RestartFlagR = crate::BitReader;
#[doc = "Field `restartFlag` writer - I2C Restart flag"]
pub type RestartFlagW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `endEnable` reader - I2C END enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EndEnableR = crate::BitReader;
#[doc = "Field `endEnable` writer - I2C END enable"]
pub type EndEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `endFlag` reader - I2C END flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EndFlagR = crate::BitReader;
#[doc = "Field `endFlag` writer - I2C END flag"]
pub type EndFlagW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `dropEnable` reader - I2C Drop enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DropEnableR = crate::BitReader;
#[doc = "Field `dropEnable` writer - I2C Drop enable"]
pub type DropEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dropFlag` reader - I2C Drop flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DropFlagR = crate::BitReader;
#[doc = "Field `dropFlag` writer - I2C Drop flag"]
pub type DropFlagW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX data interrupt enable"]
    #[inline(always)]
    pub fn rx_data_enable(&self) -> RxDataEnableR {
        RxDataEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX ack interrupt enable"]
    #[inline(always)]
    pub fn rx_ack_enable(&self) -> RxAckEnableR {
        RxAckEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX data interrupt enable"]
    #[inline(always)]
    pub fn tx_data_enable(&self) -> TxDataEnableR {
        TxDataEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX ack interrupt enable"]
    #[inline(always)]
    pub fn tx_ack_enable(&self) -> TxAckEnableR {
        TxAckEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Start enable"]
    #[inline(always)]
    pub fn start_enable(&self) -> StartEnableR {
        StartEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Start flag"]
    #[inline(always)]
    pub fn start_flag(&self) -> StartFlagR {
        StartFlagR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Restart enable"]
    #[inline(always)]
    pub fn restart_enable(&self) -> RestartEnableR {
        RestartEnableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Restart flag"]
    #[inline(always)]
    pub fn restart_flag(&self) -> RestartFlagR {
        RestartFlagR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C END enable"]
    #[inline(always)]
    pub fn end_enable(&self) -> EndEnableR {
        EndEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C END flag"]
    #[inline(always)]
    pub fn end_flag(&self) -> EndFlagR {
        EndFlagR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C Drop enable"]
    #[inline(always)]
    pub fn drop_enable(&self) -> DropEnableR {
        DropEnableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C Drop flag"]
    #[inline(always)]
    pub fn drop_flag(&self) -> DropFlagR {
        DropFlagR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX data interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_enable(&mut self) -> RxDataEnableW<InterruptSpec> {
        RxDataEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - RX ack interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ack_enable(&mut self) -> RxAckEnableW<InterruptSpec> {
        RxAckEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - TX data interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_enable(&mut self) -> TxDataEnableW<InterruptSpec> {
        TxDataEnableW::new(self, 2)
    }
    #[doc = "Bit 3 - TX ack interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ack_enable(&mut self) -> TxAckEnableW<InterruptSpec> {
        TxAckEnableW::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Start enable"]
    #[inline(always)]
    #[must_use]
    pub fn start_enable(&mut self) -> StartEnableW<InterruptSpec> {
        StartEnableW::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Start flag"]
    #[inline(always)]
    #[must_use]
    pub fn start_flag(&mut self) -> StartFlagW<InterruptSpec> {
        StartFlagW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Restart enable"]
    #[inline(always)]
    #[must_use]
    pub fn restart_enable(&mut self) -> RestartEnableW<InterruptSpec> {
        RestartEnableW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Restart flag"]
    #[inline(always)]
    #[must_use]
    pub fn restart_flag(&mut self) -> RestartFlagW<InterruptSpec> {
        RestartFlagW::new(self, 7)
    }
    #[doc = "Bit 8 - I2C END enable"]
    #[inline(always)]
    #[must_use]
    pub fn end_enable(&mut self) -> EndEnableW<InterruptSpec> {
        EndEnableW::new(self, 8)
    }
    #[doc = "Bit 9 - I2C END flag"]
    #[inline(always)]
    #[must_use]
    pub fn end_flag(&mut self) -> EndFlagW<InterruptSpec> {
        EndFlagW::new(self, 9)
    }
    #[doc = "Bit 10 - I2C Drop enable"]
    #[inline(always)]
    #[must_use]
    pub fn drop_enable(&mut self) -> DropEnableW<InterruptSpec> {
        DropEnableW::new(self, 10)
    }
    #[doc = "Bit 11 - I2C Drop flag"]
    #[inline(always)]
    #[must_use]
    pub fn drop_flag(&mut self) -> DropFlagW<InterruptSpec> {
        DropFlagW::new(self, 11)
    }
}
#[doc = "Interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptSpec;
impl crate::RegisterSpec for InterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt::R`](R) reader structure"]
impl crate::Readable for InterruptSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt::W`](W) writer structure"]
impl crate::Writable for InterruptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0aa0;
}
#[doc = "`reset()` method sets interrupt to value 0"]
impl crate::Resettable for InterruptSpec {
    const RESET_VALUE: u32 = 0;
}
