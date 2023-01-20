#[doc = "Register `LPMODE` reader"]
pub struct R(crate::R<LPMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMODE` writer"]
pub struct W(crate::W<LPMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMODE_SPEC>;
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
impl From<crate::W<LPMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPLEVEL` reader - Low Power Level"]
pub type LPLEVEL_R = crate::FieldReader<u8, LPLEVEL_A>;
#[doc = "Low Power Level\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPLEVEL_A {
    #[doc = "0: Base instruction cache functionality"]
    BASIC = 0,
    #[doc = "1: Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory"]
    ADVANCED = 1,
    #[doc = "3: Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    MINACTIVITY = 3,
}
impl From<LPLEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPLEVEL_A) -> Self {
        variant as _
    }
}
impl LPLEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPLEVEL_A> {
        match self.bits {
            0 => Some(LPLEVEL_A::BASIC),
            1 => Some(LPLEVEL_A::ADVANCED),
            3 => Some(LPLEVEL_A::MINACTIVITY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BASIC`"]
    #[inline(always)]
    pub fn is_basic(&self) -> bool {
        *self == LPLEVEL_A::BASIC
    }
    #[doc = "Checks if the value of the field is `ADVANCED`"]
    #[inline(always)]
    pub fn is_advanced(&self) -> bool {
        *self == LPLEVEL_A::ADVANCED
    }
    #[doc = "Checks if the value of the field is `MINACTIVITY`"]
    #[inline(always)]
    pub fn is_minactivity(&self) -> bool {
        *self == LPLEVEL_A::MINACTIVITY
    }
}
#[doc = "Field `LPLEVEL` writer - Low Power Level"]
pub type LPLEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMODE_SPEC, u8, LPLEVEL_A, 2, O>;
impl<'a, const O: u8> LPLEVEL_W<'a, O> {
    #[doc = "Base instruction cache functionality"]
    #[inline(always)]
    pub fn basic(self) -> &'a mut W {
        self.variant(LPLEVEL_A::BASIC)
    }
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory"]
    #[inline(always)]
    pub fn advanced(self) -> &'a mut W {
        self.variant(LPLEVEL_A::ADVANCED)
    }
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    #[inline(always)]
    pub fn minactivity(self) -> &'a mut W {
        self.variant(LPLEVEL_A::MINACTIVITY)
    }
}
#[doc = "Field `NESTFACTOR` reader - Low Power Nest Factor"]
pub type NESTFACTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NESTFACTOR` writer - Low Power Nest Factor"]
pub type NESTFACTOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMODE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Low Power Level"]
    #[inline(always)]
    pub fn lplevel(&self) -> LPLEVEL_R {
        LPLEVEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Low Power Nest Factor"]
    #[inline(always)]
    pub fn nestfactor(&self) -> NESTFACTOR_R {
        NESTFACTOR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low Power Level"]
    #[inline(always)]
    #[must_use]
    pub fn lplevel(&mut self) -> LPLEVEL_W<0> {
        LPLEVEL_W::new(self)
    }
    #[doc = "Bits 4:7 - Low Power Nest Factor"]
    #[inline(always)]
    #[must_use]
    pub fn nestfactor(&mut self) -> NESTFACTOR_W<4> {
        NESTFACTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmode](index.html) module"]
pub struct LPMODE_SPEC;
impl crate::RegisterSpec for LPMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmode::R](R) reader structure"]
impl crate::Readable for LPMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmode::W](W) writer structure"]
impl crate::Writable for LPMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMODE to value 0x23"]
impl crate::Resettable for LPMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x23;
}
