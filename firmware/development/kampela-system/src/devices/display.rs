//! display control functions

use efm32pg23_fix::{GPIO_S, Peripherals};
use cortex_m::asm::delay;
use crate::peripherals::usart::*;
use crate::peripherals::gpio_pins::{display_res_clear, display_res_set};
use crate::{FreeError, if_in_free, in_free};
use crate::parallel::Operation;

/// Draw sequence
///
/// Iterate through this to perform drawing and send display to proper sleep mode
pub struct Request<R: RequestType> {
    state: RequestState<R>,
    timer: usize,
}

pub enum RequestState<R: RequestType> {
    Init(EPDInit),
    Draw(R),
}

impl <R: RequestType> Request<R> {
        fn count(&mut self) -> bool {
        if self.timer == 0 {
            false
        } else {
            self.timer -= 1;
            true
        }
    }
}

impl <R: RequestType> Operation for Request<R> {
    type DesiredOutput = bool;
    type StateEnum = RequestState<R>;

    fn new() -> Self {
        Self {
            state: RequestState::Init(EPDInit::new()),
            timer: 0,
        }
    }

    fn wind(&mut self, state: RequestState<R>, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self) -> bool {
        match self.state {
            RequestState::Init(ref mut a) => {
                if a.advance() {
                    self.wind_d(RequestState::Draw(R::new()));
                };
                false
            },
            RequestState::Draw(ref mut a) => {
                /*
        epaper_draw_stuff_quickly(peripherals, self.data.into_inner());
        or
        epaper_draw_stuff_differently(peripherals, self.data.into_inner());
                */
                true
            },
        }
    }
}

/// BUSY is on port B, pin [`SPI_BUSY_PIN`].
pub fn display_is_busy() -> Result<bool, FreeError> {
    if_in_free(|peripherals| spi_is_busy(&mut peripherals.GPIO_S))
}

/// Send command to EPD
///
/// for critical section
fn epaper_write_command(peripherals: &mut Peripherals, command_set: &[u8]) {
    // CS clear corresponds to selected chip, see epaper docs

    deselect_display(&mut peripherals.GPIO_S);
    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start
    
    display_select_command(&mut peripherals.GPIO_S);
    for command in command_set.iter() {
        write_to_usart(peripherals, *command);
    }
    deselect_display(&mut peripherals.GPIO_S);
}

/// Send data to EPD
fn epaper_write_data(peripherals: &mut Peripherals, data_set: &[u8]) {
    deselect_display(&mut peripherals.GPIO_S);
    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start

    display_select_data(&mut peripherals.GPIO_S);
    for data in data_set.iter() {
        write_to_usart(peripherals, *data);
    }
    deselect_display(&mut peripherals.GPIO_S);
    //    display_data_command_clear(peripherals);
}

/// BUSY is on port B, pin [`SPI_BUSY_PIN`].
///
/// Blocking variant to be called from critical section (init, panic)
pub fn display_is_busy_cs(peripherals: &mut Peripherals) -> bool {
    spi_is_busy(&mut peripherals.GPIO_S)
}

/// Reset EPD, should be performed in many situations
///
/// Why these specific numbers for delays?
pub fn epaper_reset(gpio: &mut GPIO_S) {
    delay(1000);
    display_res_clear(gpio);
    delay(5000);
    display_res_set(gpio);
    delay(10000);
    display_res_clear(gpio);
    delay(5000);
    deselect_display(gpio); // this is not the default state, should not be here
    delay(5000);
}

/// Last command in drawing protocol; actually starts display action
pub fn epaper_update(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x12]);
    delay(100000);
    while display_is_busy_cs(peripherals) {}
    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "Display Update Control"
epaper_write_data(peripherals, &[0xF7]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual, Y: "Activate Display Update Sequence"
    while display_is_busy_cs(peripherals) {}
}

