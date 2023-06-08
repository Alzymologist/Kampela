#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate core;

use alloc::format;
use core::{alloc::Layout, panic::PanicInfo};
use core::ptr::addr_of;
use cortex_m::asm::delay;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_alloc::Heap;

use lazy_static::lazy_static;

use efm32pg23_fix::{interrupt, Interrupt, NVIC, Peripherals};

mod ui;
//use ui::UI;
mod nfc;
use nfc::{BufferInfo, turn_nfc_collector_correctly, NfcCollector, PreviousTail, process_nfc_payload, GOT_FRAMES, IN_BUFFER};

#[global_allocator]
static HEAP: Heap = Heap::empty();

use kampela_system::{
    PERIPHERALS, CORE_PERIPHERALS, in_free,
//    devices::{psram::ExternalPsram, se_rng, touch::{FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES}},
    draw::burning_tank, 
    init::init_peripherals,
    BUF_QUARTER, CH_TIM0, LINK_1, LINK_2, LINK_DESCRIPTORS, TIMER0_CC0_ICF, NfcXfer, NfcXferBlock,
};

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::free;
use cortex_m::interrupt::Mutex;

use p256::ecdsa::{signature::{hazmat::PrehashVerifier}, Signature, VerifyingKey};
use sha2::Digest;
use spki::DecodePublicKey;
use kampela_system::devices::psram::psram_read_at_address;

lazy_static!{
    #[derive(Debug)]
    static ref BUFFER_INFO: Mutex<RefCell<BufferInfo>> = Mutex::new(RefCell::new(BufferInfo::new()));
}

/*
static mut GPIO_ODD_INT: bool = false;
static mut COUNT_ODD: bool = false;
static mut GPIO_EVEN_INT: bool = false;
static mut COUNT_EVEN: bool = false;
static mut READER: Option<[u8;5]> = None;
*/

#[alloc_error_handler]
fn oom(l: Layout) -> ! {
    panic!("out of memory: {:?}, heap used: {}, free: {}, got frames: {}, frames in buffer: {}", l, HEAP.used(), HEAP.free(), unsafe {GOT_FRAMES}, unsafe {IN_BUFFER});
}

#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    let mut peripherals = unsafe{Peripherals::steal()};
    burning_tank(&mut peripherals, format!("{:?}", panic));
    loop {}
}

#[exception]
unsafe fn HardFault(exception_frame: &ExceptionFrame) -> ! {
    panic!("hard fault: {:?}", exception_frame)
}

#[interrupt]
fn LDMA() {
    free(|cs| {
        if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
            peripherals.LDMA_S.if_.reset();
            let mut buffer_info = BUFFER_INFO.borrow(cs).borrow_mut();
//            let buffer_info_old = buffer_info.buffer_status.clone();
            match buffer_info.buffer_status.pass_if_done7() {
                Ok(_) => {
                    if !buffer_info.buffer_status.is_write_halted() {
                        peripherals.LDMA_S.linkload.write(|w_reg| w_reg.linkload().variant(1 << CH_TIM0));
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
        const HEAP_SIZE: usize = 0x7600;
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

//    let mut touch_data = [0; LEN_NUM_TOUCHES];
//    let mut touched = false;

    let mut nfc_collector = NfcCollector::new();

//    panic!("was still alive!");

//    let mut ui = UI::init();

    let mut counter = 0usize;
//    let mut counter_frames = 0usize;


    loop {
        if HEAP.free() < 1000 {panic!("heap ending! {}; counter: {counter}", HEAP.free())}
        
        turn_nfc_collector_correctly(&mut nfc_collector, &nfc_buffer);

        if let NfcCollector::Done(a) = nfc_collector {

            NVIC::mask(Interrupt::LDMA);
            free(|cs| {
                let mut buffer_info = BUFFER_INFO.borrow(cs).borrow_mut();
                buffer_info.maybe_previous_tail = PreviousTail::Lost;
            });

            let nfc_payload = process_nfc_payload(a).unwrap();

            // calculate correct hash of the payload
            let mut first_byte: Option<u8> = None;
{
            let mut hasher = sha2::Sha256::new();
            in_free(|peripherals| {
                for shift in 0..nfc_payload.encoded_data.total_len {
                    let address = nfc_payload.encoded_data.start_address.try_shift(shift).unwrap();
                    let single_element_vec = psram_read_at_address(peripherals, address, 1usize).unwrap();
                    if shift == 0 {first_byte = Some(single_element_vec[0])}
                    hasher.update(&single_element_vec);
                }
            });
            let hash = hasher.finalize();
            
            // transform signature and verifying key from der-encoding into usable form
            let signature = match Signature::from_der(&nfc_payload.companion_signature) {
                Ok(a) => a,
                Err(_) => panic!("signature recovery, slice start {:?}, length: {}", &nfc_payload.companion_signature[..10], nfc_payload.companion_signature.len()),
            };
            let verifying_key = VerifyingKey::from_public_key_der(&nfc_payload.companion_public_key).unwrap();

            // and check
            assert!(verifying_key
                .verify_prehash(&hash, &signature)
                .is_ok());

}
            let string_addition = match first_byte.unwrap() {
                0 => format!("bytes"),
                1 => format!("derivation"),
                2 => format!("signable transaction"),
                3 => format!("specs"),
                4 => format!("specs set"),
                a => format!("unexpected {a} payload"),
            };

            panic!("got valid signed payload: {string_addition}");
        }
        counter += 1;
    }
}

