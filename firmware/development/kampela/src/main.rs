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
use nfc::{BufferInfo, BufferStatus, turn_nfc_collector, NfcCollector, process_nfc_payload, process_nfc_buffer_miller_only};

#[global_allocator]
static HEAP: Heap = Heap::empty();

use kampela_system::{
    PERIPHERALS, CORE_PERIPHERALS, in_free,
    devices::{power::ADC, se_rng, touch::{FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES}},
    draw::{FrameBuffer, burning_tank}, 
    init::init_peripherals,
    parallel::Operation,
    BUF_QUARTER, LINK_1, LINK_2, LINK_DESCRIPTORS, TIMER0_CC0_ICF, NfcXfer, NfcXferBlock,
};

use alloc::{borrow::ToOwned, collections::BTreeMap};

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::free;
use cortex_m::interrupt::Mutex;

use nfca_parser::{frame::{Frame, FrameAttributed}, miller::*};


lazy_static!{
    #[derive(Debug)]
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
    panic!("out of memory!");
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
        if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
            peripherals.LDMA_S.if_.reset();
            let mut buffer_info = BUFFER_INFO.borrow(cs).borrow_mut();
            let buffer_info_old = buffer_info.buffer_status.clone();
            match buffer_info.buffer_status.pass_if_done7() {
                Ok(_) => {
                    if !buffer_info.buffer_status.is_write_halted() {
                        peripherals.LDMA_S.linkload.write(|w_reg| w_reg.linkload().variant(0b11111111));
                    }
                },
/*
                if buffer_info.buffer_status.is_write_halted() {
                    NVIC::mask(Interrupt::LDMA);
                }
                else {
                    if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                        peripherals.LDMA_S.if_.reset();
                        peripherals.LDMA_S.linkload.write(|w_reg| w_reg.linkload().variant(1<<7));
//                        panic!("has reset ldma flags");
                    }
                    else {unreachable!()} // TODO
                }
*/
                Err(_) => {}//panic!("old: {:?}, current: {:?}", buffer_info_old, buffer_info) //TODO
            }
        }
        else {panic!("can not borrow peripherals in ldms interrupt")}
    });
}


#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 32768;
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

    let mut nfc_collector = NfcCollector::new();
    let mut frames: Vec<[u8; 240]> = Vec::new();

    let mut counter = 0usize;
    let mut counter_frames = 0usize;

    let mut frame_set: Vec<Frame> = Vec::new();

    let mut ui = UI::init();

    let mut adc = ADC::new();

    loop {
        adc.advance(());
        ui.advance(adc.read());
        turn_nfc_collector(&mut nfc_collector, &nfc_buffer);
        process_nfc_buffer_miller_only(&mut frame_set, &nfc_buffer);

      if let NfcCollector::Done(a) = nfc_collector {
            let nfc_payload = process_nfc_payload(a).unwrap();
            panic!("for nfc payload \n{:?}", nfc_payload);
        }

    }
}


// [27325, 182, 269, 269, 270, 357, 1907, 354, 278, 179, 177, 265, 360, 276, 180, 177, 181, 180, 177, 182, 259, 4601, 181, 179, 180, 179, 269, 357, 360, 272, 180, 179, 180, 179, 180, 179, 269, 180, 179, 180, 357, 359, 272, 270, 357, 183, 268, 270, 180, 269, 34034]

