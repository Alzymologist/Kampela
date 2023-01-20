#[doc = "Register `XTALCFG` reader"]
pub struct R(crate::R<XTALCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTALCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTALCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTALCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTALCFG` writer"]
pub struct W(crate::W<XTALCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTALCFG_SPEC>;
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
impl From<crate::W<XTALCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTALCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COREBIASSTARTUPI` reader - Intermediate Startup Core Bias Current"]
pub type COREBIASSTARTUPI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COREBIASSTARTUPI` writer - Intermediate Startup Core Bias Current"]
pub type COREBIASSTARTUPI_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTALCFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `COREBIASSTARTUP` reader - Startup Core Bias Current"]
pub type COREBIASSTARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COREBIASSTARTUP` writer - Startup Core Bias Current"]
pub type COREBIASSTARTUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTALCFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `CTUNEXISTARTUP` reader - Startup Tuning Capacitance on XI"]
pub type CTUNEXISTARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTUNEXISTARTUP` writer - Startup Tuning Capacitance on XI"]
pub type CTUNEXISTARTUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTALCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `CTUNEXOSTARTUP` reader - Startup Tuning Capacitance on XO"]
pub type CTUNEXOSTARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTUNEXOSTARTUP` writer - Startup Tuning Capacitance on XO"]
pub type CTUNEXOSTARTUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTALCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TIMEOUTSTEADY` reader - Steady State Timeout"]
pub type TIMEOUTSTEADY_R = crate::FieldReader<u8, TIMEOUTSTEADY_A>;
#[doc = "Steady State Timeout\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUTSTEADY_A {
    #[doc = "0: The steady state timeout is set to 16 us minimum. The maximum can be +40%."]
    T4US = 0,
    #[doc = "1: The steady state timeout is set to 41 us minimum. The maximum can be +40%."]
    T16US = 1,
    #[doc = "2: The steady state timeout is set to 83 us minimum. The maximum can be +40%."]
    T41US = 2,
    #[doc = "3: The steady state timeout is set to 125 us minimum. The maximum can be +40%."]
    T83US = 3,
    #[doc = "4: The steady state timeout is set to 166 us minimum. The maximum can be +40%."]
    T125US = 4,
    #[doc = "5: The steady state timeout is set to 208 us minimum. The maximum can be +40%."]
    T166US = 5,
    #[doc = "6: The steady state timeout is set to 250 us minimum. The maximum can be +40%."]
    T208US = 6,
    #[doc = "7: The steady state timeout is set to 333 us minimum. The maximum can be +40%."]
    T250US = 7,
    #[doc = "8: The steady state timeout is set to 416 us minimum. The maximum can be +40%."]
    T333US = 8,
    #[doc = "9: The steady state timeout is set to 500 us minimum. The maximum can be +40%."]
    T416US = 9,
    #[doc = "10: The steady state timeout is set to 666 us minimum. The maximum can be +40%."]
    T500US = 10,
    #[doc = "11: The steady state timeout is set to 833 us minimum. The maximum can be +40%."]
    T666US = 11,
    #[doc = "12: The steady state timeout is set to 1666 us minimum. The maximum can be +40%."]
    T833US = 12,
    #[doc = "13: The steady state timeout is set to 2500 us minimum. The maximum can be +40%."]
    T1666US = 13,
    #[doc = "14: The steady state timeout is set to 4166 us minimum. The maximum can be +40%."]
    T2500US = 14,
    #[doc = "15: The steady state timeout is set to 7500 us minimum. The maximum can be +40%."]
    T4166US = 15,
}
impl From<TIMEOUTSTEADY_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUTSTEADY_A) -> Self {
        variant as _
    }
}
impl TIMEOUTSTEADY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUTSTEADY_A {
        match self.bits {
            0 => TIMEOUTSTEADY_A::T4US,
            1 => TIMEOUTSTEADY_A::T16US,
            2 => TIMEOUTSTEADY_A::T41US,
            3 => TIMEOUTSTEADY_A::T83US,
            4 => TIMEOUTSTEADY_A::T125US,
            5 => TIMEOUTSTEADY_A::T166US,
            6 => TIMEOUTSTEADY_A::T208US,
            7 => TIMEOUTSTEADY_A::T250US,
            8 => TIMEOUTSTEADY_A::T333US,
            9 => TIMEOUTSTEADY_A::T416US,
            10 => TIMEOUTSTEADY_A::T500US,
            11 => TIMEOUTSTEADY_A::T666US,
            12 => TIMEOUTSTEADY_A::T833US,
            13 => TIMEOUTSTEADY_A::T1666US,
            14 => TIMEOUTSTEADY_A::T2500US,
            15 => TIMEOUTSTEADY_A::T4166US,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `T4US`"]
    #[inline(always)]
    pub fn is_t4us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T4US
    }
    #[doc = "Checks if the value of the field is `T16US`"]
    #[inline(always)]
    pub fn is_t16us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T16US
    }
    #[doc = "Checks if the value of the field is `T41US`"]
    #[inline(always)]
    pub fn is_t41us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T41US
    }
    #[doc = "Checks if the value of the field is `T83US`"]
    #[inline(always)]
    pub fn is_t83us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T83US
    }
    #[doc = "Checks if the value of the field is `T125US`"]
    #[inline(always)]
    pub fn is_t125us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T125US
    }
    #[doc = "Checks if the value of the field is `T166US`"]
    #[inline(always)]
    pub fn is_t166us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T166US
    }
    #[doc = "Checks if the value of the field is `T208US`"]
    #[inline(always)]
    pub fn is_t208us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T208US
    }
    #[doc = "Checks if the value of the field is `T250US`"]
    #[inline(always)]
    pub fn is_t250us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T250US
    }
    #[doc = "Checks if the value of the field is `T333US`"]
    #[inline(always)]
    pub fn is_t333us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T333US
    }
    #[doc = "Checks if the value of the field is `T416US`"]
    #[inline(always)]
    pub fn is_t416us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T416US
    }
    #[doc = "Checks if the value of the field is `T500US`"]
    #[inline(always)]
    pub fn is_t500us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T500US
    }
    #[doc = "Checks if the value of the field is `T666US`"]
    #[inline(always)]
    pub fn is_t666us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T666US
    }
    #[doc = "Checks if the value of the field is `T833US`"]
    #[inline(always)]
    pub fn is_t833us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T833US
    }
    #[doc = "Checks if the value of the field is `T1666US`"]
    #[inline(always)]
    pub fn is_t1666us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T1666US
    }
    #[doc = "Checks if the value of the field is `T2500US`"]
    #[inline(always)]
    pub fn is_t2500us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T2500US
    }
    #[doc = "Checks if the value of the field is `T4166US`"]
    #[inline(always)]
    pub fn is_t4166us(&self) -> bool {
        *self == TIMEOUTSTEADY_A::T4166US
    }
}
#[doc = "Field `TIMEOUTSTEADY` writer - Steady State Timeout"]
pub type TIMEOUTSTEADY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, XTALCFG_SPEC, u8, TIMEOUTSTEADY_A, 4, O>;
impl<'a, const O: u8> TIMEOUTSTEADY_W<'a, O> {
    #[doc = "The steady state timeout is set to 16 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t4us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T4US)
    }
    #[doc = "The steady state timeout is set to 41 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t16us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T16US)
    }
    #[doc = "The steady state timeout is set to 83 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t41us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T41US)
    }
    #[doc = "The steady state timeout is set to 125 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t83us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T83US)
    }
    #[doc = "The steady state timeout is set to 166 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t125us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T125US)
    }
    #[doc = "The steady state timeout is set to 208 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t166us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T166US)
    }
    #[doc = "The steady state timeout is set to 250 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t208us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T208US)
    }
    #[doc = "The steady state timeout is set to 333 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t250us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T250US)
    }
    #[doc = "The steady state timeout is set to 416 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t333us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T333US)
    }
    #[doc = "The steady state timeout is set to 500 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t416us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T416US)
    }
    #[doc = "The steady state timeout is set to 666 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t500us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T500US)
    }
    #[doc = "The steady state timeout is set to 833 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t666us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T666US)
    }
    #[doc = "The steady state timeout is set to 1666 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t833us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T833US)
    }
    #[doc = "The steady state timeout is set to 2500 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t1666us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T1666US)
    }
    #[doc = "The steady state timeout is set to 4166 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t2500us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T2500US)
    }
    #[doc = "The steady state timeout is set to 7500 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t4166us(self) -> &'a mut W {
        self.variant(TIMEOUTSTEADY_A::T4166US)
    }
}
#[doc = "Field `TIMEOUTCBLSB` reader - Core Bias LSB Change Timeout"]
pub type TIMEOUTCBLSB_R = crate::FieldReader<u8, TIMEOUTCBLSB_A>;
#[doc = "Core Bias LSB Change Timeout\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUTCBLSB_A {
    #[doc = "0: The core bias LSB change timeout is set to 8 us minimum. The maximum can be +40%."]
    T8US = 0,
    #[doc = "1: The core bias LSB change timeout is set to 20 us minimum. The maximum can be +40%."]
    T20US = 1,
    #[doc = "2: The core bias LSB change timeout is set to 41 us minimum. The maximum can be +40%."]
    T41US = 2,
    #[doc = "3: The core bias LSB change timeout is set to 62 us minimum. The maximum can be +40%."]
    T62US = 3,
    #[doc = "4: The core bias LSB change timeout is set to 83 us minimum. The maximum can be +40%."]
    T83US = 4,
    #[doc = "5: The core bias LSB change timeout is set to 104 us minimum. The maximum can be +40%."]
    T104US = 5,
    #[doc = "6: The core bias LSB change timeout is set to 125 us minimum. The maximum can be +40%."]
    T125US = 6,
    #[doc = "7: The core bias LSB change timeout is set to 166 us minimum. The maximum can be +40%."]
    T166US = 7,
    #[doc = "8: The core bias LSB change timeout is set to 208 us minimum. The maximum can be +40%."]
    T208US = 8,
    #[doc = "9: The core bias LSB change timeout is set to 250 us minimum. The maximum can be +40%."]
    T250US = 9,
    #[doc = "10: The core bias LSB change timeout is set to 333 us minimum. The maximum can be +40%."]
    T333US = 10,
    #[doc = "11: The core bias LSB change timeout is set to 416 us minimum. The maximum can be +40%."]
    T416US = 11,
    #[doc = "12: The core bias LSB change timeout is set to 833 us minimum. The maximum can be +40%."]
    T833US = 12,
    #[doc = "13: The core bias LSB change timeout is set to 1250 us minimum. The maximum can be +40%."]
    T1250US = 13,
    #[doc = "14: The core bias LSB change timeout is set to 2083 us minimum. The maximum can be +40%."]
    T2083US = 14,
    #[doc = "15: The core bias LSB change timeout is set to 3750 us minimum. The maximum can be +40%."]
    T3750US = 15,
}
impl From<TIMEOUTCBLSB_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUTCBLSB_A) -> Self {
        variant as _
    }
}
impl TIMEOUTCBLSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUTCBLSB_A {
        match self.bits {
            0 => TIMEOUTCBLSB_A::T8US,
            1 => TIMEOUTCBLSB_A::T20US,
            2 => TIMEOUTCBLSB_A::T41US,
            3 => TIMEOUTCBLSB_A::T62US,
            4 => TIMEOUTCBLSB_A::T83US,
            5 => TIMEOUTCBLSB_A::T104US,
            6 => TIMEOUTCBLSB_A::T125US,
            7 => TIMEOUTCBLSB_A::T166US,
            8 => TIMEOUTCBLSB_A::T208US,
            9 => TIMEOUTCBLSB_A::T250US,
            10 => TIMEOUTCBLSB_A::T333US,
            11 => TIMEOUTCBLSB_A::T416US,
            12 => TIMEOUTCBLSB_A::T833US,
            13 => TIMEOUTCBLSB_A::T1250US,
            14 => TIMEOUTCBLSB_A::T2083US,
            15 => TIMEOUTCBLSB_A::T3750US,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `T8US`"]
    #[inline(always)]
    pub fn is_t8us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T8US
    }
    #[doc = "Checks if the value of the field is `T20US`"]
    #[inline(always)]
    pub fn is_t20us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T20US
    }
    #[doc = "Checks if the value of the field is `T41US`"]
    #[inline(always)]
    pub fn is_t41us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T41US
    }
    #[doc = "Checks if the value of the field is `T62US`"]
    #[inline(always)]
    pub fn is_t62us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T62US
    }
    #[doc = "Checks if the value of the field is `T83US`"]
    #[inline(always)]
    pub fn is_t83us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T83US
    }
    #[doc = "Checks if the value of the field is `T104US`"]
    #[inline(always)]
    pub fn is_t104us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T104US
    }
    #[doc = "Checks if the value of the field is `T125US`"]
    #[inline(always)]
    pub fn is_t125us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T125US
    }
    #[doc = "Checks if the value of the field is `T166US`"]
    #[inline(always)]
    pub fn is_t166us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T166US
    }
    #[doc = "Checks if the value of the field is `T208US`"]
    #[inline(always)]
    pub fn is_t208us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T208US
    }
    #[doc = "Checks if the value of the field is `T250US`"]
    #[inline(always)]
    pub fn is_t250us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T250US
    }
    #[doc = "Checks if the value of the field is `T333US`"]
    #[inline(always)]
    pub fn is_t333us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T333US
    }
    #[doc = "Checks if the value of the field is `T416US`"]
    #[inline(always)]
    pub fn is_t416us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T416US
    }
    #[doc = "Checks if the value of the field is `T833US`"]
    #[inline(always)]
    pub fn is_t833us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T833US
    }
    #[doc = "Checks if the value of the field is `T1250US`"]
    #[inline(always)]
    pub fn is_t1250us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T1250US
    }
    #[doc = "Checks if the value of the field is `T2083US`"]
    #[inline(always)]
    pub fn is_t2083us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T2083US
    }
    #[doc = "Checks if the value of the field is `T3750US`"]
    #[inline(always)]
    pub fn is_t3750us(&self) -> bool {
        *self == TIMEOUTCBLSB_A::T3750US
    }
}
#[doc = "Field `TIMEOUTCBLSB` writer - Core Bias LSB Change Timeout"]
pub type TIMEOUTCBLSB_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, XTALCFG_SPEC, u8, TIMEOUTCBLSB_A, 4, O>;
impl<'a, const O: u8> TIMEOUTCBLSB_W<'a, O> {
    #[doc = "The core bias LSB change timeout is set to 8 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t8us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T8US)
    }
    #[doc = "The core bias LSB change timeout is set to 20 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t20us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T20US)
    }
    #[doc = "The core bias LSB change timeout is set to 41 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t41us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T41US)
    }
    #[doc = "The core bias LSB change timeout is set to 62 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t62us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T62US)
    }
    #[doc = "The core bias LSB change timeout is set to 83 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t83us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T83US)
    }
    #[doc = "The core bias LSB change timeout is set to 104 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t104us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T104US)
    }
    #[doc = "The core bias LSB change timeout is set to 125 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t125us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T125US)
    }
    #[doc = "The core bias LSB change timeout is set to 166 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t166us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T166US)
    }
    #[doc = "The core bias LSB change timeout is set to 208 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t208us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T208US)
    }
    #[doc = "The core bias LSB change timeout is set to 250 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t250us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T250US)
    }
    #[doc = "The core bias LSB change timeout is set to 333 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t333us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T333US)
    }
    #[doc = "The core bias LSB change timeout is set to 416 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t416us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T416US)
    }
    #[doc = "The core bias LSB change timeout is set to 833 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t833us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T833US)
    }
    #[doc = "The core bias LSB change timeout is set to 1250 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t1250us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T1250US)
    }
    #[doc = "The core bias LSB change timeout is set to 2083 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t2083us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T2083US)
    }
    #[doc = "The core bias LSB change timeout is set to 3750 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t3750us(self) -> &'a mut W {
        self.variant(TIMEOUTCBLSB_A::T3750US)
    }
}
impl R {
    #[doc = "Bits 0:5 - Intermediate Startup Core Bias Current"]
    #[inline(always)]
    pub fn corebiasstartupi(&self) -> COREBIASSTARTUPI_R {
        COREBIASSTARTUPI_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Startup Core Bias Current"]
    #[inline(always)]
    pub fn corebiasstartup(&self) -> COREBIASSTARTUP_R {
        COREBIASSTARTUP_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:15 - Startup Tuning Capacitance on XI"]
    #[inline(always)]
    pub fn ctunexistartup(&self) -> CTUNEXISTARTUP_R {
        CTUNEXISTARTUP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Startup Tuning Capacitance on XO"]
    #[inline(always)]
    pub fn ctunexostartup(&self) -> CTUNEXOSTARTUP_R {
        CTUNEXOSTARTUP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Steady State Timeout"]
    #[inline(always)]
    pub fn timeoutsteady(&self) -> TIMEOUTSTEADY_R {
        TIMEOUTSTEADY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Core Bias LSB Change Timeout"]
    #[inline(always)]
    pub fn timeoutcblsb(&self) -> TIMEOUTCBLSB_R {
        TIMEOUTCBLSB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Intermediate Startup Core Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn corebiasstartupi(&mut self) -> COREBIASSTARTUPI_W<0> {
        COREBIASSTARTUPI_W::new(self)
    }
    #[doc = "Bits 6:11 - Startup Core Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn corebiasstartup(&mut self) -> COREBIASSTARTUP_W<6> {
        COREBIASSTARTUP_W::new(self)
    }
    #[doc = "Bits 12:15 - Startup Tuning Capacitance on XI"]
    #[inline(always)]
    #[must_use]
    pub fn ctunexistartup(&mut self) -> CTUNEXISTARTUP_W<12> {
        CTUNEXISTARTUP_W::new(self)
    }
    #[doc = "Bits 16:19 - Startup Tuning Capacitance on XO"]
    #[inline(always)]
    #[must_use]
    pub fn ctunexostartup(&mut self) -> CTUNEXOSTARTUP_W<16> {
        CTUNEXOSTARTUP_W::new(self)
    }
    #[doc = "Bits 20:23 - Steady State Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutsteady(&mut self) -> TIMEOUTSTEADY_W<20> {
        TIMEOUTSTEADY_W::new(self)
    }
    #[doc = "Bits 24:27 - Core Bias LSB Change Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutcblsb(&mut self) -> TIMEOUTCBLSB_W<24> {
        TIMEOUTCBLSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalcfg](index.html) module"]
pub struct XTALCFG_SPEC;
impl crate::RegisterSpec for XTALCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtalcfg::R](R) reader structure"]
impl crate::Readable for XTALCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtalcfg::W](W) writer structure"]
impl crate::Writable for XTALCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTALCFG to value 0x0bb0_0820"]
impl crate::Resettable for XTALCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0bb0_0820;
}
