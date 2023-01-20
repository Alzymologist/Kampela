#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IPVERSION"]
    pub ipversion: IPVERSION,
    #[doc = "0x04 - Enable"]
    pub en: EN,
    #[doc = "0x08 - Software Reset"]
    pub swrst: SWRST,
    #[doc = "0x0c - Config"]
    pub cfg: CFG,
    #[doc = "0x10 - Command"]
    pub cmd: CMD,
    #[doc = "0x14 - Delay"]
    pub delay: DELAY,
    #[doc = "0x18 - Status"]
    pub status: STATUS,
    #[doc = "0x1c - Interrupt Flags"]
    pub if_: IF,
    #[doc = "0x20 - Interrupt Enables"]
    pub ien: IEN,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "IPVERSION"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "Enable"]
pub mod en;
#[doc = "SWRST (rw) register accessor: an alias for `Reg<SWRST_SPEC>`"]
pub type SWRST = crate::Reg<swrst::SWRST_SPEC>;
#[doc = "Software Reset"]
pub mod swrst;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Config"]
pub mod cfg;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command"]
pub mod cmd;
#[doc = "DELAY (rw) register accessor: an alias for `Reg<DELAY_SPEC>`"]
pub type DELAY = crate::Reg<delay::DELAY_SPEC>;
#[doc = "Delay"]
pub mod delay;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flags"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enables"]
pub mod ien;
