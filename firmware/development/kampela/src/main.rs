#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;
extern crate core;

use alloc::{format, vec::Vec};
use core::{alloc::Layout, panic::PanicInfo};
use core::ptr::addr_of;
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
    BUF_QUARTER, LINK_1, LINK_2, LINK_DESCRIPTORS, TIMER0_CC0_ICF, NfcXfer, NfcXferBlock,
};
use kampela_ui::{display_def::*, uistate, pin::Pincode, platform::Platform};

use alloc::{borrow::ToOwned, collections::BTreeMap};

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::free;
use cortex_m::interrupt::Mutex;

use nfca_parser::{frame::{Frame, FrameAttributed}, miller::*, time_record_both_ways::*};

pub const FREQ: u16 = 22;

pub struct BufferInfo {
    pub buffer_status: BufferStatus,
    pub maybe_previous_tail: Option<MillerTimesDown<FREQ>>,
}

impl BufferInfo {
    pub fn new() -> Self {
        Self {
            buffer_status: BufferStatus::new(),
            maybe_previous_tail: None,
        }
    }
}

pub enum BufferStatus {
    R0W1,
    R0W2,
    R0Wh,
    R1W2,
    R1W3,
    R1Wh,
    R2W0,
    R2W3,
    R2Wh,
    R3W0,
    R3W1,
    R3Wh,
    RhW0,
    RhW1,
    RhW2,
    RhW3,
}

pub enum BufRegion {
    Reg0,
    Reg1,
    Reg2,
    Reg3,
}


#[derive(Debug)]
pub enum BufferError {
    UnexpectedIfDone7,
    UnexpectedReadDone,
}

impl BufferStatus {
    pub fn new() -> Self {
        Self::RhW0
    }
    pub fn pass_if_done7(&mut self) -> Result<(), BufferError> {
        let new_self = match self {
            Self::R0W1 => Self::R0W2,
            Self::R0W2 => Self::R0Wh,
            Self::R0Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::R1W2 => Self::R1W3,
            Self::R1W3 => Self::R1Wh,
            Self::R1Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::R2W0 => Self::R2Wh,
            Self::R2W3 => Self::R2W0,
            Self::R2Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::R3W0 => Self::R3W1,
            Self::R3W1 => Self::R3Wh,
            Self::R3Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::RhW0 => Self::R0W1,
            Self::RhW1 => Self::R1W2,
            Self::RhW2 => Self::R2W3,
            Self::RhW3 => Self::R3W0,
        };
        *self = new_self;
        Ok(())
    }
    pub fn pass_read_done(&mut self) -> Result<(), BufferError> {
        let new_self = match self {
            Self::R0W1 => Self::RhW1,
            Self::R0W2 => Self::R1W2,
            Self::R0Wh => Self::R1W3,
            Self::R1W2 => Self::RhW2,
            Self::R1W3 => Self::R2W3,
            Self::R1Wh => Self::R2W0,
            Self::R2W0 => Self::R3W0,
            Self::R2W3 => Self::RhW3,
            Self::R2Wh => Self::R3W1,
            Self::R3W0 => Self::RhW0,
            Self::R3W1 => Self::R0W1,
            Self::R3Wh => Self::R0W2,
            Self::RhW0 => return Err(BufferError::UnexpectedReadDone),
            Self::RhW1 => return Err(BufferError::UnexpectedReadDone),
            Self::RhW2 => return Err(BufferError::UnexpectedReadDone),
            Self::RhW3 => return Err(BufferError::UnexpectedReadDone),
        };
        *self = new_self;
        Ok(())
    }
    pub fn read_from(&self) -> Option<BufRegion> {
        match self {
            Self::R0W1 => Some(BufRegion::Reg0),
            Self::R0W2 => Some(BufRegion::Reg0),
            Self::R0Wh => Some(BufRegion::Reg0),
            Self::R1W2 => Some(BufRegion::Reg1),
            Self::R1W3 => Some(BufRegion::Reg1),
            Self::R1Wh => Some(BufRegion::Reg1),
            Self::R2W0 => Some(BufRegion::Reg2),
            Self::R2W3 => Some(BufRegion::Reg2),
            Self::R2Wh => Some(BufRegion::Reg2),
            Self::R3W0 => Some(BufRegion::Reg3),
            Self::R3W1 => Some(BufRegion::Reg3),
            Self::R3Wh => Some(BufRegion::Reg3),
            Self::RhW0 => None,
            Self::RhW1 => None,
            Self::RhW2 => None,
            Self::RhW3 => None,
        }
    }
    pub fn is_write_halted(&self) -> bool {
        match self {
            Self::R0W1 => false,
            Self::R0W2 => false,
            Self::R0Wh => true,
            Self::R1W2 => false,
            Self::R1W3 => false,
            Self::R1Wh => true,
            Self::R2W0 => false,
            Self::R2W3 => false,
            Self::R2Wh => true,
            Self::R3W0 => false,
            Self::R3W1 => false,
            Self::R3Wh => true,
            Self::RhW0 => false,
            Self::RhW1 => false,
            Self::RhW2 => false,
            Self::RhW3 => false,
        }
    }
}

lazy_static!{
    static ref CORE_PERIPHERALS: Mutex<RefCell<CorePeripherals>> = Mutex::new(RefCell::new(CorePeripherals::take().unwrap()));
    static ref PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));
    static ref BUFFER_INFO: Mutex<RefCell<BufferInfo>> = Mutex::new(RefCell::new(BufferInfo::new()));
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


