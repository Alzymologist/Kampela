#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate core;

use core::{alloc::Layout, panic::PanicInfo};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use embedded_alloc::Heap;
use embedded_graphics::prelude::Point;

use efm32pg23_fix::Peripherals;

use kampela_display_common::display_def::SCREEN_SIZE_X;
use kampela_system::{
    devices::{
        se_rng::SeRng,
        touch::{ft6336_read_at, FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES},
    },
    draw::{highlight_point, FrameBuffer},
    init::init_peripherals,
    COUNT,
};
use kolibri::uistate::UIState;

#[global_allocator]
static HEAP: Heap = Heap::empty();

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

    init_peripherals(&mut peripherals);

    let mut do_update = true;
    let mut state = UIState::init(&mut SeRng {
        peripherals: &mut peripherals,
    });

    let measured_affine = loop {
        if do_update {
            let mut display = FrameBuffer::new_white();
            state.render(&mut display).unwrap();
            display.apply(&mut peripherals);
            do_update = false;
        } else if peripherals.GPIO_S.if_.read().extif0().bit_is_set() {
            peripherals
                .GPIO_S
                .if_
                .write(|w_reg| w_reg.extif0().clear_bit());

            let touch_data =
                ft6336_read_at::<LEN_NUM_TOUCHES>(&mut peripherals, FT6X36_REG_NUM_TOUCHES)
                    .unwrap();
            if touch_data[0] == 1 {
                if let UIState::Complete(a) = state {
                    break a;
                } else {
                    let detected_y =
                        ((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16;
                    let detected_x =
                        ((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16;
                    let point = Point {
                        x: SCREEN_SIZE_X as i32 - detected_x as i32,
                        y: detected_y as i32,
                    };
                    do_update = state
                        .process_touch(
                            point,
                            &mut SeRng {
                                peripherals: &mut peripherals,
                            },
                        )
                        .unwrap();
                }
            }
        }
    };

    loop {
        if peripherals.GPIO_S.if_.read().extif0().bit_is_set() {
            peripherals
                .GPIO_S
                .if_
                .write(|w_reg| w_reg.extif0().clear_bit());

            let touch_data =
                ft6336_read_at::<LEN_NUM_TOUCHES>(&mut peripherals, FT6X36_REG_NUM_TOUCHES)
                    .unwrap();
            if touch_data[0] == 1 {
                let detected_y = ((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16;
                let detected_x = ((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16;
                let point = Point {
                    x: SCREEN_SIZE_X as i32 - detected_x as i32,
                    y: detected_y as i32,
                };
                let point_on_display = measured_affine.transform(&point);
                highlight_point(&mut peripherals, point_on_display);
            }
        }
    }
}
