#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    #[doc = "0x04 - No Description"]
    pub cmd: CMD,
    #[doc = "0x08 - No Description"]
    pub ctrl: CTRL,
    #[doc = "0x0c - No Description"]
    pub eccerraddr0: ECCERRADDR0,
    #[doc = "0x10 - No Description"]
    pub eccerraddr1: ECCERRADDR1,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - No Description"]
    pub eccmerrind: ECCMERRIND,
    #[doc = "0x20 - No Description"]
    pub if_: IF,
    #[doc = "0x24 - No Description"]
    pub ien: IEN,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "ECCERRADDR0 (r) register accessor: an alias for `Reg<ECCERRADDR0_SPEC>`"]
pub type ECCERRADDR0 = crate::Reg<eccerraddr0::ECCERRADDR0_SPEC>;
#[doc = "No Description"]
pub mod eccerraddr0;
#[doc = "ECCERRADDR1 (r) register accessor: an alias for `Reg<ECCERRADDR1_SPEC>`"]
pub type ECCERRADDR1 = crate::Reg<eccerraddr1::ECCERRADDR1_SPEC>;
#[doc = "No Description"]
pub mod eccerraddr1;
#[doc = "ECCMERRIND (r) register accessor: an alias for `Reg<ECCMERRIND_SPEC>`"]
pub type ECCMERRIND = crate::Reg<eccmerrind::ECCMERRIND_SPEC>;
#[doc = "No Description"]
pub mod eccmerrind;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
