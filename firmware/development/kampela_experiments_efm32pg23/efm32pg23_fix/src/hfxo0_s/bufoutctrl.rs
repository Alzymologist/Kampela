#[doc = "Register `BUFOUTCTRL` reader"]
pub struct R(crate::R<BUFOUTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFOUTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFOUTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFOUTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUFOUTCTRL` writer"]
pub struct W(crate::W<BUFOUTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFOUTCTRL_SPEC>;
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
impl From<crate::W<BUFOUTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFOUTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOUTBIASANA` reader - Driver Bias Current"]
pub type XOUTBIASANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOUTBIASANA` writer - Driver Bias Current"]
pub type XOUTBIASANA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BUFOUTCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `XOUTCFANA` reader - Buffer Gain"]
pub type XOUTCFANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOUTCFANA` writer - Buffer Gain"]
pub type XOUTCFANA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUFOUTCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `XOUTGMANA` reader - No Description"]
pub type XOUTGMANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOUTGMANA` writer - No Description"]
pub type XOUTGMANA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUFOUTCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `PEAKDETTHRESANA` reader - Peak Detector Threshold for XOUT"]
pub type PEAKDETTHRESANA_R = crate::FieldReader<u8, PEAKDETTHRESANA_A>;
#[doc = "Peak Detector Threshold for XOUT\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PEAKDETTHRESANA_A {
    #[doc = "0: V105MV"]
    V105MV = 0,
    #[doc = "1: V132MV"]
    V132MV = 1,
    #[doc = "2: V157MV"]
    V157MV = 2,
    #[doc = "3: V184MV"]
    V184MV = 3,
    #[doc = "4: V210MV"]
    V210MV = 4,
    #[doc = "5: V236MV"]
    V236MV = 5,
    #[doc = "6: V262MV"]
    V262MV = 6,
    #[doc = "7: V289MV"]
    V289MV = 7,
    #[doc = "8: V315MV"]
    V315MV = 8,
    #[doc = "9: V341MV"]
    V341MV = 9,
    #[doc = "10: V367MV"]
    V367MV = 10,
    #[doc = "11: V394MV"]
    V394MV = 11,
    #[doc = "12: V420MV"]
    V420MV = 12,
    #[doc = "13: V446MV"]
    V446MV = 13,
    #[doc = "14: V472MV"]
    V472MV = 14,
    #[doc = "15: V499MV"]
    V499MV = 15,
}
impl From<PEAKDETTHRESANA_A> for u8 {
    #[inline(always)]
    fn from(variant: PEAKDETTHRESANA_A) -> Self {
        variant as _
    }
}
impl PEAKDETTHRESANA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEAKDETTHRESANA_A {
        match self.bits {
            0 => PEAKDETTHRESANA_A::V105MV,
            1 => PEAKDETTHRESANA_A::V132MV,
            2 => PEAKDETTHRESANA_A::V157MV,
            3 => PEAKDETTHRESANA_A::V184MV,
            4 => PEAKDETTHRESANA_A::V210MV,
            5 => PEAKDETTHRESANA_A::V236MV,
            6 => PEAKDETTHRESANA_A::V262MV,
            7 => PEAKDETTHRESANA_A::V289MV,
            8 => PEAKDETTHRESANA_A::V315MV,
            9 => PEAKDETTHRESANA_A::V341MV,
            10 => PEAKDETTHRESANA_A::V367MV,
            11 => PEAKDETTHRESANA_A::V394MV,
            12 => PEAKDETTHRESANA_A::V420MV,
            13 => PEAKDETTHRESANA_A::V446MV,
            14 => PEAKDETTHRESANA_A::V472MV,
            15 => PEAKDETTHRESANA_A::V499MV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V105MV`"]
    #[inline(always)]
    pub fn is_v105mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V105MV
    }
    #[doc = "Checks if the value of the field is `V132MV`"]
    #[inline(always)]
    pub fn is_v132mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V132MV
    }
    #[doc = "Checks if the value of the field is `V157MV`"]
    #[inline(always)]
    pub fn is_v157mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V157MV
    }
    #[doc = "Checks if the value of the field is `V184MV`"]
    #[inline(always)]
    pub fn is_v184mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V184MV
    }
    #[doc = "Checks if the value of the field is `V210MV`"]
    #[inline(always)]
    pub fn is_v210mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V210MV
    }
    #[doc = "Checks if the value of the field is `V236MV`"]
    #[inline(always)]
    pub fn is_v236mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V236MV
    }
    #[doc = "Checks if the value of the field is `V262MV`"]
    #[inline(always)]
    pub fn is_v262mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V262MV
    }
    #[doc = "Checks if the value of the field is `V289MV`"]
    #[inline(always)]
    pub fn is_v289mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V289MV
    }
    #[doc = "Checks if the value of the field is `V315MV`"]
    #[inline(always)]
    pub fn is_v315mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V315MV
    }
    #[doc = "Checks if the value of the field is `V341MV`"]
    #[inline(always)]
    pub fn is_v341mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V341MV
    }
    #[doc = "Checks if the value of the field is `V367MV`"]
    #[inline(always)]
    pub fn is_v367mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V367MV
    }
    #[doc = "Checks if the value of the field is `V394MV`"]
    #[inline(always)]
    pub fn is_v394mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V394MV
    }
    #[doc = "Checks if the value of the field is `V420MV`"]
    #[inline(always)]
    pub fn is_v420mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V420MV
    }
    #[doc = "Checks if the value of the field is `V446MV`"]
    #[inline(always)]
    pub fn is_v446mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V446MV
    }
    #[doc = "Checks if the value of the field is `V472MV`"]
    #[inline(always)]
    pub fn is_v472mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V472MV
    }
    #[doc = "Checks if the value of the field is `V499MV`"]
    #[inline(always)]
    pub fn is_v499mv(&self) -> bool {
        *self == PEAKDETTHRESANA_A::V499MV
    }
}
#[doc = "Field `PEAKDETTHRESANA` writer - Peak Detector Threshold for XOUT"]
pub type PEAKDETTHRESANA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BUFOUTCTRL_SPEC, u8, PEAKDETTHRESANA_A, 4, O>;
impl<'a, const O: u8> PEAKDETTHRESANA_W<'a, O> {
    #[doc = "V105MV"]
    #[inline(always)]
    pub fn v105mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V105MV)
    }
    #[doc = "V132MV"]
    #[inline(always)]
    pub fn v132mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V132MV)
    }
    #[doc = "V157MV"]
    #[inline(always)]
    pub fn v157mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V157MV)
    }
    #[doc = "V184MV"]
    #[inline(always)]
    pub fn v184mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V184MV)
    }
    #[doc = "V210MV"]
    #[inline(always)]
    pub fn v210mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V210MV)
    }
    #[doc = "V236MV"]
    #[inline(always)]
    pub fn v236mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V236MV)
    }
    #[doc = "V262MV"]
    #[inline(always)]
    pub fn v262mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V262MV)
    }
    #[doc = "V289MV"]
    #[inline(always)]
    pub fn v289mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V289MV)
    }
    #[doc = "V315MV"]
    #[inline(always)]
    pub fn v315mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V315MV)
    }
    #[doc = "V341MV"]
    #[inline(always)]
    pub fn v341mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V341MV)
    }
    #[doc = "V367MV"]
    #[inline(always)]
    pub fn v367mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V367MV)
    }
    #[doc = "V394MV"]
    #[inline(always)]
    pub fn v394mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V394MV)
    }
    #[doc = "V420MV"]
    #[inline(always)]
    pub fn v420mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V420MV)
    }
    #[doc = "V446MV"]
    #[inline(always)]
    pub fn v446mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V446MV)
    }
    #[doc = "V472MV"]
    #[inline(always)]
    pub fn v472mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V472MV)
    }
    #[doc = "V499MV"]
    #[inline(always)]
    pub fn v499mv(self) -> &'a mut W {
        self.variant(PEAKDETTHRESANA_A::V499MV)
    }
}
#[doc = "Field `TIMEOUTCTUNE` reader - Tuning Cap Change Timeout"]
pub type TIMEOUTCTUNE_R = crate::FieldReader<u8, TIMEOUTCTUNE_A>;
#[doc = "Tuning Cap Change Timeout\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUTCTUNE_A {
    #[doc = "0: The tuning cap change timeout is set to 2 us minimum. The maximum can be +40%."]
    T2US = 0,
    #[doc = "1: The tuning cap change timeout is set to 5 us minimum. The maximum can be +40%."]
    T5US = 1,
    #[doc = "2: The tuning cap change timeout is set to 10 us minimum. The maximum can be +40%."]
    T10US = 2,
    #[doc = "3: The tuning cap change timeout is set to 16 us minimum. The maximum can be +40%."]
    T16US = 3,
    #[doc = "4: The tuning cap change timeout is set to 21 us minimum. The maximum can be +40%."]
    T21US = 4,
    #[doc = "5: The tuning cap change timeout is set to 26 us minimum. The maximum can be +40%."]
    T26US = 5,
    #[doc = "6: The tuning cap change timeout is set to 31 us minimum. The maximum can be +40%."]
    T31US = 6,
    #[doc = "7: The tuning cap change timeout is set to 42 us minimum. The maximum can be +40%."]
    T42US = 7,
    #[doc = "8: The tuning cap change timeout is set to 52 us minimum. The maximum can be +40%."]
    T52US = 8,
    #[doc = "9: The tuning cap change timeout is set to 63 us minimum. The maximum can be +40%."]
    T63US = 9,
    #[doc = "10: The tuning cap change timeout is set to 83 us minimum. The maximum can be +40%."]
    T83US = 10,
    #[doc = "11: The tuning cap change timeout is set to 104 us minimum. The maximum can be +40%."]
    T104US = 11,
    #[doc = "12: The tuning cap change timeout is set to 208 us minimum. The maximum can be +40%."]
    T208US = 12,
    #[doc = "13: The tuning cap change timeout is set to 313 us minimum. The maximum can be +40%."]
    T313US = 13,
    #[doc = "14: The tuning cap change timeout is set to 521 us minimum. The maximum can be +40%."]
    T521US = 14,
    #[doc = "15: The tuning cap change timeout is set to 938 us minimum. The maximum can be +40%."]
    T938US = 15,
}
impl From<TIMEOUTCTUNE_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUTCTUNE_A) -> Self {
        variant as _
    }
}
impl TIMEOUTCTUNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUTCTUNE_A {
        match self.bits {
            0 => TIMEOUTCTUNE_A::T2US,
            1 => TIMEOUTCTUNE_A::T5US,
            2 => TIMEOUTCTUNE_A::T10US,
            3 => TIMEOUTCTUNE_A::T16US,
            4 => TIMEOUTCTUNE_A::T21US,
            5 => TIMEOUTCTUNE_A::T26US,
            6 => TIMEOUTCTUNE_A::T31US,
            7 => TIMEOUTCTUNE_A::T42US,
            8 => TIMEOUTCTUNE_A::T52US,
            9 => TIMEOUTCTUNE_A::T63US,
            10 => TIMEOUTCTUNE_A::T83US,
            11 => TIMEOUTCTUNE_A::T104US,
            12 => TIMEOUTCTUNE_A::T208US,
            13 => TIMEOUTCTUNE_A::T313US,
            14 => TIMEOUTCTUNE_A::T521US,
            15 => TIMEOUTCTUNE_A::T938US,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `T2US`"]
    #[inline(always)]
    pub fn is_t2us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T2US
    }
    #[doc = "Checks if the value of the field is `T5US`"]
    #[inline(always)]
    pub fn is_t5us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T5US
    }
    #[doc = "Checks if the value of the field is `T10US`"]
    #[inline(always)]
    pub fn is_t10us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T10US
    }
    #[doc = "Checks if the value of the field is `T16US`"]
    #[inline(always)]
    pub fn is_t16us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T16US
    }
    #[doc = "Checks if the value of the field is `T21US`"]
    #[inline(always)]
    pub fn is_t21us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T21US
    }
    #[doc = "Checks if the value of the field is `T26US`"]
    #[inline(always)]
    pub fn is_t26us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T26US
    }
    #[doc = "Checks if the value of the field is `T31US`"]
    #[inline(always)]
    pub fn is_t31us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T31US
    }
    #[doc = "Checks if the value of the field is `T42US`"]
    #[inline(always)]
    pub fn is_t42us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T42US
    }
    #[doc = "Checks if the value of the field is `T52US`"]
    #[inline(always)]
    pub fn is_t52us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T52US
    }
    #[doc = "Checks if the value of the field is `T63US`"]
    #[inline(always)]
    pub fn is_t63us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T63US
    }
    #[doc = "Checks if the value of the field is `T83US`"]
    #[inline(always)]
    pub fn is_t83us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T83US
    }
    #[doc = "Checks if the value of the field is `T104US`"]
    #[inline(always)]
    pub fn is_t104us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T104US
    }
    #[doc = "Checks if the value of the field is `T208US`"]
    #[inline(always)]
    pub fn is_t208us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T208US
    }
    #[doc = "Checks if the value of the field is `T313US`"]
    #[inline(always)]
    pub fn is_t313us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T313US
    }
    #[doc = "Checks if the value of the field is `T521US`"]
    #[inline(always)]
    pub fn is_t521us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T521US
    }
    #[doc = "Checks if the value of the field is `T938US`"]
    #[inline(always)]
    pub fn is_t938us(&self) -> bool {
        *self == TIMEOUTCTUNE_A::T938US
    }
}
#[doc = "Field `TIMEOUTCTUNE` writer - Tuning Cap Change Timeout"]
pub type TIMEOUTCTUNE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BUFOUTCTRL_SPEC, u8, TIMEOUTCTUNE_A, 4, O>;
impl<'a, const O: u8> TIMEOUTCTUNE_W<'a, O> {
    #[doc = "The tuning cap change timeout is set to 2 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t2us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T2US)
    }
    #[doc = "The tuning cap change timeout is set to 5 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t5us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T5US)
    }
    #[doc = "The tuning cap change timeout is set to 10 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t10us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T10US)
    }
    #[doc = "The tuning cap change timeout is set to 16 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t16us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T16US)
    }
    #[doc = "The tuning cap change timeout is set to 21 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t21us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T21US)
    }
    #[doc = "The tuning cap change timeout is set to 26 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t26us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T26US)
    }
    #[doc = "The tuning cap change timeout is set to 31 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t31us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T31US)
    }
    #[doc = "The tuning cap change timeout is set to 42 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t42us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T42US)
    }
    #[doc = "The tuning cap change timeout is set to 52 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t52us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T52US)
    }
    #[doc = "The tuning cap change timeout is set to 63 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t63us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T63US)
    }
    #[doc = "The tuning cap change timeout is set to 83 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t83us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T83US)
    }
    #[doc = "The tuning cap change timeout is set to 104 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t104us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T104US)
    }
    #[doc = "The tuning cap change timeout is set to 208 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t208us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T208US)
    }
    #[doc = "The tuning cap change timeout is set to 313 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t313us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T313US)
    }
    #[doc = "The tuning cap change timeout is set to 521 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t521us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T521US)
    }
    #[doc = "The tuning cap change timeout is set to 938 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t938us(self) -> &'a mut W {
        self.variant(TIMEOUTCTUNE_A::T938US)
    }
}
#[doc = "Field `TIMEOUTSTARTUP` reader - Oscillator Startup Timeout"]
pub type TIMEOUTSTARTUP_R = crate::FieldReader<u8, TIMEOUTSTARTUP_A>;
#[doc = "Oscillator Startup Timeout\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUTSTARTUP_A {
    #[doc = "0: The oscillator startup timeout is set to 42 us minimum. The maximum can be +40%."]
    T42US = 0,
    #[doc = "1: The oscillator startup timeout is set to 83 us minimum. The maximum can be +40%."]
    T83US = 1,
    #[doc = "2: The oscillator startup timeout is set to 108 us minimum. The maximum can be +40%."]
    T108US = 2,
    #[doc = "3: The oscillator startup timeout is set to 133 us minimum. The maximum can be +40%."]
    T133US = 3,
    #[doc = "4: The oscillator startup timeout is set to 158 us minimum. The maximum can be +40%."]
    T158US = 4,
    #[doc = "5: The oscillator startup timeout is set to 183 us minimum. The maximum can be +40%."]
    T183US = 5,
    #[doc = "6: The oscillator startup timeout is set to 208 us minimum. The maximum can be +40%."]
    T208US = 6,
    #[doc = "7: The oscillator startup timeout is set to 233 us minimum. The maximum can be +40%."]
    T233US = 7,
    #[doc = "8: The oscillator startup timeout is set to 258 us minimum. The maximum can be +40%."]
    T258US = 8,
    #[doc = "9: The oscillator startup timeout is set to 283 us minimum. The maximum can be +40%."]
    T283US = 9,
    #[doc = "10: The oscillator startup timeout is set to 333 us minimum. The maximum can be +40%."]
    T333US = 10,
    #[doc = "11: The oscillator startup timeout is set to 375 us minimum. The maximum can be +40%."]
    T375US = 11,
    #[doc = "12: The oscillator startup timeout is set to 417 us minimum. The maximum can be +40%."]
    T417US = 12,
    #[doc = "13: The oscillator startup timeout is set to 458 us minimum. The maximum can be +40%."]
    T458US = 13,
    #[doc = "14: The oscillator startup timeout is set to 500 us minimum. The maximum can be +40%."]
    T500US = 14,
    #[doc = "15: The oscillator startup timeout is set to 667 us minimum. The maximum can be +40%."]
    T667US = 15,
}
impl From<TIMEOUTSTARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUTSTARTUP_A) -> Self {
        variant as _
    }
}
impl TIMEOUTSTARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUTSTARTUP_A {
        match self.bits {
            0 => TIMEOUTSTARTUP_A::T42US,
            1 => TIMEOUTSTARTUP_A::T83US,
            2 => TIMEOUTSTARTUP_A::T108US,
            3 => TIMEOUTSTARTUP_A::T133US,
            4 => TIMEOUTSTARTUP_A::T158US,
            5 => TIMEOUTSTARTUP_A::T183US,
            6 => TIMEOUTSTARTUP_A::T208US,
            7 => TIMEOUTSTARTUP_A::T233US,
            8 => TIMEOUTSTARTUP_A::T258US,
            9 => TIMEOUTSTARTUP_A::T283US,
            10 => TIMEOUTSTARTUP_A::T333US,
            11 => TIMEOUTSTARTUP_A::T375US,
            12 => TIMEOUTSTARTUP_A::T417US,
            13 => TIMEOUTSTARTUP_A::T458US,
            14 => TIMEOUTSTARTUP_A::T500US,
            15 => TIMEOUTSTARTUP_A::T667US,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `T42US`"]
    #[inline(always)]
    pub fn is_t42us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T42US
    }
    #[doc = "Checks if the value of the field is `T83US`"]
    #[inline(always)]
    pub fn is_t83us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T83US
    }
    #[doc = "Checks if the value of the field is `T108US`"]
    #[inline(always)]
    pub fn is_t108us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T108US
    }
    #[doc = "Checks if the value of the field is `T133US`"]
    #[inline(always)]
    pub fn is_t133us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T133US
    }
    #[doc = "Checks if the value of the field is `T158US`"]
    #[inline(always)]
    pub fn is_t158us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T158US
    }
    #[doc = "Checks if the value of the field is `T183US`"]
    #[inline(always)]
    pub fn is_t183us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T183US
    }
    #[doc = "Checks if the value of the field is `T208US`"]
    #[inline(always)]
    pub fn is_t208us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T208US
    }
    #[doc = "Checks if the value of the field is `T233US`"]
    #[inline(always)]
    pub fn is_t233us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T233US
    }
    #[doc = "Checks if the value of the field is `T258US`"]
    #[inline(always)]
    pub fn is_t258us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T258US
    }
    #[doc = "Checks if the value of the field is `T283US`"]
    #[inline(always)]
    pub fn is_t283us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T283US
    }
    #[doc = "Checks if the value of the field is `T333US`"]
    #[inline(always)]
    pub fn is_t333us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T333US
    }
    #[doc = "Checks if the value of the field is `T375US`"]
    #[inline(always)]
    pub fn is_t375us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T375US
    }
    #[doc = "Checks if the value of the field is `T417US`"]
    #[inline(always)]
    pub fn is_t417us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T417US
    }
    #[doc = "Checks if the value of the field is `T458US`"]
    #[inline(always)]
    pub fn is_t458us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T458US
    }
    #[doc = "Checks if the value of the field is `T500US`"]
    #[inline(always)]
    pub fn is_t500us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T500US
    }
    #[doc = "Checks if the value of the field is `T667US`"]
    #[inline(always)]
    pub fn is_t667us(&self) -> bool {
        *self == TIMEOUTSTARTUP_A::T667US
    }
}
#[doc = "Field `TIMEOUTSTARTUP` writer - Oscillator Startup Timeout"]
pub type TIMEOUTSTARTUP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BUFOUTCTRL_SPEC, u8, TIMEOUTSTARTUP_A, 4, O>;
impl<'a, const O: u8> TIMEOUTSTARTUP_W<'a, O> {
    #[doc = "The oscillator startup timeout is set to 42 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t42us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T42US)
    }
    #[doc = "The oscillator startup timeout is set to 83 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t83us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T83US)
    }
    #[doc = "The oscillator startup timeout is set to 108 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t108us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T108US)
    }
    #[doc = "The oscillator startup timeout is set to 133 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t133us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T133US)
    }
    #[doc = "The oscillator startup timeout is set to 158 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t158us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T158US)
    }
    #[doc = "The oscillator startup timeout is set to 183 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t183us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T183US)
    }
    #[doc = "The oscillator startup timeout is set to 208 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t208us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T208US)
    }
    #[doc = "The oscillator startup timeout is set to 233 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t233us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T233US)
    }
    #[doc = "The oscillator startup timeout is set to 258 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t258us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T258US)
    }
    #[doc = "The oscillator startup timeout is set to 283 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t283us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T283US)
    }
    #[doc = "The oscillator startup timeout is set to 333 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t333us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T333US)
    }
    #[doc = "The oscillator startup timeout is set to 375 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t375us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T375US)
    }
    #[doc = "The oscillator startup timeout is set to 417 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t417us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T417US)
    }
    #[doc = "The oscillator startup timeout is set to 458 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t458us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T458US)
    }
    #[doc = "The oscillator startup timeout is set to 500 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t500us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T500US)
    }
    #[doc = "The oscillator startup timeout is set to 667 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t667us(self) -> &'a mut W {
        self.variant(TIMEOUTSTARTUP_A::T667US)
    }
}
#[doc = "Field `MINIMUMSTARTUPDELAY` reader - Minimum Startup Delay"]
pub type MINIMUMSTARTUPDELAY_R = crate::BitReader<bool>;
#[doc = "Field `MINIMUMSTARTUPDELAY` writer - Minimum Startup Delay"]
pub type MINIMUMSTARTUPDELAY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BUFOUTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Driver Bias Current"]
    #[inline(always)]
    pub fn xoutbiasana(&self) -> XOUTBIASANA_R {
        XOUTBIASANA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Buffer Gain"]
    #[inline(always)]
    pub fn xoutcfana(&self) -> XOUTCFANA_R {
        XOUTCFANA_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - No Description"]
    #[inline(always)]
    pub fn xoutgmana(&self) -> XOUTGMANA_R {
        XOUTGMANA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Peak Detector Threshold for XOUT"]
    #[inline(always)]
    pub fn peakdetthresana(&self) -> PEAKDETTHRESANA_R {
        PEAKDETTHRESANA_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Tuning Cap Change Timeout"]
    #[inline(always)]
    pub fn timeoutctune(&self) -> TIMEOUTCTUNE_R {
        TIMEOUTCTUNE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Oscillator Startup Timeout"]
    #[inline(always)]
    pub fn timeoutstartup(&self) -> TIMEOUTSTARTUP_R {
        TIMEOUTSTARTUP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Minimum Startup Delay"]
    #[inline(always)]
    pub fn minimumstartupdelay(&self) -> MINIMUMSTARTUPDELAY_R {
        MINIMUMSTARTUPDELAY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Driver Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn xoutbiasana(&mut self) -> XOUTBIASANA_W<0> {
        XOUTBIASANA_W::new(self)
    }
    #[doc = "Bits 4:7 - Buffer Gain"]
    #[inline(always)]
    #[must_use]
    pub fn xoutcfana(&mut self) -> XOUTCFANA_W<4> {
        XOUTCFANA_W::new(self)
    }
    #[doc = "Bits 8:11 - No Description"]
    #[inline(always)]
    #[must_use]
    pub fn xoutgmana(&mut self) -> XOUTGMANA_W<8> {
        XOUTGMANA_W::new(self)
    }
    #[doc = "Bits 12:15 - Peak Detector Threshold for XOUT"]
    #[inline(always)]
    #[must_use]
    pub fn peakdetthresana(&mut self) -> PEAKDETTHRESANA_W<12> {
        PEAKDETTHRESANA_W::new(self)
    }
    #[doc = "Bits 16:19 - Tuning Cap Change Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutctune(&mut self) -> TIMEOUTCTUNE_W<16> {
        TIMEOUTCTUNE_W::new(self)
    }
    #[doc = "Bits 20:23 - Oscillator Startup Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutstartup(&mut self) -> TIMEOUTSTARTUP_W<20> {
        TIMEOUTSTARTUP_W::new(self)
    }
    #[doc = "Bit 31 - Minimum Startup Delay"]
    #[inline(always)]
    #[must_use]
    pub fn minimumstartupdelay(&mut self) -> MINIMUMSTARTUPDELAY_W<31> {
        MINIMUMSTARTUPDELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bufoutctrl](index.html) module"]
pub struct BUFOUTCTRL_SPEC;
impl crate::RegisterSpec for BUFOUTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bufoutctrl::R](R) reader structure"]
impl crate::Readable for BUFOUTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bufoutctrl::W](W) writer structure"]
impl crate::Writable for BUFOUTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUFOUTCTRL to value 0x0064_3c15"]
impl crate::Resettable for BUFOUTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0064_3c15;
}
