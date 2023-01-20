#[doc = "Register `CC0_CTRL` reader"]
pub struct R(crate::R<CC0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC0_CTRL` writer"]
pub struct W(crate::W<CC0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC0_CTRL_SPEC>;
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
impl From<crate::W<CC0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTINV` reader - Output Invert"]
pub type OUTINV_R = crate::BitReader<bool>;
#[doc = "Field `OUTINV` writer - Output Invert"]
pub type OUTINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CC0_CTRL_SPEC, bool, O>;
#[doc = "Field `CMOA` reader - Compare Match Output Action"]
pub type CMOA_R = crate::FieldReader<u8, CMOA_A>;
#[doc = "Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMOA_A {
    #[doc = "0: No action on compare match"]
    NONE = 0,
    #[doc = "1: Toggle output on compare match"]
    TOGGLE = 1,
    #[doc = "2: Clear output on compare match"]
    CLEAR = 2,
    #[doc = "3: Set output on compare match"]
    SET = 3,
}
impl From<CMOA_A> for u8 {
    #[inline(always)]
    fn from(variant: CMOA_A) -> Self {
        variant as _
    }
}
impl CMOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMOA_A {
        match self.bits {
            0 => CMOA_A::NONE,
            1 => CMOA_A::TOGGLE,
            2 => CMOA_A::CLEAR,
            3 => CMOA_A::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMOA_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CMOA_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMOA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMOA_A::SET
    }
}
#[doc = "Field `CMOA` writer - Compare Match Output Action"]
pub type CMOA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CC0_CTRL_SPEC, u8, CMOA_A, 2, O>;
impl<'a, const O: u8> CMOA_W<'a, O> {
    #[doc = "No action on compare match"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CMOA_A::NONE)
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CMOA_A::TOGGLE)
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMOA_A::CLEAR)
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CMOA_A::SET)
    }
}
#[doc = "Field `COFOA` reader - Counter Overflow Output Action"]
pub type COFOA_R = crate::FieldReader<u8, COFOA_A>;
#[doc = "Counter Overflow Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COFOA_A {
    #[doc = "0: No action on counter overflow"]
    NONE = 0,
    #[doc = "1: Toggle output on counter overflow"]
    TOGGLE = 1,
    #[doc = "2: Clear output on counter overflow"]
    CLEAR = 2,
    #[doc = "3: Set output on counter overflow"]
    SET = 3,
}
impl From<COFOA_A> for u8 {
    #[inline(always)]
    fn from(variant: COFOA_A) -> Self {
        variant as _
    }
}
impl COFOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COFOA_A {
        match self.bits {
            0 => COFOA_A::NONE,
            1 => COFOA_A::TOGGLE,
            2 => COFOA_A::CLEAR,
            3 => COFOA_A::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == COFOA_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == COFOA_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == COFOA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == COFOA_A::SET
    }
}
#[doc = "Field `COFOA` writer - Counter Overflow Output Action"]
pub type COFOA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CC0_CTRL_SPEC, u8, COFOA_A, 2, O>;
impl<'a, const O: u8> COFOA_W<'a, O> {
    #[doc = "No action on counter overflow"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(COFOA_A::NONE)
    }
    #[doc = "Toggle output on counter overflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(COFOA_A::TOGGLE)
    }
    #[doc = "Clear output on counter overflow"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COFOA_A::CLEAR)
    }
    #[doc = "Set output on counter overflow"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COFOA_A::SET)
    }
}
#[doc = "Field `CUFOA` reader - Counter Underflow Output Action"]
pub type CUFOA_R = crate::FieldReader<u8, CUFOA_A>;
#[doc = "Counter Underflow Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CUFOA_A {
    #[doc = "0: No action on counter underflow"]
    NONE = 0,
    #[doc = "1: Toggle output on counter underflow"]
    TOGGLE = 1,
    #[doc = "2: Clear output on counter underflow"]
    CLEAR = 2,
    #[doc = "3: Set output on counter underflow"]
    SET = 3,
}
impl From<CUFOA_A> for u8 {
    #[inline(always)]
    fn from(variant: CUFOA_A) -> Self {
        variant as _
    }
}
impl CUFOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CUFOA_A {
        match self.bits {
            0 => CUFOA_A::NONE,
            1 => CUFOA_A::TOGGLE,
            2 => CUFOA_A::CLEAR,
            3 => CUFOA_A::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CUFOA_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CUFOA_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CUFOA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CUFOA_A::SET
    }
}
#[doc = "Field `CUFOA` writer - Counter Underflow Output Action"]
pub type CUFOA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CC0_CTRL_SPEC, u8, CUFOA_A, 2, O>;
impl<'a, const O: u8> CUFOA_W<'a, O> {
    #[doc = "No action on counter underflow"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CUFOA_A::NONE)
    }
    #[doc = "Toggle output on counter underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CUFOA_A::TOGGLE)
    }
    #[doc = "Clear output on counter underflow"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CUFOA_A::CLEAR)
    }
    #[doc = "Set output on counter underflow"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CUFOA_A::SET)
    }
}
#[doc = "Field `ICEDGE` reader - Input Capture Edge Select"]
pub type ICEDGE_R = crate::FieldReader<u8, ICEDGE_A>;
#[doc = "Input Capture Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICEDGE_A {
    #[doc = "0: Rising edges detected"]
    RISING = 0,
    #[doc = "1: Falling edges detected"]
    FALLING = 1,
    #[doc = "2: Both edges detected"]
    BOTH = 2,
    #[doc = "3: No edge detection, signal is left as it is"]
    NONE = 3,
}
impl From<ICEDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: ICEDGE_A) -> Self {
        variant as _
    }
}
impl ICEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEDGE_A {
        match self.bits {
            0 => ICEDGE_A::RISING,
            1 => ICEDGE_A::FALLING,
            2 => ICEDGE_A::BOTH,
            3 => ICEDGE_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ICEDGE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ICEDGE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == ICEDGE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ICEDGE_A::NONE
    }
}
#[doc = "Field `ICEDGE` writer - Input Capture Edge Select"]
pub type ICEDGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CC0_CTRL_SPEC, u8, ICEDGE_A, 2, O>;
impl<'a, const O: u8> ICEDGE_W<'a, O> {
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ICEDGE_A::RISING)
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ICEDGE_A::FALLING)
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(ICEDGE_A::BOTH)
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ICEDGE_A::NONE)
    }
}
#[doc = "Field `ICEVCTRL` reader - Input Capture Event Control"]
pub type ICEVCTRL_R = crate::FieldReader<u8, ICEVCTRL_A>;
#[doc = "Input Capture Event Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICEVCTRL_A {
    #[doc = "0: PRS output pulse and interrupt flag set on every capture"]
    EVERYEDGE = 0,
    #[doc = "1: PRS output pulse and interrupt flag set on every second capture"]
    EVERYSECONDEDGE = 1,
    #[doc = "2: PRS output pulse and interrupt flag set on rising edge only (if ICEDGE = BOTH)"]
    RISING = 2,
    #[doc = "3: PRS output pulse and interrupt flag set on falling edge only (if ICEDGE = BOTH)"]
    FALLING = 3,
}
impl From<ICEVCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: ICEVCTRL_A) -> Self {
        variant as _
    }
}
impl ICEVCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEVCTRL_A {
        match self.bits {
            0 => ICEVCTRL_A::EVERYEDGE,
            1 => ICEVCTRL_A::EVERYSECONDEDGE,
            2 => ICEVCTRL_A::RISING,
            3 => ICEVCTRL_A::FALLING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EVERYEDGE`"]
    #[inline(always)]
    pub fn is_everyedge(&self) -> bool {
        *self == ICEVCTRL_A::EVERYEDGE
    }
    #[doc = "Checks if the value of the field is `EVERYSECONDEDGE`"]
    #[inline(always)]
    pub fn is_everysecondedge(&self) -> bool {
        *self == ICEVCTRL_A::EVERYSECONDEDGE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ICEVCTRL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ICEVCTRL_A::FALLING
    }
}
#[doc = "Field `ICEVCTRL` writer - Input Capture Event Control"]
pub type ICEVCTRL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CC0_CTRL_SPEC, u8, ICEVCTRL_A, 2, O>;
impl<'a, const O: u8> ICEVCTRL_W<'a, O> {
    #[doc = "PRS output pulse and interrupt flag set on every capture"]
    #[inline(always)]
    pub fn everyedge(self) -> &'a mut W {
        self.variant(ICEVCTRL_A::EVERYEDGE)
    }
    #[doc = "PRS output pulse and interrupt flag set on every second capture"]
    #[inline(always)]
    pub fn everysecondedge(self) -> &'a mut W {
        self.variant(ICEVCTRL_A::EVERYSECONDEDGE)
    }
    #[doc = "PRS output pulse and interrupt flag set on rising edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ICEVCTRL_A::RISING)
    }
    #[doc = "PRS output pulse and interrupt flag set on falling edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ICEVCTRL_A::FALLING)
    }
}
impl R {
    #[doc = "Bit 2 - Output Invert"]
    #[inline(always)]
    pub fn outinv(&self) -> OUTINV_R {
        OUTINV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&self) -> CMOA_R {
        CMOA_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Counter Overflow Output Action"]
    #[inline(always)]
    pub fn cofoa(&self) -> COFOA_R {
        COFOA_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Counter Underflow Output Action"]
    #[inline(always)]
    pub fn cufoa(&self) -> CUFOA_R {
        CUFOA_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&self) -> ICEDGE_R {
        ICEDGE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Input Capture Event Control"]
    #[inline(always)]
    pub fn icevctrl(&self) -> ICEVCTRL_R {
        ICEVCTRL_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn outinv(&mut self) -> OUTINV_W<2> {
        OUTINV_W::new(self)
    }
    #[doc = "Bits 8:9 - Compare Match Output Action"]
    #[inline(always)]
    #[must_use]
    pub fn cmoa(&mut self) -> CMOA_W<8> {
        CMOA_W::new(self)
    }
    #[doc = "Bits 10:11 - Counter Overflow Output Action"]
    #[inline(always)]
    #[must_use]
    pub fn cofoa(&mut self) -> COFOA_W<10> {
        COFOA_W::new(self)
    }
    #[doc = "Bits 12:13 - Counter Underflow Output Action"]
    #[inline(always)]
    #[must_use]
    pub fn cufoa(&mut self) -> CUFOA_W<12> {
        CUFOA_W::new(self)
    }
    #[doc = "Bits 24:25 - Input Capture Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn icedge(&mut self) -> ICEDGE_W<24> {
        ICEDGE_W::new(self)
    }
    #[doc = "Bits 26:27 - Input Capture Event Control"]
    #[inline(always)]
    #[must_use]
    pub fn icevctrl(&mut self) -> ICEVCTRL_W<26> {
        ICEVCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc0_ctrl](index.html) module"]
pub struct CC0_CTRL_SPEC;
impl crate::RegisterSpec for CC0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc0_ctrl::R](R) reader structure"]
impl crate::Readable for CC0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc0_ctrl::W](W) writer structure"]
impl crate::Writable for CC0_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC0_CTRL to value 0"]
impl crate::Resettable for CC0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
