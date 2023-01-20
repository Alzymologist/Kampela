#[doc = "Register `EXTIPINSELL` reader"]
pub struct R(crate::R<EXTIPINSELL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTIPINSELL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTIPINSELL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTIPINSELL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTIPINSELL` writer"]
pub struct W(crate::W<EXTIPINSELL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTIPINSELL_SPEC>;
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
impl From<crate::W<EXTIPINSELL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTIPINSELL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTIPINSEL0` reader - External Interrupt Pin select"]
pub type EXTIPINSEL0_R = crate::FieldReader<u8, EXTIPINSEL0_A>;
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL0_A {
    #[doc = "0: OFFSET=0"]
    PIN0 = 0,
    #[doc = "1: OFFSET=1"]
    PIN1 = 1,
    #[doc = "2: OFFSET=2"]
    PIN2 = 2,
    #[doc = "3: OFFSET=3"]
    PIN3 = 3,
}
impl From<EXTIPINSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL0_A) -> Self {
        variant as _
    }
}
impl EXTIPINSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL0_A {
        match self.bits {
            0 => EXTIPINSEL0_A::PIN0,
            1 => EXTIPINSEL0_A::PIN1,
            2 => EXTIPINSEL0_A::PIN2,
            3 => EXTIPINSEL0_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL0_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL0_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL0_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL0_A::PIN3
    }
}
#[doc = "Field `EXTIPINSEL0` writer - External Interrupt Pin select"]
pub type EXTIPINSEL0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELL_SPEC, u8, EXTIPINSEL0_A, 2, O>;
impl<'a, const O: u8> EXTIPINSEL0_W<'a, O> {
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL0_A::PIN0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL0_A::PIN1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL0_A::PIN2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL0_A::PIN3)
    }
}
#[doc = "Field `EXTIPINSEL1` reader - External Interrupt Pin select"]
pub type EXTIPINSEL1_R = crate::FieldReader<u8, EXTIPINSEL1_A>;
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL1_A {
    #[doc = "0: OFFSET=0"]
    PIN0 = 0,
    #[doc = "1: OFFSET=1"]
    PIN1 = 1,
    #[doc = "2: OFFSET=2"]
    PIN2 = 2,
    #[doc = "3: OFFSET=3"]
    PIN3 = 3,
}
impl From<EXTIPINSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL1_A) -> Self {
        variant as _
    }
}
impl EXTIPINSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL1_A {
        match self.bits {
            0 => EXTIPINSEL1_A::PIN0,
            1 => EXTIPINSEL1_A::PIN1,
            2 => EXTIPINSEL1_A::PIN2,
            3 => EXTIPINSEL1_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL1_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL1_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL1_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL1_A::PIN3
    }
}
#[doc = "Field `EXTIPINSEL1` writer - External Interrupt Pin select"]
pub type EXTIPINSEL1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELL_SPEC, u8, EXTIPINSEL1_A, 2, O>;
impl<'a, const O: u8> EXTIPINSEL1_W<'a, O> {
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL1_A::PIN0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL1_A::PIN1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL1_A::PIN2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL1_A::PIN3)
    }
}
#[doc = "Field `EXTIPINSEL2` reader - External Interrupt Pin select"]
pub type EXTIPINSEL2_R = crate::FieldReader<u8, EXTIPINSEL2_A>;
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL2_A {
    #[doc = "0: OFFSET=0"]
    PIN0 = 0,
    #[doc = "1: OFFSET=1"]
    PIN1 = 1,
    #[doc = "2: OFFSET=2"]
    PIN2 = 2,
    #[doc = "3: OFFSET=3"]
    PIN3 = 3,
}
impl From<EXTIPINSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL2_A) -> Self {
        variant as _
    }
}
impl EXTIPINSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL2_A {
        match self.bits {
            0 => EXTIPINSEL2_A::PIN0,
            1 => EXTIPINSEL2_A::PIN1,
            2 => EXTIPINSEL2_A::PIN2,
            3 => EXTIPINSEL2_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL2_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL2_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL2_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL2_A::PIN3
    }
}
#[doc = "Field `EXTIPINSEL2` writer - External Interrupt Pin select"]
pub type EXTIPINSEL2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELL_SPEC, u8, EXTIPINSEL2_A, 2, O>;
impl<'a, const O: u8> EXTIPINSEL2_W<'a, O> {
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL2_A::PIN0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL2_A::PIN1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL2_A::PIN2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL2_A::PIN3)
    }
}
#[doc = "Field `EXTIPINSEL3` reader - External Interrupt Pin select"]
pub type EXTIPINSEL3_R = crate::FieldReader<u8, EXTIPINSEL3_A>;
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL3_A {
    #[doc = "0: OFFSET=0"]
    PIN0 = 0,
    #[doc = "1: OFFSET=1"]
    PIN1 = 1,
    #[doc = "2: OFFSET=2"]
    PIN2 = 2,
    #[doc = "3: OFFSET=3"]
    PIN3 = 3,
}
impl From<EXTIPINSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL3_A) -> Self {
        variant as _
    }
}
impl EXTIPINSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL3_A {
        match self.bits {
            0 => EXTIPINSEL3_A::PIN0,
            1 => EXTIPINSEL3_A::PIN1,
            2 => EXTIPINSEL3_A::PIN2,
            3 => EXTIPINSEL3_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL3_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL3_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL3_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL3_A::PIN3
    }
}
#[doc = "Field `EXTIPINSEL3` writer - External Interrupt Pin select"]
pub type EXTIPINSEL3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELL_SPEC, u8, EXTIPINSEL3_A, 2, O>;
impl<'a, const O: u8> EXTIPINSEL3_W<'a, O> {
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL3_A::PIN0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL3_A::PIN1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL3_A::PIN2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL3_A::PIN3)
    }
}
#[doc = "Field `EXTIPINSEL4` reader - External Interrupt Pin select"]
pub type EXTIPINSEL4_R = crate::FieldReader<u8, EXTIPINSEL4_A>;
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL4_A {
    #[doc = "0: OFFSET=0"]
    PIN0 = 0,
    #[doc = "1: OFFSET=1"]
    PIN1 = 1,
    #[doc = "2: OFFSET=2"]
    PIN2 = 2,
    #[doc = "3: OFFSET=3"]
    PIN3 = 3,
}
impl From<EXTIPINSEL4_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL4_A) -> Self {
        variant as _
    }
}
impl EXTIPINSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL4_A {
        match self.bits {
            0 => EXTIPINSEL4_A::PIN0,
            1 => EXTIPINSEL4_A::PIN1,
            2 => EXTIPINSEL4_A::PIN2,
            3 => EXTIPINSEL4_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL4_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL4_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL4_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL4_A::PIN3
    }
}
#[doc = "Field `EXTIPINSEL4` writer - External Interrupt Pin select"]
pub type EXTIPINSEL4_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELL_SPEC, u8, EXTIPINSEL4_A, 2, O>;
impl<'a, const O: u8> EXTIPINSEL4_W<'a, O> {
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL4_A::PIN0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL4_A::PIN1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL4_A::PIN2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL4_A::PIN3)
    }
}
#[doc = "Field `EXTIPINSEL5` reader - External Interrupt Pin select"]
pub type EXTIPINSEL5_R = crate::FieldReader<u8, EXTIPINSEL5_A>;
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL5_A {
    #[doc = "0: OFFSET=0"]
    PIN0 = 0,
    #[doc = "1: OFFSET=1"]
    PIN1 = 1,
    #[doc = "2: OFFSET=2"]
    PIN2 = 2,
    #[doc = "3: OFFSET=3"]
    PIN3 = 3,
}
impl From<EXTIPINSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL5_A) -> Self {
        variant as _
    }
}
impl EXTIPINSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL5_A {
        match self.bits {
            0 => EXTIPINSEL5_A::PIN0,
            1 => EXTIPINSEL5_A::PIN1,
            2 => EXTIPINSEL5_A::PIN2,
            3 => EXTIPINSEL5_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL5_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL5_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL5_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL5_A::PIN3
    }
}
#[doc = "Field `EXTIPINSEL5` writer - External Interrupt Pin select"]
pub type EXTIPINSEL5_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELL_SPEC, u8, EXTIPINSEL5_A, 2, O>;
impl<'a, const O: u8> EXTIPINSEL5_W<'a, O> {
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL5_A::PIN0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL5_A::PIN1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL5_A::PIN2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL5_A::PIN3)
    }
}
#[doc = "Field `EXTIPINSEL6` reader - External Interrupt Pin select"]
pub type EXTIPINSEL6_R = crate::FieldReader<u8, EXTIPINSEL6_A>;
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL6_A {
    #[doc = "0: OFFSET=0"]
    PIN0 = 0,
    #[doc = "1: OFFSET=1"]
    PIN1 = 1,
    #[doc = "2: OFFSET=2"]
    PIN2 = 2,
    #[doc = "3: OFFSET=3"]
    PIN3 = 3,
}
impl From<EXTIPINSEL6_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL6_A) -> Self {
        variant as _
    }
}
impl EXTIPINSEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL6_A {
        match self.bits {
            0 => EXTIPINSEL6_A::PIN0,
            1 => EXTIPINSEL6_A::PIN1,
            2 => EXTIPINSEL6_A::PIN2,
            3 => EXTIPINSEL6_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL6_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL6_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL6_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL6_A::PIN3
    }
}
#[doc = "Field `EXTIPINSEL6` writer - External Interrupt Pin select"]
pub type EXTIPINSEL6_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELL_SPEC, u8, EXTIPINSEL6_A, 2, O>;
impl<'a, const O: u8> EXTIPINSEL6_W<'a, O> {
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL6_A::PIN0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL6_A::PIN1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL6_A::PIN2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL6_A::PIN3)
    }
}
#[doc = "Field `EXTIPINSEL7` reader - External Interrupt Pin select"]
pub type EXTIPINSEL7_R = crate::FieldReader<u8, EXTIPINSEL7_A>;
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPINSEL7_A {
    #[doc = "0: OFFSET=0"]
    PIN0 = 0,
    #[doc = "1: OFFSET=1"]
    PIN1 = 1,
    #[doc = "2: OFFSET=2"]
    PIN2 = 2,
    #[doc = "3: OFFSET=3"]
    PIN3 = 3,
}
impl From<EXTIPINSEL7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL7_A) -> Self {
        variant as _
    }
}
impl EXTIPINSEL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL7_A {
        match self.bits {
            0 => EXTIPINSEL7_A::PIN0,
            1 => EXTIPINSEL7_A::PIN1,
            2 => EXTIPINSEL7_A::PIN2,
            3 => EXTIPINSEL7_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL7_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL7_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL7_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL7_A::PIN3
    }
}
#[doc = "Field `EXTIPINSEL7` writer - External Interrupt Pin select"]
pub type EXTIPINSEL7_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELL_SPEC, u8, EXTIPINSEL7_A, 2, O>;
impl<'a, const O: u8> EXTIPINSEL7_W<'a, O> {
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL7_A::PIN0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL7_A::PIN1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL7_A::PIN2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL7_A::PIN3)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel0(&self) -> EXTIPINSEL0_R {
        EXTIPINSEL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel1(&self) -> EXTIPINSEL1_R {
        EXTIPINSEL1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel2(&self) -> EXTIPINSEL2_R {
        EXTIPINSEL2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel3(&self) -> EXTIPINSEL3_R {
        EXTIPINSEL3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel4(&self) -> EXTIPINSEL4_R {
        EXTIPINSEL4_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel5(&self) -> EXTIPINSEL5_R {
        EXTIPINSEL5_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel6(&self) -> EXTIPINSEL6_R {
        EXTIPINSEL6_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel7(&self) -> EXTIPINSEL7_R {
        EXTIPINSEL7_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt Pin select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel0(&mut self) -> EXTIPINSEL0_W<0> {
        EXTIPINSEL0_W::new(self)
    }
    #[doc = "Bits 4:5 - External Interrupt Pin select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel1(&mut self) -> EXTIPINSEL1_W<4> {
        EXTIPINSEL1_W::new(self)
    }
    #[doc = "Bits 8:9 - External Interrupt Pin select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel2(&mut self) -> EXTIPINSEL2_W<8> {
        EXTIPINSEL2_W::new(self)
    }
    #[doc = "Bits 12:13 - External Interrupt Pin select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel3(&mut self) -> EXTIPINSEL3_W<12> {
        EXTIPINSEL3_W::new(self)
    }
    #[doc = "Bits 16:17 - External Interrupt Pin select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel4(&mut self) -> EXTIPINSEL4_W<16> {
        EXTIPINSEL4_W::new(self)
    }
    #[doc = "Bits 20:21 - External Interrupt Pin select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel5(&mut self) -> EXTIPINSEL5_W<20> {
        EXTIPINSEL5_W::new(self)
    }
    #[doc = "Bits 24:25 - External Interrupt Pin select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel6(&mut self) -> EXTIPINSEL6_W<24> {
        EXTIPINSEL6_W::new(self)
    }
    #[doc = "Bits 28:29 - External Interrupt Pin select"]
    #[inline(always)]
    #[must_use]
    pub fn extipinsel7(&mut self) -> EXTIPINSEL7_W<28> {
        EXTIPINSEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Pin Select Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extipinsell](index.html) module"]
pub struct EXTIPINSELL_SPEC;
impl crate::RegisterSpec for EXTIPINSELL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extipinsell::R](R) reader structure"]
impl crate::Readable for EXTIPINSELL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extipinsell::W](W) writer structure"]
impl crate::Writable for EXTIPINSELL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTIPINSELL to value 0"]
impl crate::Resettable for EXTIPINSELL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
