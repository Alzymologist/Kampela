#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod derivation;
pub mod error;
pub mod nfc_fountain;
pub mod process_input;
pub mod sign_with_companion;
pub mod storage;

#[cfg(test)]
mod tests;
