//! Everything high-level related to interfacing with user

use nalgebra::{Affine2, OMatrix, Point2, RowVector3};
use alloc::vec::Vec;
use alloc::string::String;
use lazy_static::lazy_static;

use kampela_system::{
    if_in_free,
    devices::{se_rng, touch::{Read, LEN_NUM_TOUCHES, FT6X36_REG_NUM_TOUCHES}},
    draw::FrameBuffer, 
    parallel::Operation,
};

use kampela_ui::{display_def::*, uistate, pin::Pincode, platform::Platform};
use embedded_graphics::prelude::Point;

/// UI handler
pub struct UI {
    state: uistate::UIState<Hardware>,
    status: UIStatus,
    update: uistate::UpdateRequest,
}

impl UI {
    /// Start of UI.
    pub fn init() -> Self {
        let mut update = uistate::UpdateRequest::new();
        update.set_slow();
        let hardware = Hardware::new();
        let state = uistate::UIState::new(hardware);
        return Self {
            state: state,
            status: UIStatus::Listen,
            update: update,
        }
    }

    /// Call in event loop to progress through UI state
    pub fn advance(&mut self, voltage: i32) {
        match self.status {
            UIStatus::Listen => {
                self.listen();
            },
            UIStatus::DisplayOperation => {
                if self.state.display().advance(voltage) {
                    self.status = UIStatus::Listen;
                }
            },
            UIStatus::TouchOperation(ref mut touch) => {
                match touch.advance(()) {
                    Ok(Some(touch)) => if let Some(point) = convert(touch) {
                        self.update = self.state.handle_event::<FrameBuffer>(point, &mut ()).unwrap();
                        self.status = UIStatus::Listen;
                    },
                    Ok(None) => {},
                    Err(e) => panic!("{:?}", e),
                } 
            },
        }
    }

    fn listen(&mut self) {
        // 1. update ui if needed
        if self.update.read_fast() {
            self.state.display().request_fast();
            self.status = UIStatus::DisplayOperation;
            return;
        }
        if self.update.read_slow() {
            self.state.render::<FrameBuffer>().expect("guaranteed to work, no errors implemented");
            self.state.display().request_full();
            self.status = UIStatus::DisplayOperation;
            return;
        }
        
        // 2. read input if possible
        if if_in_free(|peripherals|
            peripherals.GPIO_S.if_.read().extif0().bit_is_set()
        ).unwrap() {
            self.status = UIStatus::TouchOperation(Read::new());
        };
    }

    pub fn handle_rx(&mut self, transaction: String, extensions: String) {
        self.update = self.state.handle_rx(transaction, extensions);
    }
}

/// General status of UI
///
/// There is no sense in reading input while screen processes last event, nor refreshing the screen
/// before touch was parsed
enum UIStatus {
    /// Event listening state, default
    Listen,
    /// Screen update started
    DisplayOperation,
    /// Touch event processing
    TouchOperation(Read<LEN_NUM_TOUCHES, FT6X36_REG_NUM_TOUCHES>),
}

struct Hardware {
    pin: Pincode,
    entropy: Vec<u8>,
    display: FrameBuffer,
    transaction: Option<String>,
    extensions: Option<String>,
}

impl Hardware {
    pub fn new() -> Self {
        let entropy = Vec::new();
        let pin_set = false; // TODO query storage
        let pin = Pincode::new(&mut Self::rng(&mut ()), pin_set);
        let display = FrameBuffer::new_white();
        Self {
            pin: pin,
            entropy: entropy,
            display: display,
            transaction: None,
            extensions: None,
        }
    }
}

impl <'a> Platform for Hardware {
    type HAL = ();
    type Rng<'c> = se_rng::SeRng;
    type Display = FrameBuffer;

    fn rng<'b>(_: &'b mut ()) -> Self::Rng<'static> {
        se_rng::SeRng{}
    }

    fn pin(&self) -> &Pincode {
        &self.pin
    }

    fn pin_mut(&mut self) -> &mut Pincode {
        &mut self.pin
    }

    fn display(&mut self) -> &mut <Self as Platform>::Display {
        &mut self.display
    }

    fn pin_display(&mut self) -> (&mut Pincode, &mut <Self as Platform>::Display) {
        (&mut self.pin, &mut self.display)
    }

    fn set_entropy(&mut self, e: &[u8]) {
        self.entropy = e.to_vec(); // TODO: dedicated array storage maybe
    }

    fn entropy_display(&mut self) -> (&Vec<u8>, &mut <Self as Platform>::Display) {
        (&self.entropy, &mut self.display)
    }

    fn set_transaction(&mut self, transaction: String, extensions: String) {
        self.transaction = Some(transaction);
        self.extensions = Some(extensions);
    }

    fn transaction(&mut self) -> Option<(&str, &mut <Self as Platform>::Display)> {
        if let Some(ref a) = self.transaction {
            Some((a, &mut self.display))
        } else {
            None
        }
    }

    fn extensions(&mut self) -> Option<(&str, &mut <Self as Platform>::Display)> {
        if let Some(ref a) = self.extensions {
            Some((a, &mut self.display))
        } else {
            None
        }
    }
}

lazy_static! {
    // MAGIC calibration numbers obtained through KOLIBRI tool
    static ref AFFINE_MATRIX: Affine2<f32> = Affine2::from_matrix_unchecked(
        OMatrix::from_rows(&[
            RowVector3::<f32>::new(1.0022, -0.0216, -4.2725),
            RowVector3::<f32>::new(0.0061, 1.1433, -13.7305),
            RowVector3::<f32>::new(0.0, 0.0, 1.0),
        ])
    );
}

pub fn convert(touch_data: [u8; LEN_NUM_TOUCHES]) -> Option<Point> {
    if touch_data[0] == 1 {
        let detected_y = (((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16) as i32;
        let detected_x = (((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16) as i32;
        let touch = Point::new(SCREEN_SIZE_X as i32 - detected_x, detected_y);

        let touch_as_point2 = Point2::new(touch.x as f32, touch.y as f32);
        let display_as_point2 = AFFINE_MATRIX.transform_point(&touch_as_point2);
           
        Some(
            Point {
                x: display_as_point2.coords[0] as i32,
                y: display_as_point2.coords[1] as i32,
            }
        )
    } else { None }
}

