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
pub struct Request<R: for <'a> RequestType<Input<'a> = &'a [u8]>> {
    state: RequestState<R>,
}

pub enum RequestState<R: for <'a> RequestType<Input<'a> = &'a [u8]>> {
    Init(EPDInit),
    Draw(R),
}

impl <R: for <'a> RequestType<Input<'a> = &'a [u8], Output = bool>> Operation for Request<R> {
    type Input<'a> = &'a [u8];
    type Output = bool;
    type StateEnum = RequestState<R>;

    fn new() -> Self {
        Self {
            state: RequestState::Init(EPDInit::new()),
        }
    }

    fn wind(&mut self, state: RequestState<R>, _delay: usize) {
        self.state = state;
    }

    fn advance(&mut self, data: Self::Input<'_>) -> bool {
        match self.state {
            RequestState::Init(ref mut a) => {
                if a.advance(()) {
                    let new_state = RequestState::Draw(R::new());
                    self.wind_d(new_state);
                };
                false
            },
            RequestState::Draw(ref mut a) => {
                if display_is_busy() != Ok(false) { return false };
                a.advance(data)
                /*
        epaper_draw_stuff_quickly(peripherals, self.data.into_inner());
        or
        epaper_draw_stuff_differently(peripherals, self.data.into_inner());
                */
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
///
/// for critical section
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
/// for critical section
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
    WakeUp(EPDCommand<0x12>),
}

impl Operation for EPDInit {
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = EPDInitState;

    fn new() -> Self {
        Self {
            state: EPDInitState::Reset(Reset::new()),
        }
    }

    fn wind(&mut self, state: EPDInitState, _: usize) {
        self.state = state;
    }

    fn advance(&mut self, _: ()) -> bool {
        match self.state{
            EPDInitState::Reset(ref mut a) => {
                if a.advance(()) {
                    self.wind(EPDInitState::WakeUp(EPDCommand::<0x12>::new()), 5000)
                }
                false
            },
            EPDInitState::WakeUp(ref mut a) => {
                if display_is_busy() != Ok(false) { return false };
                a.advance(())
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
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = ResetState;

    fn new() -> Self {
        Self {
            state: ResetState::R0,
            timer: 0
        }
    }

    
    fn wind(&mut self, state: ResetState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }


    fn advance(&mut self, _: ()) -> bool {
        if self.count() { return false };
        match self.state {
            ResetState::R0 => {
                in_free(|peripherals| display_res_clear(&mut peripherals.GPIO_S));
                self.wind_d(ResetState::R1);
                false
            },
            ResetState::R1 => {
                in_free(|peripherals| display_res_set(&mut peripherals.GPIO_S));
                self.wind_d(ResetState::R2);
                false
            },
            ResetState::R2 => {
                in_free(|peripherals| display_res_clear(&mut peripherals.GPIO_S));
                self.wind_d(ResetState::R3);
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
    PrepareC1(EPDCommand<0x4E>),
    PrepareD1(EPDDataB<0x00>),
    PrepareC2(EPDCommand<0x4F>),
    PrepareD2(EPDDataB<0x07>),
    PrepareC3(EPDCommand<0x3C>),
    PrepareD3(EPDDataB<0x80>),
    SendC1(EPDCommand<0x24>),
    SendD1(EPDData<BUFSIZE>),
    Update(UpdateFast),
}

impl Operation for FastDraw {
    type Input<'a> = &'a [u8];
    type Output = bool;
    type StateEnum = FastDrawState;

    fn new() -> Self {
        Self {
            state: FastDrawState::PrepareC1(EPDCommand::<0x4E>::new()),
            timer: 0,
        }
    }

    fn wind(&mut self, state: FastDrawState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, data: Self::Input<'_>) -> bool {
        match self.state{
            FastDrawState::PrepareC1(ref mut a) => {
                if a.advance(()) {
                    self.change(FastDrawState::PrepareD1(EPDDataB::<0x00>::new()));
                }
                false
            },
            FastDrawState::PrepareD1(ref mut a) => {
                if a.advance(()) {
                    self.change(FastDrawState::PrepareC2(EPDCommand::<0x4F>::new()));
                }
                false
            },
            FastDrawState::PrepareC2(ref mut a) => {
                if a.advance(()) {
                    self.change(FastDrawState::PrepareD2(EPDDataB::<0x07>::new()));
                }
                false
            },
            FastDrawState::PrepareD2(ref mut a) => {
                if a.advance(()) {
                    self.change(FastDrawState::PrepareC3(EPDCommand::<0x3C>::new()));
                }
                false
            },
            FastDrawState::PrepareC3(ref mut a) => {
                if a.advance(()) {
                    self.change(FastDrawState::PrepareD3(EPDDataB::<0x80>::new()));
                }
                false
            },
            FastDrawState::PrepareD3(ref mut a) => {
                if a.advance(()) {
                    self.change(FastDrawState::SendC1(EPDCommand::<0x24>::new()));
                }
                false
            },
            FastDrawState::SendC1(ref mut a) => {
                if a.advance(()) {
                    self.change(FastDrawState::SendD1(EPDData::<BUFSIZE>::new()));
                }
                false
            },
            FastDrawState::SendD1(ref mut a) => {
                if a.advance(data) {
                    self.change(FastDrawState::Update(UpdateFast::new()));
                }
                false
            },
            FastDrawState::Update(ref mut a) => {
                a.advance(())
            }
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
    PrepareC1(EPDCommand<0x4E>),
    PrepareD1(EPDDataB<0x00>),
    PrepareC2(EPDCommand<0x4F>),
    PrepareD2(EPDDataB<0x07>),
    SendC1(EPDCommand<0x24>),
    SendD1(EPDData<BUFSIZE>),
    SendC2(EPDCommand<0x26>),
    SendD2(EPDData<BUFSIZE>),
    Update(UpdateFull),
}

impl Operation for FullDraw {
    type Input<'a> = &'a [u8];
    type Output = bool;
    type StateEnum = FullDrawState;

    fn new() -> Self {
        Self {
            state: FullDrawState::PrepareC1(EPDCommand::<0x4E>::new()),
            timer: 0,
        }
    }

    fn wind(&mut self, state: FullDrawState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, data: Self::Input<'_>) -> bool {
        match self.state{
            FullDrawState::PrepareC1(ref mut a) => {
                if a.advance(()) {
                    self.change(FullDrawState::PrepareD1(EPDDataB::<0x00>::new()));
                }
                false
            },
            FullDrawState::PrepareD1(ref mut a) => {
                if a.advance(()) {
                    self.change(FullDrawState::PrepareC2(EPDCommand::<0x4F>::new()));
                }
                false
            },
            FullDrawState::PrepareC2(ref mut a) => {
                if a.advance(()) {
                    self.change(FullDrawState::PrepareD2(EPDDataB::<0x07>::new()));
                }
                false
            },
            FullDrawState::PrepareD2(ref mut a) => {
                if a.advance(()) {
                    self.change(FullDrawState::SendC1(EPDCommand::<0x24>::new()));
                }
                false
            },
            FullDrawState::SendC1(ref mut a) => {
                if a.advance(()) {
                    self.change(FullDrawState::SendD1(EPDData::<BUFSIZE>::new()));
                }
                false
            },
            FullDrawState::SendD1(ref mut a) => {
                if a.advance(data) {
                    self.change(FullDrawState::SendC2(EPDCommand::<0x26>::new()));
                }
                false
            },
            FullDrawState::SendC2(ref mut a) => {
                if a.advance(()) {
                    self.change(FullDrawState::SendD2(EPDData::<BUFSIZE>::new()));
                }
                false
            },
            FullDrawState::SendD2(ref mut a) => {
                if a.advance(data) {
                    self.change(FullDrawState::Update(UpdateFull::new()));
                }
                false
            },
            FullDrawState::Update(ref mut a) => {
                a.advance(())
            }
        }
    }
}

/// Send command `C` to EPD
pub struct EPDCommand<const C: u8>{
    state: EPDByteState,
    timer: usize,
}

pub enum EPDByteState {
    /// State where command is actually sent
    Init,
    /// Receive something to keep protocol running and close connection
    Aftermath,
}

impl <const C: u8> Operation for EPDCommand<C> {
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = EPDByteState;

    fn new() -> Self {
        Self {
            state: EPDByteState::Init,
            timer: 0,
        }
    }

    fn wind(&mut self, state: EPDByteState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }
    
    fn advance(&mut self, _: ()) -> bool {
        match self.state {
            EPDByteState::Init => {
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
                    self.change(EPDByteState::Aftermath);
                }
                false
            },
            EPDByteState::Aftermath => {
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



/// Send data byte `B` to EPD
pub struct EPDDataB<const B: u8>{
    state: EPDByteState,
    timer: usize,
}

impl <const B: u8> Operation for EPDDataB<B> {
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = EPDByteState;

    fn new() -> Self {
        Self {
            state: EPDByteState::Init,
            timer: 0,
        }
    }

    fn wind(&mut self, state: EPDByteState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, _: ()) -> bool {
        match self.state {
            EPDByteState::Init => {
                in_free(|peripherals| {
                    deselect_display(&mut peripherals.GPIO_S);
                    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start
                    display_select_data(&mut peripherals.GPIO_S);
                });
                if if_in_free(|peripherals|
                    peripherals.USART0_S.status.read().txbl().bit_is_clear()
                ) == Ok(false) {
                    in_free(|peripherals|
                        peripherals
                            .USART0_S
                            .txdata
                            .write(|w_reg| w_reg.txdata().variant(B))
                            );
                    self.change(EPDByteState::Aftermath);
                }
                false
            },
            EPDByteState::Aftermath => {
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

/// Send data byte `B` to EPD
pub struct EPDData<const LEN: usize>{
    state: EPDDataState,
    position: usize,
    timer: usize,
}

pub enum EPDDataState {
    /// State where command is actually sent
    Init,
    /// Receive something to keep protocol running and close connection
    Aftermath,
}

impl <const LEN: usize> Operation for EPDData<LEN> {
    type Input<'a> = &'a [u8];
    type Output = bool;
    type StateEnum = EPDDataState;

    fn new() -> Self {
        Self {
            state: EPDDataState::Init,
            position: 0,
            timer: 0,
        }
    }

    fn wind(&mut self, state: EPDDataState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, data: Self::Input<'_>) -> bool {
        match self.state {
            EPDDataState::Init => {
                in_free(|peripherals| {
                    deselect_display(&mut peripherals.GPIO_S);
                    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start
                    display_select_data(&mut peripherals.GPIO_S);
                });
                if if_in_free(|peripherals|
                    peripherals.USART0_S.status.read().txbl().bit_is_clear()
                ) == Ok(false) {
                    in_free(|peripherals|
                        peripherals
                            .USART0_S
                            .txdata
                            .write(|w_reg| w_reg.txdata().variant(data[self.position]))
                            );
                    if self.position < LEN-1 {
                        self.position += 1;
                    } else {
                        self.change(EPDDataState::Aftermath);
                    }
                }
                false
            },
            EPDDataState::Aftermath => {
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

pub struct UpdateFull {
    state: UpdateFullState,
    timer: usize,
}

pub enum UpdateFullState {
    Init(EPDCommand<0x12>),
    UpdateC1(EPDCommand<0x22>),
    UpdateD1(EPDDataB<0xF7>),
    UpdateC2(EPDCommand<0x20>),
}

impl UpdateFull {
    fn count(&mut self) -> bool {
        if self.timer == 0 {
            false
        } else {
            self.timer -= 1;
            true
        }
    }
}

impl Operation for UpdateFull {
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = UpdateFullState;

    fn new() -> Self {
        Self {
            state: UpdateFullState::Init(EPDCommand::<0x12>::new()),
            timer: 0
        }
    }
    
    fn wind(&mut self, state: UpdateFullState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }


    fn advance(&mut self, _: ()) -> bool {
        if self.count() { return false };
        match self.state {
            UpdateFullState::Init(ref mut a) => {
                if a.advance(()) {
                    self.wind_d(UpdateFullState::UpdateC1(EPDCommand::<0x22>::new()));
                }
                false
            },
            UpdateFullState::UpdateC1(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFullState::UpdateD1(EPDDataB::<0xF7>::new()));
                }
                false
            },
            UpdateFullState::UpdateD1(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFullState::UpdateC2(EPDCommand::<0x20>::new()));
                }
                false
            },
            UpdateFullState::UpdateC2(ref mut a) => {
                if a.advance(()) {
                    true
                } else {
                    false
                }
            },
        }
    }
}

pub struct UpdateFast {
    state: UpdateFastState,
    timer: usize,
}

pub enum UpdateFastState {
    UpdateC1(EPDCommand<0x22>),
    UpdateD1(EPDDataB<0xFF>),
    UpdateC2(EPDCommand<0x20>),
}

impl UpdateFast {
    fn count(&mut self) -> bool {
        if self.timer == 0 {
            false
        } else {
            self.timer -= 1;
            true
        }
    }
}

impl Operation for UpdateFast {
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = UpdateFastState;

    fn new() -> Self {
        Self {
            state: UpdateFastState::UpdateC1(EPDCommand::<0x22>::new()),
            timer: 0
        }
    }
    
    fn wind(&mut self, state: UpdateFastState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, _: ()) -> bool {
        if self.count() { return false };
        match self.state {
            UpdateFastState::UpdateC1(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::UpdateD1(EPDDataB::<0xFF>::new()));
                }
                false
            },
            UpdateFastState::UpdateD1(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::UpdateC2(EPDCommand::<0x20>::new()));
                }
                false
            },
            UpdateFastState::UpdateC2(ref mut a) => {
                if a.advance(()) {
                    true
                } else {
                    false
                }
            },
        }
    }
}



