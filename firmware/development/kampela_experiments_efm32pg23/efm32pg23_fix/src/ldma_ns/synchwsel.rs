#[doc = "Register `SYNCHWSEL` reader"]
pub struct R(crate::R<SYNCHWSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCHWSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCHWSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCHWSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCHWSEL` writer"]
pub struct W(crate::W<SYNCHWSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCHWSEL_SPEC>;
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
impl From<crate::W<SYNCHWSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCHWSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCSETEDGE` reader - Hardware Sync Trigger Set Edge Select"]
pub type SYNCSETEDGE_R = crate::FieldReader<u8, SYNCSETEDGE_A>;
#[doc = "Hardware Sync Trigger Set Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCSETEDGE_A {
    #[doc = "0: Use rising edge detection"]
    RISE = 0,
    #[doc = "1: Use falling edge detection"]
    FALL = 1,
}
impl From<SYNCSETEDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCSETEDGE_A) -> Self {
        variant as _
    }
}
impl SYNCSETEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCSETEDGE_A> {
        match self.bits {
            0 => Some(SYNCSETEDGE_A::RISE),
            1 => Some(SYNCSETEDGE_A::FALL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SYNCSETEDGE_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SYNCSETEDGE_A::FALL
    }
}
#[doc = "Field `SYNCSETEDGE` writer - Hardware Sync Trigger Set Edge Select"]
pub type SYNCSETEDGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYNCHWSEL_SPEC, u8, SYNCSETEDGE_A, 8, O>;
impl<'a, const O: u8> SYNCSETEDGE_W<'a, O> {
    #[doc = "Use rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SYNCSETEDGE_A::RISE)
    }
    #[doc = "Use falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SYNCSETEDGE_A::FALL)
    }
}
#[doc = "Field `SYNCCLREDGE` reader - Hardware Sync Trigger Clear Edge Select"]
pub type SYNCCLREDGE_R = crate::FieldReader<u8, SYNCCLREDGE_A>;
#[doc = "Hardware Sync Trigger Clear Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCCLREDGE_A {
    #[doc = "0: Use rising edge detection"]
    RISE = 0,
    #[doc = "1: Use falling edge detection"]
    FALL = 1,
}
impl From<SYNCCLREDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCCLREDGE_A) -> Self {
        variant as _
    }
}
impl SYNCCLREDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCCLREDGE_A> {
        match self.bits {
            0 => Some(SYNCCLREDGE_A::RISE),
            1 => Some(SYNCCLREDGE_A::FALL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SYNCCLREDGE_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SYNCCLREDGE_A::FALL
    }
}
#[doc = "Field `SYNCCLREDGE` writer - Hardware Sync Trigger Clear Edge Select"]
pub type SYNCCLREDGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYNCHWSEL_SPEC, u8, SYNCCLREDGE_A, 8, O>;
impl<'a, const O: u8> SYNCCLREDGE_W<'a, O> {
    #[doc = "Use rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SYNCCLREDGE_A::RISE)
    }
    #[doc = "Use falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SYNCCLREDGE_A::FALL)
    }
}
impl R {
    #[doc = "Bits 0:7 - Hardware Sync Trigger Set Edge Select"]
    #[inline(always)]
    pub fn syncsetedge(&self) -> SYNCSETEDGE_R {
        SYNCSETEDGE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Hardware Sync Trigger Clear Edge Select"]
    #[inline(always)]
    pub fn syncclredge(&self) -> SYNCCLREDGE_R {
        SYNCCLREDGE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Hardware Sync Trigger Set Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn syncsetedge(&mut self) -> SYNCSETEDGE_W<0> {
        SYNCSETEDGE_W::new(self)
    }
    #[doc = "Bits 16:23 - Hardware Sync Trigger Clear Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn syncclredge(&mut self) -> SYNCCLREDGE_W<16> {
        SYNCCLREDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [synchwsel](index.html) module"]
pub struct SYNCHWSEL_SPEC;
impl crate::RegisterSpec for SYNCHWSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [synchwsel::R](R) reader structure"]
impl crate::Readable for SYNCHWSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [synchwsel::W](W) writer structure"]
impl crate::Writable for SYNCHWSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCHWSEL to value 0"]
impl crate::Resettable for SYNCHWSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
