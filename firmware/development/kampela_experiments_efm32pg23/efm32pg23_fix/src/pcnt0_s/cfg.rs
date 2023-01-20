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
#[doc = "Field `MODE` reader - Mode Select"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Single input EM23GRPACLK oversampling mode (available in EM0-EM3)."]
    OVSSINGLE = 0,
    #[doc = "1: Externally clocked single input counter mode (available in EM0-EM3)."]
    EXTCLKSINGLE = 1,
    #[doc = "2: Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    EXTCLKQUAD = 2,
    #[doc = "3: EM23GRPACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    OVSQUAD1X = 3,
    #[doc = "4: EM23GRPACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    OVSQUAD2X = 4,
    #[doc = "5: EM23GRPACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    OVSQUAD4X = 5,
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
            0 => Some(MODE_A::OVSSINGLE),
            1 => Some(MODE_A::EXTCLKSINGLE),
            2 => Some(MODE_A::EXTCLKQUAD),
            3 => Some(MODE_A::OVSQUAD1X),
            4 => Some(MODE_A::OVSQUAD2X),
            5 => Some(MODE_A::OVSQUAD4X),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OVSSINGLE`"]
    #[inline(always)]
    pub fn is_ovssingle(&self) -> bool {
        *self == MODE_A::OVSSINGLE
    }
    #[doc = "Checks if the value of the field is `EXTCLKSINGLE`"]
    #[inline(always)]
    pub fn is_extclksingle(&self) -> bool {
        *self == MODE_A::EXTCLKSINGLE
    }
    #[doc = "Checks if the value of the field is `EXTCLKQUAD`"]
    #[inline(always)]
    pub fn is_extclkquad(&self) -> bool {
        *self == MODE_A::EXTCLKQUAD
    }
    #[doc = "Checks if the value of the field is `OVSQUAD1X`"]
    #[inline(always)]
    pub fn is_ovsquad1x(&self) -> bool {
        *self == MODE_A::OVSQUAD1X
    }
    #[doc = "Checks if the value of the field is `OVSQUAD2X`"]
    #[inline(always)]
    pub fn is_ovsquad2x(&self) -> bool {
        *self == MODE_A::OVSQUAD2X
    }
    #[doc = "Checks if the value of the field is `OVSQUAD4X`"]
    #[inline(always)]
    pub fn is_ovsquad4x(&self) -> bool {
        *self == MODE_A::OVSQUAD4X
    }
}
#[doc = "Field `MODE` writer - Mode Select"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, MODE_A, 3, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Single input EM23GRPACLK oversampling mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovssingle(self) -> &'a mut W {
        self.variant(MODE_A::OVSSINGLE)
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclksingle(self) -> &'a mut W {
        self.variant(MODE_A::EXTCLKSINGLE)
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclkquad(self) -> &'a mut W {
        self.variant(MODE_A::EXTCLKQUAD)
    }
    #[doc = "EM23GRPACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad1x(self) -> &'a mut W {
        self.variant(MODE_A::OVSQUAD1X)
    }
    #[doc = "EM23GRPACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad2x(self) -> &'a mut W {
        self.variant(MODE_A::OVSQUAD2X)
    }
    #[doc = "EM23GRPACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad4x(self) -> &'a mut W {
        self.variant(MODE_A::OVSQUAD4X)
    }
}
#[doc = "Field `DEBUGHALT` reader - Debug Mode Halt Enable"]
pub type DEBUGHALT_R = crate::BitReader<DEBUGHALT_A>;
#[doc = "Debug Mode Halt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBUGHALT_A {
    #[doc = "0: PCNT is running in debug mode."]
    DISABLE = 0,
    #[doc = "1: PCNT is frozen in debug mode."]
    ENABLE = 1,
}
impl From<DEBUGHALT_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUGHALT_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBUGHALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUGHALT_A {
        match self.bits {
            false => DEBUGHALT_A::DISABLE,
            true => DEBUGHALT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEBUGHALT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DEBUGHALT_A::ENABLE
    }
}
#[doc = "Field `DEBUGHALT` writer - Debug Mode Halt Enable"]
pub type DEBUGHALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, DEBUGHALT_A, O>;
impl<'a, const O: u8> DEBUGHALT_W<'a, O> {
    #[doc = "PCNT is running in debug mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DEBUGHALT_A::DISABLE)
    }
    #[doc = "PCNT is frozen in debug mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DEBUGHALT_A::ENABLE)
    }
}
#[doc = "Field `FILTEN` reader - Enable Digital Pulse Width Filter"]
pub type FILTEN_R = crate::BitReader<bool>;
#[doc = "Field `FILTEN` writer - Enable Digital Pulse Width Filter"]
pub type FILTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `HYST` reader - Enable Hysteresis"]
pub type HYST_R = crate::BitReader<bool>;
#[doc = "Field `HYST` writer - Enable Hysteresis"]
pub type HYST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `S0PRSEN` reader - S0IN PRS Enable"]
pub type S0PRSEN_R = crate::BitReader<bool>;
#[doc = "Field `S0PRSEN` writer - S0IN PRS Enable"]
pub type S0PRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `S1PRSEN` reader - S1IN PRS Enable"]
pub type S1PRSEN_R = crate::BitReader<bool>;
#[doc = "Field `S1PRSEN` writer - S1IN PRS Enable"]
pub type S1PRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn debughalt(&self) -> DEBUGHALT_R {
        DEBUGHALT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filten(&self) -> FILTEN_R {
        FILTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&self) -> S0PRSEN_R {
        S0PRSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&self) -> S1PRSEN_R {
        S1PRSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - Debug Mode Halt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debughalt(&mut self) -> DEBUGHALT_W<4> {
        DEBUGHALT_W::new(self)
    }
    #[doc = "Bit 5 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filten(&mut self) -> FILTEN_W<5> {
        FILTEN_W::new(self)
    }
    #[doc = "Bit 6 - Enable Hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<6> {
        HYST_W::new(self)
    }
    #[doc = "Bit 8 - S0IN PRS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn s0prsen(&mut self) -> S0PRSEN_W<8> {
        S0PRSEN_W::new(self)
    }
    #[doc = "Bit 9 - S1IN PRS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1prsen(&mut self) -> S1PRSEN_W<9> {
        S1PRSEN_W::new(self)
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
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
