#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate core;

use alloc::{format, string::{String, ToString}, vec::Vec};
use core::{alloc::Layout, panic::PanicInfo};
use core::ptr::addr_of;
use cortex_m::asm::delay;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_alloc::Heap;
use lazy_static::lazy_static;
use parity_scale_codec::Decode;
use primitive_types::H256;

use efm32pg23_fix::{interrupt, Interrupt, NVIC, Peripherals};

mod ui;
use ui::UI;
mod nfc;
use nfc::{BufferStatus, turn_nfc_collector_correctly, NfcCollector, process_nfc_payload};

#[global_allocator]
static HEAP: Heap = Heap::empty();

use kampela_system::{
    PERIPHERALS, CORE_PERIPHERALS, in_free,
    devices::{power::ADC, psram::{psram_read_at_address, CheckedMetadataMetal, ExternalPsram, PsramAccess}, se_rng::SeRng},
    draw::burning_tank, 
    init::init_peripherals,
    parallel::Operation,
    BUF_QUARTER, CH_TIM0, LINK_1, LINK_2, LINK_DESCRIPTORS, TIMER0_CC0_ICF, NfcXfer, NfcXferBlock,
};

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::free;
use cortex_m::interrupt::Mutex;

//use p256::ecdsa::{signature::{hazmat::PrehashVerifier}, Signature, VerifyingKey};
//use sha2::Digest;
//use spki::DecodePublicKey;
use substrate_parser::{MarkedData, ShortSpecs, compacts::find_compact, parse_transaction};
use schnorrkel::{
    context::attach_rng,
    keys::Keypair,
    signing_context,
};

lazy_static!{
    #[derive(Debug)]
    static ref BUFFER_STATUS: Mutex<RefCell<BufferStatus>> = Mutex::new(RefCell::new(BufferStatus::new()));
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
    panic!("out of memory: {:?}, heap used: {}, free: {}", l, HEAP.used(), HEAP.free());
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
            let mut buffer_status = BUFFER_STATUS.borrow(cs).borrow_mut();
            match buffer_status.pass_if_done7() {
                Ok(_) => {
                    if !buffer_status.is_write_halted() {
                        peripherals.LDMA_S.linkload.write(|w_reg| w_reg.linkload().variant(1 << CH_TIM0));
                    }
                },
                Err(_) => {}
            }
        }
        else {panic!("can not borrow peripherals in ldma interrupt")}
    });
}

const ALICE_KAMPELA_KEY: &[u8] = &[24, 79, 109, 158, 13, 45, 121, 126, 185, 49, 212, 255, 134, 18, 243, 96, 119, 210, 175, 115, 48, 181, 19, 238, 61, 135, 28, 186, 185, 31, 59, 9, 172, 24, 200, 176, 25, 207, 214, 199, 221, 214, 171, 143, 80, 246, 86, 104, 48, 40, 21, 99, 114, 3, 232, 85, 101, 7, 128, 198, 36, 11, 101, 63, 180, 120, 97, 66, 191, 43, 74, 35, 69, 3, 219, 194, 72, 141, 68, 185, 188, 177, 117, 246, 178, 250, 89, 134, 116, 20, 248, 247, 151, 45, 130, 59];
const SIGNING_CTX: &[u8] = b"substrate";

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 0x4600;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let westend_specs = ShortSpecs {
        base58prefix: 42,
        decimals: 12,
        name: "westend".to_string(),
        unit: "WND".to_string(),
    };

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

    let pair_derived = Keypair::from_bytes(ALICE_KAMPELA_KEY).unwrap();

