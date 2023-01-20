#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub msgptr0: MSGPTR0,
    #[doc = "0x04 - No Description"]
    pub msgptr1: MSGPTR1,
    #[doc = "0x08 - No Description"]
    pub msgptr2: MSGPTR2,
    #[doc = "0x0c - No Description"]
    pub msgptr3: MSGPTR3,
    _reserved4: [u8; 0x30],
    #[doc = "0x40 - No Description"]
    pub if_: IF,
    #[doc = "0x44 - No Description"]
    pub ien: IEN,
}
#[doc = "MSGPTR0 (rw) register accessor: an alias for `Reg<MSGPTR0_SPEC>`"]
pub type MSGPTR0 = crate::Reg<msgptr0::MSGPTR0_SPEC>;
#[doc = "No Description"]
pub mod msgptr0;
#[doc = "MSGPTR1 (rw) register accessor: an alias for `Reg<MSGPTR1_SPEC>`"]
pub type MSGPTR1 = crate::Reg<msgptr1::MSGPTR1_SPEC>;
#[doc = "No Description"]
pub mod msgptr1;
#[doc = "MSGPTR2 (rw) register accessor: an alias for `Reg<MSGPTR2_SPEC>`"]
pub type MSGPTR2 = crate::Reg<msgptr2::MSGPTR2_SPEC>;
#[doc = "No Description"]
pub mod msgptr2;
#[doc = "MSGPTR3 (rw) register accessor: an alias for `Reg<MSGPTR3_SPEC>`"]
pub type MSGPTR3 = crate::Reg<msgptr3::MSGPTR3_SPEC>;
#[doc = "No Description"]
pub mod msgptr3;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
