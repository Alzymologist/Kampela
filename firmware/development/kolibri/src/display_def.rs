use embedded_graphics::primitives::Rectangle;
use embedded_graphics_core::geometry::{Point, Size};

use kampela_display_common::display_def::{SCREEN_SIZE_X, SCREEN_SIZE_Y};

pub const HALF_LEN: i32 = 7;
pub const LINE_WIDTH: u32 = 3;

const GAP: u32 = 12;
const BUTTON_WIDTH: u32 = 72;
const BUTTON_TOP: i32 = 120;

pub const RESTART_BUTTON_AREA: Rectangle = Rectangle::new(
    Point::new(GAP as i32, BUTTON_TOP),
    Size::new(BUTTON_WIDTH, SCREEN_SIZE_Y - BUTTON_TOP as u32 - GAP),
);
pub const DECLINE_BUTTON_AREA: Rectangle = Rectangle::new(
    Point::new(2 * GAP as i32 + BUTTON_WIDTH as i32, BUTTON_TOP),
    Size::new(BUTTON_WIDTH, SCREEN_SIZE_Y - BUTTON_TOP as u32 - GAP),
);
pub const APPROVE_BUTTON_AREA: Rectangle = Rectangle::new(
    Point::new(3 * GAP as i32 + 2 * BUTTON_WIDTH as i32, BUTTON_TOP),
    Size::new(BUTTON_WIDTH, SCREEN_SIZE_Y - BUTTON_TOP as u32 - GAP),
);
pub const DISPLAY_DATA_AREA: Rectangle = Rectangle::new(
    Point::new(GAP as i32, GAP as i32),
    Size::new(SCREEN_SIZE_X - 2 * GAP, BUTTON_TOP as u32 - 2 * GAP),
);
