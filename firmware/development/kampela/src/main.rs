#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;
extern crate core;

use alloc::{format, vec::Vec};
use core::{alloc::Layout, panic::PanicInfo};
use cortex_m::{asm::delay, peripheral::syst::SystClkSource};
use cortex_m_rt::{entry, exception};
use embedded_alloc::Heap;

use embedded_graphics::prelude::Point;
use lazy_static::lazy_static;
use nalgebra::{linalg::SVD, Affine2, Const, OMatrix, Point2, RowVector1, RowVector3, RowVector6};

use efm32pg23_fix::{CorePeripherals, interrupt, Interrupt, NVIC, Peripherals};

#[global_allocator]
static HEAP: Heap = Heap::empty();

use kampela_system::{
    devices::{power::measure_voltage, se_rng, touch::{ft6336_read_at, FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES}},
    draw::{FrameBuffer, make_text, highlight_point, burning_tank}, 
    init::init_peripherals, 
};
use kampela_ui::{display_def::*, uistate, pin::Pincode, platform::Platform};

use alloc::{borrow::ToOwned, collections::BTreeMap};

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::free;
use cortex_m::interrupt::Mutex;

lazy_static!{
    static ref CORE_PERIPHERALS: Mutex<RefCell<CorePeripherals>> = Mutex::new(RefCell::new(CorePeripherals::take().unwrap()));
    static ref PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));
}

static mut LDMA_INTERRUPT: bool = false;
static mut GPIO_ODD_INT: bool = false;
static mut COUNT_ODD: bool = false;
static mut GPIO_EVEN_INT: bool = false;
static mut COUNT_EVEN: bool = false;

static mut READER: Option<[u8;5]> = None;

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    let mut peripherals = unsafe{Peripherals::steal()};
    burning_tank(&mut peripherals, format!("{:?}", panic));
    loop {}
}

/*
fn init_systick(cortex_periph: &mut cortex_m::Peripherals) {
    let syst = &mut cortex_periph.SYST;
    const DEFAULT_HZ: u32 = 14_000_000u32;

    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(DEFAULT_HZ / 1_000u32);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
}
*/

struct Hardware {
    pin: Pincode,
    entropy: Vec<u8>,
    display: FrameBuffer,
}

impl Hardware {
    pub fn new(h: &mut Peripherals) -> Self {
        let entropy = Vec::new();
        let pin_set = false; // TODO query storage
        let pin = Pincode::new(&mut Self::rng(h), pin_set);
        let mut display = FrameBuffer::new_white();
        Self {
            pin: pin,
            entropy: entropy,
            display: display,
        }

    }
}

impl Platform for Hardware {
    type HAL = Peripherals;
    type Rng<'a> = se_rng::SeRng<'a>;
    type Display = FrameBuffer;

    fn rng<'a>(h: &'a mut Self::HAL) -> Self::Rng<'a> {
        se_rng::SeRng{peripherals: h}
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
}

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 16384;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    free(|cs| {
        let mut core_periph = CORE_PERIPHERALS.borrow(cs).borrow_mut();
        //init_systick(&mut core_periph);
    });

    let mut peripherals = Peripherals::take().unwrap();

    init_peripherals(&mut peripherals);

    delay(1000);

    /*
    free(|cs| {
        let mut core_periph = CORE_PERIPHERALS.borrow(cs).borrow_mut();
        /*
        NVIC::unpend(Interrupt::LDMA);
        NVIC::mask(Interrupt::LDMA);
        unsafe {
            core_periph.NVIC.set_priority(Interrupt::LDMA, 3);
            NVIC::unmask(Interrupt::LDMA);
        }
        */
    });
    */

    delay(1000);

    let hardware = Hardware::new(&mut peripherals);

    free(|cs| {
        PERIPHERALS.borrow(cs).replace(Some(peripherals));
    });

    let affine_matrix  = Affine2::from_matrix_unchecked(
        OMatrix::from_rows(&[
            RowVector3::<f32>::new(1.0022, -0.0216, -4.2725),
            RowVector3::<f32>::new(0.0061, 1.1433, -13.7305),
            RowVector3::<f32>::new(0.0, 0.0, 1.0),
        ])
    );

    let mut state = uistate::UIState::new(hardware); 

    let mut update = uistate::UpdateRequest::new();
    update.set_slow();

    let mut input = None;
    let mut touch_data = [0; LEN_NUM_TOUCHES];
    let mut touched = false;

    loop {
        // 1. update ui if needed
        if update.read_fast() {
            free(|cs| {
                if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                    state.display().apply_fast(peripherals);
                    peripherals
                        .GPIO_S
                        .if_
                        .write(|w_reg| w_reg.extif0().clear_bit())
                }
            });
        }
        if update.read_slow() {
            state.render::<FrameBuffer>();
        delay(1000);
            free(|cs| {
                if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                    state.display().apply(peripherals);
                    peripherals
                        .GPIO_S
                        .if_
                        .write(|w_reg| w_reg.extif0().clear_bit());
                }
            });
        }

        // 2. read input if possible

        free(|cs| {
            if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                touched = peripherals.GPIO_S.if_.read().extif0().bit_is_set();
                if touched {
                    touch_data = ft6336_read_at::<LEN_NUM_TOUCHES>(peripherals, FT6X36_REG_NUM_TOUCHES).unwrap();
                }
            }
        });

        if touched {
            if touch_data[0] == 1 {
            let detected_y = (((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16) as i32;
            let detected_x = (((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16) as i32;
            let touch = Point::new(SCREEN_SIZE_X as i32 - detected_x, detected_y);

            let touch_as_point2 = Point2::new(touch.x as f32, touch.y as f32);
            let display_as_point2 = affine_matrix.transform_point(&touch_as_point2);
            
            input = Some(
                Point {
                    x: display_as_point2.coords[0] as i32,
                    y: display_as_point2.coords[1] as i32,
                }
            );
            }

            free(|cs| {
                if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                    peripherals
                        .GPIO_S
                        .if_
                        .write(|w_reg| w_reg.extif0().clear_bit())
                }
            });
        }
        touched = false;

        // 3. handle input
        if let Some(point) = input {
            free(|cs| {
                if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                    update = state.handle_event::<FrameBuffer>(point, peripherals).unwrap();
                }
            });
            input = None;
        }


        // 4. non-UI loop time
        //unsafe{PUSHED = false;}

        // 5. restore responsiveness at some point
    }
}
