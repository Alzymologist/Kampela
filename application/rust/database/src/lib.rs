#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

pub use kampela_common::{SpecsKey, SpecsValue};

pub mod error;
pub mod process_input;
pub mod sign_with_companion;
pub mod storage;
pub mod traits;

#[cfg(test)]
mod tests;
