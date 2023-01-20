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
    pub frame: FRAME,
    #[doc = "0x10 - No Description"]
    pub trigctrl: TRIGCTRL,
    #[doc = "0x14 - No Description"]
    pub cmd: CMD,
    #[doc = "0x18 - No Description"]
    pub status: STATUS,
    #[doc = "0x1c - No Description"]
    pub clkdiv: CLKDIV,
    #[doc = "0x20 - No Description"]
    pub rxdatax: RXDATAX,
    #[doc = "0x24 - No Description"]
    pub rxdata: RXDATA,
    #[doc = "0x28 - No Description"]
    pub rxdoublex: RXDOUBLEX,
    #[doc = "0x2c - No Description"]
    pub rxdouble: RXDOUBLE,
    #[doc = "0x30 - No Description"]
    pub rxdataxp: RXDATAXP,
    #[doc = "0x34 - No Description"]
    pub rxdoublexp: RXDOUBLEXP,
    #[doc = "0x38 - No Description"]
    pub txdatax: TXDATAX,
    #[doc = "0x3c - No Description"]
    pub txdata: TXDATA,
    #[doc = "0x40 - No Description"]
    pub txdoublex: TXDOUBLEX,
    #[doc = "0x44 - No Description"]
    pub txdouble: TXDOUBLE,
    #[doc = "0x48 - No Description"]
    pub if_: IF,
    #[doc = "0x4c - No Description"]
    pub ien: IEN,
    #[doc = "0x50 - No Description"]
    pub irctrl: IRCTRL,
    #[doc = "0x54 - No Description"]
    pub i2sctrl: I2SCTRL,
    #[doc = "0x58 - No Description"]
    pub timing: TIMING,
    #[doc = "0x5c - No Description"]
    pub ctrlx: CTRLX,
    #[doc = "0x60 - No Description"]
    pub timecmp0: TIMECMP0,
    #[doc = "0x64 - No Description"]
    pub timecmp1: TIMECMP1,
    #[doc = "0x68 - No Description"]
    pub timecmp2: TIMECMP2,
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
#[doc = "FRAME (rw) register accessor: an alias for `Reg<FRAME_SPEC>`"]
pub type FRAME = crate::Reg<frame::FRAME_SPEC>;
#[doc = "No Description"]
pub mod frame;
#[doc = "TRIGCTRL (rw) register accessor: an alias for `Reg<TRIGCTRL_SPEC>`"]
pub type TRIGCTRL = crate::Reg<trigctrl::TRIGCTRL_SPEC>;
#[doc = "No Description"]
pub mod trigctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "No Description"]
pub mod clkdiv;
#[doc = "RXDATAX (r) register accessor: an alias for `Reg<RXDATAX_SPEC>`"]
pub type RXDATAX = crate::Reg<rxdatax::RXDATAX_SPEC>;
#[doc = "No Description"]
pub mod rxdatax;
#[doc = "RXDATA (r) register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "No Description"]
pub mod rxdata;
#[doc = "RXDOUBLEX (r) register accessor: an alias for `Reg<RXDOUBLEX_SPEC>`"]
pub type RXDOUBLEX = crate::Reg<rxdoublex::RXDOUBLEX_SPEC>;
#[doc = "No Description"]
pub mod rxdoublex;
#[doc = "RXDOUBLE (r) register accessor: an alias for `Reg<RXDOUBLE_SPEC>`"]
pub type RXDOUBLE = crate::Reg<rxdouble::RXDOUBLE_SPEC>;
#[doc = "No Description"]
pub mod rxdouble;
#[doc = "RXDATAXP (r) register accessor: an alias for `Reg<RXDATAXP_SPEC>`"]
pub type RXDATAXP = crate::Reg<rxdataxp::RXDATAXP_SPEC>;
#[doc = "No Description"]
pub mod rxdataxp;
#[doc = "RXDOUBLEXP (r) register accessor: an alias for `Reg<RXDOUBLEXP_SPEC>`"]
pub type RXDOUBLEXP = crate::Reg<rxdoublexp::RXDOUBLEXP_SPEC>;
#[doc = "No Description"]
pub mod rxdoublexp;
#[doc = "TXDATAX (w) register accessor: an alias for `Reg<TXDATAX_SPEC>`"]
pub type TXDATAX = crate::Reg<txdatax::TXDATAX_SPEC>;
#[doc = "No Description"]
pub mod txdatax;
#[doc = "TXDATA (w) register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "No Description"]
pub mod txdata;
#[doc = "TXDOUBLEX (w) register accessor: an alias for `Reg<TXDOUBLEX_SPEC>`"]
pub type TXDOUBLEX = crate::Reg<txdoublex::TXDOUBLEX_SPEC>;
#[doc = "No Description"]
pub mod txdoublex;
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
#[doc = "IRCTRL (rw) register accessor: an alias for `Reg<IRCTRL_SPEC>`"]
pub type IRCTRL = crate::Reg<irctrl::IRCTRL_SPEC>;
#[doc = "No Description"]
pub mod irctrl;
#[doc = "I2SCTRL (rw) register accessor: an alias for `Reg<I2SCTRL_SPEC>`"]
pub type I2SCTRL = crate::Reg<i2sctrl::I2SCTRL_SPEC>;
#[doc = "No Description"]
pub mod i2sctrl;
#[doc = "TIMING (rw) register accessor: an alias for `Reg<TIMING_SPEC>`"]
pub type TIMING = crate::Reg<timing::TIMING_SPEC>;
#[doc = "No Description"]
pub mod timing;
#[doc = "CTRLX (rw) register accessor: an alias for `Reg<CTRLX_SPEC>`"]
pub type CTRLX = crate::Reg<ctrlx::CTRLX_SPEC>;
#[doc = "No Description"]
pub mod ctrlx;
#[doc = "TIMECMP0 (rw) register accessor: an alias for `Reg<TIMECMP0_SPEC>`"]
pub type TIMECMP0 = crate::Reg<timecmp0::TIMECMP0_SPEC>;
#[doc = "No Description"]
pub mod timecmp0;
#[doc = "TIMECMP1 (rw) register accessor: an alias for `Reg<TIMECMP1_SPEC>`"]
pub type TIMECMP1 = crate::Reg<timecmp1::TIMECMP1_SPEC>;
#[doc = "No Description"]
pub mod timecmp1;
#[doc = "TIMECMP2 (rw) register accessor: an alias for `Reg<TIMECMP2_SPEC>`"]
pub type TIMECMP2 = crate::Reg<timecmp2::TIMECMP2_SPEC>;
#[doc = "No Description"]
pub mod timecmp2;
