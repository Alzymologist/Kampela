#[doc = "Register `DELAY` reader"]
pub struct R(crate::R<DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DELAY` writer"]
pub struct W(crate::W<DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELAY_SPEC>;
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
impl From<crate::W<DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCANDLY` reader - Scan Delay"]
pub type SCANDLY_R = crate::FieldReader<u8, SCANDLY_A>;
#[doc = "Scan Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCANDLY_A {
    #[doc = "0: 2ms Scan Delay"]
    SCANDLY2 = 0,
    #[doc = "1: 4ms Scan Delay"]
    SCANDLY4 = 1,
    #[doc = "2: 6ms Scan Delay"]
    SCANDLY6 = 2,
    #[doc = "3: 8ms Scan Delay"]
    SCANDLY8 = 3,
    #[doc = "4: 10ms Scan Delay"]
    SCANDLY10 = 4,
    #[doc = "5: 12ms Scan Delay"]
    SCANDLY12 = 5,
    #[doc = "6: 14ms Scan Delay"]
    SCANDLY14 = 6,
    #[doc = "7: 16ms Scan Delay"]
    SCANDLY16 = 7,
    #[doc = "8: 18ms Scan Delay"]
    SCANDLY18 = 8,
    #[doc = "9: 20ms Scan Delay"]
    SCANDLY20 = 9,
    #[doc = "10: 22ms Scan Delay"]
    SCANDLY22 = 10,
    #[doc = "11: 24ms Scan Delay"]
    SCANDLY24 = 11,
    #[doc = "12: 26ms Scan Delay"]
    SCANDLY26 = 12,
    #[doc = "13: 28ms Scan Delay"]
    SCANDLY28 = 13,
    #[doc = "14: 30ms Scan Delay"]
    SCANDLY30 = 14,
    #[doc = "15: 32ms Scan Delay"]
    SCANDLY32 = 15,
}
impl From<SCANDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: SCANDLY_A) -> Self {
        variant as _
    }
}
impl SCANDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCANDLY_A {
        match self.bits {
            0 => SCANDLY_A::SCANDLY2,
            1 => SCANDLY_A::SCANDLY4,
            2 => SCANDLY_A::SCANDLY6,
            3 => SCANDLY_A::SCANDLY8,
            4 => SCANDLY_A::SCANDLY10,
            5 => SCANDLY_A::SCANDLY12,
            6 => SCANDLY_A::SCANDLY14,
            7 => SCANDLY_A::SCANDLY16,
            8 => SCANDLY_A::SCANDLY18,
            9 => SCANDLY_A::SCANDLY20,
            10 => SCANDLY_A::SCANDLY22,
            11 => SCANDLY_A::SCANDLY24,
            12 => SCANDLY_A::SCANDLY26,
            13 => SCANDLY_A::SCANDLY28,
            14 => SCANDLY_A::SCANDLY30,
            15 => SCANDLY_A::SCANDLY32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SCANDLY2`"]
    #[inline(always)]
    pub fn is_scandly2(&self) -> bool {
        *self == SCANDLY_A::SCANDLY2
    }
    #[doc = "Checks if the value of the field is `SCANDLY4`"]
    #[inline(always)]
    pub fn is_scandly4(&self) -> bool {
        *self == SCANDLY_A::SCANDLY4
    }
    #[doc = "Checks if the value of the field is `SCANDLY6`"]
    #[inline(always)]
    pub fn is_scandly6(&self) -> bool {
        *self == SCANDLY_A::SCANDLY6
    }
    #[doc = "Checks if the value of the field is `SCANDLY8`"]
    #[inline(always)]
    pub fn is_scandly8(&self) -> bool {
        *self == SCANDLY_A::SCANDLY8
    }
    #[doc = "Checks if the value of the field is `SCANDLY10`"]
    #[inline(always)]
    pub fn is_scandly10(&self) -> bool {
        *self == SCANDLY_A::SCANDLY10
    }
    #[doc = "Checks if the value of the field is `SCANDLY12`"]
    #[inline(always)]
    pub fn is_scandly12(&self) -> bool {
        *self == SCANDLY_A::SCANDLY12
    }
    #[doc = "Checks if the value of the field is `SCANDLY14`"]
    #[inline(always)]
    pub fn is_scandly14(&self) -> bool {
        *self == SCANDLY_A::SCANDLY14
    }
    #[doc = "Checks if the value of the field is `SCANDLY16`"]
    #[inline(always)]
    pub fn is_scandly16(&self) -> bool {
        *self == SCANDLY_A::SCANDLY16
    }
    #[doc = "Checks if the value of the field is `SCANDLY18`"]
    #[inline(always)]
    pub fn is_scandly18(&self) -> bool {
        *self == SCANDLY_A::SCANDLY18
    }
    #[doc = "Checks if the value of the field is `SCANDLY20`"]
    #[inline(always)]
    pub fn is_scandly20(&self) -> bool {
        *self == SCANDLY_A::SCANDLY20
    }
    #[doc = "Checks if the value of the field is `SCANDLY22`"]
    #[inline(always)]
    pub fn is_scandly22(&self) -> bool {
        *self == SCANDLY_A::SCANDLY22
    }
    #[doc = "Checks if the value of the field is `SCANDLY24`"]
    #[inline(always)]
    pub fn is_scandly24(&self) -> bool {
        *self == SCANDLY_A::SCANDLY24
    }
    #[doc = "Checks if the value of the field is `SCANDLY26`"]
    #[inline(always)]
    pub fn is_scandly26(&self) -> bool {
        *self == SCANDLY_A::SCANDLY26
    }
    #[doc = "Checks if the value of the field is `SCANDLY28`"]
    #[inline(always)]
    pub fn is_scandly28(&self) -> bool {
        *self == SCANDLY_A::SCANDLY28
    }
    #[doc = "Checks if the value of the field is `SCANDLY30`"]
    #[inline(always)]
    pub fn is_scandly30(&self) -> bool {
        *self == SCANDLY_A::SCANDLY30
    }
    #[doc = "Checks if the value of the field is `SCANDLY32`"]
    #[inline(always)]
    pub fn is_scandly32(&self) -> bool {
        *self == SCANDLY_A::SCANDLY32
    }
}
#[doc = "Field `SCANDLY` writer - Scan Delay"]
pub type SCANDLY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DELAY_SPEC, u8, SCANDLY_A, 4, O>;
impl<'a, const O: u8> SCANDLY_W<'a, O> {
    #[doc = "2ms Scan Delay"]
    #[inline(always)]
    pub fn scandly2(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY2)
    }
    #[doc = "4ms Scan Delay"]
    #[inline(always)]
    pub fn scandly4(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY4)
    }
    #[doc = "6ms Scan Delay"]
    #[inline(always)]
    pub fn scandly6(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY6)
    }
    #[doc = "8ms Scan Delay"]
    #[inline(always)]
    pub fn scandly8(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY8)
    }
    #[doc = "10ms Scan Delay"]
    #[inline(always)]
    pub fn scandly10(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY10)
    }
    #[doc = "12ms Scan Delay"]
    #[inline(always)]
    pub fn scandly12(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY12)
    }
    #[doc = "14ms Scan Delay"]
    #[inline(always)]
    pub fn scandly14(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY14)
    }
    #[doc = "16ms Scan Delay"]
    #[inline(always)]
    pub fn scandly16(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY16)
    }
    #[doc = "18ms Scan Delay"]
    #[inline(always)]
    pub fn scandly18(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY18)
    }
    #[doc = "20ms Scan Delay"]
    #[inline(always)]
    pub fn scandly20(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY20)
    }
    #[doc = "22ms Scan Delay"]
    #[inline(always)]
    pub fn scandly22(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY22)
    }
    #[doc = "24ms Scan Delay"]
    #[inline(always)]
    pub fn scandly24(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY24)
    }
    #[doc = "26ms Scan Delay"]
    #[inline(always)]
    pub fn scandly26(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY26)
    }
    #[doc = "28ms Scan Delay"]
    #[inline(always)]
    pub fn scandly28(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY28)
    }
    #[doc = "30ms Scan Delay"]
    #[inline(always)]
    pub fn scandly30(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY30)
    }
    #[doc = "32ms Scan Delay"]
    #[inline(always)]
    pub fn scandly32(self) -> &'a mut W {
        self.variant(SCANDLY_A::SCANDLY32)
    }
}
#[doc = "Field `DEBDLY` reader - Debounce Delay"]
pub type DEBDLY_R = crate::FieldReader<u8, DEBDLY_A>;
#[doc = "Debounce Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEBDLY_A {
    #[doc = "0: 2ms Debounce Delay"]
    DEBDLY2 = 0,
    #[doc = "1: 4ms Debounce Delay"]
    DEBDLY4 = 1,
    #[doc = "2: 6ms Debounce Delay"]
    DEBDLY6 = 2,
    #[doc = "3: 8ms Debounce Delay"]
    DEBDLY8 = 3,
    #[doc = "4: 10ms Debounce Delay"]
    DEBDLY10 = 4,
    #[doc = "5: 12ms Debounce Delay"]
    DEBDLY12 = 5,
    #[doc = "6: 14ms Debounce Delay"]
    DEBDLY14 = 6,
    #[doc = "7: 16ms Debounce Delay"]
    DEBDLY16 = 7,
    #[doc = "8: 18ms Debounce Delay"]
    DEBDLY18 = 8,
    #[doc = "9: 20ms Debounce Delay"]
    DEBDLY20 = 9,
    #[doc = "10: 22ms Debounce Delay"]
    DEBDLY22 = 10,
    #[doc = "11: 24ms Debounce Delay"]
    DEBDLY24 = 11,
    #[doc = "12: 26ms Debounce Delay"]
    DEBDLY26 = 12,
    #[doc = "13: 28ms Debounce Delay"]
    DEBDLY28 = 13,
    #[doc = "14: 30ms Debounce Delay"]
    DEBDLY30 = 14,
    #[doc = "15: 32ms Debounce Delay"]
    DEBDLY32 = 15,
}
impl From<DEBDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: DEBDLY_A) -> Self {
        variant as _
    }
}
impl DEBDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBDLY_A {
        match self.bits {
            0 => DEBDLY_A::DEBDLY2,
            1 => DEBDLY_A::DEBDLY4,
            2 => DEBDLY_A::DEBDLY6,
            3 => DEBDLY_A::DEBDLY8,
            4 => DEBDLY_A::DEBDLY10,
            5 => DEBDLY_A::DEBDLY12,
            6 => DEBDLY_A::DEBDLY14,
            7 => DEBDLY_A::DEBDLY16,
            8 => DEBDLY_A::DEBDLY18,
            9 => DEBDLY_A::DEBDLY20,
            10 => DEBDLY_A::DEBDLY22,
            11 => DEBDLY_A::DEBDLY24,
            12 => DEBDLY_A::DEBDLY26,
            13 => DEBDLY_A::DEBDLY28,
            14 => DEBDLY_A::DEBDLY30,
            15 => DEBDLY_A::DEBDLY32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEBDLY2`"]
    #[inline(always)]
    pub fn is_debdly2(&self) -> bool {
        *self == DEBDLY_A::DEBDLY2
    }
    #[doc = "Checks if the value of the field is `DEBDLY4`"]
    #[inline(always)]
    pub fn is_debdly4(&self) -> bool {
        *self == DEBDLY_A::DEBDLY4
    }
    #[doc = "Checks if the value of the field is `DEBDLY6`"]
    #[inline(always)]
    pub fn is_debdly6(&self) -> bool {
        *self == DEBDLY_A::DEBDLY6
    }
    #[doc = "Checks if the value of the field is `DEBDLY8`"]
    #[inline(always)]
    pub fn is_debdly8(&self) -> bool {
        *self == DEBDLY_A::DEBDLY8
    }
    #[doc = "Checks if the value of the field is `DEBDLY10`"]
    #[inline(always)]
    pub fn is_debdly10(&self) -> bool {
        *self == DEBDLY_A::DEBDLY10
    }
    #[doc = "Checks if the value of the field is `DEBDLY12`"]
    #[inline(always)]
    pub fn is_debdly12(&self) -> bool {
        *self == DEBDLY_A::DEBDLY12
    }
    #[doc = "Checks if the value of the field is `DEBDLY14`"]
    #[inline(always)]
    pub fn is_debdly14(&self) -> bool {
        *self == DEBDLY_A::DEBDLY14
    }
    #[doc = "Checks if the value of the field is `DEBDLY16`"]
    #[inline(always)]
    pub fn is_debdly16(&self) -> bool {
        *self == DEBDLY_A::DEBDLY16
    }
    #[doc = "Checks if the value of the field is `DEBDLY18`"]
    #[inline(always)]
    pub fn is_debdly18(&self) -> bool {
        *self == DEBDLY_A::DEBDLY18
    }
    #[doc = "Checks if the value of the field is `DEBDLY20`"]
    #[inline(always)]
    pub fn is_debdly20(&self) -> bool {
        *self == DEBDLY_A::DEBDLY20
    }
    #[doc = "Checks if the value of the field is `DEBDLY22`"]
    #[inline(always)]
    pub fn is_debdly22(&self) -> bool {
        *self == DEBDLY_A::DEBDLY22
    }
    #[doc = "Checks if the value of the field is `DEBDLY24`"]
    #[inline(always)]
    pub fn is_debdly24(&self) -> bool {
        *self == DEBDLY_A::DEBDLY24
    }
    #[doc = "Checks if the value of the field is `DEBDLY26`"]
    #[inline(always)]
    pub fn is_debdly26(&self) -> bool {
        *self == DEBDLY_A::DEBDLY26
    }
    #[doc = "Checks if the value of the field is `DEBDLY28`"]
    #[inline(always)]
    pub fn is_debdly28(&self) -> bool {
        *self == DEBDLY_A::DEBDLY28
    }
    #[doc = "Checks if the value of the field is `DEBDLY30`"]
    #[inline(always)]
    pub fn is_debdly30(&self) -> bool {
        *self == DEBDLY_A::DEBDLY30
    }
    #[doc = "Checks if the value of the field is `DEBDLY32`"]
    #[inline(always)]
    pub fn is_debdly32(&self) -> bool {
        *self == DEBDLY_A::DEBDLY32
    }
}
#[doc = "Field `DEBDLY` writer - Debounce Delay"]
pub type DEBDLY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DELAY_SPEC, u8, DEBDLY_A, 4, O>;
impl<'a, const O: u8> DEBDLY_W<'a, O> {
    #[doc = "2ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly2(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY2)
    }
    #[doc = "4ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly4(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY4)
    }
    #[doc = "6ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly6(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY6)
    }
    #[doc = "8ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly8(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY8)
    }
    #[doc = "10ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly10(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY10)
    }
    #[doc = "12ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly12(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY12)
    }
    #[doc = "14ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly14(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY14)
    }
    #[doc = "16ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly16(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY16)
    }
    #[doc = "18ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly18(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY18)
    }
    #[doc = "20ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly20(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY20)
    }
    #[doc = "22ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly22(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY22)
    }
    #[doc = "24ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly24(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY24)
    }
    #[doc = "26ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly26(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY26)
    }
    #[doc = "28ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly28(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY28)
    }
    #[doc = "30ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly30(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY30)
    }
    #[doc = "32ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly32(self) -> &'a mut W {
        self.variant(DEBDLY_A::DEBDLY32)
    }
}
#[doc = "Field `STABDLY` reader - Row stable Delay"]
pub type STABDLY_R = crate::FieldReader<u8, STABDLY_A>;
#[doc = "Row stable Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STABDLY_A {
    #[doc = "0: 2ms Row Stable Delay"]
    STABDLY2 = 0,
    #[doc = "1: 4ms Row Stable Delay"]
    STABDLY4 = 1,
    #[doc = "2: 6ms Row Stable Delay"]
    STABDLY6 = 2,
    #[doc = "3: 8ms Row Stable Delay"]
    STABDLY8 = 3,
    #[doc = "4: 10ms Row Stable Delay"]
    STABDLY10 = 4,
    #[doc = "5: 12ms Row Stable Delay"]
    STABDLY12 = 5,
    #[doc = "6: 14ms Row Stable Delay"]
    STABDLY14 = 6,
    #[doc = "7: 16ms Row Stable Delay"]
    STABDLY16 = 7,
    #[doc = "8: 18ms Row Stable Delay"]
    STABDLY18 = 8,
    #[doc = "9: 20ms Row Stable Delay"]
    STABDLY20 = 9,
    #[doc = "10: 22ms Row Stable Delay"]
    STABDLY22 = 10,
    #[doc = "11: 24ms Row Stable Delay"]
    STABDLY24 = 11,
    #[doc = "12: 26ms Row Stable Delay"]
    STABDLY26 = 12,
    #[doc = "13: 28ms Row Stable Delay"]
    STABDLY28 = 13,
    #[doc = "14: 30ms Row Stable Delay"]
    STABDLY30 = 14,
    #[doc = "15: 32ms Row Stable Delay"]
    STABDLY32 = 15,
}
impl From<STABDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: STABDLY_A) -> Self {
        variant as _
    }
}
impl STABDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STABDLY_A {
        match self.bits {
            0 => STABDLY_A::STABDLY2,
            1 => STABDLY_A::STABDLY4,
            2 => STABDLY_A::STABDLY6,
            3 => STABDLY_A::STABDLY8,
            4 => STABDLY_A::STABDLY10,
            5 => STABDLY_A::STABDLY12,
            6 => STABDLY_A::STABDLY14,
            7 => STABDLY_A::STABDLY16,
            8 => STABDLY_A::STABDLY18,
            9 => STABDLY_A::STABDLY20,
            10 => STABDLY_A::STABDLY22,
            11 => STABDLY_A::STABDLY24,
            12 => STABDLY_A::STABDLY26,
            13 => STABDLY_A::STABDLY28,
            14 => STABDLY_A::STABDLY30,
            15 => STABDLY_A::STABDLY32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STABDLY2`"]
    #[inline(always)]
    pub fn is_stabdly2(&self) -> bool {
        *self == STABDLY_A::STABDLY2
    }
    #[doc = "Checks if the value of the field is `STABDLY4`"]
    #[inline(always)]
    pub fn is_stabdly4(&self) -> bool {
        *self == STABDLY_A::STABDLY4
    }
    #[doc = "Checks if the value of the field is `STABDLY6`"]
    #[inline(always)]
    pub fn is_stabdly6(&self) -> bool {
        *self == STABDLY_A::STABDLY6
    }
    #[doc = "Checks if the value of the field is `STABDLY8`"]
    #[inline(always)]
    pub fn is_stabdly8(&self) -> bool {
        *self == STABDLY_A::STABDLY8
    }
    #[doc = "Checks if the value of the field is `STABDLY10`"]
    #[inline(always)]
    pub fn is_stabdly10(&self) -> bool {
        *self == STABDLY_A::STABDLY10
    }
    #[doc = "Checks if the value of the field is `STABDLY12`"]
    #[inline(always)]
    pub fn is_stabdly12(&self) -> bool {
        *self == STABDLY_A::STABDLY12
    }
    #[doc = "Checks if the value of the field is `STABDLY14`"]
    #[inline(always)]
    pub fn is_stabdly14(&self) -> bool {
        *self == STABDLY_A::STABDLY14
    }
    #[doc = "Checks if the value of the field is `STABDLY16`"]
    #[inline(always)]
    pub fn is_stabdly16(&self) -> bool {
        *self == STABDLY_A::STABDLY16
    }
    #[doc = "Checks if the value of the field is `STABDLY18`"]
    #[inline(always)]
    pub fn is_stabdly18(&self) -> bool {
        *self == STABDLY_A::STABDLY18
    }
    #[doc = "Checks if the value of the field is `STABDLY20`"]
    #[inline(always)]
    pub fn is_stabdly20(&self) -> bool {
        *self == STABDLY_A::STABDLY20
    }
    #[doc = "Checks if the value of the field is `STABDLY22`"]
    #[inline(always)]
    pub fn is_stabdly22(&self) -> bool {
        *self == STABDLY_A::STABDLY22
    }
    #[doc = "Checks if the value of the field is `STABDLY24`"]
    #[inline(always)]
    pub fn is_stabdly24(&self) -> bool {
        *self == STABDLY_A::STABDLY24
    }
    #[doc = "Checks if the value of the field is `STABDLY26`"]
    #[inline(always)]
    pub fn is_stabdly26(&self) -> bool {
        *self == STABDLY_A::STABDLY26
    }
    #[doc = "Checks if the value of the field is `STABDLY28`"]
    #[inline(always)]
    pub fn is_stabdly28(&self) -> bool {
        *self == STABDLY_A::STABDLY28
    }
    #[doc = "Checks if the value of the field is `STABDLY30`"]
    #[inline(always)]
    pub fn is_stabdly30(&self) -> bool {
        *self == STABDLY_A::STABDLY30
    }
    #[doc = "Checks if the value of the field is `STABDLY32`"]
    #[inline(always)]
    pub fn is_stabdly32(&self) -> bool {
        *self == STABDLY_A::STABDLY32
    }
}
#[doc = "Field `STABDLY` writer - Row stable Delay"]
pub type STABDLY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DELAY_SPEC, u8, STABDLY_A, 4, O>;
impl<'a, const O: u8> STABDLY_W<'a, O> {
    #[doc = "2ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly2(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY2)
    }
    #[doc = "4ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly4(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY4)
    }
    #[doc = "6ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly6(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY6)
    }
    #[doc = "8ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly8(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY8)
    }
    #[doc = "10ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly10(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY10)
    }
    #[doc = "12ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly12(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY12)
    }
    #[doc = "14ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly14(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY14)
    }
    #[doc = "16ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly16(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY16)
    }
    #[doc = "18ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly18(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY18)
    }
    #[doc = "20ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly20(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY20)
    }
    #[doc = "22ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly22(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY22)
    }
    #[doc = "24ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly24(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY24)
    }
    #[doc = "26ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly26(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY26)
    }
    #[doc = "28ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly28(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY28)
    }
    #[doc = "30ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly30(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY30)
    }
    #[doc = "32ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly32(self) -> &'a mut W {
        self.variant(STABDLY_A::STABDLY32)
    }
}
impl R {
    #[doc = "Bits 8:11 - Scan Delay"]
    #[inline(always)]
    pub fn scandly(&self) -> SCANDLY_R {
        SCANDLY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Debounce Delay"]
    #[inline(always)]
    pub fn debdly(&self) -> DEBDLY_R {
        DEBDLY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Row stable Delay"]
    #[inline(always)]
    pub fn stabdly(&self) -> STABDLY_R {
        STABDLY_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Scan Delay"]
    #[inline(always)]
    #[must_use]
    pub fn scandly(&mut self) -> SCANDLY_W<8> {
        SCANDLY_W::new(self)
    }
    #[doc = "Bits 16:19 - Debounce Delay"]
    #[inline(always)]
    #[must_use]
    pub fn debdly(&mut self) -> DEBDLY_W<16> {
        DEBDLY_W::new(self)
    }
    #[doc = "Bits 24:27 - Row stable Delay"]
    #[inline(always)]
    #[must_use]
    pub fn stabdly(&mut self) -> STABDLY_W<24> {
        STABDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delay](index.html) module"]
pub struct DELAY_SPEC;
impl crate::RegisterSpec for DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [delay::R](R) reader structure"]
impl crate::Readable for DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delay::W](W) writer structure"]
impl crate::Writable for DELAY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DELAY to value 0"]
impl crate::Resettable for DELAY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
