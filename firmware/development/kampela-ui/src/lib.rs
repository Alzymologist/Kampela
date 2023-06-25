#![no_std]

pub mod uistate;
pub mod platform;

pub mod display_def;
pub mod pin;
mod seed_entry;
pub mod backup;
mod restore_or_generate;

pub mod transaction;

#[macro_use]
extern crate lazy_static;

#[cfg(not(feature="std"))]
extern crate alloc;
#[cfg(not(feature="std"))]
extern crate core;
#[cfg(feature="std")]
extern crate std;

pub mod data_state;
