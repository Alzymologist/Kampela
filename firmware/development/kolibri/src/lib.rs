#![no_std]
#![deny(unused_crate_dependencies)]

pub mod calibration;
pub mod display_def;
pub mod uistate;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
extern crate core;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use embedded_graphics_simulator as _;
