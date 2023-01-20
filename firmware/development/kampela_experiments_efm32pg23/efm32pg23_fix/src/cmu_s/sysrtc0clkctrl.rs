#[doc = "Register `SYSRTC0CLKCTRL` reader"]
pub struct R(crate::R<SYSRTC0CLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSRTC0CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSRTC0CLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSRTC0CLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSRTC0CLKCTRL` writer"]
pub struct W(crate::W<SYSRTC0CLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSRTC0CLKCTRL_SPEC>;
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
impl From<crate::W<SYSRTC0CLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSRTC0CLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "1: LFRCO is clocking SYSRTC0CLK"]
    LFRCO = 1,
    #[doc = "2: LFXO is clocking SYSRTC0CLK"]
    LFXO = 2,
    #[doc = "3: ULFRCO is clocking SYSRTC0CLK"]
    ULFRCO = 3,
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
            1 => Some(CLKSEL_A::LFRCO),
            2 => Some(CLKSEL_A::LFXO),
            3 => Some(CLKSEL_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKSEL_A::ULFRCO
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSRTC0CLKCTRL_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "LFRCO is clocking SYSRTC0CLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFRCO)
    }
    #[doc = "LFXO is clocking SYSRTC0CLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFXO)
    }
    #[doc = "ULFRCO is clocking SYSRTC0CLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::ULFRCO)
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
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysrtc0clkctrl](index.html) module"]
pub struct SYSRTC0CLKCTRL_SPEC;
impl crate::RegisterSpec for SYSRTC0CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysrtc0clkctrl::R](R) reader structure"]
impl crate::Readable for SYSRTC0CLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysrtc0clkctrl::W](W) writer structure"]
impl crate::Writable for SYSRTC0CLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSRTC0CLKCTRL to value 0x01"]
impl crate::Resettable for SYSRTC0CLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
