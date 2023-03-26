//! Screen for seed phrase recovery

#[cfg(not(feature="std"))]
use alloc::{format, string::String, vec::Vec};

#[cfg(feature="std")]
use std::{format, string::String, vec::Vec};

use embedded_graphics::{
    geometry::AnchorPoint,
    mono_font::{
        ascii::{FONT_10X20, FONT_4X6, FONT_6X10},
        MonoTextStyle,
    },
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    Drawable,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};

use patches::phrase::{entropy_to_phrase, phrase_to_entropy, wordlist_english, Bits11, WordList, WordListElement};

use crate::uistate::{EventResult, UIState, UpdateRequest};
use crate::display_def::*;

const WORD_LENGTH: usize = 8;
const MAX_SEED: usize = 24;
const SHORT_SEED: usize = 12;


const PHRASE_AREA: Rectangle = Rectangle::new(
    Point::new(GAP as i32, GAP as i32),
    Size::new(SCREEN_SIZE_X - 2 * GAP, BUTTON_TOP as u32 - GAP),
);
const WORD_AREA: Rectangle = Rectangle::new(Point::new(103, BUTTON_TOP), Size::new(50, 14));

const KEY_COUNT: usize = 26;
const KEY_BUTTON_DIAMETER: u32 = 28;

lazy_static! {
    static ref KEY_BUTTONS: [KeyButton; KEY_COUNT] = {
        let button_area_width: i32 = (SCREEN_SIZE_X - 2 * CONTROL_BUTTON_WIDTH - 5 * GAP) as i32;
        let h_tick: i32 = button_area_width / 12;
        let h_offset: i32 = (CONTROL_BUTTON_WIDTH + 2 * GAP) as i32;
        let button_area_height: i32 = SCREEN_SIZE_Y as i32 - BUTTON_TOP - GAP as i32;
        let v_tick: i32 = button_area_height / 10;
        let v_offset: i32 = BUTTON_TOP;
        // 6 vertical rails
        let xr1: i32 = h_offset + h_tick;
        let xr2: i32 = h_offset + 3*h_tick;
        let xr3: i32 = h_offset + 5*h_tick;
        let xr4: i32 = h_offset + 7*h_tick + GAP as i32;
        let xr5: i32 = h_offset + 9*h_tick + GAP as i32;
        let xr6: i32 = h_offset + 11*h_tick + GAP as i32;
        // 5 large horizontal rails
        let yrl1: i32 = v_offset + v_tick;
        let yrl2: i32 = v_offset + 3*v_tick;
        let yrl3: i32 = v_offset + 5*v_tick;
        let yrl4: i32 = v_offset + 7*v_tick;
        let yrl5: i32 = v_offset + 9*v_tick;
        // 4 small horizontal rails
        let yrs1: i32 = v_offset + 2*v_tick;
        let yrs2: i32 = v_offset + 4*v_tick;
        let yrs3: i32 = v_offset + 6*v_tick;
        let yrs4: i32 = v_offset + 8*v_tick;
        [
            KeyButton::new('Q', Point::new(xr1, yrs4)),
            KeyButton::new('W', Point::new(xr1, yrs3)),
            KeyButton::new('E', Point::new(xr1, yrs2)),
            KeyButton::new('R', Point::new(xr1, yrs1)),
            KeyButton::new('T', Point::new(xr4, yrs4)),
            KeyButton::new('Y', Point::new(xr4, yrs3)),
            KeyButton::new('U', Point::new(xr4, yrs2)),
            KeyButton::new('I', Point::new(xr4, yrs1)),
            KeyButton::new('O', Point::new(xr5, yrl1)),
            KeyButton::new('P', Point::new(xr6, yrs1)),
            KeyButton::new('A', Point::new(xr2, yrl5)),
            KeyButton::new('S', Point::new(xr2, yrl4)),
            KeyButton::new('D', Point::new(xr2, yrl3)),
            KeyButton::new('F', Point::new(xr2, yrl2)),
            KeyButton::new('G', Point::new(xr2, yrl1)),
            KeyButton::new('H', Point::new(xr5, yrl5)),
            KeyButton::new('J', Point::new(xr5, yrl4)),
            KeyButton::new('K', Point::new(xr5, yrl3)),
            KeyButton::new('L', Point::new(xr5, yrl2)),
            KeyButton::new('Z', Point::new(xr3, yrs4)),
            KeyButton::new('X', Point::new(xr3, yrs3)),
            KeyButton::new('C', Point::new(xr3, yrs2)),
            KeyButton::new('V', Point::new(xr3, yrs1)),
            KeyButton::new('B', Point::new(xr6, yrs4)),
            KeyButton::new('N', Point::new(xr6, yrs3)),
            KeyButton::new('M', Point::new(xr6, yrs2)),
        ]
    };
}

