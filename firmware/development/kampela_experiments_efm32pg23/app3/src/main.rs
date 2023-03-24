#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate core;

use alloc::format;
use core::{alloc::Layout, panic::PanicInfo};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use embedded_alloc::Heap;

use embedded_graphics::prelude::Point;

use efm32pg23_fix::{interrupt, Interrupt, NVIC, Peripherals};

#[global_allocator]
static HEAP: Heap = Heap::empty();

use app::{draw::{FrameBuffer, make_text, highlight_point, burning_tank}, init::{ft6336_read_at, init_peripherals, FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES}, COUNT, visible_delay};
use app::devices::{power::measure_voltage, se_rng};
use kampela_ui::{display_def::*, uistate};

static mut PUSHED: bool = false;
static mut TOUCH_UNDEBOUNCE: bool = true;
static mut VOLTAGE: u32 = 0;

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

#[interrupt]
fn I2C0() {
    static mut SOMESTUFF: i32 = 0;
    *SOMESTUFF += 1;
}

#[interrupt]
fn GPIO_ODD() {
    unsafe{
        PUSHED = true;
    }
}

#[interrupt]
fn IADC() {
    let measure = 0;
    unsafe{
        VOLTAGE = measure;
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

    let mut cortex_periph = cortex_m::Peripherals::take().unwrap();
    init_systick(&mut cortex_periph);
   

    unsafe {
        NVIC::unmask(Interrupt::GPIO_ODD);
        //NVIC::unmask(Interrupt::TIMER1);
        //cortex_periph.NVIC.set_priority(Interrupt::GPIO_ODD, 3);
        //NVIC::unmask(Interrupt::I2C0);
    }
    
    let mut peripherals = Peripherals::take().unwrap();

    init_peripherals(&mut peripherals);

    //let test_voltage = measure_voltage(&mut peripherals);
    //burning_tank(&mut peripherals, format!("voltage: {}", test_voltage));

    let mut state = uistate::UIState::new(&mut se_rng::SeRng{peripherals: &mut peripherals}); 
    // line for debug init messages
    //panic!("lol: {}", test_voltage);

    let mut update = uistate::UpdateRequest::new();
    update.set_slow();

    // display abstraction
    let mut slow_screen = FrameBuffer::new_white();

    let mut input = None;

    loop {
        // 1. update ui if needed
        if update.read_fast() {
            //let test_voltage = measure_voltage(&mut peripherals);
            //burning_tank(&mut peripherals, format!("voltage: {}", test_voltage));
            slow_screen.apply_fast(&mut peripherals);
            peripherals
                .GPIO_S
                .if_
                .write(|w_reg| w_reg.extif0().clear_bit())
        }
        if update.read_slow() {
            //let test_voltage = measure_voltage(&mut peripherals);
            //burning_tank(&mut peripherals, format!("voltage: {}", test_voltage));
            state.render(&mut slow_screen);
            slow_screen.apply(&mut peripherals);
            peripherals
                .GPIO_S
                .if_
                .write(|w_reg| w_reg.extif0().clear_bit());

        }

        // 2. read input if possible
        if peripherals.GPIO_S.if_.read().extif0().bit_is_set() {
            let touch_data = ft6336_read_at::<LEN_NUM_TOUCHES>(&mut peripherals, FT6X36_REG_NUM_TOUCHES).unwrap();
            
            let detected_y = (((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16) as i32;
            let detected_x = (((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16) as i32;
            input = Some(Point::new(SCREEN_SIZE_X as i32 - detected_x, /*SCREEN_SIZE_Y as i32 - */detected_y));

            peripherals
                .GPIO_S
                .if_
                .write(|w_reg| w_reg.extif0().clear_bit())
        }

        // 3. handle input
        if let Some(point) = input {
            update = state.handle_event(point, &mut se_rng::SeRng{peripherals: &mut peripherals}, &mut slow_screen).unwrap();
            input = None;
        }


        // 4. non-UI loop time
        //unsafe{PUSHED = false;}

        // 5. restore responsiveness at some point
    }
}
