#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    #[doc = "0x04 - No Description"]
    pub status: STATUS,
    #[doc = "0x08 - No Description"]
    pub lock: LOCK,
    #[doc = "0x0c - No Description"]
    pub if_: IF,
    #[doc = "0x10 - No Description"]
    pub ien: IEN,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - Holds the M33 control settings"]
    pub m33ctrl: M33CTRL,
    _reserved6: [u8; 0x1c],
    #[doc = "0x40 - Set peripheral bits to 1 to mark as privileged access only"]
    pub ppupatd0: PPUPATD0,
    #[doc = "0x44 - Set peripheral bits to 1 to mark as privileged access only"]
    pub ppupatd1: PPUPATD1,
    _reserved8: [u8; 0x18],
    #[doc = "0x60 - Set peripheral bits to 1 to mark as secure access only"]
    pub ppusatd0: PPUSATD0,
    #[doc = "0x64 - Set peripheral bits to 1 to mark as secure access only"]
    pub ppusatd1: PPUSATD1,
    _reserved10: [u8; 0xd8],
    #[doc = "0x140 - No Description"]
    pub ppufs: PPUFS,
    _reserved11: [u8; 0x0c],
    #[doc = "0x150 - Set master bits to 1 to mark as a privileged master"]
    pub bmpupatd0: BMPUPATD0,
    _reserved12: [u8; 0x1c],
    #[doc = "0x170 - Set master bits to 1 to mark as a secure master"]
    pub bmpusatd0: BMPUSATD0,
    _reserved13: [u8; 0xdc],
    #[doc = "0x250 - No Description"]
    pub bmpufs: BMPUFS,
    #[doc = "0x254 - No Description"]
    pub bmpufsaddr: BMPUFSADDR,
    _reserved15: [u8; 0x08],
    #[doc = "0x260 - No Description"]
    pub esaurtypes0: ESAURTYPES0,
    #[doc = "0x264 - No Description"]
    pub esaurtypes1: ESAURTYPES1,
    _reserved17: [u8; 0x08],
    #[doc = "0x270 - No Description"]
    pub esaumrb01: ESAUMRB01,
    #[doc = "0x274 - No Description"]
    pub esaumrb12: ESAUMRB12,
    _reserved19: [u8; 0x08],
    #[doc = "0x280 - No Description"]
    pub esaumrb45: ESAUMRB45,
    #[doc = "0x284 - No Description"]
    pub esaumrb56: ESAUMRB56,
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
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "M33CTRL (rw) register accessor: an alias for `Reg<M33CTRL_SPEC>`"]
pub type M33CTRL = crate::Reg<m33ctrl::M33CTRL_SPEC>;
#[doc = "Holds the M33 control settings"]
pub mod m33ctrl;
#[doc = "PPUPATD0 (rw) register accessor: an alias for `Reg<PPUPATD0_SPEC>`"]
pub type PPUPATD0 = crate::Reg<ppupatd0::PPUPATD0_SPEC>;
#[doc = "Set peripheral bits to 1 to mark as privileged access only"]
pub mod ppupatd0;
#[doc = "PPUPATD1 (rw) register accessor: an alias for `Reg<PPUPATD1_SPEC>`"]
pub type PPUPATD1 = crate::Reg<ppupatd1::PPUPATD1_SPEC>;
#[doc = "Set peripheral bits to 1 to mark as privileged access only"]
pub mod ppupatd1;
#[doc = "PPUSATD0 (rw) register accessor: an alias for `Reg<PPUSATD0_SPEC>`"]
pub type PPUSATD0 = crate::Reg<ppusatd0::PPUSATD0_SPEC>;
#[doc = "Set peripheral bits to 1 to mark as secure access only"]
pub mod ppusatd0;
#[doc = "PPUSATD1 (rw) register accessor: an alias for `Reg<PPUSATD1_SPEC>`"]
pub type PPUSATD1 = crate::Reg<ppusatd1::PPUSATD1_SPEC>;
#[doc = "Set peripheral bits to 1 to mark as secure access only"]
pub mod ppusatd1;
#[doc = "PPUFS (r) register accessor: an alias for `Reg<PPUFS_SPEC>`"]
pub type PPUFS = crate::Reg<ppufs::PPUFS_SPEC>;
#[doc = "No Description"]
pub mod ppufs;
#[doc = "BMPUPATD0 (rw) register accessor: an alias for `Reg<BMPUPATD0_SPEC>`"]
pub type BMPUPATD0 = crate::Reg<bmpupatd0::BMPUPATD0_SPEC>;
#[doc = "Set master bits to 1 to mark as a privileged master"]
pub mod bmpupatd0;
#[doc = "BMPUSATD0 (rw) register accessor: an alias for `Reg<BMPUSATD0_SPEC>`"]
pub type BMPUSATD0 = crate::Reg<bmpusatd0::BMPUSATD0_SPEC>;
#[doc = "Set master bits to 1 to mark as a secure master"]
pub mod bmpusatd0;
#[doc = "BMPUFS (r) register accessor: an alias for `Reg<BMPUFS_SPEC>`"]
pub type BMPUFS = crate::Reg<bmpufs::BMPUFS_SPEC>;
#[doc = "No Description"]
pub mod bmpufs;
#[doc = "BMPUFSADDR (r) register accessor: an alias for `Reg<BMPUFSADDR_SPEC>`"]
pub type BMPUFSADDR = crate::Reg<bmpufsaddr::BMPUFSADDR_SPEC>;
#[doc = "No Description"]
pub mod bmpufsaddr;
#[doc = "ESAURTYPES0 (rw) register accessor: an alias for `Reg<ESAURTYPES0_SPEC>`"]
pub type ESAURTYPES0 = crate::Reg<esaurtypes0::ESAURTYPES0_SPEC>;
#[doc = "No Description"]
pub mod esaurtypes0;
#[doc = "ESAURTYPES1 (rw) register accessor: an alias for `Reg<ESAURTYPES1_SPEC>`"]
pub type ESAURTYPES1 = crate::Reg<esaurtypes1::ESAURTYPES1_SPEC>;
#[doc = "No Description"]
pub mod esaurtypes1;
#[doc = "ESAUMRB01 (rw) register accessor: an alias for `Reg<ESAUMRB01_SPEC>`"]
pub type ESAUMRB01 = crate::Reg<esaumrb01::ESAUMRB01_SPEC>;
#[doc = "No Description"]
pub mod esaumrb01;
#[doc = "ESAUMRB12 (rw) register accessor: an alias for `Reg<ESAUMRB12_SPEC>`"]
pub type ESAUMRB12 = crate::Reg<esaumrb12::ESAUMRB12_SPEC>;
#[doc = "No Description"]
pub mod esaumrb12;
#[doc = "ESAUMRB45 (rw) register accessor: an alias for `Reg<ESAUMRB45_SPEC>`"]
pub type ESAUMRB45 = crate::Reg<esaumrb45::ESAUMRB45_SPEC>;
#[doc = "No Description"]
pub mod esaumrb45;
#[doc = "ESAUMRB56 (rw) register accessor: an alias for `Reg<ESAUMRB56_SPEC>`"]
pub type ESAUMRB56 = crate::Reg<esaumrb56::ESAUMRB56_SPEC>;
#[doc = "No Description"]
pub mod esaumrb56;
