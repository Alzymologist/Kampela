use embedded_graphics::{
    geometry::AnchorPoint,
    mono_font::{
        ascii::{FONT_10X20, FONT_6X10},
        MonoTextStyle,
    },
    prelude::Primitive,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    Drawable,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

pub const SCREEN_SIZE_X: u32 = 264;
pub const SCREEN_SIZE_Y: u32 = 176;
pub const GAP: u32 = 4;

pub const BUTTON_TOP: i32 = 30;
pub const CONTROL_BUTTON_WIDTH: u32 = 40;

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

