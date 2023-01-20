#[doc = "Register `PRSMODE` reader"]
pub struct R(crate::R<PRSMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSMODE` writer"]
pub struct W(crate::W<PRSMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSMODE_SPEC>;
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
impl From<crate::W<PRSMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRSSTARTMODE` reader - PRS Start Mode"]
pub type PRSSTARTMODE_R = crate::FieldReader<u8, PRSSTARTMODE_A>;
#[doc = "PRS Start Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTARTMODE_A {
    #[doc = "0: PRS cannot start the LETIMER"]
    NONE = 0,
    #[doc = "1: Rising edge of selected PRS input can start the LETIMER"]
    RISING = 1,
    #[doc = "2: Falling edge of selected PRS input can start the LETIMER"]
    FALLING = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    BOTH = 3,
}
impl From<PRSSTARTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTARTMODE_A) -> Self {
        variant as _
    }
}
impl PRSSTARTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSTARTMODE_A {
        match self.bits {
            0 => PRSSTARTMODE_A::NONE,
            1 => PRSSTARTMODE_A::RISING,
            2 => PRSSTARTMODE_A::FALLING,
            3 => PRSSTARTMODE_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSSTARTMODE_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSSTARTMODE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSSTARTMODE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSSTARTMODE_A::BOTH
    }
}
#[doc = "Field `PRSSTARTMODE` writer - PRS Start Mode"]
pub type PRSSTARTMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRSMODE_SPEC, u8, PRSSTARTMODE_A, 2, O>;
impl<'a, const O: u8> PRSSTARTMODE_W<'a, O> {
    #[doc = "PRS cannot start the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PRSSTARTMODE_A::NONE)
    }
    #[doc = "Rising edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(PRSSTARTMODE_A::RISING)
    }
    #[doc = "Falling edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(PRSSTARTMODE_A::FALLING)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(PRSSTARTMODE_A::BOTH)
    }
}
#[doc = "Field `PRSSTOPMODE` reader - PRS Stop Mode"]
pub type PRSSTOPMODE_R = crate::FieldReader<u8, PRSSTOPMODE_A>;
#[doc = "PRS Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTOPMODE_A {
    #[doc = "0: PRS cannot stop the LETIMER"]
    NONE = 0,
    #[doc = "1: Rising edge of selected PRS input can stop the LETIMER"]
    RISING = 1,
    #[doc = "2: Falling edge of selected PRS input can stop the LETIMER"]
    FALLING = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    BOTH = 3,
}
impl From<PRSSTOPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTOPMODE_A) -> Self {
        variant as _
    }
}
impl PRSSTOPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSTOPMODE_A {
        match self.bits {
            0 => PRSSTOPMODE_A::NONE,
            1 => PRSSTOPMODE_A::RISING,
            2 => PRSSTOPMODE_A::FALLING,
            3 => PRSSTOPMODE_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSSTOPMODE_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSSTOPMODE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSSTOPMODE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSSTOPMODE_A::BOTH
    }
}
#[doc = "Field `PRSSTOPMODE` writer - PRS Stop Mode"]
pub type PRSSTOPMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRSMODE_SPEC, u8, PRSSTOPMODE_A, 2, O>;
impl<'a, const O: u8> PRSSTOPMODE_W<'a, O> {
    #[doc = "PRS cannot stop the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PRSSTOPMODE_A::NONE)
    }
    #[doc = "Rising edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(PRSSTOPMODE_A::RISING)
    }
    #[doc = "Falling edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(PRSSTOPMODE_A::FALLING)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(PRSSTOPMODE_A::BOTH)
    }
}
#[doc = "Field `PRSCLEARMODE` reader - PRS Clear Mode"]
pub type PRSCLEARMODE_R = crate::FieldReader<u8, PRSCLEARMODE_A>;
#[doc = "PRS Clear Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSCLEARMODE_A {
    #[doc = "0: PRS cannot clear the LETIMER"]
    NONE = 0,
    #[doc = "1: Rising edge of selected PRS input can clear the LETIMER"]
    RISING = 1,
    #[doc = "2: Falling edge of selected PRS input can clear the LETIMER"]
    FALLING = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    BOTH = 3,
}
impl From<PRSCLEARMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSCLEARMODE_A) -> Self {
        variant as _
    }
}
impl PRSCLEARMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSCLEARMODE_A {
        match self.bits {
            0 => PRSCLEARMODE_A::NONE,
            1 => PRSCLEARMODE_A::RISING,
            2 => PRSCLEARMODE_A::FALLING,
            3 => PRSCLEARMODE_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSCLEARMODE_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSCLEARMODE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSCLEARMODE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSCLEARMODE_A::BOTH
    }
}
#[doc = "Field `PRSCLEARMODE` writer - PRS Clear Mode"]
pub type PRSCLEARMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRSMODE_SPEC, u8, PRSCLEARMODE_A, 2, O>;
impl<'a, const O: u8> PRSCLEARMODE_W<'a, O> {
    #[doc = "PRS cannot clear the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PRSCLEARMODE_A::NONE)
    }
    #[doc = "Rising edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(PRSCLEARMODE_A::RISING)
    }
    #[doc = "Falling edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(PRSCLEARMODE_A::FALLING)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(PRSCLEARMODE_A::BOTH)
    }
}
impl R {
    #[doc = "Bits 18:19 - PRS Start Mode"]
    #[inline(always)]
    pub fn prsstartmode(&self) -> PRSSTARTMODE_R {
        PRSSTARTMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PRS Stop Mode"]
    #[inline(always)]
    pub fn prsstopmode(&self) -> PRSSTOPMODE_R {
        PRSSTOPMODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PRS Clear Mode"]
    #[inline(always)]
    pub fn prsclearmode(&self) -> PRSCLEARMODE_R {
        PRSCLEARMODE_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - PRS Start Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsstartmode(&mut self) -> PRSSTARTMODE_W<18> {
        PRSSTARTMODE_W::new(self)
    }
    #[doc = "Bits 22:23 - PRS Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsstopmode(&mut self) -> PRSSTOPMODE_W<22> {
        PRSSTOPMODE_W::new(self)
    }
    #[doc = "Bits 26:27 - PRS Clear Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsclearmode(&mut self) -> PRSCLEARMODE_W<26> {
        PRSCLEARMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prsmode](index.html) module"]
pub struct PRSMODE_SPEC;
impl crate::RegisterSpec for PRSMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prsmode::R](R) reader structure"]
impl crate::Readable for PRSMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prsmode::W](W) writer structure"]
impl crate::Writable for PRSMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSMODE to value 0"]
impl crate::Resettable for PRSMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
