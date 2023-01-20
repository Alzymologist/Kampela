#[doc = "Register `EXPORTCLKCTRL` reader"]
pub struct R(crate::R<EXPORTCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXPORTCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXPORTCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXPORTCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXPORTCLKCTRL` writer"]
pub struct W(crate::W<EXPORTCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXPORTCLKCTRL_SPEC>;
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
impl From<crate::W<EXPORTCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXPORTCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKOUTSEL0` reader - Clock Output Select 0"]
pub type CLKOUTSEL0_R = crate::FieldReader<u8, CLKOUTSEL0_A>;
#[doc = "Clock Output Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUTSEL0_A {
    #[doc = "0: CLKOUT0 is not clocked"]
    DISABLED = 0,
    #[doc = "1: HCLK is clocking CLKOUT0"]
    HCLK = 1,
    #[doc = "2: EXPORTCLK is clocking CLKOUT0"]
    HFEXPCLK = 2,
    #[doc = "3: ULFRCO is clocking CLKOUT0"]
    ULFRCO = 3,
    #[doc = "4: LFRCO is clocking CLKOUT0"]
    LFRCO = 4,
    #[doc = "5: LFXO is clocking CLKOUT0"]
    LFXO = 5,
    #[doc = "6: HFRCODPLL is clocking CLKOUT0"]
    HFRCODPLL = 6,
    #[doc = "7: HFXO is clocking CLKOUT0"]
    HFXO = 7,
    #[doc = "8: FSRCO is clocking CLKOUT0"]
    FSRCO = 8,
    #[doc = "9: HFRCOEM23 is clocking CLKOUT0"]
    HFRCOEM23 = 9,
}
impl From<CLKOUTSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL0_A) -> Self {
        variant as _
    }
}
impl CLKOUTSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL0_A> {
        match self.bits {
            0 => Some(CLKOUTSEL0_A::DISABLED),
            1 => Some(CLKOUTSEL0_A::HCLK),
            2 => Some(CLKOUTSEL0_A::HFEXPCLK),
            3 => Some(CLKOUTSEL0_A::ULFRCO),
            4 => Some(CLKOUTSEL0_A::LFRCO),
            5 => Some(CLKOUTSEL0_A::LFXO),
            6 => Some(CLKOUTSEL0_A::HFRCODPLL),
            7 => Some(CLKOUTSEL0_A::HFXO),
            8 => Some(CLKOUTSEL0_A::FSRCO),
            9 => Some(CLKOUTSEL0_A::HFRCOEM23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == CLKOUTSEL0_A::HCLK
    }
    #[doc = "Checks if the value of the field is `HFEXPCLK`"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL0_A::HFEXPCLK
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL0_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCODPLL`"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == CLKOUTSEL0_A::HFRCODPLL
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL0_A::HFXO
    }
    #[doc = "Checks if the value of the field is `FSRCO`"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == CLKOUTSEL0_A::FSRCO
    }
    #[doc = "Checks if the value of the field is `HFRCOEM23`"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == CLKOUTSEL0_A::HFRCOEM23
    }
}
#[doc = "Field `CLKOUTSEL0` writer - Clock Output Select 0"]
pub type CLKOUTSEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXPORTCLKCTRL_SPEC, u8, CLKOUTSEL0_A, 4, O>;
impl<'a, const O: u8> CLKOUTSEL0_W<'a, O> {
    #[doc = "CLKOUT0 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::DISABLED)
    }
    #[doc = "HCLK is clocking CLKOUT0"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HCLK)
    }
    #[doc = "EXPORTCLK is clocking CLKOUT0"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFEXPCLK)
    }
    #[doc = "ULFRCO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::ULFRCO)
    }
    #[doc = "LFRCO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::LFRCO)
    }
    #[doc = "LFXO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::LFXO)
    }
    #[doc = "HFRCODPLL is clocking CLKOUT0"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFRCODPLL)
    }
    #[doc = "HFXO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFXO)
    }
    #[doc = "FSRCO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::FSRCO)
    }
    #[doc = "HFRCOEM23 is clocking CLKOUT0"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFRCOEM23)
    }
}
#[doc = "Field `CLKOUTSEL1` reader - Clock Output Select 1"]
pub type CLKOUTSEL1_R = crate::FieldReader<u8, CLKOUTSEL1_A>;
#[doc = "Clock Output Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUTSEL1_A {
    #[doc = "0: CLKOUT1 is not clocked"]
    DISABLED = 0,
    #[doc = "1: HCLK is clocking CLKOUT1"]
    HCLK = 1,
    #[doc = "2: EXPORTCLK is clocking CLKOUT1"]
    HFEXPCLK = 2,
    #[doc = "3: ULFRCO is clocking CLKOUT1"]
    ULFRCO = 3,
    #[doc = "4: LFRCO is clocking CLKOUT1"]
    LFRCO = 4,
    #[doc = "5: LFXO is clocking CLKOUT1"]
    LFXO = 5,
    #[doc = "6: HFRCODPLL is clocking CLKOUT1"]
    HFRCODPLL = 6,
    #[doc = "7: HFXO is clocking CLKOUT1"]
    HFXO = 7,
    #[doc = "8: FSRCO is clocking CLKOUT1"]
    FSRCO = 8,
    #[doc = "9: HFRCOEM23 is clocking CLKOUT1"]
    HFRCOEM23 = 9,
}
impl From<CLKOUTSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL1_A) -> Self {
        variant as _
    }
}
impl CLKOUTSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL1_A> {
        match self.bits {
            0 => Some(CLKOUTSEL1_A::DISABLED),
            1 => Some(CLKOUTSEL1_A::HCLK),
            2 => Some(CLKOUTSEL1_A::HFEXPCLK),
            3 => Some(CLKOUTSEL1_A::ULFRCO),
            4 => Some(CLKOUTSEL1_A::LFRCO),
            5 => Some(CLKOUTSEL1_A::LFXO),
            6 => Some(CLKOUTSEL1_A::HFRCODPLL),
            7 => Some(CLKOUTSEL1_A::HFXO),
            8 => Some(CLKOUTSEL1_A::FSRCO),
            9 => Some(CLKOUTSEL1_A::HFRCOEM23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == CLKOUTSEL1_A::HCLK
    }
    #[doc = "Checks if the value of the field is `HFEXPCLK`"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL1_A::HFEXPCLK
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL1_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL1_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL1_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCODPLL`"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == CLKOUTSEL1_A::HFRCODPLL
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL1_A::HFXO
    }
    #[doc = "Checks if the value of the field is `FSRCO`"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == CLKOUTSEL1_A::FSRCO
    }
    #[doc = "Checks if the value of the field is `HFRCOEM23`"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == CLKOUTSEL1_A::HFRCOEM23
    }
}
#[doc = "Field `CLKOUTSEL1` writer - Clock Output Select 1"]
pub type CLKOUTSEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXPORTCLKCTRL_SPEC, u8, CLKOUTSEL1_A, 4, O>;
impl<'a, const O: u8> CLKOUTSEL1_W<'a, O> {
    #[doc = "CLKOUT1 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::DISABLED)
    }
    #[doc = "HCLK is clocking CLKOUT1"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HCLK)
    }
    #[doc = "EXPORTCLK is clocking CLKOUT1"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFEXPCLK)
    }
    #[doc = "ULFRCO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::ULFRCO)
    }
    #[doc = "LFRCO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFRCO)
    }
    #[doc = "LFXO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFXO)
    }
    #[doc = "HFRCODPLL is clocking CLKOUT1"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFRCODPLL)
    }
    #[doc = "HFXO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFXO)
    }
    #[doc = "FSRCO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::FSRCO)
    }
    #[doc = "HFRCOEM23 is clocking CLKOUT1"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFRCOEM23)
    }
}
#[doc = "Field `CLKOUTSEL2` reader - Clock Output Select 2"]
pub type CLKOUTSEL2_R = crate::FieldReader<u8, CLKOUTSEL2_A>;
#[doc = "Clock Output Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUTSEL2_A {
    #[doc = "0: CLKOUT2 is not clocked"]
    DISABLED = 0,
    #[doc = "1: HCLK is clocking CLKOUT2"]
    HCLK = 1,
    #[doc = "2: EXPORTCLK is clocking CLKOUT2"]
    HFEXPCLK = 2,
    #[doc = "3: ULFRCO is clocking CLKOUT2"]
    ULFRCO = 3,
    #[doc = "4: LFRCO is clocking CLKOUT2"]
    LFRCO = 4,
    #[doc = "5: LFXO is clocking CLKOUT2"]
    LFXO = 5,
    #[doc = "6: HFRCODPLL is clocking CLKOUT2"]
    HFRCODPLL = 6,
    #[doc = "7: HFXO is clocking CLKOUT2"]
    HFXO = 7,
    #[doc = "8: FSRCO is clocking CLKOUT2"]
    FSRCO = 8,
    #[doc = "9: HFRCOEM23 is clocking CLKOUT2"]
    HFRCOEM23 = 9,
}
impl From<CLKOUTSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL2_A) -> Self {
        variant as _
    }
}
impl CLKOUTSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL2_A> {
        match self.bits {
            0 => Some(CLKOUTSEL2_A::DISABLED),
            1 => Some(CLKOUTSEL2_A::HCLK),
            2 => Some(CLKOUTSEL2_A::HFEXPCLK),
            3 => Some(CLKOUTSEL2_A::ULFRCO),
            4 => Some(CLKOUTSEL2_A::LFRCO),
            5 => Some(CLKOUTSEL2_A::LFXO),
            6 => Some(CLKOUTSEL2_A::HFRCODPLL),
            7 => Some(CLKOUTSEL2_A::HFXO),
            8 => Some(CLKOUTSEL2_A::FSRCO),
            9 => Some(CLKOUTSEL2_A::HFRCOEM23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == CLKOUTSEL2_A::HCLK
    }
    #[doc = "Checks if the value of the field is `HFEXPCLK`"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL2_A::HFEXPCLK
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL2_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL2_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL2_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCODPLL`"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == CLKOUTSEL2_A::HFRCODPLL
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL2_A::HFXO
    }
    #[doc = "Checks if the value of the field is `FSRCO`"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == CLKOUTSEL2_A::FSRCO
    }
    #[doc = "Checks if the value of the field is `HFRCOEM23`"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == CLKOUTSEL2_A::HFRCOEM23
    }
}
#[doc = "Field `CLKOUTSEL2` writer - Clock Output Select 2"]
pub type CLKOUTSEL2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXPORTCLKCTRL_SPEC, u8, CLKOUTSEL2_A, 4, O>;
impl<'a, const O: u8> CLKOUTSEL2_W<'a, O> {
    #[doc = "CLKOUT2 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::DISABLED)
    }
    #[doc = "HCLK is clocking CLKOUT2"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HCLK)
    }
    #[doc = "EXPORTCLK is clocking CLKOUT2"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFEXPCLK)
    }
    #[doc = "ULFRCO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::ULFRCO)
    }
    #[doc = "LFRCO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::LFRCO)
    }
    #[doc = "LFXO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::LFXO)
    }
    #[doc = "HFRCODPLL is clocking CLKOUT2"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFRCODPLL)
    }
    #[doc = "HFXO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFXO)
    }
    #[doc = "FSRCO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::FSRCO)
    }
    #[doc = "HFRCOEM23 is clocking CLKOUT2"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFRCOEM23)
    }
}
#[doc = "Field `PRESC` reader - EXPORTCLK Prescaler"]
pub type PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESC` writer - EXPORTCLK Prescaler"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXPORTCLKCTRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:3 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&self) -> CLKOUTSEL0_R {
        CLKOUTSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&self) -> CLKOUTSEL1_R {
        CLKOUTSEL1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Clock Output Select 2"]
    #[inline(always)]
    pub fn clkoutsel2(&self) -> CLKOUTSEL2_R {
        CLKOUTSEL2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - EXPORTCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Output Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel0(&mut self) -> CLKOUTSEL0_W<0> {
        CLKOUTSEL0_W::new(self)
    }
    #[doc = "Bits 8:11 - Clock Output Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel1(&mut self) -> CLKOUTSEL1_W<8> {
        CLKOUTSEL1_W::new(self)
    }
    #[doc = "Bits 16:19 - Clock Output Select 2"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel2(&mut self) -> CLKOUTSEL2_W<16> {
        CLKOUTSEL2_W::new(self)
    }
    #[doc = "Bits 24:28 - EXPORTCLK Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<24> {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exportclkctrl](index.html) module"]
pub struct EXPORTCLKCTRL_SPEC;
impl crate::RegisterSpec for EXPORTCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exportclkctrl::R](R) reader structure"]
impl crate::Readable for EXPORTCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exportclkctrl::W](W) writer structure"]
impl crate::Writable for EXPORTCLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXPORTCLKCTRL to value 0"]
impl crate::Resettable for EXPORTCLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
