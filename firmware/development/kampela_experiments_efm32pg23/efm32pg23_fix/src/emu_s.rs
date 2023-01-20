#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - No Description"]
    pub decbod: DECBOD,
    _reserved1: [u8; 0x0c],
    #[doc = "0x20 - No Description"]
    pub bod3sense: BOD3SENSE,
    _reserved2: [u8; 0x18],
    #[doc = "0x3c - No Description"]
    pub vregvddcmpctrl: VREGVDDCMPCTRL,
    #[doc = "0x40 - No Description"]
    pub pd1paretctrl: PD1PARETCTRL,
    _reserved4: [u8; 0x18],
    #[doc = "0x5c - IP Version"]
    pub ipversion: IPVERSION,
    #[doc = "0x60 - No Description"]
    pub lock: LOCK,
    #[doc = "0x64 - No Description"]
    pub if_: IF,
    #[doc = "0x68 - No Description"]
    pub ien: IEN,
    #[doc = "0x6c - No Description"]
    pub em4ctrl: EM4CTRL,
    #[doc = "0x70 - No Description"]
    pub cmd: CMD,
    #[doc = "0x74 - No Description"]
    pub ctrl: CTRL,
    #[doc = "0x78 - No Description"]
    pub templimits: TEMPLIMITS,
    _reserved12: [u8; 0x08],
    #[doc = "0x84 - No Description"]
    pub status: STATUS,
    #[doc = "0x88 - No Description"]
    pub temp: TEMP,
    _reserved14: [u8; 0x04],
    #[doc = "0x90 - No Description"]
    pub rstctrl: RSTCTRL,
    #[doc = "0x94 - No Description"]
    pub rstcause: RSTCAUSE,
    #[doc = "0x98 - No Description"]
    pub tamperrstcause: TAMPERRSTCAUSE,
    _reserved17: [u8; 0x04],
    #[doc = "0xa0 - No Description"]
    pub dgif: DGIF,
    #[doc = "0xa4 - No Description"]
    pub dgien: DGIEN,
    _reserved19: [u8; 0x58],
    #[doc = "0x100 - No Description"]
    pub efpif: EFPIF,
    #[doc = "0x104 - No Description"]
    pub efpien: EFPIEN,
}
#[doc = "DECBOD (rw) register accessor: an alias for `Reg<DECBOD_SPEC>`"]
pub type DECBOD = crate::Reg<decbod::DECBOD_SPEC>;
#[doc = "No Description"]
pub mod decbod;
#[doc = "BOD3SENSE (rw) register accessor: an alias for `Reg<BOD3SENSE_SPEC>`"]
pub type BOD3SENSE = crate::Reg<bod3sense::BOD3SENSE_SPEC>;
#[doc = "No Description"]
pub mod bod3sense;
#[doc = "VREGVDDCMPCTRL (rw) register accessor: an alias for `Reg<VREGVDDCMPCTRL_SPEC>`"]
pub type VREGVDDCMPCTRL = crate::Reg<vregvddcmpctrl::VREGVDDCMPCTRL_SPEC>;
#[doc = "No Description"]
pub mod vregvddcmpctrl;
#[doc = "PD1PARETCTRL (rw) register accessor: an alias for `Reg<PD1PARETCTRL_SPEC>`"]
pub type PD1PARETCTRL = crate::Reg<pd1paretctrl::PD1PARETCTRL_SPEC>;
#[doc = "No Description"]
pub mod pd1paretctrl;
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "IP Version"]
pub mod ipversion;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "EM4CTRL (rw) register accessor: an alias for `Reg<EM4CTRL_SPEC>`"]
pub type EM4CTRL = crate::Reg<em4ctrl::EM4CTRL_SPEC>;
#[doc = "No Description"]
pub mod em4ctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "TEMPLIMITS (rw) register accessor: an alias for `Reg<TEMPLIMITS_SPEC>`"]
pub type TEMPLIMITS = crate::Reg<templimits::TEMPLIMITS_SPEC>;
#[doc = "No Description"]
pub mod templimits;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "TEMP (r) register accessor: an alias for `Reg<TEMP_SPEC>`"]
pub type TEMP = crate::Reg<temp::TEMP_SPEC>;
#[doc = "No Description"]
pub mod temp;
#[doc = "RSTCTRL (rw) register accessor: an alias for `Reg<RSTCTRL_SPEC>`"]
pub type RSTCTRL = crate::Reg<rstctrl::RSTCTRL_SPEC>;
#[doc = "No Description"]
pub mod rstctrl;
#[doc = "RSTCAUSE (r) register accessor: an alias for `Reg<RSTCAUSE_SPEC>`"]
pub type RSTCAUSE = crate::Reg<rstcause::RSTCAUSE_SPEC>;
#[doc = "No Description"]
pub mod rstcause;
#[doc = "TAMPERRSTCAUSE (r) register accessor: an alias for `Reg<TAMPERRSTCAUSE_SPEC>`"]
pub type TAMPERRSTCAUSE = crate::Reg<tamperrstcause::TAMPERRSTCAUSE_SPEC>;
#[doc = "No Description"]
pub mod tamperrstcause;
#[doc = "DGIF (rw) register accessor: an alias for `Reg<DGIF_SPEC>`"]
pub type DGIF = crate::Reg<dgif::DGIF_SPEC>;
#[doc = "No Description"]
pub mod dgif;
#[doc = "DGIEN (rw) register accessor: an alias for `Reg<DGIEN_SPEC>`"]
pub type DGIEN = crate::Reg<dgien::DGIEN_SPEC>;
#[doc = "No Description"]
pub mod dgien;
#[doc = "EFPIF (rw) register accessor: an alias for `Reg<EFPIF_SPEC>`"]
pub type EFPIF = crate::Reg<efpif::EFPIF_SPEC>;
#[doc = "No Description"]
pub mod efpif;
#[doc = "EFPIEN (rw) register accessor: an alias for `Reg<EFPIEN_SPEC>`"]
pub type EFPIEN = crate::Reg<efpien::EFPIEN_SPEC>;
#[doc = "No Description"]
pub mod efpien;
