//! HAL for Kampela devices

use efm32pg23_fix::Peripherals;

pub mod power;
pub mod psram;
pub mod display;
pub mod se_rng;
pub mod se_aes_gcm;
pub mod touch;

/// Default (non-blocking) delay for operation start
const DELAY: usize = 10000;

/// Asynchronous procedures should implement this.
///
/// To call, iterate over advance()
pub trait Operation {
    type DesiredOutput;
    type StateEnum;

    fn new() -> Self;

    /// Generally delayed state transition
    fn wind(&mut self, state: Self::StateEnum, delay: usize);

    /// Call this repeatedly to progress through operation
    fn advance(&mut self) -> Self::DesiredOutput;

    /// change state instantly
    fn change(&mut self, state: Self::StateEnum) {
        self.wind(state, 0);
    }

    /// delayed change state with default delay
    fn wind_d(&mut self, state: Self::StateEnum) {
        self.wind(state, DELAY);
    }

}
