#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    #[doc = "0x04 - No Description"]
    pub en: EN,
    #[doc = "0x08 - No Description"]
    pub ctrl: CTRL,
    #[doc = "0x0c - No Description"]
    pub cmd: CMD,
    #[doc = "0x10 - No Description"]
    pub state: STATE,
    #[doc = "0x14 - No Description"]
    pub status: STATUS,
    #[doc = "0x18 - No Description"]
    pub clkdiv: CLKDIV,
    #[doc = "0x1c - No Description"]
    pub saddr: SADDR,
    #[doc = "0x20 - No Description"]
    pub saddrmask: SADDRMASK,
    #[doc = "0x24 - No Description"]
    pub rxdata: RXDATA,
    #[doc = "0x28 - No Description"]
    pub rxdouble: RXDOUBLE,
    #[doc = "0x2c - No Description"]
    pub rxdatap: RXDATAP,
    #[doc = "0x30 - No Description"]
    pub rxdoublep: RXDOUBLEP,
    #[doc = "0x34 - No Description"]
    pub txdata: TXDATA,
    #[doc = "0x38 - No Description"]
    pub txdouble: TXDOUBLE,
    #[doc = "0x3c - No Description"]
    pub if_: IF,
    #[doc = "0x40 - No Description"]
    pub ien: IEN,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "No Description"]
pub mod en;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "STATE (r) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "No Description"]
pub mod state;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "No Description"]
pub mod clkdiv;
#[doc = "SADDR (rw) register accessor: an alias for `Reg<SADDR_SPEC>`"]
pub type SADDR = crate::Reg<saddr::SADDR_SPEC>;
#[doc = "No Description"]
pub mod saddr;
#[doc = "SADDRMASK (rw) register accessor: an alias for `Reg<SADDRMASK_SPEC>`"]
pub type SADDRMASK = crate::Reg<saddrmask::SADDRMASK_SPEC>;
#[doc = "No Description"]
pub mod saddrmask;
#[doc = "RXDATA (r) register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "No Description"]
pub mod rxdata;
#[doc = "RXDOUBLE (r) register accessor: an alias for `Reg<RXDOUBLE_SPEC>`"]
pub type RXDOUBLE = crate::Reg<rxdouble::RXDOUBLE_SPEC>;
#[doc = "No Description"]
pub mod rxdouble;
#[doc = "RXDATAP (r) register accessor: an alias for `Reg<RXDATAP_SPEC>`"]
pub type RXDATAP = crate::Reg<rxdatap::RXDATAP_SPEC>;
#[doc = "No Description"]
pub mod rxdatap;
#[doc = "RXDOUBLEP (r) register accessor: an alias for `Reg<RXDOUBLEP_SPEC>`"]
pub type RXDOUBLEP = crate::Reg<rxdoublep::RXDOUBLEP_SPEC>;
#[doc = "No Description"]
pub mod rxdoublep;
#[doc = "TXDATA (w) register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "No Description"]
pub mod txdata;
#[doc = "TXDOUBLE (w) register accessor: an alias for `Reg<TXDOUBLE_SPEC>`"]
pub type TXDOUBLE = crate::Reg<txdouble::TXDOUBLE_SPEC>;
#[doc = "No Description"]
pub mod txdouble;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
