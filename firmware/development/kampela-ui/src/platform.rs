//! Platform definitions

#[cfg(not(feature="std"))]
use alloc::vec::Vec;
#[cfg(feature="std")]
use std::vec::Vec;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::{DrawTarget, Point}};
use rand::Rng;
use crate::pin::Pincode;
use crate::uistate::EventResult;
use crate::backup::draw_backup_screen;

const ENTROPY_LEN: usize = 32; //TODO: move to appropriate place

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

    /// Getter for canvas
    fn display(&mut self) -> &mut Self::Display;

    /// Getter for pincode and canvas simultaneously (they should be independent)
    fn pin_display(&mut self) -> (&mut Pincode, &mut Self::Display);

    /// Set new seed
    fn set_entropy(&mut self, e: &[u8]);
    
    /// Getter for pincode and canvas
    fn entropy_display(&mut self) -> (&Vec<u8>, &mut Self::Display);
   
    //----derivatives----

    fn generate_seed_entropy(h: &mut Self::HAL) -> [u8; ENTROPY_LEN] {
        let mut entropy: [u8; ENTROPY_LEN]= [0; ENTROPY_LEN];
        Self::rng(h).fill(&mut entropy);
        entropy
    }

    fn generate_seed(&mut self, h: &mut Self::HAL) {
        self.set_entropy(&Self::generate_seed_entropy(h));
    }

    fn handle_pin_event(&mut self, point: Point, h: &mut Self::HAL) -> Result<EventResult, <Self::Display as DrawTarget>::Error> {
        let (a, b) = self.pin_display();
        a.handle_event(point, &mut Self::rng(h), b)
    }

    fn handle_pin_event_repeat(&mut self, point: Point, h: &mut Self::HAL) -> Result<EventResult, <Self::Display as DrawTarget>::Error> {
        let (a, b) = self.pin_display();
        a.handle_event_repeat(point, &mut Self::rng(h), b)
    }

    fn draw_pincode(&mut self) -> Result<(), <Self::Display as DrawTarget>::Error> {
        let (p, d) = self.pin_display();
        p.draw(d)
    }

    fn draw_backup(&mut self) -> Result<(), <Self::Display as DrawTarget>::Error> {
        let (s, d) = self.entropy_display();
        draw_backup_screen(s, d)
    }
}

