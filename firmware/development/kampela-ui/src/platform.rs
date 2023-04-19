//! Platform definitions

use embedded_graphics::{pixelcolor::BinaryColor, prelude::{DrawTarget, Point}};
use rand::Rng;
use crate::pin::Pincode;
use crate::uistate::EventResult;

/// Implement this on platform to make crate work
pub trait Platform<R: Rng + ?Sized> {
    /// Peripherals access should be external to this type since it is used elsewhere in general;
    /// Thus an external object HAL would be passed to all operations. Generally it should happen
    /// within mutex lock, so make sure to set up some kind of critical section aroung this object.
    type HAL;

    /// RNG getter
    fn rng(h: &mut Self::HAL) -> &mut R;

    /// Device-specific "global" storage and management of pin state
    fn pin(&mut self) -> &mut Pincode;

    /// Special pin handle event that might use some generic
    ///
    /// TODO: remove this
    fn handle_pin_event<D>(&mut self, point: Point, fast_display: &mut D, h: &mut Self::HAL) -> Result<EventResult, D::Error> where
        D: DrawTarget<Color = BinaryColor>,
    {
        let rng = Self::rng(h);
        let pin = self.pin();
        pin.handle_event(point, rng, fast_display)
    }
}