//    let mut touch_data = [0; LEN_NUM_TOUCHES];
//    let mut touched = false;

    let mut nfc_collector = NfcCollector::new();

    let mut ui = UI::init();
    let mut adc = ADC::new();

    let mut got_transaction = None;

    loop {
        adc.advance(());
        ui.advance(adc.read());
        
        turn_nfc_collector_correctly(&mut nfc_collector, &nfc_buffer);

        if let NfcCollector::Done(ref a) = nfc_collector {
            NVIC::mask(Interrupt::LDMA);

            let nfc_payload = process_nfc_payload(a).unwrap();

            let mut first_byte: Option<u8> = None;
            in_free(|peripherals| {
                first_byte = Some(psram_read_at_address(peripherals, nfc_payload.encoded_data.start_address, 1usize).unwrap()[0]);
            });
            
/* // calculate correct hash of the payload
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
            let signature = Signature::from_der(&nfc_payload.companion_signature).unwrap();
            let verifying_key = VerifyingKey::from_public_key_der(&nfc_payload.companion_public_key).unwrap();

            // and check
            assert!(verifying_key
                .verify_prehash(&hash, &signature)
                .is_ok());

}
*/

            match first_byte {
                Some(0) => {break},
                Some(3) => {
                    let mut genesis_hash_bytes_option = None;
                    in_free(|peripherals| {
                        let address = nfc_payload.encoded_data.start_address.try_shift(1usize).unwrap();
                        genesis_hash_bytes_option = Some(psram_read_at_address(peripherals, address, 32usize).unwrap());
                    });
                    let genesis_hash = H256(genesis_hash_bytes_option.unwrap().try_into().expect("static size"));

                    let mut metadata_psram_access_option = None;
                    let mut position = 1usize + 32usize;
                    in_free(|peripherals| {
                        let mut external_psram = ExternalPsram{peripherals};
                        let compact_meta = find_compact::<u32, PsramAccess, ExternalPsram>(&nfc_payload.encoded_data, &mut external_psram, position).unwrap();
                        let start_address = nfc_payload.encoded_data.start_address.try_shift(compact_meta.start_next_unit).unwrap();
                        metadata_psram_access_option = Some(PsramAccess{start_address, total_len: compact_meta.compact as usize});
                        position = compact_meta.start_next_unit + compact_meta.compact as usize;
                    });
                    let metadata_psram_access = metadata_psram_access_option.unwrap();

                    let mut checked_metadata_metal_option = None;
                    in_free(|peripherals| {
                        let mut external_psram = ExternalPsram{peripherals};
                        checked_metadata_metal_option = Some(CheckedMetadataMetal::from(&metadata_psram_access, &mut external_psram).unwrap());
                    });
                    let checked_metadata_metal = checked_metadata_metal_option.unwrap();

                    let mut signable_transaction_option = None;
                    in_free(|peripherals| {
                        let mut external_psram = ExternalPsram{peripherals};
                        let compact_transaction = find_compact::<u32, PsramAccess, ExternalPsram>(&nfc_payload.encoded_data, &mut external_psram, position).unwrap();
                        let start_address = nfc_payload.encoded_data.start_address.try_shift(compact_transaction.start_next_unit).unwrap();
                        let transaction_encoded = psram_read_at_address(peripherals, start_address, compact_transaction.compact as usize).unwrap();
                        signable_transaction_option = Some(Vec::<u8>::decode(&mut &transaction_encoded[..]).unwrap());
                        position = compact_transaction.start_next_unit + compact_transaction.compact as usize;
                    });
                    let signable_transaction = signable_transaction_option.unwrap();

                    in_free(|peripherals| {
                        let start_address = nfc_payload.encoded_data.start_address.try_shift(position).unwrap();
                        let public_key = psram_read_at_address(peripherals, start_address, 33usize).unwrap();
                        assert!(public_key.starts_with(&[1u8]) & (public_key[1..] == ALICE_KAMPELA_KEY[64..]), "Unknown address.");
                    });

                    in_free(|peripherals| {
                        let mut external_psram = ExternalPsram{peripherals};
                        let binding = signable_transaction.as_ref();
                        let marked_data: MarkedData<'_, &[u8], ()> = MarkedData::mark(&binding, &mut ()).unwrap();
                        let data_to_sign = binding[marked_data.call_start()..].to_vec();
                        let decoded_transaction = parse_transaction(
                            &signable_transaction.as_ref(),
                            &mut external_psram,
                            &checked_metadata_metal,
                            genesis_hash
                        ).unwrap();
                        let carded = decoded_transaction.card(&westend_specs);
                        got_transaction = Some((
                            carded.call_result.unwrap().iter().map(|card| card.show()).collect::<Vec<String>>().join("\n"),
                            carded.extensions.iter().map(|card| card.show()).collect::<Vec<String>>().join("\n"),
                            data_to_sign
                        ));
                    });
                },
                _ => {nfc_collector = NfcCollector::new();}
            }
        }
        if got_transaction.is_some() {

            let transaction = got_transaction.unwrap();
            let context = signing_context(SIGNING_CTX);
            let signature = pair_derived.sign(attach_rng(context.bytes(&transaction.2), &mut SeRng{}));
            let mut signature_with_id: [u8; 65] = [1; 65];
            signature_with_id[1..].copy_from_slice(&signature.to_bytes());
            let signature_into_qr: [u8; 130] = hex::encode(signature_with_id).into_bytes().try_into().expect("static known length");

            ui.handle_rx(transaction.0, transaction.1, signature_into_qr);

            break
        }
    }

    loop {
        adc.advance(());
        ui.advance(adc.read());
    }
}

