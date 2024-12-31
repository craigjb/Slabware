#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    scan_rate: ScanRate,
    col0key_state: Col0keyState,
    col1key_state: Col1keyState,
    col2key_state: Col2keyState,
    col3key_state: Col3keyState,
    col4key_state: Col4keyState,
    col5key_state: Col5keyState,
    col6key_state: Col6keyState,
    col7key_state: Col7keyState,
    col8key_state: Col8keyState,
    col9key_state: Col9keyState,
    col10key_state: Col10keyState,
    col11key_state: Col11keyState,
    col12key_state: Col12keyState,
    col13key_state: Col13keyState,
    col14key_state: Col14keyState,
    col15key_state: Col15keyState,
    col16key_state: Col16keyState,
    col17key_state: Col17keyState,
}
impl RegisterBlock {
    #[doc = "0x00 - SlabGrid control"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - Grid key scan rate"]
    #[inline(always)]
    pub const fn scan_rate(&self) -> &ScanRate {
        &self.scan_rate
    }
    #[doc = "0x08 - Column 0 key state"]
    #[inline(always)]
    pub const fn col0key_state(&self) -> &Col0keyState {
        &self.col0key_state
    }
    #[doc = "0x0c - Column 1 key state"]
    #[inline(always)]
    pub const fn col1key_state(&self) -> &Col1keyState {
        &self.col1key_state
    }
    #[doc = "0x10 - Column 2 key state"]
    #[inline(always)]
    pub const fn col2key_state(&self) -> &Col2keyState {
        &self.col2key_state
    }
    #[doc = "0x14 - Column 3 key state"]
    #[inline(always)]
    pub const fn col3key_state(&self) -> &Col3keyState {
        &self.col3key_state
    }
    #[doc = "0x18 - Column 4 key state"]
    #[inline(always)]
    pub const fn col4key_state(&self) -> &Col4keyState {
        &self.col4key_state
    }
    #[doc = "0x1c - Column 5 key state"]
    #[inline(always)]
    pub const fn col5key_state(&self) -> &Col5keyState {
        &self.col5key_state
    }
    #[doc = "0x20 - Column 6 key state"]
    #[inline(always)]
    pub const fn col6key_state(&self) -> &Col6keyState {
        &self.col6key_state
    }
    #[doc = "0x24 - Column 7 key state"]
    #[inline(always)]
    pub const fn col7key_state(&self) -> &Col7keyState {
        &self.col7key_state
    }
    #[doc = "0x28 - Column 8 key state"]
    #[inline(always)]
    pub const fn col8key_state(&self) -> &Col8keyState {
        &self.col8key_state
    }
    #[doc = "0x2c - Column 9 key state"]
    #[inline(always)]
    pub const fn col9key_state(&self) -> &Col9keyState {
        &self.col9key_state
    }
    #[doc = "0x30 - Column 10 key state"]
    #[inline(always)]
    pub const fn col10key_state(&self) -> &Col10keyState {
        &self.col10key_state
    }
    #[doc = "0x34 - Column 11 key state"]
    #[inline(always)]
    pub const fn col11key_state(&self) -> &Col11keyState {
        &self.col11key_state
    }
    #[doc = "0x38 - Column 12 key state"]
    #[inline(always)]
    pub const fn col12key_state(&self) -> &Col12keyState {
        &self.col12key_state
    }
    #[doc = "0x3c - Column 13 key state"]
    #[inline(always)]
    pub const fn col13key_state(&self) -> &Col13keyState {
        &self.col13key_state
    }
    #[doc = "0x40 - Column 14 key state"]
    #[inline(always)]
    pub const fn col14key_state(&self) -> &Col14keyState {
        &self.col14key_state
    }
    #[doc = "0x44 - Column 15 key state"]
    #[inline(always)]
    pub const fn col15key_state(&self) -> &Col15keyState {
        &self.col15key_state
    }
    #[doc = "0x48 - Column 16 key state"]
    #[inline(always)]
    pub const fn col16key_state(&self) -> &Col16keyState {
        &self.col16key_state
    }
    #[doc = "0x4c - Column 17 key state"]
    #[inline(always)]
    pub const fn col17key_state(&self) -> &Col17keyState {
        &self.col17key_state
    }
}
#[doc = "Control (rw) register accessor: SlabGrid control\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "SlabGrid control"]
pub mod control;
#[doc = "ScanRate (r) register accessor: Grid key scan rate\n\nYou can [`read`](crate::Reg::read) this register and get [`scan_rate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan_rate`]
module"]
pub type ScanRate = crate::Reg<scan_rate::ScanRateSpec>;
#[doc = "Grid key scan rate"]
pub mod scan_rate;
#[doc = "Col0KeyState (r) register accessor: Column 0 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col0key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col0key_state`]
module"]
#[doc(alias = "Col0KeyState")]
pub type Col0keyState = crate::Reg<col0key_state::Col0keyStateSpec>;
#[doc = "Column 0 key state"]
pub mod col0key_state;
#[doc = "Col1KeyState (r) register accessor: Column 1 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col1key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col1key_state`]
module"]
#[doc(alias = "Col1KeyState")]
pub type Col1keyState = crate::Reg<col1key_state::Col1keyStateSpec>;
#[doc = "Column 1 key state"]
pub mod col1key_state;
#[doc = "Col2KeyState (r) register accessor: Column 2 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col2key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col2key_state`]
module"]
#[doc(alias = "Col2KeyState")]
pub type Col2keyState = crate::Reg<col2key_state::Col2keyStateSpec>;
#[doc = "Column 2 key state"]
pub mod col2key_state;
#[doc = "Col3KeyState (r) register accessor: Column 3 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col3key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col3key_state`]
module"]
#[doc(alias = "Col3KeyState")]
pub type Col3keyState = crate::Reg<col3key_state::Col3keyStateSpec>;
#[doc = "Column 3 key state"]
pub mod col3key_state;
#[doc = "Col4KeyState (r) register accessor: Column 4 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col4key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col4key_state`]
module"]
#[doc(alias = "Col4KeyState")]
pub type Col4keyState = crate::Reg<col4key_state::Col4keyStateSpec>;
#[doc = "Column 4 key state"]
pub mod col4key_state;
#[doc = "Col5KeyState (r) register accessor: Column 5 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col5key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col5key_state`]
module"]
#[doc(alias = "Col5KeyState")]
pub type Col5keyState = crate::Reg<col5key_state::Col5keyStateSpec>;
#[doc = "Column 5 key state"]
pub mod col5key_state;
#[doc = "Col6KeyState (r) register accessor: Column 6 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col6key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col6key_state`]
module"]
#[doc(alias = "Col6KeyState")]
pub type Col6keyState = crate::Reg<col6key_state::Col6keyStateSpec>;
#[doc = "Column 6 key state"]
pub mod col6key_state;
#[doc = "Col7KeyState (r) register accessor: Column 7 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col7key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col7key_state`]
module"]
#[doc(alias = "Col7KeyState")]
pub type Col7keyState = crate::Reg<col7key_state::Col7keyStateSpec>;
#[doc = "Column 7 key state"]
pub mod col7key_state;
#[doc = "Col8KeyState (r) register accessor: Column 8 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col8key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col8key_state`]
module"]
#[doc(alias = "Col8KeyState")]
pub type Col8keyState = crate::Reg<col8key_state::Col8keyStateSpec>;
#[doc = "Column 8 key state"]
pub mod col8key_state;
#[doc = "Col9KeyState (r) register accessor: Column 9 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col9key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col9key_state`]
module"]
#[doc(alias = "Col9KeyState")]
pub type Col9keyState = crate::Reg<col9key_state::Col9keyStateSpec>;
#[doc = "Column 9 key state"]
pub mod col9key_state;
#[doc = "Col10KeyState (r) register accessor: Column 10 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col10key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col10key_state`]
module"]
#[doc(alias = "Col10KeyState")]
pub type Col10keyState = crate::Reg<col10key_state::Col10keyStateSpec>;
#[doc = "Column 10 key state"]
pub mod col10key_state;
#[doc = "Col11KeyState (r) register accessor: Column 11 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col11key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col11key_state`]
module"]
#[doc(alias = "Col11KeyState")]
pub type Col11keyState = crate::Reg<col11key_state::Col11keyStateSpec>;
#[doc = "Column 11 key state"]
pub mod col11key_state;
#[doc = "Col12KeyState (r) register accessor: Column 12 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col12key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col12key_state`]
module"]
#[doc(alias = "Col12KeyState")]
pub type Col12keyState = crate::Reg<col12key_state::Col12keyStateSpec>;
#[doc = "Column 12 key state"]
pub mod col12key_state;
#[doc = "Col13KeyState (r) register accessor: Column 13 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col13key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col13key_state`]
module"]
#[doc(alias = "Col13KeyState")]
pub type Col13keyState = crate::Reg<col13key_state::Col13keyStateSpec>;
#[doc = "Column 13 key state"]
pub mod col13key_state;
#[doc = "Col14KeyState (r) register accessor: Column 14 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col14key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col14key_state`]
module"]
#[doc(alias = "Col14KeyState")]
pub type Col14keyState = crate::Reg<col14key_state::Col14keyStateSpec>;
#[doc = "Column 14 key state"]
pub mod col14key_state;
#[doc = "Col15KeyState (r) register accessor: Column 15 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col15key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col15key_state`]
module"]
#[doc(alias = "Col15KeyState")]
pub type Col15keyState = crate::Reg<col15key_state::Col15keyStateSpec>;
#[doc = "Column 15 key state"]
pub mod col15key_state;
#[doc = "Col16KeyState (r) register accessor: Column 16 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col16key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col16key_state`]
module"]
#[doc(alias = "Col16KeyState")]
pub type Col16keyState = crate::Reg<col16key_state::Col16keyStateSpec>;
#[doc = "Column 16 key state"]
pub mod col16key_state;
#[doc = "Col17KeyState (r) register accessor: Column 17 key state\n\nYou can [`read`](crate::Reg::read) this register and get [`col17key_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col17key_state`]
module"]
#[doc(alias = "Col17KeyState")]
pub type Col17keyState = crate::Reg<col17key_state::Col17keyStateSpec>;
#[doc = "Column 17 key state"]
pub mod col17key_state;
