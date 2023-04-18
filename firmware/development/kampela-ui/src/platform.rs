//! Platform definitions

use embedded_graphics::{pixelcolor::BinaryColor, prelude::{DrawTarget, Point}};
use rand::Rng;
use crate::pin::Pincode;
use crate::uistate::EventResult;

/// Implement this on platform to make crate work
pub trait Platform<R: Rng + ?Sized> {
    type Lock;

    fn rng(&mut self, l: Self::Lock) -> &mut R;

    fn pin(&mut self, l: Self::Lock) -> &mut Pincode;

    fn pin_rng(&mut self, l: Self::Lock) -> (&mut Pincode, &mut R);

    fn handle_pin_event<D>(&mut self, point: Point, fast_display: &mut D, l: Self::Lock) -> Result<EventResult, D::Error> where
        D: DrawTarget<Color = BinaryColor>,
    {
        let (a, b) = self.pin_rng(l);
        a.handle_event(point, b, fast_display)
    }
}
