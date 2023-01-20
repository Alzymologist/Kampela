#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - A write access to any address in this area will be mapped to the TX FIFO (only for the payload). A read access to any address in this area will be mapped to the RX FIFO (only for the payload). Using an address range (16 x 32-bit) rather than one single address mapped to the FIFO allows using incremental bursts."]
    pub fifo: FIFO,
    _reserved1: [u8; 0x3c],
    #[doc = "0x40 - TX Status register."]
    pub tx_status: TX_STATUS,
    #[doc = "0x44 - RX Status register."]
    pub rx_status: RX_STATUS,
    #[doc = "0x48 - TX Protection register."]
    pub tx_prot: TX_PROT,
    #[doc = "0x4c - RX Protection register."]
    pub rx_prot: RX_PROT,
    #[doc = "0x50 - A write access to this register will be mapped to the TX FIFO (only for header)."]
    pub tx_header: TX_HEADER,
    #[doc = "0x54 - A read access to this register will be mapped to the RX FIFO (only for the header)."]
    pub rx_header: RX_HEADER,
    #[doc = "0x58 - Configuration register."]
    pub configuration: CONFIGURATION,
}
#[doc = "FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "A write access to any address in this area will be mapped to the TX FIFO (only for the payload). A read access to any address in this area will be mapped to the RX FIFO (only for the payload). Using an address range (16 x 32-bit) rather than one single address mapped to the FIFO allows using incremental bursts."]
pub mod fifo;
#[doc = "TX_STATUS (r) register accessor: an alias for `Reg<TX_STATUS_SPEC>`"]
pub type TX_STATUS = crate::Reg<tx_status::TX_STATUS_SPEC>;
#[doc = "TX Status register."]
pub mod tx_status;
#[doc = "RX_STATUS (r) register accessor: an alias for `Reg<RX_STATUS_SPEC>`"]
pub type RX_STATUS = crate::Reg<rx_status::RX_STATUS_SPEC>;
#[doc = "RX Status register."]
pub mod rx_status;
#[doc = "TX_PROT (r) register accessor: an alias for `Reg<TX_PROT_SPEC>`"]
pub type TX_PROT = crate::Reg<tx_prot::TX_PROT_SPEC>;
#[doc = "TX Protection register."]
pub mod tx_prot;
#[doc = "RX_PROT (r) register accessor: an alias for `Reg<RX_PROT_SPEC>`"]
pub type RX_PROT = crate::Reg<rx_prot::RX_PROT_SPEC>;
#[doc = "RX Protection register."]
pub mod rx_prot;
#[doc = "TX_HEADER (w) register accessor: an alias for `Reg<TX_HEADER_SPEC>`"]
pub type TX_HEADER = crate::Reg<tx_header::TX_HEADER_SPEC>;
#[doc = "A write access to this register will be mapped to the TX FIFO (only for header)."]
pub mod tx_header;
#[doc = "RX_HEADER (r) register accessor: an alias for `Reg<RX_HEADER_SPEC>`"]
pub type RX_HEADER = crate::Reg<rx_header::RX_HEADER_SPEC>;
#[doc = "A read access to this register will be mapped to the RX FIFO (only for the header)."]
pub mod rx_header;
#[doc = "CONFIGURATION (rw) register accessor: an alias for `Reg<CONFIGURATION_SPEC>`"]
pub type CONFIGURATION = crate::Reg<configuration::CONFIGURATION_SPEC>;
#[doc = "Configuration register."]
pub mod configuration;
