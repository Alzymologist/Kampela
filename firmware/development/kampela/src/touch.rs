//! Asynchronous touchpad query

pub struct TouchState {
    state: touch::Read,
}

impl TouchState {
    pub fn new(&mut self) -> Self {
        Self {
            state: touch::Read::new();
        }
    }

    pub fn query_touch(&mut self, peripherals: &mut Peripherals) -> Option<Point> {
        
        if let Some(touch_data) = self.advance() {
            peripherals
                .GPIO_S
                .if_
                .write(|w_reg| w_reg.extif0().clear_bit());
            if touch_data[0] == 1 {
                    let detected_y = (((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16) as i32;
                    let detected_x = (((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16) as i32;
                    let touch = Point::new(SCREEN_SIZE_X as i32 - detected_x, detected_y);

                    let touch_as_point2 = Point2::new(touch.x as f32, touch.y as f32);
                    let display_as_point2 = affine_matrix.transform_point(&touch_as_point2);
            
                    Some( Point {
                        x: display_as_point2.coords[0] as i32,
                        y: display_as_point2.coords[1] as i32,
                    })
            } else { None }
        } else { None }
    }

    fn advance(&mut self) -> Option<[u8; LEN_NUM_TOUCHES]> {
free(|cs| {
            if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                if peripherals.GPIO_S.if_.read().extif0().bit_is_set() {
                    self.status = UIStatus::TouchState(ft6336_read_at::<LEN_NUM_TOUCHES>(peripherals, FT6X36_REG_NUM_TOUCHES).unwrap());
                }
            }
        });

        None
    }
}