struct KeyButton {
    label: char,
    area: Circle,
}

impl KeyButton {
    pub fn new(label: char, center: Point) -> Self {
        KeyButton {
            label: label,
            area: Circle::with_center(center, KEY_BUTTON_DIAMETER),
        }
    }

    fn handle(&self, point: Point) -> Option<char> {
        if self.area.offset(-5).contains(point) {
            Some(
                self.label
                    .to_lowercase()
                    .next()
                    .expect("button should contain at least 1 letter"),
            )
        } else {
            None
        }
    }

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 2);

        self.area.into_styled(thin_stroke).draw(display)?;

        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        TextBox::with_textbox_style(
            &format!("{}", self.label),
            self.area.bounding_box(),
            character_style,
            textbox_style,
        )
        .draw(display)?;
        Ok(())
    }
}

struct SeedBuffer {
    seed_phrase: Vec<WordListElement>,
    ready: Option<Vec<u8>>,
}

impl SeedBuffer {
    pub fn new() -> Self {
        SeedBuffer {
            seed_phrase: Vec::with_capacity(MAX_SEED),
            ready: None,
        }
    }

    pub fn words_entered(&self) -> usize {
        self.seed_phrase.len()
    }

    pub fn last_word(&self) -> &str {
        match self.seed_phrase.last() {
            Some(a) => a.word(),
            None => "",
        }
    }

    pub fn len(&self) -> usize {
        self.seed_phrase.len()
    }

    pub fn proposed_phrase(&self) -> String {
        self.seed_phrase
            .iter()
            .map(|a| String::from(a.word()))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn remove_last(&mut self) -> bool {
        self.seed_phrase.pop().is_some()
    }

    pub fn submit_word(&mut self, word: WordListElement) {
        self.seed_phrase.push(word);
    }

    pub fn validate(&mut self) -> bool {
        match phrase_to_entropy(
            &self
                .seed_phrase
                .iter()
                .map(|a| String::from(a.word()))
                .collect::<Vec<String>>()
                .join(" "),
        ) {
            Ok(a) => {
                self.ready = Some(a);
                true
            }
            Err(_) => false,
        }
    }
}

/// Key entry state for seed phrase recovery screen
struct Proposal {
    entry: String,
    guess: Vec<WordListElement>,
}

impl Proposal {
    pub fn new() -> Self {
        Self {
            entry: String::with_capacity(WORD_LENGTH),
            guess: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        *self = Proposal::new();
    }

    pub fn add_letter(&mut self, letter: char) -> bool {
        let mut new = self.entry.clone();
        new.push(letter);
        let guess = wordlist_english().get_words_by_prefix(&new);
        if guess.len() > 0 {
            self.entry = new;
            self.guess = guess;
            true
        } else {
            false
        }
    }

    pub fn word(&self) -> String {
        self.entry.clone()
    }

    pub fn remove_letter(&mut self) -> bool {
        self.entry.pop();
        self.guess = if self.proposed_len() == 0 {
            Vec::new()
        } else {
            wordlist_english().get_words_by_prefix(&self.entry)
        };
        true
    }

    pub fn proposed_len(&self) -> usize {
        self.guess.len()
    }

    pub fn forward_button_action(&mut self) -> Option<WordListElement> {
        if self.guess.len() == 1 {
            let out = self.guess.pop();
            self.clear();
            out
        } else {
            let mut out = None;
            for (index, element) in self.guess.iter().enumerate() {
                if element.word() == self.entry {
                    out = Some(self.guess.remove(index));
                    self.clear();
                    break;
                }
            }
            out
        }
    }

    pub fn get_only_word(&mut self) -> WordListElement {
        if let Some(a) = self.guess.pop() {
            a
        } else {
            panic!()
        }
    }
}

/// UI state for seed phrase recovery
pub struct SeedEntryState {
    seed_phrase: SeedBuffer,
    proposal: Proposal,
}

impl SeedEntryState {
    pub fn new() -> Self {
        SeedEntryState {
            seed_phrase: SeedBuffer::new(),
            proposal: Proposal::new(),
        }
    }

    pub fn new_state(&self) -> Option<UIState> {
        if let Some(ref a) = self.seed_phrase.ready {
            Some(UIState::OnboardingBackup(entropy_to_phrase(&a).unwrap()))
        } else { None }
    }

    fn update_entry<D>(&self, fast_display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
        let clear = PrimitiveStyle::with_fill(BinaryColor::Off);
        WORD_AREA.into_styled(clear).draw(fast_display)?;
        TextBox::with_textbox_style(
            &format!("{}", self.proposal.word()),
            WORD_AREA,
            character_style,
            textbox_style,
        )
        .draw(fast_display)?;

        Ok(())
    }

