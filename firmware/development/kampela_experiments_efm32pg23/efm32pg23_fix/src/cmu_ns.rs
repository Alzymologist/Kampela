#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - No Description"]
    pub status: STATUS,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - No Description"]
    pub lock: LOCK,
    #[doc = "0x14 - No Description"]
    pub wdoglock: WDOGLOCK,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - No Description"]
    pub if_: IF,
    #[doc = "0x24 - No Description"]
    pub ien: IEN,
    _reserved6: [u8; 0x28],
    #[doc = "0x50 - No Description"]
    pub calcmd: CALCMD,
    #[doc = "0x54 - No Description"]
    pub calctrl: CALCTRL,
    #[doc = "0x58 - No Description"]
    pub calcnt: CALCNT,
    _reserved9: [u8; 0x08],
    #[doc = "0x64 - No Description"]
    pub clken0: CLKEN0,
    #[doc = "0x68 - No Description"]
    pub clken1: CLKEN1,
    _reserved11: [u8; 0x04],
    #[doc = "0x70 - No Description"]
    pub sysclkctrl: SYSCLKCTRL,
    _reserved12: [u8; 0x0c],
    #[doc = "0x80 - No Description"]
    pub traceclkctrl: TRACECLKCTRL,
    _reserved13: [u8; 0x0c],
    #[doc = "0x90 - No Description"]
    pub exportclkctrl: EXPORTCLKCTRL,
    _reserved14: [u8; 0x6c],
    #[doc = "0x100 - No Description"]
    pub dpllrefclkctrl: DPLLREFCLKCTRL,
    _reserved15: [u8; 0x1c],
    #[doc = "0x120 - No Description"]
    pub em01grpaclkctrl: EM01GRPACLKCTRL,
    _reserved16: [u8; 0x04],
    #[doc = "0x128 - No Description"]
    pub em01grpcclkctrl: EM01GRPCCLKCTRL,
    _reserved17: [u8; 0x14],
    #[doc = "0x140 - No Description"]
    pub em23grpaclkctrl: EM23GRPACLKCTRL,
    _reserved18: [u8; 0x1c],
    #[doc = "0x160 - No Description"]
    pub em4grpaclkctrl: EM4GRPACLKCTRL,
    _reserved19: [u8; 0x1c],
    #[doc = "0x180 - No Description"]
    pub iadcclkctrl: IADCCLKCTRL,
    _reserved20: [u8; 0x7c],
    #[doc = "0x200 - No Description"]
    pub wdog0clkctrl: WDOG0CLKCTRL,
    _reserved21: [u8; 0x04],
    #[doc = "0x208 - No Description"]
    pub wdog1clkctrl: WDOG1CLKCTRL,
    _reserved22: [u8; 0x14],
    #[doc = "0x220 - No Description"]
    pub eusart0clkctrl: EUSART0CLKCTRL,
    _reserved23: [u8; 0x1c],
    #[doc = "0x240 - No Description"]
    pub sysrtc0clkctrl: SYSRTC0CLKCTRL,
    _reserved24: [u8; 0x0c],
    #[doc = "0x250 - No Description"]
    pub lcdclkctrl: LCDCLKCTRL,
    _reserved25: [u8; 0x0c],
    #[doc = "0x260 - No Description"]
    pub vdac0clkctrl: VDAC0CLKCTRL,
    _reserved26: [u8; 0x0c],
    #[doc = "0x270 - No Description"]
    pub pcnt0clkctrl: PCNT0CLKCTRL,
    _reserved27: [u8; 0x1c],
    #[doc = "0x290 - No Description"]
    pub lesensehfclkctrl: LESENSEHFCLKCTRL,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
