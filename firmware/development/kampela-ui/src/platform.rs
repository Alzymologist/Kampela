//! Platform definitions

use embedded_graphics::{pixelcolor::BinaryColor, prelude::{DrawTarget, Point}};
use rand::Rng;
use crate::pin::Pincode;
use crate::uistate::EventResult;

/// Implement this on platform to make crate work
pub trait Platform {
    /// Peripherals access should be external to this type since it is used elsewhere in general;
    /// Thus an external object HAL would be passed to all operations. Generally it should happen
    /// within mutex lock, so make sure to set up some kind of critical section aroung this object.
    type HAL;

    /// Sufficiently good random source used everywhere
    type Rng<'a>: Rng + Sized;

    /// Device-specific screen canvas abstraction
    type Display: DrawTarget<Color = BinaryColor>;

    /// RNG getter
    fn rng<'a>(h: &'a mut Self::HAL) -> Self::Rng<'a>;

    /// Device-specific "global" storage and management of pincode state RO
    fn pin(&self) -> &Pincode;

    /// Device-specific "global" storage and management of pincode state RW
    fn pin_mut(&mut self) -> &mut Pincode;

    fn display(&mut self) -> &mut Self::Display;

    fn pin_display(&mut self) -> (&mut Pincode, &mut Self::Display);

    fn handle_pin_event(&mut self, point: Point, h: &mut Self::HAL) -> Result<EventResult, <Self::Display as DrawTarget>::Error> {
        let (a, b) = self.pin_display();
        a.handle_event(point, &mut Self::rng(h), b)
    }

    fn draw_pincode(&mut self) -> Result<(), <Self::Display as DrawTarget>::Error> {
        let (a, b) = self.pin_display();
        a.draw(b)
    }

}

