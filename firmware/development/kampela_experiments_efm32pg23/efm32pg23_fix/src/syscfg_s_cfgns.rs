#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c],
    #[doc = "0x1c - Configure to define the system tick for the M33."]
    pub cfgnstcalib: CFGNSTCALIB,
    _reserved1: [u8; 0x05e0],
    #[doc = "0x600 - Generic data space for user to pass to root, e.g., address of struct in mem"]
    pub rootnsdata0: ROOTNSDATA0,
    #[doc = "0x604 - Generic data space for user to pass to root, e.g., address of struct in mem"]
    pub rootnsdata1: ROOTNSDATA1,
}
#[doc = "CFGNSTCALIB (rw) register accessor: an alias for `Reg<CFGNSTCALIB_SPEC>`"]
pub type CFGNSTCALIB = crate::Reg<cfgnstcalib::CFGNSTCALIB_SPEC>;
#[doc = "Configure to define the system tick for the M33."]
pub mod cfgnstcalib;
#[doc = "ROOTNSDATA0 (rw) register accessor: an alias for `Reg<ROOTNSDATA0_SPEC>`"]
pub type ROOTNSDATA0 = crate::Reg<rootnsdata0::ROOTNSDATA0_SPEC>;
#[doc = "Generic data space for user to pass to root, e.g., address of struct in mem"]
pub mod rootnsdata0;
#[doc = "ROOTNSDATA1 (rw) register accessor: an alias for `Reg<ROOTNSDATA1_SPEC>`"]
pub type ROOTNSDATA1 = crate::Reg<rootnsdata1::ROOTNSDATA1_SPEC>;
#[doc = "Generic data space for user to pass to root, e.g., address of struct in mem"]
pub mod rootnsdata1;
