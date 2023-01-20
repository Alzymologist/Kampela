#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REPMODE` reader - Repeat Mode"]
pub type REPMODE_R = crate::FieldReader<u8, REPMODE_A>;
#[doc = "Repeat Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REPMODE_A {
    #[doc = "0: When started, the LETIMER counts down until it is stopped by software"]
    FREE = 0,
    #[doc = "1: The counter counts REP0 times. When REP0 reaches zero, the counter stops"]
    ONESHOT = 1,
    #[doc = "2: The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero, otherwise the counter stops"]
    BUFFERED = 2,
    #[doc = "3: Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    DOUBLE = 3,
}
impl From<REPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REPMODE_A) -> Self {
        variant as _
    }
}
impl REPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPMODE_A {
        match self.bits {
            0 => REPMODE_A::FREE,
            1 => REPMODE_A::ONESHOT,
            2 => REPMODE_A::BUFFERED,
            3 => REPMODE_A::DOUBLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == REPMODE_A::FREE
    }
    #[doc = "Checks if the value of the field is `ONESHOT`"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == REPMODE_A::ONESHOT
    }
    #[doc = "Checks if the value of the field is `BUFFERED`"]
    #[inline(always)]
    pub fn is_buffered(&self) -> bool {
        *self == REPMODE_A::BUFFERED
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == REPMODE_A::DOUBLE
    }
}
#[doc = "Field `REPMODE` writer - Repeat Mode"]
pub type REPMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, REPMODE_A, 2, O>;
impl<'a, const O: u8> REPMODE_W<'a, O> {
    #[doc = "When started, the LETIMER counts down until it is stopped by software"]
    #[inline(always)]
    pub fn free(self) -> &'a mut W {
        self.variant(REPMODE_A::FREE)
    }
    #[doc = "The counter counts REP0 times. When REP0 reaches zero, the counter stops"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut W {
        self.variant(REPMODE_A::ONESHOT)
    }
    #[doc = "The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero, otherwise the counter stops"]
    #[inline(always)]
    pub fn buffered(self) -> &'a mut W {
        self.variant(REPMODE_A::BUFFERED)
    }
    #[doc = "Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(REPMODE_A::DOUBLE)
    }
}
#[doc = "Field `UFOA0` reader - Underflow Output Action 0"]
pub type UFOA0_R = crate::FieldReader<u8, UFOA0_A>;
#[doc = "Underflow Output Action 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UFOA0_A {
    #[doc = "0: LETIMERn_OUT0 is held at its idle value as defined by OPOL0"]
    NONE = 0,
    #[doc = "1: LETIMERn_OUT0 is toggled on CNT underflow"]
    TOGGLE = 1,
    #[doc = "2: LETIMERn_OUT0 is held active for one LETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0"]
    PULSE = 2,
    #[doc = "3: LETIMERn_OUT0 is set idle on CNT underflow, and active on compare match with COMP1"]
    PWM = 3,
}
impl From<UFOA0_A> for u8 {
    #[inline(always)]
    fn from(variant: UFOA0_A) -> Self {
        variant as _
    }
}
impl UFOA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UFOA0_A {
        match self.bits {
            0 => UFOA0_A::NONE,
            1 => UFOA0_A::TOGGLE,
            2 => UFOA0_A::PULSE,
            3 => UFOA0_A::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UFOA0_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == UFOA0_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == UFOA0_A::PULSE
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == UFOA0_A::PWM
    }
}
#[doc = "Field `UFOA0` writer - Underflow Output Action 0"]
pub type UFOA0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, UFOA0_A, 2, O>;
impl<'a, const O: u8> UFOA0_W<'a, O> {
    #[doc = "LETIMERn_OUT0 is held at its idle value as defined by OPOL0"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(UFOA0_A::NONE)
    }
    #[doc = "LETIMERn_OUT0 is toggled on CNT underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(UFOA0_A::TOGGLE)
    }
    #[doc = "LETIMERn_OUT0 is held active for one LETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(UFOA0_A::PULSE)
    }
    #[doc = "LETIMERn_OUT0 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(UFOA0_A::PWM)
    }
}
#[doc = "Field `UFOA1` reader - Underflow Output Action 1"]
pub type UFOA1_R = crate::FieldReader<u8, UFOA1_A>;
#[doc = "Underflow Output Action 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UFOA1_A {
    #[doc = "0: LETIMERn_OUT1 is held at its idle value as defined by OPOL1"]
    NONE = 0,
    #[doc = "1: LETIMERn_OUT1 is toggled on CNT underflow"]
    TOGGLE = 1,
    #[doc = "2: LETIMERn_OUT1 is held active for one LETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1"]
    PULSE = 2,
    #[doc = "3: LETIMERn_OUT1 is set idle on CNT underflow, and active on compare match with COMP1"]
    PWM = 3,
}
impl From<UFOA1_A> for u8 {
    #[inline(always)]
    fn from(variant: UFOA1_A) -> Self {
        variant as _
    }
}
impl UFOA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UFOA1_A {
        match self.bits {
            0 => UFOA1_A::NONE,
            1 => UFOA1_A::TOGGLE,
            2 => UFOA1_A::PULSE,
            3 => UFOA1_A::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UFOA1_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == UFOA1_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == UFOA1_A::PULSE
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == UFOA1_A::PWM
    }
}
#[doc = "Field `UFOA1` writer - Underflow Output Action 1"]
pub type UFOA1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, UFOA1_A, 2, O>;
impl<'a, const O: u8> UFOA1_W<'a, O> {
    #[doc = "LETIMERn_OUT1 is held at its idle value as defined by OPOL1"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(UFOA1_A::NONE)
    }
    #[doc = "LETIMERn_OUT1 is toggled on CNT underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(UFOA1_A::TOGGLE)
    }
    #[doc = "LETIMERn_OUT1 is held active for one LETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(UFOA1_A::PULSE)
    }
    #[doc = "LETIMERn_OUT1 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(UFOA1_A::PWM)
    }
}
#[doc = "Field `OPOL0` reader - Output 0 Polarity"]
pub type OPOL0_R = crate::BitReader<bool>;
#[doc = "Field `OPOL0` writer - Output 0 Polarity"]
pub type OPOL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OPOL1` reader - Output 1 Polarity"]
pub type OPOL1_R = crate::BitReader<bool>;
#[doc = "Field `OPOL1` writer - Output 1 Polarity"]
pub type OPOL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BUFTOP` reader - Buffered Top"]
pub type BUFTOP_R = crate::BitReader<BUFTOP_A>;
#[doc = "Buffered Top\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFTOP_A {
    #[doc = "0: COMP0 is only written by software"]
    DISABLE = 0,
    #[doc = "1: COMP0 is set to COMP1 when REP0 reaches 0"]
    ENABLE = 1,
}
impl From<BUFTOP_A> for bool {
    #[inline(always)]
    fn from(variant: BUFTOP_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFTOP_A {
        match self.bits {
            false => BUFTOP_A::DISABLE,
            true => BUFTOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BUFTOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BUFTOP_A::ENABLE
    }
}
#[doc = "Field `BUFTOP` writer - Buffered Top"]
pub type BUFTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, BUFTOP_A, O>;
impl<'a, const O: u8> BUFTOP_W<'a, O> {
    #[doc = "COMP0 is only written by software"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BUFTOP_A::DISABLE)
    }
    #[doc = "COMP0 is set to COMP1 when REP0 reaches 0"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BUFTOP_A::ENABLE)
    }
}
#[doc = "Field `CNTTOPEN` reader - Compare Value 0 Is Top Value"]
pub type CNTTOPEN_R = crate::BitReader<CNTTOPEN_A>;
#[doc = "Compare Value 0 Is Top Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTTOPEN_A {
    #[doc = "0: The top value of the LETIMER is 65535 (0xFFFF)"]
    DISABLE = 0,
    #[doc = "1: The top value of the LETIMER is given by COMP0"]
    ENABLE = 1,
}
impl From<CNTTOPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CNTTOPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTTOPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTTOPEN_A {
        match self.bits {
            false => CNTTOPEN_A::DISABLE,
            true => CNTTOPEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CNTTOPEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CNTTOPEN_A::ENABLE
    }
}
#[doc = "Field `CNTTOPEN` writer - Compare Value 0 Is Top Value"]
pub type CNTTOPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CNTTOPEN_A, O>;
impl<'a, const O: u8> CNTTOPEN_W<'a, O> {
    #[doc = "The top value of the LETIMER is 65535 (0xFFFF)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CNTTOPEN_A::DISABLE)
    }
    #[doc = "The top value of the LETIMER is given by COMP0"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CNTTOPEN_A::ENABLE)
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader<DEBUGRUN_A>;
#[doc = "Debug Mode Run Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBUGRUN_A {
    #[doc = "0: LETIMER is frozen in debug mode"]
    DISABLE = 0,
    #[doc = "1: LETIMER is running in debug mode"]
    ENABLE = 1,
}
impl From<DEBUGRUN_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUGRUN_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBUGRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUGRUN_A {
        match self.bits {
            false => DEBUGRUN_A::DISABLE,
            true => DEBUGRUN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEBUGRUN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DEBUGRUN_A::ENABLE
    }
}
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DEBUGRUN_A, O>;
impl<'a, const O: u8> DEBUGRUN_W<'a, O> {
    #[doc = "LETIMER is frozen in debug mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DEBUGRUN_A::DISABLE)
    }
    #[doc = "LETIMER is running in debug mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DEBUGRUN_A::ENABLE)
    }
}
#[doc = "Field `CNTPRESC` reader - Counter prescaler value"]
pub type CNTPRESC_R = crate::FieldReader<u8, CNTPRESC_A>;
#[doc = "Counter prescaler value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTPRESC_A {
    #[doc = "0: CLK_CNT = (LETIMER LF CLK)/1"]
    DIV1 = 0,
    #[doc = "1: CLK_CNT = (LETIMER LF CLK)/2"]
    DIV2 = 1,
    #[doc = "2: CLK_CNT = (LETIMER LF CLK)/4"]
    DIV4 = 2,
    #[doc = "3: CLK_CNT = (LETIMER LF CLK)/8"]
    DIV8 = 3,
    #[doc = "4: CLK_CNT = (LETIMER LF CLK)/16"]
    DIV16 = 4,
    #[doc = "5: CLK_CNT = (LETIMER LF CLK)/32"]
    DIV32 = 5,
    #[doc = "6: CLK_CNT = (LETIMER LF CLK)/64"]
    DIV64 = 6,
    #[doc = "7: CLK_CNT = (LETIMER LF CLK)/128"]
    DIV128 = 7,
    #[doc = "8: CLK_CNT = (LETIMER LF CLK)/256"]
    DIV256 = 8,
}
impl From<CNTPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTPRESC_A) -> Self {
        variant as _
    }
}
impl CNTPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNTPRESC_A> {
        match self.bits {
            0 => Some(CNTPRESC_A::DIV1),
            1 => Some(CNTPRESC_A::DIV2),
            2 => Some(CNTPRESC_A::DIV4),
            3 => Some(CNTPRESC_A::DIV8),
            4 => Some(CNTPRESC_A::DIV16),
            5 => Some(CNTPRESC_A::DIV32),
            6 => Some(CNTPRESC_A::DIV64),
            7 => Some(CNTPRESC_A::DIV128),
            8 => Some(CNTPRESC_A::DIV256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CNTPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CNTPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CNTPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CNTPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CNTPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CNTPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CNTPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CNTPRESC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == CNTPRESC_A::DIV256
    }
}
#[doc = "Field `CNTPRESC` writer - Counter prescaler value"]
pub type CNTPRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, CNTPRESC_A, 4, O>;
impl<'a, const O: u8> CNTPRESC_W<'a, O> {
    #[doc = "CLK_CNT = (LETIMER LF CLK)/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV1)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV2)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV4)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV8)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV16)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV32)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV64)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV128)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV256)
    }
}
impl R {
    #[doc = "Bits 0:1 - Repeat Mode"]
    #[inline(always)]
    pub fn repmode(&self) -> REPMODE_R {
        REPMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Underflow Output Action 0"]
    #[inline(always)]
    pub fn ufoa0(&self) -> UFOA0_R {
        UFOA0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Underflow Output Action 1"]
    #[inline(always)]
    pub fn ufoa1(&self) -> UFOA1_R {
        UFOA1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Output 0 Polarity"]
    #[inline(always)]
    pub fn opol0(&self) -> OPOL0_R {
        OPOL0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output 1 Polarity"]
    #[inline(always)]
    pub fn opol1(&self) -> OPOL1_R {
        OPOL1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Buffered Top"]
    #[inline(always)]
    pub fn buftop(&self) -> BUFTOP_R {
        BUFTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare Value 0 Is Top Value"]
    #[inline(always)]
    pub fn cnttopen(&self) -> CNTTOPEN_R {
        CNTTOPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Counter prescaler value"]
    #[inline(always)]
    pub fn cntpresc(&self) -> CNTPRESC_R {
        CNTPRESC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Repeat Mode"]
    #[inline(always)]
    #[must_use]
    pub fn repmode(&mut self) -> REPMODE_W<0> {
        REPMODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Underflow Output Action 0"]
    #[inline(always)]
    #[must_use]
    pub fn ufoa0(&mut self) -> UFOA0_W<2> {
        UFOA0_W::new(self)
    }
    #[doc = "Bits 4:5 - Underflow Output Action 1"]
    #[inline(always)]
    #[must_use]
    pub fn ufoa1(&mut self) -> UFOA1_W<4> {
        UFOA1_W::new(self)
    }
    #[doc = "Bit 6 - Output 0 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn opol0(&mut self) -> OPOL0_W<6> {
        OPOL0_W::new(self)
    }
    #[doc = "Bit 7 - Output 1 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn opol1(&mut self) -> OPOL1_W<7> {
        OPOL1_W::new(self)
    }
    #[doc = "Bit 8 - Buffered Top"]
    #[inline(always)]
    #[must_use]
    pub fn buftop(&mut self) -> BUFTOP_W<8> {
        BUFTOP_W::new(self)
    }
    #[doc = "Bit 9 - Compare Value 0 Is Top Value"]
    #[inline(always)]
    #[must_use]
    pub fn cnttopen(&mut self) -> CNTTOPEN_W<9> {
        CNTTOPEN_W::new(self)
    }
    #[doc = "Bit 12 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<12> {
        DEBUGRUN_W::new(self)
    }
    #[doc = "Bits 16:19 - Counter prescaler value"]
    #[inline(always)]
    #[must_use]
    pub fn cntpresc(&mut self) -> CNTPRESC_W<16> {
        CNTPRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
