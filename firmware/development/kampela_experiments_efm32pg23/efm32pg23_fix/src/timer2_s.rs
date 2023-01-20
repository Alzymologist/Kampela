#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    #[doc = "0x04 - No Description"]
    pub cfg: CFG,
    #[doc = "0x08 - No Description"]
    pub ctrl: CTRL,
    #[doc = "0x0c - No Description"]
    pub cmd: CMD,
    #[doc = "0x10 - No Description"]
    pub status: STATUS,
    #[doc = "0x14 - No Description"]
    pub if_: IF,
    #[doc = "0x18 - No Description"]
    pub ien: IEN,
    #[doc = "0x1c - No Description"]
    pub top: TOP,
    #[doc = "0x20 - No Description"]
    pub topb: TOPB,
    #[doc = "0x24 - No Description"]
    pub cnt: CNT,
    _reserved10: [u8; 0x04],
    #[doc = "0x2c - No Description"]
    pub lock: LOCK,
    #[doc = "0x30 - No Description"]
    pub en: EN,
    _reserved12: [u8; 0x2c],
    #[doc = "0x60 - No Description"]
    pub cc0_cfg: CC0_CFG,
    #[doc = "0x64 - No Description"]
    pub cc0_ctrl: CC0_CTRL,
    #[doc = "0x68 - No Description"]
    pub cc0_oc: CC0_OC,
    _reserved15: [u8; 0x04],
    #[doc = "0x70 - No Description"]
    pub cc0_ocb: CC0_OCB,
    #[doc = "0x74 - No Description"]
    pub cc0_icf: CC0_ICF,
    #[doc = "0x78 - No Description"]
    pub cc0_icof: CC0_ICOF,
    _reserved18: [u8; 0x04],
    #[doc = "0x80 - No Description"]
    pub cc1_cfg: CC1_CFG,
    #[doc = "0x84 - No Description"]
    pub cc1_ctrl: CC1_CTRL,
    #[doc = "0x88 - No Description"]
    pub cc1_oc: CC1_OC,
    _reserved21: [u8; 0x04],
    #[doc = "0x90 - No Description"]
    pub cc1_ocb: CC1_OCB,
    #[doc = "0x94 - No Description"]
    pub cc1_icf: CC1_ICF,
    #[doc = "0x98 - No Description"]
    pub cc1_icof: CC1_ICOF,
    _reserved24: [u8; 0x04],
    #[doc = "0xa0 - No Description"]
    pub cc2_cfg: CC2_CFG,
    #[doc = "0xa4 - No Description"]
    pub cc2_ctrl: CC2_CTRL,
    #[doc = "0xa8 - No Description"]
    pub cc2_oc: CC2_OC,
    _reserved27: [u8; 0x04],
    #[doc = "0xb0 - No Description"]
    pub cc2_ocb: CC2_OCB,
    #[doc = "0xb4 - No Description"]
    pub cc2_icf: CC2_ICF,
    #[doc = "0xb8 - No Description"]
    pub cc2_icof: CC2_ICOF,
    _reserved30: [u8; 0x24],
    #[doc = "0xe0 - No Description"]
    pub dtcfg: DTCFG,
    #[doc = "0xe4 - No Description"]
    pub dttimecfg: DTTIMECFG,
    #[doc = "0xe8 - No Description"]
    pub dtfcfg: DTFCFG,
    #[doc = "0xec - No Description"]
    pub dtctrl: DTCTRL,
    #[doc = "0xf0 - No Description"]
    pub dtogen: DTOGEN,
    #[doc = "0xf4 - No Description"]
    pub dtfault: DTFAULT,
    #[doc = "0xf8 - No Description"]
    pub dtfaultc: DTFAULTC,
    #[doc = "0xfc - No Description"]
    pub dtlock: DTLOCK,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "No Description"]