    fn update_proposal<D>(&self, fast_display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        self.update_entry(fast_display)?;
        let character_style = MonoTextStyle::new(&FONT_4X6, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
        let clear = PrimitiveStyle::with_fill(BinaryColor::Off);
        PHRASE_AREA.into_styled(clear).draw(fast_display)?;
        let mut proposal = self.seed_phrase.proposed_phrase();
        if proposal == "" {
            proposal = String::from("please enter seed phrase")
        };
        TextBox::with_textbox_style(&proposal, PHRASE_AREA, character_style, textbox_style)
            .draw(fast_display)?;

        Ok(())
    }

    fn back_button_event<D>(&mut self, fast_display: &mut D) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut out = UpdateRequest::new();
        if self.proposal.entry.len() > 0 {
            self.proposal.remove_letter();
            self.update_entry(fast_display)?;
            out.set_fast();
        } else if self.seed_phrase.len() > 0 {
            self.seed_phrase.remove_last();
            self.update_proposal(fast_display)?;
            out.set_slow();
        };
        Ok(out)
    }

    fn forward_button_event<D>(&mut self, fast_display: &mut D) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut out = UpdateRequest::new();
        if let Some(a) = self.proposal.forward_button_action() {
            self.seed_phrase.submit_word(a);
            self.update_proposal(fast_display)?;
            self.update_entry(fast_display)?;
            out.set_slow();
        } else {
            if self.proposal.proposed_len() == 0 {
                if self.seed_phrase.validate() {
                    out.set_slow();
                }
            }
        };
        Ok(out)
    }

    fn key_button_event<D>(&mut self, key: char, fast_display: &mut D) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut out = UpdateRequest::new();
        if self.proposal.add_letter(key) {
            self.update_entry(fast_display)?;
            out.set_fast();
        }
        Ok(out)
    }

    fn handle_button<D>(&mut self, point: Point, fast_display: &mut D) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut out = UpdateRequest::new();
        if BACK_BUTTON_AREA.contains(point) {
            out.propagate(self.back_button_event(fast_display)?);
        } else if FORWARD_BUTTON_AREA.contains(point) {
            out.propagate(self.forward_button_event(fast_display)?);
        } else {
            for button in KEY_BUTTONS.iter() {
                if let Some(a) = button.handle(point) {
                    out.propagate(self.key_button_event(a, fast_display)?);
                    break;
                }
            }
        }
        Ok(out)
    }



    /// Input event (user touched screen in pin entry mode)
    pub fn handle_event<D>(&mut self, point: Point, fast_display: &mut D) -> Result<EventResult, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let request = self.handle_button(point, fast_display)?;
        let state = self.new_state();
        Ok(EventResult {request, state})
    }

    fn draw_progress<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
        let bounds = Rectangle::new(
            Point::new(0, 0),
            Size::new(SCREEN_SIZE_X, BUTTON_TOP as u32),
        );

        TextBox::with_textbox_style(
            &format!("Words entered: {}", self.seed_phrase.proposed_phrase()),
            PHRASE_AREA,
            character_style,
            textbox_style,
        )
        .draw(display)?;
        //TextBox::with_textbox_style(&format!("Words entered: {}, last word: {}", self.seed_phrase.words_entered(), self.seed_phrase.last_word()), WORD_AREA, character_style, textbox_style)

        Ok(())
    }

    fn draw_back_button<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        BACK_BUTTON_AREA
            .clone()
            .into_styled(thin_stroke)
            .draw(display)?;
        BACK_BUTTON_TRIANGLE
            .into_styled(thin_stroke)
            .draw(display)?;

        Ok(())
    }

    fn draw_forward_button<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        FORWARD_BUTTON_AREA
            .clone()
            .into_styled(thin_stroke)
            .draw(display)?;
        FORWARD_BUTTON_TRIANGLE
            .into_styled(thin_stroke)
            .draw(display)?;
        Ok(())
    }

    fn draw_control_buttons<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        self.draw_back_button(display)?;
        self.draw_forward_button(display)?;
        Ok(())
    }

    fn draw_key_buttons<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        for button in KEY_BUTTONS.iter() {
            button.draw(display)?;
        }
        Ok(())
    }

    fn draw_buttons<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        self.draw_control_buttons(display)?;
        self.draw_key_buttons(display)?;
        Ok(())
    }

    /// Draw seed recovery screen
    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        //self.draw_progress(display)?;
        self.update_proposal(display)?;
        self.draw_buttons(display)?;
        Ok(())
    }
}