#[doc = "WDOGLOCK (w) register accessor: an alias for `Reg<WDOGLOCK_SPEC>`"]
pub type WDOGLOCK = crate::Reg<wdoglock::WDOGLOCK_SPEC>;
#[doc = "No Description"]
pub mod wdoglock;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "CALCMD (w) register accessor: an alias for `Reg<CALCMD_SPEC>`"]
pub type CALCMD = crate::Reg<calcmd::CALCMD_SPEC>;
#[doc = "No Description"]
pub mod calcmd;
#[doc = "CALCTRL (rw) register accessor: an alias for `Reg<CALCTRL_SPEC>`"]
pub type CALCTRL = crate::Reg<calctrl::CALCTRL_SPEC>;
#[doc = "No Description"]
pub mod calctrl;
#[doc = "CALCNT (r) register accessor: an alias for `Reg<CALCNT_SPEC>`"]
pub type CALCNT = crate::Reg<calcnt::CALCNT_SPEC>;
#[doc = "No Description"]
pub mod calcnt;
#[doc = "CLKEN0 (rw) register accessor: an alias for `Reg<CLKEN0_SPEC>`"]
pub type CLKEN0 = crate::Reg<clken0::CLKEN0_SPEC>;
#[doc = "No Description"]
pub mod clken0;
#[doc = "CLKEN1 (rw) register accessor: an alias for `Reg<CLKEN1_SPEC>`"]
pub type CLKEN1 = crate::Reg<clken1::CLKEN1_SPEC>;
#[doc = "No Description"]
pub mod clken1;
#[doc = "SYSCLKCTRL (rw) register accessor: an alias for `Reg<SYSCLKCTRL_SPEC>`"]
pub type SYSCLKCTRL = crate::Reg<sysclkctrl::SYSCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod sysclkctrl;
#[doc = "TRACECLKCTRL (rw) register accessor: an alias for `Reg<TRACECLKCTRL_SPEC>`"]
pub type TRACECLKCTRL = crate::Reg<traceclkctrl::TRACECLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod traceclkctrl;
#[doc = "EXPORTCLKCTRL (rw) register accessor: an alias for `Reg<EXPORTCLKCTRL_SPEC>`"]
pub type EXPORTCLKCTRL = crate::Reg<exportclkctrl::EXPORTCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod exportclkctrl;
#[doc = "DPLLREFCLKCTRL (rw) register accessor: an alias for `Reg<DPLLREFCLKCTRL_SPEC>`"]
pub type DPLLREFCLKCTRL = crate::Reg<dpllrefclkctrl::DPLLREFCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod dpllrefclkctrl;
#[doc = "EM01GRPACLKCTRL (rw) register accessor: an alias for `Reg<EM01GRPACLKCTRL_SPEC>`"]
pub type EM01GRPACLKCTRL = crate::Reg<em01grpaclkctrl::EM01GRPACLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod em01grpaclkctrl;
#[doc = "EM01GRPCCLKCTRL (rw) register accessor: an alias for `Reg<EM01GRPCCLKCTRL_SPEC>`"]
pub type EM01GRPCCLKCTRL = crate::Reg<em01grpcclkctrl::EM01GRPCCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod em01grpcclkctrl;
#[doc = "EM23GRPACLKCTRL (rw) register accessor: an alias for `Reg<EM23GRPACLKCTRL_SPEC>`"]
pub type EM23GRPACLKCTRL = crate::Reg<em23grpaclkctrl::EM23GRPACLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod em23grpaclkctrl;
#[doc = "EM4GRPACLKCTRL (rw) register accessor: an alias for `Reg<EM4GRPACLKCTRL_SPEC>`"]
pub type EM4GRPACLKCTRL = crate::Reg<em4grpaclkctrl::EM4GRPACLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod em4grpaclkctrl;
#[doc = "IADCCLKCTRL (rw) register accessor: an alias for `Reg<IADCCLKCTRL_SPEC>`"]
pub type IADCCLKCTRL = crate::Reg<iadcclkctrl::IADCCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod iadcclkctrl;
#[doc = "WDOG0CLKCTRL (rw) register accessor: an alias for `Reg<WDOG0CLKCTRL_SPEC>`"]
pub type WDOG0CLKCTRL = crate::Reg<wdog0clkctrl::WDOG0CLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod wdog0clkctrl;
#[doc = "WDOG1CLKCTRL (rw) register accessor: an alias for `Reg<WDOG1CLKCTRL_SPEC>`"]
pub type WDOG1CLKCTRL = crate::Reg<wdog1clkctrl::WDOG1CLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod wdog1clkctrl;
#[doc = "EUSART0CLKCTRL (rw) register accessor: an alias for `Reg<EUSART0CLKCTRL_SPEC>`"]
pub type EUSART0CLKCTRL = crate::Reg<eusart0clkctrl::EUSART0CLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod eusart0clkctrl;
#[doc = "SYSRTC0CLKCTRL (rw) register accessor: an alias for `Reg<SYSRTC0CLKCTRL_SPEC>`"]
pub type SYSRTC0CLKCTRL = crate::Reg<sysrtc0clkctrl::SYSRTC0CLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod sysrtc0clkctrl;
#[doc = "LCDCLKCTRL (rw) register accessor: an alias for `Reg<LCDCLKCTRL_SPEC>`"]
pub type LCDCLKCTRL = crate::Reg<lcdclkctrl::LCDCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod lcdclkctrl;
#[doc = "VDAC0CLKCTRL (rw) register accessor: an alias for `Reg<VDAC0CLKCTRL_SPEC>`"]
pub type VDAC0CLKCTRL = crate::Reg<vdac0clkctrl::VDAC0CLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod vdac0clkctrl;
#[doc = "PCNT0CLKCTRL (rw) register accessor: an alias for `Reg<PCNT0CLKCTRL_SPEC>`"]
pub type PCNT0CLKCTRL = crate::Reg<pcnt0clkctrl::PCNT0CLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod pcnt0clkctrl;
#[doc = "LESENSEHFCLKCTRL (rw) register accessor: an alias for `Reg<LESENSEHFCLKCTRL_SPEC>`"]
pub type LESENSEHFCLKCTRL = crate::Reg<lesensehfclkctrl::LESENSEHFCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod lesensehfclkctrl;
