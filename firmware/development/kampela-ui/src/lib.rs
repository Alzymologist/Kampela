#![no_std]

pub mod uistate;

pub mod display_def;
mod pin;
mod seed_entry;
mod restore_or_generate;

#[macro_use]
extern crate lazy_static;

#[cfg(not(feature="std"))]
extern crate alloc;
#[cfg(not(feature="std"))]
extern crate core;
#[cfg(feature="std")]
extern crate std;

