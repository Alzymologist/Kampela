#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Crystal Oscillator Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Crystal Oscillator Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: crystal oscillator"]
    XTAL = 0,
    #[doc = "1: external sinusoidal clock can be supplied on XI pin."]
    EXTCLK = 1,
    #[doc = "2: external sinusoidal clock can be supplied on XI pin (peak detector used)."]
    EXTCLKPKDET = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::XTAL),
            1 => Some(MODE_A::EXTCLK),
            2 => Some(MODE_A::EXTCLKPKDET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == MODE_A::XTAL
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == MODE_A::EXTCLK
    }
    #[doc = "Checks if the value of the field is `EXTCLKPKDET`"]
    #[inline(always)]
    pub fn is_extclkpkdet(&self) -> bool {
        *self == MODE_A::EXTCLKPKDET
    }
}
#[doc = "Field `MODE` writer - Crystal Oscillator Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(MODE_A::XTAL)
    }
    #[doc = "external sinusoidal clock can be supplied on XI pin."]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(MODE_A::EXTCLK)
    }
    #[doc = "external sinusoidal clock can be supplied on XI pin (peak detector used)."]
    #[inline(always)]
    pub fn extclkpkdet(self) -> &'a mut W {
        self.variant(MODE_A::EXTCLKPKDET)
    }
}
#[doc = "Field `ENXIDCBIASANA` reader - Enable XI Internal DC Bias"]
pub type ENXIDCBIASANA_R = crate::BitReader<bool>;
#[doc = "Field `ENXIDCBIASANA` writer - Enable XI Internal DC Bias"]
pub type ENXIDCBIASANA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `SQBUFSCHTRGANA` reader - Squaring Buffer Schmitt Trigger"]
pub type SQBUFSCHTRGANA_R = crate::BitReader<SQBUFSCHTRGANA_A>;
#[doc = "Squaring Buffer Schmitt Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQBUFSCHTRGANA_A {
    #[doc = "0: Squaring buffer schmitt trigger is disabled"]
    DISABLE = 0,
    #[doc = "1: Squaring buffer schmitt trigger is enabled"]
    ENABLE = 1,
}
impl From<SQBUFSCHTRGANA_A> for bool {
    #[inline(always)]
    fn from(variant: SQBUFSCHTRGANA_A) -> Self {
        variant as u8 != 0
    }
}
impl SQBUFSCHTRGANA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQBUFSCHTRGANA_A {
        match self.bits {
            false => SQBUFSCHTRGANA_A::DISABLE,
            true => SQBUFSCHTRGANA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SQBUFSCHTRGANA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SQBUFSCHTRGANA_A::ENABLE
    }
}
#[doc = "Field `SQBUFSCHTRGANA` writer - Squaring Buffer Schmitt Trigger"]
pub type SQBUFSCHTRGANA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFG_SPEC, SQBUFSCHTRGANA_A, O>;
impl<'a, const O: u8> SQBUFSCHTRGANA_W<'a, O> {
    #[doc = "Squaring buffer schmitt trigger is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SQBUFSCHTRGANA_A::DISABLE)
    }
    #[doc = "Squaring buffer schmitt trigger is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SQBUFSCHTRGANA_A::ENABLE)
    }
}
#[doc = "Field `FORCELFTIMEOUT` reader - Force Low Frequency Timeout"]
pub type FORCELFTIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `FORCELFTIMEOUT` writer - Force Low Frequency Timeout"]
pub type FORCELFTIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Crystal Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Enable XI Internal DC Bias"]
    #[inline(always)]
    pub fn enxidcbiasana(&self) -> ENXIDCBIASANA_R {
        ENXIDCBIASANA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Squaring Buffer Schmitt Trigger"]
    #[inline(always)]
    pub fn sqbufschtrgana(&self) -> SQBUFSCHTRGANA_R {
        SQBUFSCHTRGANA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 28 - Force Low Frequency Timeout"]
    #[inline(always)]
    pub fn forcelftimeout(&self) -> FORCELFTIMEOUT_R {
        FORCELFTIMEOUT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Crystal Oscillator Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - Enable XI Internal DC Bias"]
    #[inline(always)]
    #[must_use]
    pub fn enxidcbiasana(&mut self) -> ENXIDCBIASANA_W<2> {
        ENXIDCBIASANA_W::new(self)
    }
    #[doc = "Bit 3 - Squaring Buffer Schmitt Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn sqbufschtrgana(&mut self) -> SQBUFSCHTRGANA_W<3> {
        SQBUFSCHTRGANA_W::new(self)
    }
    #[doc = "Bit 28 - Force Low Frequency Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn forcelftimeout(&mut self) -> FORCELFTIMEOUT_W<28> {
        FORCELFTIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0x1000_0000"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0000;
}
