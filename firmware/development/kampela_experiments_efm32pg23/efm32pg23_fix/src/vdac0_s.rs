#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    #[doc = "0x04 - No Description"]
    pub en: EN,
    #[doc = "0x08 - No Description"]
    pub swrst: SWRST,
    #[doc = "0x0c - No Description"]
    pub cfg: CFG,
    #[doc = "0x10 - No Description"]
    pub status: STATUS,
    #[doc = "0x14 - No Description"]
    pub ch0cfg: CH0CFG,
    #[doc = "0x18 - No Description"]
    pub ch1cfg: CH1CFG,
    #[doc = "0x1c - No Description"]
    pub cmd: CMD,
    #[doc = "0x20 - No Description"]
    pub if_: IF,
    #[doc = "0x24 - No Description"]
    pub ien: IEN,
    #[doc = "0x28 - No Description"]
    pub ch0f: CH0F,
    #[doc = "0x2c - No Description"]
    pub ch1f: CH1F,
    #[doc = "0x30 - No Description"]
    pub outctrl: OUTCTRL,
    #[doc = "0x34 - No Description"]
    pub outtimercfg: OUTTIMERCFG,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "No Description"]
pub mod en;
#[doc = "SWRST (rw) register accessor: an alias for `Reg<SWRST_SPEC>`"]
pub type SWRST = crate::Reg<swrst::SWRST_SPEC>;
#[doc = "No Description"]
pub mod swrst;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "No Description"]
pub mod cfg;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "CH0CFG (rw) register accessor: an alias for `Reg<CH0CFG_SPEC>`"]
pub type CH0CFG = crate::Reg<ch0cfg::CH0CFG_SPEC>;
#[doc = "No Description"]
pub mod ch0cfg;
#[doc = "CH1CFG (rw) register accessor: an alias for `Reg<CH1CFG_SPEC>`"]
pub type CH1CFG = crate::Reg<ch1cfg::CH1CFG_SPEC>;
#[doc = "No Description"]
pub mod ch1cfg;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "CH0F (w) register accessor: an alias for `Reg<CH0F_SPEC>`"]
pub type CH0F = crate::Reg<ch0f::CH0F_SPEC>;
#[doc = "No Description"]
pub mod ch0f;
#[doc = "CH1F (w) register accessor: an alias for `Reg<CH1F_SPEC>`"]
pub type CH1F = crate::Reg<ch1f::CH1F_SPEC>;
#[doc = "No Description"]
pub mod ch1f;
#[doc = "OUTCTRL (rw) register accessor: an alias for `Reg<OUTCTRL_SPEC>`"]
pub type OUTCTRL = crate::Reg<outctrl::OUTCTRL_SPEC>;
#[doc = "No Description"]
pub mod outctrl;
#[doc = "OUTTIMERCFG (rw) register accessor: an alias for `Reg<OUTTIMERCFG_SPEC>`"]
pub type OUTTIMERCFG = crate::Reg<outtimercfg::OUTTIMERCFG_SPEC>;
#[doc = "No Description"]
pub mod outtimercfg;
