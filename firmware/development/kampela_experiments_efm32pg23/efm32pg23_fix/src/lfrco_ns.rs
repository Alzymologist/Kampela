#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Contains the LFRCO ip version."]
    pub ipversion: IPVERSION,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Status register"]
    pub status: STATUS,
    #[doc = "0x0c - Calibration register"]
    pub cal: CAL,
    _reserved3: [u8; 0x04],
    #[doc = "0x14 - Interrupt flag register"]
    pub if_: IF,
    #[doc = "0x18 - Interrupt enable register."]
    pub ien: IEN,
    #[doc = "0x1c - Synchronization busy register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x20 - Configuration lock register. Locks/unlocks access to cofiguration registers."]
    pub lock: LOCK,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "Contains the LFRCO ip version."]
pub mod ipversion;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "CAL (rw) register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration register"]
pub mod cal;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt flag register"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt enable register."]
pub mod ien;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization busy register"]
pub mod syncbusy;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration lock register. Locks/unlocks access to cofiguration registers."]
pub mod lock;
