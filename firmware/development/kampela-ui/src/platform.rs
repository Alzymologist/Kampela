//! Platform definitions

use embedded_graphics::{pixelcolor::BinaryColor, prelude::{DrawTarget, Point}};
use rand::Rng;
use crate::pin::Pincode;
use crate::uistate::EventResult;

/// Implement this on platform to make crate work
pub trait Platform<R: Rng + ?Sized> {
    fn rng(&mut self) -> &mut R;

    fn pin(&mut self) -> &mut Pincode;

    fn pin_rng(&mut self) -> (&mut Pincode, &mut R);

    fn handle_pin_event<D>(&mut self, point: Point, fast_display: &mut D) -> Result<EventResult, D::Error> where
        D: DrawTarget<Color = BinaryColor>,
    {
        let (mut a, mut b) = self.pin_rng();
        a.handle_event(point, b, fast_display)
    }
}