/// Partial display update; used to initiate display action when performing fast drawing without
/// full clear
pub fn epaper_update_part(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "Display Update Control"
    epaper_write_data(peripherals, &[0xFF]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual, Y: "Activate Display Update Sequence"
    delay(1000); // why delay, from where the number?
    while display_is_busy_cs(peripherals) {}
}

pub const BUFSIZE: usize = 5808;

/// Normal drawing protocol, with full screen clearing
pub fn epaper_draw_stuff_differently(peripherals: &mut Peripherals, stuff: [u8; BUFSIZE]) {
    epaper_reset(&mut peripherals.GPIO_S);
    epaper_write_command(peripherals, &[0x4E]);
    epaper_write_data(peripherals, &[0x00]);
    epaper_write_command(peripherals, &[0x4F]);
    epaper_write_data(peripherals, &[0x07]);
    epaper_write_command(peripherals, &[0x24]); // from manual, Y: "Write Black and White image to RAM"
    epaper_write_data(peripherals, &stuff);
    epaper_write_command(peripherals, &[0x26]);
    epaper_write_data(peripherals, &stuff);
    epaper_update(peripherals);
}

/// Fast and dirty refresh drawing
pub fn epaper_draw_stuff_quickly(peripherals: &mut Peripherals, stuff: [u8; BUFSIZE]) {
    epaper_reset(&mut peripherals.GPIO_S);
    epaper_write_command(peripherals, &[0x4E]);
    epaper_write_data(peripherals, &[0x00]);
    epaper_write_command(peripherals, &[0x4F]);
    epaper_write_data(peripherals, &[0x07]);
    epaper_write_command(peripherals, &[0x3C]);
    epaper_write_data(peripherals, &[0x80]);
    epaper_write_command(peripherals, &[0x24]); // from manual, Y: "Write Black and White image to RAM"
    epaper_write_data(peripherals, &stuff);
    epaper_update_part(peripherals);
}

/// Send EPD to low power state; should be performed when screen is not drawing at all times to
/// extend component life
pub fn epaper_deep_sleep(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x10]); // from manual, enter deep sleep
    epaper_write_data(peripherals, &[0x01]); // ?
    delay(100000); // why delay, from where the number?
}

/// EPD init, also should be performed to wake screen from sleep
///
/// used within critical section
pub fn epaper_hw_init_cs(peripherals: &mut Peripherals) {
    epaper_reset(&mut peripherals.GPIO_S);
    while display_is_busy_cs(peripherals) {}
    epaper_write_command(peripherals, &[0x12]);
    delay(10000);
    while display_is_busy_cs(peripherals) {}
}

/// EPD init to wake up display
pub struct EPDInit {
    state: EPDInitState,
}

pub enum EPDInitState {
    Reset(Reset),
    WakeUp(EPDCommand<12>),
}

impl Operation for EPDInit {
    type DesiredOutput = bool;
    type StateEnum = EPDInitState;

    fn new() -> Self {
        Self {
            state: EPDInitState::Reset(Reset::new()),
        }
    }

    fn wind(&mut self, state: EPDInitState, _: usize) {
        self.state = state;
    }

    fn advance(&mut self) -> bool {
        match self.state{
            EPDInitState::Reset(ref mut a) => {
                if a.advance() {
                    self.wind(EPDInitState::WakeUp(EPDCommand::<12>::new()), 5000)
                }
                false
            },
            EPDInitState::WakeUp(ref mut a) => {
                a.advance()
            },
        }
    }
}

/// Reset display
///
/// notably used for waking up
pub struct Reset {
    state: ResetState,
    timer: usize,
}

pub enum ResetState {
    R0,
    R1,
    R2,
    R3,
}

impl Reset {
    fn count(&mut self) -> bool {
        if self.timer == 0 {
            false
        } else {
            self.timer -= 1;
            true
        }
    }
}

impl Operation for Reset {
    type DesiredOutput = bool;
    type StateEnum = ResetState;