#[interrupt]
fn LDMA() {
    free(|cs| {
        let mut buffer_info = BUFFER_INFO.borrow(cs).borrow_mut();
        match buffer_info.buffer_status.pass_if_done7() {
            Ok(_) => {
                if buffer_info.buffer_status.is_write_halted() {
                    NVIC::mask(Interrupt::LDMA);
                }
                else {
                    if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                        peripherals.LDMA_S.if_.reset();
                    }
                    else {unreachable!()} // TODO
                }
            },
            Err(_) => unreachable!() //TODO
        }
    });
}

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

    let nfc_buffer: [u16; 4*BUF_QUARTER] = [1; 4*BUF_QUARTER];
    let nfc_transfer_block = NfcXferBlock {
        block0: NfcXfer {
            descriptors: LINK_DESCRIPTORS,
            source: TIMER0_CC0_ICF,
            dest: addr_of!(nfc_buffer[0]) as u32,
            link: LINK_1,
        },
        block1: NfcXfer {
            descriptors: LINK_DESCRIPTORS,
            source: TIMER0_CC0_ICF,
            dest: addr_of!(nfc_buffer[BUF_QUARTER]) as u32,
            link: LINK_1,
        },
        block2: NfcXfer {
            descriptors: LINK_DESCRIPTORS,
            source: TIMER0_CC0_ICF,
            dest: addr_of!(nfc_buffer[2*BUF_QUARTER]) as u32,
            link: LINK_1,
        },
        block3: NfcXfer {
            descriptors: LINK_DESCRIPTORS,
            source: TIMER0_CC0_ICF,
            dest: addr_of!(nfc_buffer[3*BUF_QUARTER]) as u32,
            link: LINK_2,
        },
    };

    let mut peripherals = Peripherals::take().unwrap();

    init_peripherals(&mut peripherals, addr_of!(nfc_transfer_block));

    delay(1000);

    free(|cs| {
        let mut core_periph = CORE_PERIPHERALS.borrow(cs).borrow_mut();
        NVIC::unpend(Interrupt::LDMA);
        NVIC::mask(Interrupt::LDMA);
        unsafe {
            core_periph.NVIC.set_priority(Interrupt::LDMA, 3);
            NVIC::unmask(Interrupt::LDMA);
        }
    });

    delay(1000);

    let hardware = Hardware::new(&mut peripherals);

    free(|cs| {
        PERIPHERALS.borrow(cs).replace(Some(peripherals));
    });

    // MAGIC calibration numbers obtained through KOLIBRI tool
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

    let mut frame_set: Vec<Frame> = Vec::new();

        

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
        process_nfc_buffer_miller_only(&mut frame_set, &nfc_buffer);

        if frame_set.len() != 0 {
            let mut map = BTreeMap::new();
            for element in frame_set.iter() {
                map.entry(element).and_modify(|count| *count += 1).or_insert(1);
            }
            panic!("{:?}", map)
        }


        // 5. restore responsiveness at some point
    }
}

const TARGET_PACKET_LEN: usize = 252;

pub struct MillerProcessed {
    frame_set: Vec<Frame>,
    tail: MillerTimesDown<FREQ>,
}

pub fn get_miller_frames(input: &[u16], maybe_previous_tail: Option<MillerTimesDown<FREQ>>) -> MillerProcessed {
    let times_down = MillerTimesDown::<FREQ>::from_raw(input);
    let (new_nose, full_frames_with_tail) = times_down.split_first().expect("expect at least one break with current buffer settings");
    let first_chunk = {
        if let Some(previous_tail) = maybe_previous_tail {
            new_nose.stitch_with_tail(&previous_tail)
        }
        else {new_nose.to_owned()}
    };
    let (new_tail, full_frames) = full_frames_with_tail.split_last().map(|(tail, full_frames)| (tail.to_owned(), full_frames)).expect("expect at least one break with current buffer settings");
    let mut frame_set: Vec<Frame> = Vec::new();
    for chunk in core::iter::once(&first_chunk).chain(full_frames.into_iter()) {
        if let Ok(miller_element_set) = chunk.convert() {
            if let Ok(frame) = miller_element_set.collect_frame() {
                if let Frame::Standard(ref standard_frame) = frame {
                    if standard_frame.len() == TARGET_PACKET_LEN {
                        frame_set.push(frame);
                    }
                }
            }
        }
    }
    MillerProcessed {
        frame_set,
        tail: new_tail,
    }
}

pub fn process_nfc_buffer_miller_only(frame_set: &mut Vec<Frame>, nfc_buffer: &[u16; 4*BUF_QUARTER]) {
    let mut read_from = None;
    let mut maybe_previous_tail = None;
    free(|cs| {
        let buffer_info = BUFFER_INFO.borrow(cs).borrow();
        read_from = buffer_info.buffer_status.read_from();
        maybe_previous_tail = buffer_info.maybe_previous_tail.clone();
    });
    let decoder_input = match read_from {
        Some(BufRegion::Reg0) => &nfc_buffer[..BUF_QUARTER],
        Some(BufRegion::Reg1) => &nfc_buffer[BUF_QUARTER..2*BUF_QUARTER],
        Some(BufRegion::Reg2) => &nfc_buffer[2*BUF_QUARTER..3*BUF_QUARTER],
        Some(BufRegion::Reg3) => &nfc_buffer[3*BUF_QUARTER..],
        None => return,
    };
    let mut miller_processed = get_miller_frames(decoder_input, maybe_previous_tail);
    frame_set.append(&mut miller_processed.frame_set);
    free(|cs| {
        let mut buffer_info = BUFFER_INFO.borrow(cs).borrow_mut();
        buffer_info.maybe_previous_tail = Some(miller_processed.tail);
        let was_write_halted = buffer_info.buffer_status.is_write_halted();
        buffer_info.buffer_status.pass_read_done().expect("to do");
        if was_write_halted {
            panic!("was write halted");
            unsafe {
                NVIC::unmask(Interrupt::LDMA);
            }
        }
    });
}

