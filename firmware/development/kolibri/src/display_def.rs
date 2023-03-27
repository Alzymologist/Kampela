use embedded_graphics::primitives::Rectangle;
use embedded_graphics_core::geometry::{Point, Size};

pub const SCREEN_SIZE_X: u32 = 264;
pub const SCREEN_SIZE_Y: u32 = 176;
pub const SCREEN_SIZE: Size = Size {
    width: SCREEN_SIZE_X,
    height: SCREEN_SIZE_Y,
};
pub const SCREEN_ZERO: Point = Point { x: 0, y: 0 };

pub const HALF_LEN: i32 = 7;
pub const LINE_WIDTH: u32 = 3;

pub const GAP: u32 = 20;
pub const BUTTON_WIDTH: u32 = 80;
pub const BUTTON_TOP: i32 = 120;

pub const RESTART_BUTTON_AREA: Rectangle = Rectangle::new(
    Point::new(GAP as i32, BUTTON_TOP),
    Size::new(BUTTON_WIDTH, SCREEN_SIZE_Y - BUTTON_TOP as u32 - GAP),
);
pub const APPROVE_BUTTON_AREA: Rectangle = Rectangle::new(
    Point::new(
        SCREEN_SIZE_X as i32 - BUTTON_WIDTH as i32 - GAP as i32,
        BUTTON_TOP,
    ),
    Size::new(BUTTON_WIDTH, SCREEN_SIZE_Y - BUTTON_TOP as u32 - GAP),
);
pub const DISPLAY_DATA_AREA: Rectangle = Rectangle::new(
    Point::new(GAP as i32, GAP as i32),
    Size::new(SCREEN_SIZE_X - 2 * GAP, BUTTON_TOP as u32 - GAP),
);
