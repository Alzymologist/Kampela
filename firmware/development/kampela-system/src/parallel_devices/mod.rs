//! This module exposes self-contained async state machines that can be almost atomically advanced
//! without locking thread in single-thread embedded system

use efm32pg23_fix::Peripherals;

pub mod touch;
pub mod display;

/// Asynchronous procedures should implement this.
///
/// To call, iterate over
///
/// ```
/// let device = Operation::new()
/// loop {
///     if device.advance_check() {
///         free(|cs| {
///             if let Ok(Some(a)) = device.advance(&mut peripherals) { return a }
///         });
///     }
/// }
/// ```
///
pub trait Operation {
    type DesiredOutput;

    fn new() -> Self;

    fn advance_check(&mut self) -> bool;

    fn advance(&mut self, peripherals: &mut Peripherals) -> Self::DesiredOutput;
}
