#[doc = "Register `DPLLREFCLKCTRL` reader"]
pub struct R(crate::R<DPLLREFCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLLREFCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLLREFCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLLREFCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPLLREFCLKCTRL` writer"]
pub struct W(crate::W<DPLLREFCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPLLREFCLKCTRL_SPEC>;
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
impl From<crate::W<DPLLREFCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPLLREFCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: DPLLREFCLK is not clocked"]
    DISABLED = 0,
    #[doc = "1: HFXO is clocking DPLLREFCLK"]
    HFXO = 1,
    #[doc = "2: LFXO is clocking DPLLREFCLK"]
    LFXO = 2,
    #[doc = "3: CLKIN0 is clocking DPLLREFCLK"]
    CLKIN0 = 3,
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
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::DISABLED,
            1 => CLKSEL_A::HFXO,
            2 => CLKSEL_A::LFXO,
            3 => CLKSEL_A::CLKIN0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `CLKIN0`"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == CLKSEL_A::CLKIN0
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DPLLREFCLKCTRL_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "DPLLREFCLK is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKSEL_A::DISABLED)
    }
    #[doc = "HFXO is clocking DPLLREFCLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFXO)
    }
    #[doc = "LFXO is clocking DPLLREFCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFXO)
    }
    #[doc = "CLKIN0 is clocking DPLLREFCLK"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut W {
        self.variant(CLKSEL_A::CLKIN0)
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
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllrefclkctrl](index.html) module"]
pub struct DPLLREFCLKCTRL_SPEC;
impl crate::RegisterSpec for DPLLREFCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpllrefclkctrl::R](R) reader structure"]
impl crate::Readable for DPLLREFCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpllrefclkctrl::W](W) writer structure"]
impl crate::Writable for DPLLREFCLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPLLREFCLKCTRL to value 0"]
impl crate::Resettable for DPLLREFCLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
