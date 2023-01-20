#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    #[doc = "0x04 - No Description"]
    pub en: EN,
    #[doc = "0x08 - No Description"]
    pub ctrl: CTRL,
    #[doc = "0x0c - No Description"]
    pub cmd: CMD,
    #[doc = "0x10 - No Description"]
    pub init: INIT,
    #[doc = "0x14 - No Description"]
    pub poly: POLY,
    #[doc = "0x18 - No Description"]
    pub inputdata: INPUTDATA,
    #[doc = "0x1c - No Description"]
    pub inputdatahword: INPUTDATAHWORD,
    #[doc = "0x20 - No Description"]
    pub inputdatabyte: INPUTDATABYTE,
    #[doc = "0x24 - No Description"]
    pub data: DATA,
    #[doc = "0x28 - No Description"]
    pub datarev: DATAREV,
    #[doc = "0x2c - No Description"]
    pub databyterev: DATABYTEREV,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "No Description"]
pub mod en;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "INIT (rw) register accessor: an alias for `Reg<INIT_SPEC>`"]
pub type INIT = crate::Reg<init::INIT_SPEC>;
#[doc = "No Description"]
pub mod init;
#[doc = "POLY (rw) register accessor: an alias for `Reg<POLY_SPEC>`"]
pub type POLY = crate::Reg<poly::POLY_SPEC>;
#[doc = "No Description"]
pub mod poly;
#[doc = "INPUTDATA (w) register accessor: an alias for `Reg<INPUTDATA_SPEC>`"]
pub type INPUTDATA = crate::Reg<inputdata::INPUTDATA_SPEC>;
#[doc = "No Description"]
pub mod inputdata;
#[doc = "INPUTDATAHWORD (w) register accessor: an alias for `Reg<INPUTDATAHWORD_SPEC>`"]
pub type INPUTDATAHWORD = crate::Reg<inputdatahword::INPUTDATAHWORD_SPEC>;
#[doc = "No Description"]
pub mod inputdatahword;
#[doc = "INPUTDATABYTE (w) register accessor: an alias for `Reg<INPUTDATABYTE_SPEC>`"]
pub type INPUTDATABYTE = crate::Reg<inputdatabyte::INPUTDATABYTE_SPEC>;
#[doc = "No Description"]
pub mod inputdatabyte;
#[doc = "DATA (r) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "No Description"]
pub mod data;
#[doc = "DATAREV (r) register accessor: an alias for `Reg<DATAREV_SPEC>`"]
pub type DATAREV = crate::Reg<datarev::DATAREV_SPEC>;
#[doc = "No Description"]
pub mod datarev;
#[doc = "DATABYTEREV (r) register accessor: an alias for `Reg<DATABYTEREV_SPEC>`"]
pub type DATABYTEREV = crate::Reg<databyterev::DATABYTEREV_SPEC>;
#[doc = "No Description"]
pub mod databyterev;
