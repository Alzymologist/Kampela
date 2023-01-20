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
    pub cmd: CMD,
    #[doc = "0x14 - No Description"]
    pub status: STATUS,
    #[doc = "0x18 - No Description"]
    pub cnt: CNT,
    #[doc = "0x1c - No Description"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x20 - No Description"]
    pub lock: LOCK,
    _reserved9: [u8; 0x1c],
    #[doc = "0x40 - No Description"]
    pub grp0_if: GRP0_IF,
    #[doc = "0x44 - No Description"]
    pub grp0_ien: GRP0_IEN,
    #[doc = "0x48 - No Description"]
    pub grp0_ctrl: GRP0_CTRL,
    #[doc = "0x4c - No Description"]
    pub grp0_cmp0value: GRP0_CMP0VALUE,
    #[doc = "0x50 - No Description"]
    pub grp0_cmp1value: GRP0_CMP1VALUE,
    #[doc = "0x54 - No Description"]
    pub grp0_cap0value: GRP0_CAP0VALUE,
    #[doc = "0x58 - No Description"]
    pub grp0_syncbusy: GRP0_SYNCBUSY,
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
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "No Description"]
pub mod syncbusy;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
#[doc = "GRP0_IF (rw) register accessor: an alias for `Reg<GRP0_IF_SPEC>`"]
pub type GRP0_IF = crate::Reg<grp0_if::GRP0_IF_SPEC>;
#[doc = "No Description"]
pub mod grp0_if;
#[doc = "GRP0_IEN (rw) register accessor: an alias for `Reg<GRP0_IEN_SPEC>`"]
pub type GRP0_IEN = crate::Reg<grp0_ien::GRP0_IEN_SPEC>;
#[doc = "No Description"]
pub mod grp0_ien;
#[doc = "GRP0_CTRL (rw) register accessor: an alias for `Reg<GRP0_CTRL_SPEC>`"]
pub type GRP0_CTRL = crate::Reg<grp0_ctrl::GRP0_CTRL_SPEC>;
#[doc = "No Description"]
pub mod grp0_ctrl;
#[doc = "GRP0_CMP0VALUE (rw) register accessor: an alias for `Reg<GRP0_CMP0VALUE_SPEC>`"]
pub type GRP0_CMP0VALUE = crate::Reg<grp0_cmp0value::GRP0_CMP0VALUE_SPEC>;
#[doc = "No Description"]
pub mod grp0_cmp0value;
#[doc = "GRP0_CMP1VALUE (rw) register accessor: an alias for `Reg<GRP0_CMP1VALUE_SPEC>`"]
pub type GRP0_CMP1VALUE = crate::Reg<grp0_cmp1value::GRP0_CMP1VALUE_SPEC>;
#[doc = "No Description"]
pub mod grp0_cmp1value;
#[doc = "GRP0_CAP0VALUE (r) register accessor: an alias for `Reg<GRP0_CAP0VALUE_SPEC>`"]
pub type GRP0_CAP0VALUE = crate::Reg<grp0_cap0value::GRP0_CAP0VALUE_SPEC>;
#[doc = "No Description"]
pub mod grp0_cap0value;
#[doc = "GRP0_SYNCBUSY (r) register accessor: an alias for `Reg<GRP0_SYNCBUSY_SPEC>`"]
pub type GRP0_SYNCBUSY = crate::Reg<grp0_syncbusy::GRP0_SYNCBUSY_SPEC>;
#[doc = "No Description"]
pub mod grp0_syncbusy;
