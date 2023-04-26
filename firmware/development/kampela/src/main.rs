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

use efm32pg23_fix::{CorePeripherals, interrupt, Interrupt, NVIC, Peripherals};

mod ui;
use ui::UI;
mod nfc;
use nfc::{BufferInfo, BufferStatus, process_nfc_buffer_miller_only};

#[global_allocator]
static HEAP: Heap = Heap::empty();

use kampela_system::{
    PERIPHERALS, CORE_PERIPHERALS, in_free,
    devices::{power::measure_voltage, se_rng, touch::{FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES}},
    draw::{FrameBuffer, make_text, burning_tank}, 
    init::init_peripherals,
    BUF_QUARTER, LINK_1, LINK_2, LINK_DESCRIPTORS, TIMER0_CC0_ICF, NfcXfer, NfcXferBlock,
};

use alloc::{borrow::ToOwned, collections::BTreeMap};

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::free;
use cortex_m::interrupt::Mutex;

use nfca_parser::{frame::{Frame, FrameAttributed}, miller::*, time_record_both_ways::*};


lazy_static!{
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


    free(|cs| {
        PERIPHERALS.borrow(cs).replace(Some(peripherals));
    });


    let mut touch_data = [0; LEN_NUM_TOUCHES];
    let mut touched = false;

    let mut frame_set: Vec<Frame> = Vec::new();

    let mut ui = UI::init();
    in_free(|p| {
        let x = se_rng::SeRng{peripherals: p};
    });

    loop {
        ui.advance();
        //nfc.advance();
        // 4. non-UI loop time
        process_nfc_buffer_miller_only(&mut frame_set, &nfc_buffer);

        if frame_set.len() != 0 {
            let mut map = BTreeMap::new();
            for element in frame_set.iter() {
                map.entry(element).and_modify(|count| *count += 1).or_insert(1);
            }
            panic!("{:?}", map)
        }

    }

}




