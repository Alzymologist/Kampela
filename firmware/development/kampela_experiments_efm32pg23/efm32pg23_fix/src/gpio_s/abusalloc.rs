#[doc = "Register `ABUSALLOC` reader"]
pub struct R(crate::R<ABUSALLOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ABUSALLOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ABUSALLOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ABUSALLOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ABUSALLOC` writer"]
pub struct W(crate::W<ABUSALLOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ABUSALLOC_SPEC>;
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
impl From<crate::W<ABUSALLOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ABUSALLOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AEVEN0` reader - A Bus Even 0"]
pub type AEVEN0_R = crate::FieldReader<u8, AEVEN0_A>;
#[doc = "A Bus Even 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AEVEN0_A {
    #[doc = "0: The bus is not allocated"]
    TRISTATE = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    ADC0 = 1,
    #[doc = "2: The bus is allocated to ACMP0"]
    ACMP0 = 2,
    #[doc = "3: The bus is allocated to ACMP1"]
    ACMP1 = 3,
    #[doc = "4: The bus is allocated to VDAC0 CH0"]
    VDAC0CH0 = 4,
}
impl From<AEVEN0_A> for u8 {
    #[inline(always)]
    fn from(variant: AEVEN0_A) -> Self {
        variant as _
    }
}
impl AEVEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AEVEN0_A> {
        match self.bits {
            0 => Some(AEVEN0_A::TRISTATE),
            1 => Some(AEVEN0_A::ADC0),
            2 => Some(AEVEN0_A::ACMP0),
            3 => Some(AEVEN0_A::ACMP1),
            4 => Some(AEVEN0_A::VDAC0CH0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == AEVEN0_A::TRISTATE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == AEVEN0_A::ADC0
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == AEVEN0_A::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == AEVEN0_A::ACMP1
    }
    #[doc = "Checks if the value of the field is `VDAC0CH0`"]
    #[inline(always)]
    pub fn is_vdac0ch0(&self) -> bool {
        *self == AEVEN0_A::VDAC0CH0
    }
}
#[doc = "Field `AEVEN0` writer - A Bus Even 0"]
pub type AEVEN0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ABUSALLOC_SPEC, u8, AEVEN0_A, 4, O>;
impl<'a, const O: u8> AEVEN0_W<'a, O> {
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(AEVEN0_A::TRISTATE)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(AEVEN0_A::ADC0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut W {
        self.variant(AEVEN0_A::ACMP0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut W {
        self.variant(AEVEN0_A::ACMP1)
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn vdac0ch0(self) -> &'a mut W {
        self.variant(AEVEN0_A::VDAC0CH0)
    }
}
#[doc = "Field `AEVEN1` reader - A Bus Even 1"]
pub type AEVEN1_R = crate::FieldReader<u8, AEVEN1_A>;
#[doc = "A Bus Even 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AEVEN1_A {
    #[doc = "0: The bus is not allocated"]
    TRISTATE = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    ADC0 = 1,
    #[doc = "2: The bus is allocated to ACMP0"]
    ACMP0 = 2,
    #[doc = "3: The bus is allocated to ACMP1"]
    ACMP1 = 3,
    #[doc = "4: The bus is allocated to VDAC0 CH1"]
    VDAC0CH1 = 4,
}
impl From<AEVEN1_A> for u8 {
    #[inline(always)]
    fn from(variant: AEVEN1_A) -> Self {
        variant as _
    }
}
impl AEVEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AEVEN1_A> {
        match self.bits {
            0 => Some(AEVEN1_A::TRISTATE),
            1 => Some(AEVEN1_A::ADC0),
            2 => Some(AEVEN1_A::ACMP0),
            3 => Some(AEVEN1_A::ACMP1),
            4 => Some(AEVEN1_A::VDAC0CH1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == AEVEN1_A::TRISTATE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == AEVEN1_A::ADC0
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == AEVEN1_A::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == AEVEN1_A::ACMP1
    }
    #[doc = "Checks if the value of the field is `VDAC0CH1`"]
    #[inline(always)]
    pub fn is_vdac0ch1(&self) -> bool {
        *self == AEVEN1_A::VDAC0CH1
    }
}
#[doc = "Field `AEVEN1` writer - A Bus Even 1"]
pub type AEVEN1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ABUSALLOC_SPEC, u8, AEVEN1_A, 4, O>;
impl<'a, const O: u8> AEVEN1_W<'a, O> {
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(AEVEN1_A::TRISTATE)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(AEVEN1_A::ADC0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut W {
        self.variant(AEVEN1_A::ACMP0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut W {
        self.variant(AEVEN1_A::ACMP1)
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn vdac0ch1(self) -> &'a mut W {
        self.variant(AEVEN1_A::VDAC0CH1)
    }
}
#[doc = "Field `AODD0` reader - A Bus Odd 0"]
pub type AODD0_R = crate::FieldReader<u8, AODD0_A>;
#[doc = "A Bus Odd 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AODD0_A {
    #[doc = "0: The bus is not allocated"]
    TRISTATE = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    ADC0 = 1,
    #[doc = "2: The bus is allocated to ACMP0"]
    ACMP0 = 2,
    #[doc = "3: The bus is allocated to ACMP1"]
    ACMP1 = 3,
    #[doc = "4: The bus is allocated to VDAC0 CH0"]
    VDAC0CH0 = 4,
}
impl From<AODD0_A> for u8 {
    #[inline(always)]
    fn from(variant: AODD0_A) -> Self {
        variant as _
    }
}
impl AODD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AODD0_A> {
        match self.bits {
            0 => Some(AODD0_A::TRISTATE),
            1 => Some(AODD0_A::ADC0),
            2 => Some(AODD0_A::ACMP0),
            3 => Some(AODD0_A::ACMP1),
            4 => Some(AODD0_A::VDAC0CH0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == AODD0_A::TRISTATE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == AODD0_A::ADC0
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == AODD0_A::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == AODD0_A::ACMP1
    }
    #[doc = "Checks if the value of the field is `VDAC0CH0`"]
    #[inline(always)]
    pub fn is_vdac0ch0(&self) -> bool {
        *self == AODD0_A::VDAC0CH0
    }
}
#[doc = "Field `AODD0` writer - A Bus Odd 0"]
pub type AODD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ABUSALLOC_SPEC, u8, AODD0_A, 4, O>;
impl<'a, const O: u8> AODD0_W<'a, O> {
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(AODD0_A::TRISTATE)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(AODD0_A::ADC0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut W {
        self.variant(AODD0_A::ACMP0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut W {
        self.variant(AODD0_A::ACMP1)
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn vdac0ch0(self) -> &'a mut W {
        self.variant(AODD0_A::VDAC0CH0)
    }
}
#[doc = "Field `AODD1` reader - A Bus Odd 1"]
pub type AODD1_R = crate::FieldReader<u8, AODD1_A>;
#[doc = "A Bus Odd 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AODD1_A {
    #[doc = "0: The bus is not allocated"]
    TRISTATE = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    ADC0 = 1,
    #[doc = "2: The bus is allocated to ACMP0"]
    ACMP0 = 2,
    #[doc = "3: The bus is allocated to ACMP1"]
    ACMP1 = 3,
    #[doc = "4: The bus is allocated to VDAC0 CH1"]
    VDAC0CH1 = 4,
}
impl From<AODD1_A> for u8 {
    #[inline(always)]
    fn from(variant: AODD1_A) -> Self {
        variant as _
    }
}
impl AODD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AODD1_A> {
        match self.bits {
            0 => Some(AODD1_A::TRISTATE),
            1 => Some(AODD1_A::ADC0),
            2 => Some(AODD1_A::ACMP0),
            3 => Some(AODD1_A::ACMP1),
            4 => Some(AODD1_A::VDAC0CH1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == AODD1_A::TRISTATE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == AODD1_A::ADC0
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == AODD1_A::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == AODD1_A::ACMP1
    }
    #[doc = "Checks if the value of the field is `VDAC0CH1`"]
    #[inline(always)]
    pub fn is_vdac0ch1(&self) -> bool {
        *self == AODD1_A::VDAC0CH1
    }
}
#[doc = "Field `AODD1` writer - A Bus Odd 1"]
pub type AODD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ABUSALLOC_SPEC, u8, AODD1_A, 4, O>;
impl<'a, const O: u8> AODD1_W<'a, O> {
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(AODD1_A::TRISTATE)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(AODD1_A::ADC0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut W {
        self.variant(AODD1_A::ACMP0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut W {
        self.variant(AODD1_A::ACMP1)
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn vdac0ch1(self) -> &'a mut W {
        self.variant(AODD1_A::VDAC0CH1)
    }
}
impl R {
    #[doc = "Bits 0:3 - A Bus Even 0"]
    #[inline(always)]
    pub fn aeven0(&self) -> AEVEN0_R {
        AEVEN0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - A Bus Even 1"]
    #[inline(always)]
    pub fn aeven1(&self) -> AEVEN1_R {
        AEVEN1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - A Bus Odd 0"]
    #[inline(always)]
    pub fn aodd0(&self) -> AODD0_R {
        AODD0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - A Bus Odd 1"]
    #[inline(always)]
    pub fn aodd1(&self) -> AODD1_R {
        AODD1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - A Bus Even 0"]
    #[inline(always)]
    #[must_use]
    pub fn aeven0(&mut self) -> AEVEN0_W<0> {
        AEVEN0_W::new(self)
    }
    #[doc = "Bits 8:11 - A Bus Even 1"]
    #[inline(always)]
    #[must_use]
    pub fn aeven1(&mut self) -> AEVEN1_W<8> {
        AEVEN1_W::new(self)
    }
    #[doc = "Bits 16:19 - A Bus Odd 0"]
    #[inline(always)]
    #[must_use]
    pub fn aodd0(&mut self) -> AODD0_W<16> {
        AODD0_W::new(self)
    }
    #[doc = "Bits 24:27 - A Bus Odd 1"]
    #[inline(always)]
    #[must_use]
    pub fn aodd1(&mut self) -> AODD1_W<24> {
        AODD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A Bus allocation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abusalloc](index.html) module"]
pub struct ABUSALLOC_SPEC;
impl crate::RegisterSpec for ABUSALLOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [abusalloc::R](R) reader structure"]
impl crate::Readable for ABUSALLOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [abusalloc::W](W) writer structure"]
impl crate::Writable for ABUSALLOC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ABUSALLOC to value 0"]
impl crate::Resettable for ABUSALLOC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
