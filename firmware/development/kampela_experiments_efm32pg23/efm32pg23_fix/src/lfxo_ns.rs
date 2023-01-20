#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    #[doc = "0x04 - No Description"]
    pub ctrl: CTRL,
    #[doc = "0x08 - Do not write to this register unless the oscillator is forced off. The oscillator is forced off if DISONDEMAND is set and FORCEEN is cleared."]
    pub cfg: CFG,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - No Description"]
    pub status: STATUS,
    #[doc = "0x14 - Do not write to this register unless CALBSY in SYNCBUSY register is low."]
    pub cal: CAL,
    #[doc = "0x18 - No Description"]
    pub if_: IF,
    #[doc = "0x1c - No Description"]
    pub ien: IEN,
    #[doc = "0x20 - No Description"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x24 - No Description"]
    pub lock: LOCK,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Do not write to this register unless the oscillator is forced off. The oscillator is forced off if DISONDEMAND is set and FORCEEN is cleared."]
pub mod cfg;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "CAL (rw) register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Do not write to this register unless CALBSY in SYNCBUSY register is low."]
pub mod cal;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "No Description"]
pub mod syncbusy;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
