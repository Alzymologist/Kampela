#![no_std]
#![deny(unused_crate_dependencies)]
extern crate alloc;

mod wordlist;

pub mod big_seed;
pub use big_seed::entropy_to_big_seed;

pub mod derivation;
pub use derivation::cut_derivations;

pub mod error;

pub mod phrase;
pub use phrase::{entropy_to_phrase, phrase_to_entropy};

