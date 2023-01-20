#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    #[doc = "0x04 - No Description"]
    pub en: EN,
    #[doc = "0x08 - No Description"]
    pub cfg0: CFG0,
    #[doc = "0x0c - No Description"]
    pub cfg1: CFG1,
    #[doc = "0x10 - No Description"]
    pub cfg2: CFG2,
    #[doc = "0x14 - No Description"]
    pub framecfg: FRAMECFG,
    #[doc = "0x18 - No Description"]
    pub dtxdatcfg: DTXDATCFG,
    #[doc = "0x1c - No Description"]
    pub irhfcfg: IRHFCFG,
    #[doc = "0x20 - No Description"]
    pub irlfcfg: IRLFCFG,
    #[doc = "0x24 - No Description"]
    pub timingcfg: TIMINGCFG,
    #[doc = "0x28 - No Description"]
    pub startframecfg: STARTFRAMECFG,
    #[doc = "0x2c - No Description"]
    pub sigframecfg: SIGFRAMECFG,
    #[doc = "0x30 - No Description"]
    pub clkdiv: CLKDIV,
    #[doc = "0x34 - No Description"]
    pub trigctrl: TRIGCTRL,
    #[doc = "0x38 - No Description"]
    pub cmd: CMD,
    #[doc = "0x3c - No Description"]
    pub rxdata: RXDATA,
    #[doc = "0x40 - No Description"]
    pub rxdatap: RXDATAP,
    #[doc = "0x44 - No Description"]
    pub txdata: TXDATA,
    #[doc = "0x48 - No Description"]
    pub status: STATUS,
    #[doc = "0x4c - No Description"]
    pub if_: IF,
    #[doc = "0x50 - No Description"]
    pub ien: IEN,
    #[doc = "0x54 - No Description"]
    pub syncbusy: SYNCBUSY,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "No Description"]
pub mod en;
#[doc = "CFG0 (rw) register accessor: an alias for `Reg<CFG0_SPEC>`"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "No Description"]
pub mod cfg0;
#[doc = "CFG1 (rw) register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "No Description"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "No Description"]
pub mod cfg2;
#[doc = "FRAMECFG (rw) register accessor: an alias for `Reg<FRAMECFG_SPEC>`"]
pub type FRAMECFG = crate::Reg<framecfg::FRAMECFG_SPEC>;
#[doc = "No Description"]
pub mod framecfg;
#[doc = "DTXDATCFG (rw) register accessor: an alias for `Reg<DTXDATCFG_SPEC>`"]
pub type DTXDATCFG = crate::Reg<dtxdatcfg::DTXDATCFG_SPEC>;
#[doc = "No Description"]
pub mod dtxdatcfg;
#[doc = "IRHFCFG (rw) register accessor: an alias for `Reg<IRHFCFG_SPEC>`"]
pub type IRHFCFG = crate::Reg<irhfcfg::IRHFCFG_SPEC>;
#[doc = "No Description"]
pub mod irhfcfg;
#[doc = "IRLFCFG (rw) register accessor: an alias for `Reg<IRLFCFG_SPEC>`"]
pub type IRLFCFG = crate::Reg<irlfcfg::IRLFCFG_SPEC>;
#[doc = "No Description"]
pub mod irlfcfg;
#[doc = "TIMINGCFG (rw) register accessor: an alias for `Reg<TIMINGCFG_SPEC>`"]
pub type TIMINGCFG = crate::Reg<timingcfg::TIMINGCFG_SPEC>;
#[doc = "No Description"]
pub mod timingcfg;
#[doc = "STARTFRAMECFG (rw) register accessor: an alias for `Reg<STARTFRAMECFG_SPEC>`"]
pub type STARTFRAMECFG = crate::Reg<startframecfg::STARTFRAMECFG_SPEC>;
#[doc = "No Description"]
pub mod startframecfg;
#[doc = "SIGFRAMECFG (rw) register accessor: an alias for `Reg<SIGFRAMECFG_SPEC>`"]
pub type SIGFRAMECFG = crate::Reg<sigframecfg::SIGFRAMECFG_SPEC>;
#[doc = "No Description"]
pub mod sigframecfg;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "No Description"]
pub mod clkdiv;
#[doc = "TRIGCTRL (rw) register accessor: an alias for `Reg<TRIGCTRL_SPEC>`"]
pub type TRIGCTRL = crate::Reg<trigctrl::TRIGCTRL_SPEC>;
#[doc = "No Description"]
pub mod trigctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "RXDATA (r) register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "No Description"]
pub mod rxdata;
#[doc = "RXDATAP (r) register accessor: an alias for `Reg<RXDATAP_SPEC>`"]
pub type RXDATAP = crate::Reg<rxdatap::RXDATAP_SPEC>;
#[doc = "No Description"]
pub mod rxdatap;
#[doc = "TXDATA (w) register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "No Description"]
pub mod txdata;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
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
