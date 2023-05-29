//! Testing basic functionality with `efm32pg23` chip.
//!
//! Operations used here are based on
//!
//! - [reference manual](https://www.silabs.com/documents/public/reference-manuals/efm32pg23-rm.pdf)
//! - [devboard user guide](https://www.silabs.com/documents/public/user-guides/ug515-efm32pg23-brd2504a-user-guide.pdf)
//! - [official API docs](https://docs.silabs.com/gecko-platform/latest/emlib/api/efm32xg23/modules)
//! - [published official open source SDK in C](https://github.com/SiliconLabs/gecko_sdk/tree/gsdk_4.2/platform)

#![no_std]

extern crate alloc;

pub mod init;
mod peripherals;
pub mod devices;
pub mod draw;
pub mod parallel;

use efm32pg23_fix::{CorePeripherals, Peripherals};

pub use peripherals::ldma::{BUF_QUARTER, CH_TIM0, LINK_1, LINK_2, LINK_DESCRIPTORS, TIMER0_CC0_ICF, NfcXfer, NfcXferBlock};

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::free;
use cortex_m::interrupt::Mutex;

use lazy_static::lazy_static;

lazy_static!{
    pub static ref CORE_PERIPHERALS: Mutex<RefCell<CorePeripherals>> = Mutex::new(RefCell::new(CorePeripherals::take().unwrap()));
    pub static ref PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));
}

use core::ops::FnMut;

/// Mutexed global access to peripherals
pub fn in_free<F>(mut action: F)
    where F: FnMut(&mut Peripherals)
{
    free(|cs| {
        if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
            action(peripherals);
        }
    });
}

/// Mutexed global access to peripherals
pub fn if_in_free<F>(mut action: F) -> Result<bool, FreeError>
    where F: FnMut(&mut Peripherals) -> bool
{
    free(|cs| {
        if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
            return Ok(action(peripherals))
        } else {
            return Err(FreeError::MutexLocked)
        }
    })
}

#[derive(Debug, PartialEq)]
pub enum FreeError {
    MutexLocked,
}
