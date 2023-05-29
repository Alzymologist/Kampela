//! HAL for Kampela devices

use efm32pg23_fix::Peripherals;

pub mod power;
pub mod psram;
pub mod display;
pub mod se_rng;
pub mod se_aes_gcm;
pub mod touch;