pub mod cfg;
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
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "TOP (rw) register accessor: an alias for `Reg<TOP_SPEC>`"]
pub type TOP = crate::Reg<top::TOP_SPEC>;
#[doc = "No Description"]
pub mod top;
#[doc = "TOPB (rw) register accessor: an alias for `Reg<TOPB_SPEC>`"]
pub type TOPB = crate::Reg<topb::TOPB_SPEC>;
#[doc = "No Description"]
pub mod topb;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "No Description"]
pub mod cnt;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "No Description"]
pub mod en;
#[doc = "CC0_CFG (rw) register accessor: an alias for `Reg<CC0_CFG_SPEC>`"]
pub type CC0_CFG = crate::Reg<cc0_cfg::CC0_CFG_SPEC>;
#[doc = "No Description"]
pub mod cc0_cfg;
#[doc = "CC0_CTRL (rw) register accessor: an alias for `Reg<CC0_CTRL_SPEC>`"]
pub type CC0_CTRL = crate::Reg<cc0_ctrl::CC0_CTRL_SPEC>;
#[doc = "No Description"]
pub mod cc0_ctrl;
#[doc = "CC0_OC (rw) register accessor: an alias for `Reg<CC0_OC_SPEC>`"]
pub type CC0_OC = crate::Reg<cc0_oc::CC0_OC_SPEC>;
#[doc = "No Description"]
pub mod cc0_oc;
#[doc = "CC0_OCB (rw) register accessor: an alias for `Reg<CC0_OCB_SPEC>`"]
pub type CC0_OCB = crate::Reg<cc0_ocb::CC0_OCB_SPEC>;
#[doc = "No Description"]
pub mod cc0_ocb;
#[doc = "CC0_ICF (r) register accessor: an alias for `Reg<CC0_ICF_SPEC>`"]
pub type CC0_ICF = crate::Reg<cc0_icf::CC0_ICF_SPEC>;
#[doc = "No Description"]
pub mod cc0_icf;
#[doc = "CC0_ICOF (r) register accessor: an alias for `Reg<CC0_ICOF_SPEC>`"]
pub type CC0_ICOF = crate::Reg<cc0_icof::CC0_ICOF_SPEC>;
#[doc = "No Description"]
pub mod cc0_icof;
#[doc = "CC1_CFG (rw) register accessor: an alias for `Reg<CC1_CFG_SPEC>`"]
pub type CC1_CFG = crate::Reg<cc1_cfg::CC1_CFG_SPEC>;
#[doc = "No Description"]
pub mod cc1_cfg;
#[doc = "CC1_CTRL (rw) register accessor: an alias for `Reg<CC1_CTRL_SPEC>`"]
pub type CC1_CTRL = crate::Reg<cc1_ctrl::CC1_CTRL_SPEC>;
#[doc = "No Description"]
pub mod cc1_ctrl;
#[doc = "CC1_OC (rw) register accessor: an alias for `Reg<CC1_OC_SPEC>`"]
pub type CC1_OC = crate::Reg<cc1_oc::CC1_OC_SPEC>;
#[doc = "No Description"]
pub mod cc1_oc;
#[doc = "CC1_OCB (rw) register accessor: an alias for `Reg<CC1_OCB_SPEC>`"]
pub type CC1_OCB = crate::Reg<cc1_ocb::CC1_OCB_SPEC>;
#[doc = "No Description"]
pub mod cc1_ocb;
#[doc = "CC1_ICF (r) register accessor: an alias for `Reg<CC1_ICF_SPEC>`"]
pub type CC1_ICF = crate::Reg<cc1_icf::CC1_ICF_SPEC>;
#[doc = "No Description"]
pub mod cc1_icf;
#[doc = "CC1_ICOF (r) register accessor: an alias for `Reg<CC1_ICOF_SPEC>`"]
pub type CC1_ICOF = crate::Reg<cc1_icof::CC1_ICOF_SPEC>;
#[doc = "No Description"]
pub mod cc1_icof;
#[doc = "CC2_CFG (rw) register accessor: an alias for `Reg<CC2_CFG_SPEC>`"]
pub type CC2_CFG = crate::Reg<cc2_cfg::CC2_CFG_SPEC>;
#[doc = "No Description"]
pub mod cc2_cfg;
#[doc = "CC2_CTRL (rw) register accessor: an alias for `Reg<CC2_CTRL_SPEC>`"]
pub type CC2_CTRL = crate::Reg<cc2_ctrl::CC2_CTRL_SPEC>;
#[doc = "No Description"]
pub mod cc2_ctrl;
#[doc = "CC2_OC (rw) register accessor: an alias for `Reg<CC2_OC_SPEC>`"]
pub type CC2_OC = crate::Reg<cc2_oc::CC2_OC_SPEC>;
#[doc = "No Description"]
pub mod cc2_oc;
#[doc = "CC2_OCB (rw) register accessor: an alias for `Reg<CC2_OCB_SPEC>`"]
pub type CC2_OCB = crate::Reg<cc2_ocb::CC2_OCB_SPEC>;
#[doc = "No Description"]
pub mod cc2_ocb;
#[doc = "CC2_ICF (r) register accessor: an alias for `Reg<CC2_ICF_SPEC>`"]
pub type CC2_ICF = crate::Reg<cc2_icf::CC2_ICF_SPEC>;
#[doc = "No Description"]
pub mod cc2_icf;
#[doc = "CC2_ICOF (r) register accessor: an alias for `Reg<CC2_ICOF_SPEC>`"]
pub type CC2_ICOF = crate::Reg<cc2_icof::CC2_ICOF_SPEC>;
#[doc = "No Description"]
pub mod cc2_icof;
#[doc = "DTCFG (rw) register accessor: an alias for `Reg<DTCFG_SPEC>`"]
pub type DTCFG = crate::Reg<dtcfg::DTCFG_SPEC>;
#[doc = "No Description"]
pub mod dtcfg;
#[doc = "DTTIMECFG (rw) register accessor: an alias for `Reg<DTTIMECFG_SPEC>`"]
pub type DTTIMECFG = crate::Reg<dttimecfg::DTTIMECFG_SPEC>;
#[doc = "No Description"]
pub mod dttimecfg;
#[doc = "DTFCFG (rw) register accessor: an alias for `Reg<DTFCFG_SPEC>`"]
pub type DTFCFG = crate::Reg<dtfcfg::DTFCFG_SPEC>;
#[doc = "No Description"]
pub mod dtfcfg;
#[doc = "DTCTRL (rw) register accessor: an alias for `Reg<DTCTRL_SPEC>`"]
pub type DTCTRL = crate::Reg<dtctrl::DTCTRL_SPEC>;
#[doc = "No Description"]
pub mod dtctrl;
#[doc = "DTOGEN (rw) register accessor: an alias for `Reg<DTOGEN_SPEC>`"]
pub type DTOGEN = crate::Reg<dtogen::DTOGEN_SPEC>;
#[doc = "No Description"]
pub mod dtogen;
#[doc = "DTFAULT (r) register accessor: an alias for `Reg<DTFAULT_SPEC>`"]
pub type DTFAULT = crate::Reg<dtfault::DTFAULT_SPEC>;
#[doc = "No Description"]
pub mod dtfault;
#[doc = "DTFAULTC (w) register accessor: an alias for `Reg<DTFAULTC_SPEC>`"]
pub type DTFAULTC = crate::Reg<dtfaultc::DTFAULTC_SPEC>;
#[doc = "No Description"]
pub mod dtfaultc;
#[doc = "DTLOCK (w) register accessor: an alias for `Reg<DTLOCK_SPEC>`"]
pub type DTLOCK = crate::Reg<dtlock::DTLOCK_SPEC>;
#[doc = "No Description"]
pub mod dtlock;
