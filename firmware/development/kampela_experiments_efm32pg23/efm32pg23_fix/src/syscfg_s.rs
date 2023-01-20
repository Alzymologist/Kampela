#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - No Description"]
    pub ipversion: IPVERSION,
    #[doc = "0x08 - Read to get system status."]
    pub if_: IF,
    #[doc = "0x0c - Write to enable interrupts."]
    pub ien: IEN,
    _reserved3: [u8; 0x04],
    #[doc = "0x14 - Read to get the hard-wired chip revision."]
    pub chiprevhw: CHIPREVHW,
    #[doc = "0x18 - Read to get the chip revision programmed by feature configuration."]
    pub chiprev: CHIPREV,
    _reserved5: [u8; 0x08],
    #[doc = "0x24 - Configure the source of the system tick for the M33."]
    pub cfgsystic: CFGSYSTIC,
    _reserved6: [u8; 0x01d8],
    #[doc = "0x200 - Configure to provide general RAM configuration."]
    pub ctrl: CTRL,
    _reserved7: [u8; 0x04],
    #[doc = "0x208 - Configure to provide general RAM retention configuration."]
    pub dmem0retnctrl: DMEM0RETNCTRL,
    _reserved8: [u8; 0x0100],
    #[doc = "0x30c - Configure RAM bias configure bits."]
    pub rambiasconf: RAMBIASCONF,
    _reserved9: [u8; 0x0108],
    #[doc = "0x418 - Configure Host ICACHERAM retention configuration."]
    pub icacheramretnctrl: ICACHERAMRETNCTRL,
    #[doc = "0x41c - Configure DMEM0 port remap selection."]
    pub dmem0portmapsel: DMEM0PORTMAPSEL,
    _reserved11: [u8; 0x01e0],
    #[doc = "0x600 - Generic data space for user to pass to root, e.g., address of struct in mem"]
    pub rootdata0: ROOTDATA0,
    #[doc = "0x604 - Generic data space for user to pass to root, e.g., address of struct in mem"]
    pub rootdata1: ROOTDATA1,
    #[doc = "0x608 - This register returns the status of the SE managed locks."]
    pub rootlockstatus: ROOTLOCKSTATUS,
    #[doc = "0x60c - SE Software version"]
    pub rootseswversion: ROOTSESWVERSION,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Read to get system status."]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Write to enable interrupts."]
pub mod ien;
#[doc = "CHIPREVHW (rw) register accessor: an alias for `Reg<CHIPREVHW_SPEC>`"]
pub type CHIPREVHW = crate::Reg<chiprevhw::CHIPREVHW_SPEC>;
#[doc = "Read to get the hard-wired chip revision."]
pub mod chiprevhw;
#[doc = "CHIPREV (rw) register accessor: an alias for `Reg<CHIPREV_SPEC>`"]
pub type CHIPREV = crate::Reg<chiprev::CHIPREV_SPEC>;
#[doc = "Read to get the chip revision programmed by feature configuration."]
pub mod chiprev;
#[doc = "CFGSYSTIC (rw) register accessor: an alias for `Reg<CFGSYSTIC_SPEC>`"]
pub type CFGSYSTIC = crate::Reg<cfgsystic::CFGSYSTIC_SPEC>;
#[doc = "Configure the source of the system tick for the M33."]
pub mod cfgsystic;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Configure to provide general RAM configuration."]
pub mod ctrl;
#[doc = "DMEM0RETNCTRL (rw) register accessor: an alias for `Reg<DMEM0RETNCTRL_SPEC>`"]
pub type DMEM0RETNCTRL = crate::Reg<dmem0retnctrl::DMEM0RETNCTRL_SPEC>;
#[doc = "Configure to provide general RAM retention configuration."]
pub mod dmem0retnctrl;
#[doc = "RAMBIASCONF (rw) register accessor: an alias for `Reg<RAMBIASCONF_SPEC>`"]
pub type RAMBIASCONF = crate::Reg<rambiasconf::RAMBIASCONF_SPEC>;
#[doc = "Configure RAM bias configure bits."]
pub mod rambiasconf;
#[doc = "ICACHERAMRETNCTRL (rw) register accessor: an alias for `Reg<ICACHERAMRETNCTRL_SPEC>`"]
pub type ICACHERAMRETNCTRL = crate::Reg<icacheramretnctrl::ICACHERAMRETNCTRL_SPEC>;
#[doc = "Configure Host ICACHERAM retention configuration."]
pub mod icacheramretnctrl;
#[doc = "DMEM0PORTMAPSEL (rw) register accessor: an alias for `Reg<DMEM0PORTMAPSEL_SPEC>`"]
pub type DMEM0PORTMAPSEL = crate::Reg<dmem0portmapsel::DMEM0PORTMAPSEL_SPEC>;
#[doc = "Configure DMEM0 port remap selection."]
pub mod dmem0portmapsel;
#[doc = "ROOTDATA0 (rw) register accessor: an alias for `Reg<ROOTDATA0_SPEC>`"]
pub type ROOTDATA0 = crate::Reg<rootdata0::ROOTDATA0_SPEC>;
#[doc = "Generic data space for user to pass to root, e.g., address of struct in mem"]
pub mod rootdata0;
#[doc = "ROOTDATA1 (rw) register accessor: an alias for `Reg<ROOTDATA1_SPEC>`"]
pub type ROOTDATA1 = crate::Reg<rootdata1::ROOTDATA1_SPEC>;
#[doc = "Generic data space for user to pass to root, e.g., address of struct in mem"]
pub mod rootdata1;
#[doc = "ROOTLOCKSTATUS (r) register accessor: an alias for `Reg<ROOTLOCKSTATUS_SPEC>`"]
pub type ROOTLOCKSTATUS = crate::Reg<rootlockstatus::ROOTLOCKSTATUS_SPEC>;
#[doc = "This register returns the status of the SE managed locks."]
pub mod rootlockstatus;
#[doc = "ROOTSESWVERSION (rw) register accessor: an alias for `Reg<ROOTSESWVERSION_SPEC>`"]
pub type ROOTSESWVERSION = crate::Reg<rootseswversion::ROOTSESWVERSION_SPEC>;
#[doc = "SE Software version"]
pub mod rootseswversion;