    fn new() -> Self {
        Self {
            state: ResetState::R0,
            timer: 1000
        }
    }

    
    fn wind(&mut self, state: ResetState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }


    fn advance(&mut self) -> bool {
        if self.count() { return false };
        match self.state {
            ResetState::R0 => {
                in_free(|peripherals| display_res_clear(&mut peripherals.GPIO_S));
                self.wind(ResetState::R1, 5000);
                false
            },
            ResetState::R1 => {
                in_free(|peripherals| display_res_set(&mut peripherals.GPIO_S));
                self.wind(ResetState::R2, 10000);
                false
            },
            ResetState::R2 => {
                in_free(|peripherals| display_res_clear(&mut peripherals.GPIO_S));
                self.wind(ResetState::R3, 5000);
                false
            },
            ResetState::R3 => { // TODO: this is not ZERO operation, should it be here?
                in_free(|peripherals| deselect_display(&mut peripherals.GPIO_S));
                true
            },
        }
    }
}


pub trait RequestType: Operation {}
impl RequestType for FastDraw {}
impl RequestType for FullDraw {}


/// Fast draw sequence without full refresh
///
/// display should be awake
pub struct FastDraw {
    state: FastDrawState,
    timer: usize,
}

pub enum FastDrawState {
    Init,
}

impl Operation for FastDraw {
    type DesiredOutput = bool;
    type StateEnum = FastDrawState;

    fn new() -> Self {
        Self {
            state: FastDrawState::Init,
            timer: 0,
        }
    }

    fn wind(&mut self, state: FastDrawState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self) -> bool {
        match self.state{
            FastDrawState::Init => false
        }
    }

}

/// Slow drawing sequence with full refresh;
///
/// display should be awake
pub struct FullDraw {
    state: FullDrawState,
    timer: usize,
}

pub enum FullDrawState {
    Init,
}

impl Operation for FullDraw {
    type DesiredOutput = bool;
    type StateEnum = FullDrawState;

    fn new() -> Self {
        Self {
            state: FullDrawState::Init,
            timer: 0,
        }
    }

    fn wind(&mut self, state: FullDrawState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self) -> bool {
        match self.state{
            FullDrawState::Init => false
            
        }
    }
}

/// Send command `C` to EPD
pub struct EPDCommand<const C: u8>{
    state: EPDCommandState,
    timer: usize,
}

pub enum EPDCommandState {
    /// State where command is actually sent
    Init,
    /// Receive something to keep protocol running and close connection
    Aftermath,
}

impl <const C: u8> Operation for EPDCommand<C> {
    type DesiredOutput = bool;
    type StateEnum = EPDCommandState;

    fn new() -> Self {
        Self {
            state: EPDCommandState::Init,
            timer: 0,
        }
    }

    fn wind(&mut self, state: EPDCommandState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }
    
    fn advance(&mut self) -> bool {
        match self.state {
            EPDCommandState::Init => {
                in_free(|peripherals| {
                    deselect_display(&mut peripherals.GPIO_S);
                    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start
                    display_select_command(&mut peripherals.GPIO_S);
                });
                if if_in_free(|peripherals|
                    peripherals.USART0_S.status.read().txbl().bit_is_clear()
                ) == Ok(false) {
                    in_free(|peripherals|
                        peripherals
                            .USART0_S
                            .txdata
                            .write(|w_reg| w_reg.txdata().variant(C))
                            );
                    self.change(EPDCommandState::Aftermath);
                }
                false
            },
            EPDCommandState::Aftermath => {
                if if_in_free(|peripherals|
                    peripherals
                        .USART0_S
                        .status
                        .read()
                        .txc()
                        .bit_is_clear()
                ) == Ok(false) { 
                    in_free(|peripherals| {
                        peripherals
                            .USART0_S
                            .rxdata
                            .read()
                            .rxdata()
                            .bits();
                        deselect_display(&mut peripherals.GPIO_S);
                    });
                    true
                } else {
                    false
                }
            },
        }
    }
}
