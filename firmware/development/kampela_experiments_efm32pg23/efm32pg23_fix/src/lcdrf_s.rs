#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub rfimlcdctrl: RFIMLCDCTRL,
}
#[doc = "RFIMLCDCTRL (rw) register accessor: an alias for `Reg<RFIMLCDCTRL_SPEC>`"]
pub type RFIMLCDCTRL = crate::Reg<rfimlcdctrl::RFIMLCDCTRL_SPEC>;
#[doc = "No Description"]
pub mod rfimlcdctrl;
