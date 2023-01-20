#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate core;

use core::{alloc::Layout, panic::PanicInfo};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use embedded_alloc::Heap;

use efm32pg23_fix::Peripherals;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use app::{
    led_blinker::{blink_left, blink_right},
    test_aes_gcm_sequence, test_entropy_seed, test_pairs_derive_sign_verify, test_rng_interaction,
    COUNT,
};

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[allow(non_snake_case)]
#[exception]
fn SysTick() {
    unsafe {
        COUNT = COUNT.wrapping_add(1);
    }
}

fn init_systick(cortex_periph: &mut cortex_m::Peripherals) {
    let syst = &mut cortex_periph.SYST;
    const DEFAULT_HZ: u32 = 14_000_000u32;

    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(DEFAULT_HZ / 1_000u32);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
}

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 16384;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let mut cortex_periph = cortex_m::Peripherals::take().unwrap();
    init_systick(&mut cortex_periph);

    let mut peripherals = Peripherals::take().unwrap();

    blink_left(&mut peripherals, 10, 300, 100);

    test_aes_gcm_sequence(&mut peripherals);
    blink_right(&mut peripherals, 1, 600, 100);

    test_rng_interaction(&mut peripherals);
    blink_right(&mut peripherals, 1, 600, 100);

    test_entropy_seed(&mut peripherals);
    blink_right(&mut peripherals, 1, 600, 100);

    test_pairs_derive_sign_verify(&mut peripherals);

    blink_right(&mut peripherals, 10, 300, 100);

    loop {}
}
