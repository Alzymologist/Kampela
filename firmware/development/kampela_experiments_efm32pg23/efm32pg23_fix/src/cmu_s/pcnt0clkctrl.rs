#[doc = "Register `PCNT0CLKCTRL` reader"]
pub struct R(crate::R<PCNT0CLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNT0CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNT0CLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNT0CLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCNT0CLKCTRL` writer"]
pub struct W(crate::W<PCNT0CLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNT0CLKCTRL_SPEC>;
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
impl From<crate::W<PCNT0CLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNT0CLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: PCNT0 is not clocked"]
    DISABLED = 0,
    #[doc = "1: EM23GRPACLK is clocking PCNT0"]
    EM23GRPACLK = 1,
    #[doc = "2: External pin PCNT_S0 is clocking PCNT0"]
    PCNTS0 = 2,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::DISABLED),
            1 => Some(CLKSEL_A::EM23GRPACLK),
            2 => Some(CLKSEL_A::PCNTS0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `EM23GRPACLK`"]
    #[inline(always)]
    pub fn is_em23grpaclk(&self) -> bool {
        *self == CLKSEL_A::EM23GRPACLK
    }
    #[doc = "Checks if the value of the field is `PCNTS0`"]
    #[inline(always)]
    pub fn is_pcnts0(&self) -> bool {
        *self == CLKSEL_A::PCNTS0
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCNT0CLKCTRL_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "PCNT0 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKSEL_A::DISABLED)
    }
    #[doc = "EM23GRPACLK is clocking PCNT0"]
    #[inline(always)]
    pub fn em23grpaclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::EM23GRPACLK)
    }
    #[doc = "External pin PCNT_S0 is clocking PCNT0"]
    #[inline(always)]
    pub fn pcnts0(self) -> &'a mut W {
        self.variant(CLKSEL_A::PCNTS0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcnt0clkctrl](index.html) module"]
pub struct PCNT0CLKCTRL_SPEC;
impl crate::RegisterSpec for PCNT0CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcnt0clkctrl::R](R) reader structure"]
impl crate::Readable for PCNT0CLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcnt0clkctrl::W](W) writer structure"]
impl crate::Writable for PCNT0CLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCNT0CLKCTRL to value 0x01"]
impl crate::Resettable for PCNT0CLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
