#[doc = "Register `EXTIPSELH` reader"]
pub struct R(crate::R<EXTIPSELH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTIPSELH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTIPSELH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTIPSELH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTIPSELH` writer"]
pub struct W(crate::W<EXTIPSELH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTIPSELH_SPEC>;
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
impl From<crate::W<EXTIPSELH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTIPSELH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTIPSEL0` reader - External Interrupt Port Select"]
pub type EXTIPSEL0_R = crate::FieldReader<u8, EXTIPSEL0_A>;
#[doc = "External Interrupt Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL0_A {
    #[doc = "0: Port A group selected"]
    PORTA = 0,
    #[doc = "1: Port B group selected"]
    PORTB = 1,
    #[doc = "2: Port C group selected"]
    PORTC = 2,
    #[doc = "3: Port D group selected"]
    PORTD = 3,
}
impl From<EXTIPSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL0_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPSEL0_A {
        match self.bits {
            0 => EXTIPSEL0_A::PORTA,
            1 => EXTIPSEL0_A::PORTB,
            2 => EXTIPSEL0_A::PORTC,
            3 => EXTIPSEL0_A::PORTD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL0_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL0_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL0_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL0_A::PORTD
    }
}
#[doc = "Field `EXTIPSEL0` writer - External Interrupt Port Select"]
pub type EXTIPSEL0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EXTIPSELH_SPEC, u8, EXTIPSEL0_A, 2, O>;
impl<'a, const O: u8> EXTIPSEL0_W<'a, O> {
    #[doc = "Port A group selected"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTA)
    }
    #[doc = "Port B group selected"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTB)
    }
    #[doc = "Port C group selected"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTC)
    }
    #[doc = "Port D group selected"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTD)
    }
}
#[doc = "Field `EXTIPSEL1` reader - External Interrupt Port Select"]
pub type EXTIPSEL1_R = crate::FieldReader<u8, EXTIPSEL1_A>;
#[doc = "External Interrupt Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL1_A {
    #[doc = "0: Port A group selected"]
    PORTA = 0,
    #[doc = "1: Port B group selected"]
    PORTB = 1,
    #[doc = "2: Port C group selected"]
    PORTC = 2,
    #[doc = "3: Port D group selected"]
    PORTD = 3,
}
impl From<EXTIPSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL1_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPSEL1_A {
        match self.bits {
            0 => EXTIPSEL1_A::PORTA,
            1 => EXTIPSEL1_A::PORTB,
            2 => EXTIPSEL1_A::PORTC,
            3 => EXTIPSEL1_A::PORTD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL1_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL1_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL1_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL1_A::PORTD
    }
}
#[doc = "Field `EXTIPSEL1` writer - External Interrupt Port Select"]
pub type EXTIPSEL1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EXTIPSELH_SPEC, u8, EXTIPSEL1_A, 2, O>;
impl<'a, const O: u8> EXTIPSEL1_W<'a, O> {
    #[doc = "Port A group selected"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTA)
    }
    #[doc = "Port B group selected"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTB)
    }
    #[doc = "Port C group selected"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTC)
    }
    #[doc = "Port D group selected"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTD)
    }
}
#[doc = "Field `EXTIPSEL2` reader - External Interrupt Port Select"]
pub type EXTIPSEL2_R = crate::FieldReader<u8, EXTIPSEL2_A>;
#[doc = "External Interrupt Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL2_A {
    #[doc = "0: Port A group selected"]
    PORTA = 0,
    #[doc = "1: Port B group selected"]
    PORTB = 1,
    #[doc = "2: Port C group selected"]
    PORTC = 2,
    #[doc = "3: Port D group selected"]
    PORTD = 3,
}
impl From<EXTIPSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL2_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPSEL2_A {
        match self.bits {
            0 => EXTIPSEL2_A::PORTA,
            1 => EXTIPSEL2_A::PORTB,
            2 => EXTIPSEL2_A::PORTC,
            3 => EXTIPSEL2_A::PORTD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL2_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL2_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL2_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL2_A::PORTD
    }
}
#[doc = "Field `EXTIPSEL2` writer - External Interrupt Port Select"]
pub type EXTIPSEL2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EXTIPSELH_SPEC, u8, EXTIPSEL2_A, 2, O>;
impl<'a, const O: u8> EXTIPSEL2_W<'a, O> {
    #[doc = "Port A group selected"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTA)
    }
    #[doc = "Port B group selected"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTB)
    }
    #[doc = "Port C group selected"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTC)
    }
    #[doc = "Port D group selected"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTD)
    }
}
#[doc = "Field `EXTIPSEL3` reader - External Interrupt Port Select"]
pub type EXTIPSEL3_R = crate::FieldReader<u8, EXTIPSEL3_A>;
#[doc = "External Interrupt Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL3_A {
    #[doc = "0: Port A group selected"]
    PORTA = 0,
    #[doc = "1: Port B group selected"]
    PORTB = 1,
    #[doc = "2: Port C group selected"]
    PORTC = 2,
    #[doc = "3: Port D group selected"]
    PORTD = 3,
}
impl From<EXTIPSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL3_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPSEL3_A {
        match self.bits {
            0 => EXTIPSEL3_A::PORTA,
            1 => EXTIPSEL3_A::PORTB,
            2 => EXTIPSEL3_A::PORTC,
            3 => EXTIPSEL3_A::PORTD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL3_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL3_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL3_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL3_A::PORTD
    }
}
#[doc = "Field `EXTIPSEL3` writer - External Interrupt Port Select"]
pub type EXTIPSEL3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EXTIPSELH_SPEC, u8, EXTIPSEL3_A, 2, O>;
impl<'a, const O: u8> EXTIPSEL3_W<'a, O> {
    #[doc = "Port A group selected"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTA)
    }
    #[doc = "Port B group selected"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTB)
    }
    #[doc = "Port C group selected"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTC)
    }
    #[doc = "Port D group selected"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTD)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt Port Select"]
    #[inline(always)]
    pub fn extipsel0(&self) -> EXTIPSEL0_R {
        EXTIPSEL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Interrupt Port Select"]
    #[inline(always)]
    pub fn extipsel1(&self) -> EXTIPSEL1_R {
        EXTIPSEL1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Interrupt Port Select"]
    #[inline(always)]
    pub fn extipsel2(&self) -> EXTIPSEL2_R {
        EXTIPSEL2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Interrupt Port Select"]
    #[inline(always)]
    pub fn extipsel3(&self) -> EXTIPSEL3_R {
        EXTIPSEL3_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel0(&mut self) -> EXTIPSEL0_W<0> {
        EXTIPSEL0_W::new(self)
    }
    #[doc = "Bits 4:5 - External Interrupt Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel1(&mut self) -> EXTIPSEL1_W<4> {
        EXTIPSEL1_W::new(self)
    }
    #[doc = "Bits 8:9 - External Interrupt Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel2(&mut self) -> EXTIPSEL2_W<8> {
        EXTIPSEL2_W::new(self)
    }
    #[doc = "Bits 12:13 - External Interrupt Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel3(&mut self) -> EXTIPSEL3_W<12> {
        EXTIPSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External interrupt Port Select High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extipselh](index.html) module"]
pub struct EXTIPSELH_SPEC;
impl crate::RegisterSpec for EXTIPSELH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extipselh::R](R) reader structure"]
impl crate::Readable for EXTIPSELH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extipselh::W](W) writer structure"]
impl crate::Writable for EXTIPSELH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTIPSELH to value 0"]
impl crate::Resettable for EXTIPSELH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
