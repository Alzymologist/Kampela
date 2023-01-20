#[doc = "Register `ICACHERAMRETNCTRL` reader"]
pub struct R(crate::R<ICACHERAMRETNCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHERAMRETNCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHERAMRETNCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHERAMRETNCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHERAMRETNCTRL` writer"]
pub struct W(crate::W<ICACHERAMRETNCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHERAMRETNCTRL_SPEC>;
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
impl From<crate::W<ICACHERAMRETNCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHERAMRETNCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMRETNCTRL` reader - ICACHERAM Retention control"]
pub type RAMRETNCTRL_R = crate::BitReader<RAMRETNCTRL_A>;
#[doc = "ICACHERAM Retention control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMRETNCTRL_A {
    #[doc = "0: None of the Host ICACHE RAM blocks powered down"]
    ALLON = 0,
    #[doc = "1: Power down all Host ICACHE RAM blocks"]
    ALLOFF = 1,
}
impl From<RAMRETNCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRETNCTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMRETNCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMRETNCTRL_A {
        match self.bits {
            false => RAMRETNCTRL_A::ALLON,
            true => RAMRETNCTRL_A::ALLOFF,
        }
    }
    #[doc = "Checks if the value of the field is `ALLON`"]
    #[inline(always)]
    pub fn is_allon(&self) -> bool {
        *self == RAMRETNCTRL_A::ALLON
    }
    #[doc = "Checks if the value of the field is `ALLOFF`"]
    #[inline(always)]
    pub fn is_alloff(&self) -> bool {
        *self == RAMRETNCTRL_A::ALLOFF
    }
}
#[doc = "Field `RAMRETNCTRL` writer - ICACHERAM Retention control"]
pub type RAMRETNCTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ICACHERAMRETNCTRL_SPEC, RAMRETNCTRL_A, O>;
impl<'a, const O: u8> RAMRETNCTRL_W<'a, O> {
    #[doc = "None of the Host ICACHE RAM blocks powered down"]
    #[inline(always)]
    pub fn allon(self) -> &'a mut W {
        self.variant(RAMRETNCTRL_A::ALLON)
    }
    #[doc = "Power down all Host ICACHE RAM blocks"]
    #[inline(always)]
    pub fn alloff(self) -> &'a mut W {
        self.variant(RAMRETNCTRL_A::ALLOFF)
    }
}
impl R {
    #[doc = "Bit 0 - ICACHERAM Retention control"]
    #[inline(always)]
    pub fn ramretnctrl(&self) -> RAMRETNCTRL_R {
        RAMRETNCTRL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ICACHERAM Retention control"]
    #[inline(always)]
    #[must_use]
    pub fn ramretnctrl(&mut self) -> RAMRETNCTRL_W<0> {
        RAMRETNCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure Host ICACHERAM retention configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icacheramretnctrl](index.html) module"]
pub struct ICACHERAMRETNCTRL_SPEC;
impl crate::RegisterSpec for ICACHERAMRETNCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icacheramretnctrl::R](R) reader structure"]
impl crate::Readable for ICACHERAMRETNCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icacheramretnctrl::W](W) writer structure"]
impl crate::Writable for ICACHERAMRETNCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICACHERAMRETNCTRL to value 0"]
impl crate::Resettable for ICACHERAMRETNCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
