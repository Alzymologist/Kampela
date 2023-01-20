#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The read only IPVERSION field gives the version for this module. There may be minor software changes required for modules with different values of IPVERSION."]
    pub ipversion: IPVERSION,
    #[doc = "0x04 - No Description"]
    pub ctrl: CTRL,
    #[doc = "0x08 - No Description"]
    pub pchits: PCHITS,
    #[doc = "0x0c - No Description"]
    pub pcmisses: PCMISSES,
    #[doc = "0x10 - No Description"]
    pub pcahits: PCAHITS,
    #[doc = "0x14 - No Description"]
    pub status: STATUS,
    #[doc = "0x18 - No Description"]
    pub cmd: CMD,
    #[doc = "0x1c - No Description"]
    pub lpmode: LPMODE,
    #[doc = "0x20 - No Description"]
    pub if_: IF,
    #[doc = "0x24 - No Description"]
    pub ien: IEN,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "The read only IPVERSION field gives the version for this module. There may be minor software changes required for modules with different values of IPVERSION."]
pub mod ipversion;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "PCHITS (r) register accessor: an alias for `Reg<PCHITS_SPEC>`"]
pub type PCHITS = crate::Reg<pchits::PCHITS_SPEC>;
#[doc = "No Description"]
pub mod pchits;
#[doc = "PCMISSES (r) register accessor: an alias for `Reg<PCMISSES_SPEC>`"]
pub type PCMISSES = crate::Reg<pcmisses::PCMISSES_SPEC>;
#[doc = "No Description"]
pub mod pcmisses;
#[doc = "PCAHITS (r) register accessor: an alias for `Reg<PCAHITS_SPEC>`"]
pub type PCAHITS = crate::Reg<pcahits::PCAHITS_SPEC>;
#[doc = "No Description"]
pub mod pcahits;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "LPMODE (rw) register accessor: an alias for `Reg<LPMODE_SPEC>`"]
pub type LPMODE = crate::Reg<lpmode::LPMODE_SPEC>;
#[doc = "No Description"]
pub mod lpmode;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
