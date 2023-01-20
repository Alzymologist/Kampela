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
    pub ctrl: CTRL,
    #[doc = "0x10 - No Description"]
    pub cmd: CMD,
    #[doc = "0x14 - No Description"]
    pub status: STATUS,
    #[doc = "0x18 - No Description"]
    pub cnt: CNT,
    #[doc = "0x1c - No Description"]
    pub comp0: COMP0,
    #[doc = "0x20 - No Description"]
    pub comp1: COMP1,
    #[doc = "0x24 - No Description"]
    pub top: TOP,
    #[doc = "0x28 - No Description"]
    pub topbuff: TOPBUFF,
    #[doc = "0x2c - No Description"]
    pub rep0: REP0,
    #[doc = "0x30 - No Description"]
    pub rep1: REP1,
    #[doc = "0x34 - No Description"]
    pub if_: IF,
    #[doc = "0x38 - No Description"]
    pub ien: IEN,
    #[doc = "0x3c - No Description"]
    pub lock: LOCK,
    #[doc = "0x40 - No Description"]
    pub syncbusy: SYNCBUSY,
    _reserved17: [u8; 0x0c],
    #[doc = "0x50 - No Description"]
    pub prsmode: PRSMODE,
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
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "No Description"]
pub mod cnt;
#[doc = "COMP0 (rw) register accessor: an alias for `Reg<COMP0_SPEC>`"]
pub type COMP0 = crate::Reg<comp0::COMP0_SPEC>;
#[doc = "No Description"]
pub mod comp0;
#[doc = "COMP1 (rw) register accessor: an alias for `Reg<COMP1_SPEC>`"]
pub type COMP1 = crate::Reg<comp1::COMP1_SPEC>;
#[doc = "No Description"]
pub mod comp1;
#[doc = "TOP (rw) register accessor: an alias for `Reg<TOP_SPEC>`"]
pub type TOP = crate::Reg<top::TOP_SPEC>;
#[doc = "No Description"]
pub mod top;
#[doc = "TOPBUFF (rw) register accessor: an alias for `Reg<TOPBUFF_SPEC>`"]
pub type TOPBUFF = crate::Reg<topbuff::TOPBUFF_SPEC>;
#[doc = "No Description"]
pub mod topbuff;
#[doc = "REP0 (rw) register accessor: an alias for `Reg<REP0_SPEC>`"]
pub type REP0 = crate::Reg<rep0::REP0_SPEC>;
#[doc = "No Description"]
pub mod rep0;
#[doc = "REP1 (rw) register accessor: an alias for `Reg<REP1_SPEC>`"]
pub type REP1 = crate::Reg<rep1::REP1_SPEC>;
#[doc = "No Description"]
pub mod rep1;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "No Description"]
pub mod syncbusy;
#[doc = "PRSMODE (rw) register accessor: an alias for `Reg<PRSMODE_SPEC>`"]
pub type PRSMODE = crate::Reg<prsmode::PRSMODE_SPEC>;
#[doc = "No Description"]
pub mod prsmode;
