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
#[doc = "Field `RISEA` reader - Timer Rising Input Edge Action"]
pub type RISEA_R = crate::FieldReader<u8, RISEA_A>;
#[doc = "Timer Rising Input Edge Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RISEA_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: Start counter without reload"]
    START = 1,
    #[doc = "2: Stop counter without reload"]
    STOP = 2,
    #[doc = "3: Reload and start counter"]
    RELOADSTART = 3,
}
impl From<RISEA_A> for u8 {
    #[inline(always)]
    fn from(variant: RISEA_A) -> Self {
        variant as _
    }
}
impl RISEA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RISEA_A {
        match self.bits {
            0 => RISEA_A::NONE,
            1 => RISEA_A::START,
            2 => RISEA_A::STOP,
            3 => RISEA_A::RELOADSTART,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RISEA_A::NONE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RISEA_A::START
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RISEA_A::STOP
    }
    #[doc = "Checks if the value of the field is `RELOADSTART`"]
    #[inline(always)]
    pub fn is_reloadstart(&self) -> bool {
        *self == RISEA_A::RELOADSTART
    }
}
#[doc = "Field `RISEA` writer - Timer Rising Input Edge Action"]
pub type RISEA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, RISEA_A, 2, O>;
impl<'a, const O: u8> RISEA_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RISEA_A::NONE)
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(RISEA_A::START)
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(RISEA_A::STOP)
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn reloadstart(self) -> &'a mut W {
        self.variant(RISEA_A::RELOADSTART)
    }
}
#[doc = "Field `FALLA` reader - Timer Falling Input Edge Action"]
pub type FALLA_R = crate::FieldReader<u8, FALLA_A>;
#[doc = "Timer Falling Input Edge Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FALLA_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: Start counter without reload"]
    START = 1,
    #[doc = "2: Stop counter without reload"]
    STOP = 2,
    #[doc = "3: Reload and start counter"]
    RELOADSTART = 3,
}
impl From<FALLA_A> for u8 {
    #[inline(always)]
    fn from(variant: FALLA_A) -> Self {
        variant as _
    }
}
impl FALLA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FALLA_A {
        match self.bits {
            0 => FALLA_A::NONE,
            1 => FALLA_A::START,
            2 => FALLA_A::STOP,
            3 => FALLA_A::RELOADSTART,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == FALLA_A::NONE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == FALLA_A::START
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == FALLA_A::STOP
    }
    #[doc = "Checks if the value of the field is `RELOADSTART`"]
    #[inline(always)]
    pub fn is_reloadstart(&self) -> bool {
        *self == FALLA_A::RELOADSTART
    }
}
#[doc = "Field `FALLA` writer - Timer Falling Input Edge Action"]
pub type FALLA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, FALLA_A, 2, O>;
impl<'a, const O: u8> FALLA_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(FALLA_A::NONE)
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(FALLA_A::START)
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(FALLA_A::STOP)
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn reloadstart(self) -> &'a mut W {
        self.variant(FALLA_A::RELOADSTART)
    }
}
#[doc = "Field `X2CNT` reader - 2x Count Mode"]
pub type X2CNT_R = crate::BitReader<bool>;
#[doc = "Field `X2CNT` writer - 2x Count Mode"]
pub type X2CNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Timer Rising Input Edge Action"]
    #[inline(always)]
    pub fn risea(&self) -> RISEA_R {
        RISEA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Timer Falling Input Edge Action"]
    #[inline(always)]
    pub fn falla(&self) -> FALLA_R {
        FALLA_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 2x Count Mode"]
    #[inline(always)]
    pub fn x2cnt(&self) -> X2CNT_R {
        X2CNT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Rising Input Edge Action"]
    #[inline(always)]
    #[must_use]
    pub fn risea(&mut self) -> RISEA_W<0> {
        RISEA_W::new(self)
    }
    #[doc = "Bits 2:3 - Timer Falling Input Edge Action"]
    #[inline(always)]
    #[must_use]
    pub fn falla(&mut self) -> FALLA_W<2> {
        FALLA_W::new(self)
    }
    #[doc = "Bit 4 - 2x Count Mode"]
    #[inline(always)]
    #[must_use]
    pub fn x2cnt(&mut self) -> X2CNT_W<4> {
        X2CNT_W::new(self)
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
