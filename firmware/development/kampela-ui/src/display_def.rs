use embedded_graphics::{
    primitives::{
        Rectangle, Triangle,
    },
};
use embedded_graphics_core::geometry::{Point, Size};

pub use kampela_display_common::display_def::*;

pub const GAP: u32 = 4;

pub const BUTTON_TOP: i32 = 30;
pub const CONTROL_BUTTON_WIDTH: u32 = 20;

pub const BACK_BUTTON_AREA: Rectangle = Rectangle::new(
    Point::new(GAP as i32, BUTTON_TOP),
    Size::new(
        CONTROL_BUTTON_WIDTH,
        SCREEN_SIZE_Y - BUTTON_TOP as u32 - GAP,
    ),
);
pub const FORWARD_BUTTON_AREA: Rectangle = Rectangle::new(
    Point::new(
        SCREEN_SIZE_X as i32 - CONTROL_BUTTON_WIDTH as i32 - GAP as i32,
        BUTTON_TOP,
    ),
    Size::new(
        CONTROL_BUTTON_WIDTH,
        SCREEN_SIZE_Y - BUTTON_TOP as u32 - GAP,
    ),
);

pub const BACK_BUTTON_TRIANGLE: Triangle = Triangle::new(
    Point::new(CONTROL_BUTTON_WIDTH as i32, BUTTON_TOP + GAP as i32),
    Point::new(
        CONTROL_BUTTON_WIDTH as i32,
        SCREEN_SIZE_Y as i32 - GAP as i32 * 2,
    ),
    Point::new(GAP as i32 * 2, BUTTON_TOP / 2 + SCREEN_SIZE_Y as i32 / 2),
);
pub const FORWARD_BUTTON_TRIANGLE: Triangle = Triangle::new(
    Point::new(
        SCREEN_SIZE_X as i32 - CONTROL_BUTTON_WIDTH as i32,
        BUTTON_TOP + GAP as i32,
    ),
    Point::new(
        SCREEN_SIZE_X as i32 - CONTROL_BUTTON_WIDTH as i32,
        SCREEN_SIZE_Y as i32 - GAP as i32 * 2,
    ),
    Point::new(
        SCREEN_SIZE_X as i32 - GAP as i32 * 2,
        BUTTON_TOP / 2 + SCREEN_SIZE_Y as i32 / 2,
    ),
);

