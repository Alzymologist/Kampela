//! Asynchronous operation generic code

/// Default (non-blocking) delay for operation start
pub const DELAY: usize = 100; //This is magic; don't change without testing or UI will get mean

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

