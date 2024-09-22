#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tx_data: TxData,
    tx_ack: TxAck,
    rx_data: RxData,
    rx_ack: RxAck,
    _reserved4: [u8; 0x18],
    sampling_clock_divider: SamplingClockDivider,
    timeout: Timeout,
    tsu_data: TsuData,
    _reserved7: [u8; 0x0c],
    master_status: MasterStatus,
    slave_status: SlaveStatus,
    slave_override: SlaveOverride,
    _reserved10: [u8; 0x04],
    tlow: Tlow,
    thigh: Thigh,
    tbuf: Tbuf,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit data"]
    #[inline(always)]
    pub const fn tx_data(&self) -> &TxData {
        &self.tx_data
    }
    #[doc = "0x04 - Transmit acknowledge"]
    #[inline(always)]
    pub const fn tx_ack(&self) -> &TxAck {
        &self.tx_ack
    }
    #[doc = "0x08 - Receive data"]
    #[inline(always)]
    pub const fn rx_data(&self) -> &RxData {
        &self.rx_data
    }
    #[doc = "0x0c - Receive acknowledge"]
    #[inline(always)]
    pub const fn rx_ack(&self) -> &RxAck {
        &self.rx_ack
    }
    #[doc = "0x28 - Sampling clock"]
    #[inline(always)]
    pub const fn sampling_clock_divider(&self) -> &SamplingClockDivider {
        &self.sampling_clock_divider
    }
    #[doc = "0x2c - Timeout"]
    #[inline(always)]
    pub const fn timeout(&self) -> &Timeout {
        &self.timeout
    }
    #[doc = "0x30 - TSU Data"]
    #[inline(always)]
    pub const fn tsu_data(&self) -> &TsuData {
        &self.tsu_data
    }
    #[doc = "0x40 - Master status"]
    #[inline(always)]
    pub const fn master_status(&self) -> &MasterStatus {
        &self.master_status
    }
    #[doc = "0x44 - Slave status"]
    #[inline(always)]
    pub const fn slave_status(&self) -> &SlaveStatus {
        &self.slave_status
    }
    #[doc = "0x48 - Slave override"]
    #[inline(always)]
    pub const fn slave_override(&self) -> &SlaveOverride {
        &self.slave_override
    }
    #[doc = "0x50 - I2C low timing"]
    #[inline(always)]
    pub const fn tlow(&self) -> &Tlow {
        &self.tlow
    }
    #[doc = "0x54 - I2C high timing"]
    #[inline(always)]
    pub const fn thigh(&self) -> &Thigh {
        &self.thigh
    }
    #[doc = "0x58 - I2C idle timing"]
    #[inline(always)]
    pub const fn tbuf(&self) -> &Tbuf {
        &self.tbuf
    }
}
#[doc = "rxData (rw) register accessor: Receive data\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data`]
module"]
#[doc(alias = "rxData")]
pub type RxData = crate::Reg<rx_data::RxDataSpec>;
#[doc = "Receive data"]
pub mod rx_data;
#[doc = "rxAck (rw) register accessor: Receive acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ack`]
module"]
#[doc(alias = "rxAck")]
pub type RxAck = crate::Reg<rx_ack::RxAckSpec>;
#[doc = "Receive acknowledge"]
pub mod rx_ack;
#[doc = "txData (rw) register accessor: Transmit data\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data`]
module"]
#[doc(alias = "txData")]
pub type TxData = crate::Reg<tx_data::TxDataSpec>;
#[doc = "Transmit data"]
pub mod tx_data;
#[doc = "txAck (rw) register accessor: Transmit acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ack`]
module"]
#[doc(alias = "txAck")]
pub type TxAck = crate::Reg<tx_ack::TxAckSpec>;
#[doc = "Transmit acknowledge"]
pub mod tx_ack;
#[doc = "masterStatus (rw) register accessor: Master status\n\nYou can [`read`](crate::Reg::read) this register and get [`master_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master_status`]
module"]
#[doc(alias = "masterStatus")]
pub type MasterStatus = crate::Reg<master_status::MasterStatusSpec>;
#[doc = "Master status"]
pub mod master_status;
#[doc = "TLOW (w) register accessor: I2C low timing\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlow::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlow`]
module"]
#[doc(alias = "TLOW")]
pub type Tlow = crate::Reg<tlow::TlowSpec>;
#[doc = "I2C low timing"]
pub mod tlow;
#[doc = "THIGH (w) register accessor: I2C high timing\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thigh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thigh`]
module"]
#[doc(alias = "THIGH")]
pub type Thigh = crate::Reg<thigh::ThighSpec>;
#[doc = "I2C high timing"]
pub mod thigh;
#[doc = "TBUF (w) register accessor: I2C idle timing\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbuf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbuf`]
module"]
#[doc(alias = "TBUF")]
pub type Tbuf = crate::Reg<tbuf::TbufSpec>;
#[doc = "I2C idle timing"]
pub mod tbuf;
#[doc = "samplingClockDivider (w) register accessor: Sampling clock\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampling_clock_divider::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampling_clock_divider`]
module"]
#[doc(alias = "samplingClockDivider")]
pub type SamplingClockDivider = crate::Reg<sampling_clock_divider::SamplingClockDividerSpec>;
#[doc = "Sampling clock"]
pub mod sampling_clock_divider;
#[doc = "TIMEOUT (w) register accessor: Timeout\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout`]
module"]
#[doc(alias = "TIMEOUT")]
pub type Timeout = crate::Reg<timeout::TimeoutSpec>;
#[doc = "Timeout"]
pub mod timeout;
#[doc = "tsuData (w) register accessor: TSU Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsu_data::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsu_data`]
module"]
#[doc(alias = "tsuData")]
pub type TsuData = crate::Reg<tsu_data::TsuDataSpec>;
#[doc = "TSU Data"]
pub mod tsu_data;
#[doc = "slaveStatus (r) register accessor: Slave status\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave_status`]
module"]
#[doc(alias = "slaveStatus")]
pub type SlaveStatus = crate::Reg<slave_status::SlaveStatusSpec>;
#[doc = "Slave status"]
pub mod slave_status;
#[doc = "slaveOverride (rw) register accessor: Slave override\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_override::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave_override::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave_override`]
module"]
#[doc(alias = "slaveOverride")]
pub type SlaveOverride = crate::Reg<slave_override::SlaveOverrideSpec>;
#[doc = "Slave override"]
pub mod slave_override;
