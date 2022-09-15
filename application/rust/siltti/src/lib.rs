#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::let_unit_value)]

mod ffi_types;

use crate::ffi_types::*;

include!(concat!(env!("OUT_DIR"), "/siltti.uniffi.rs"));
