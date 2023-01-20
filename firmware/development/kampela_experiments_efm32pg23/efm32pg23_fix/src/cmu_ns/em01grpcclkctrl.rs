#[doc = "Register `EM01GRPCCLKCTRL` reader"]
pub struct R(crate::R<EM01GRPCCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM01GRPCCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM01GRPCCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM01GRPCCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM01GRPCCLKCTRL` writer"]
pub struct W(crate::W<EM01GRPCCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM01GRPCCLKCTRL_SPEC>;
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
impl From<crate::W<EM01GRPCCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM01GRPCCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "1: HFRCODPLL is clocking EM01GRPCCLK"]
    HFRCODPLL = 1,
    #[doc = "2: HFXO is clocking EM01GRPCCLK"]
    HFXO = 2,
    #[doc = "3: FSRCO is clocking EM01GRPCCLK"]
    FSRCO = 3,
    #[doc = "4: HFRCOEM23 is clocking EM01GRPCCLK"]
    HFRCOEM23 = 4,
    #[doc = "5: HFRCODPLL (retimed) is clocking EM01GRPCCLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
    HFRCODPLLRT = 5,
    #[doc = "6: HFXO (retimed) is clocking EM01GRPCCLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
    HFXORT = 6,
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
            1 => Some(CLKSEL_A::HFRCODPLL),
            2 => Some(CLKSEL_A::HFXO),
            3 => Some(CLKSEL_A::FSRCO),
            4 => Some(CLKSEL_A::HFRCOEM23),
            5 => Some(CLKSEL_A::HFRCODPLLRT),
            6 => Some(CLKSEL_A::HFXORT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFRCODPLL`"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == CLKSEL_A::HFRCODPLL
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `FSRCO`"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == CLKSEL_A::FSRCO
    }
    #[doc = "Checks if the value of the field is `HFRCOEM23`"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == CLKSEL_A::HFRCOEM23
    }
    #[doc = "Checks if the value of the field is `HFRCODPLLRT`"]
    #[inline(always)]
    pub fn is_hfrcodpllrt(&self) -> bool {
        *self == CLKSEL_A::HFRCODPLLRT
    }
    #[doc = "Checks if the value of the field is `HFXORT`"]
    #[inline(always)]
    pub fn is_hfxort(&self) -> bool {
        *self == CLKSEL_A::HFXORT
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EM01GRPCCLKCTRL_SPEC, u8, CLKSEL_A, 3, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "HFRCODPLL is clocking EM01GRPCCLK"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCODPLL)
    }
    #[doc = "HFXO is clocking EM01GRPCCLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFXO)
    }
    #[doc = "FSRCO is clocking EM01GRPCCLK"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::FSRCO)
    }
    #[doc = "HFRCOEM23 is clocking EM01GRPCCLK"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCOEM23)
    }
    #[doc = "HFRCODPLL (retimed) is clocking EM01GRPCCLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
    #[inline(always)]
    pub fn hfrcodpllrt(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCODPLLRT)
    }
    #[doc = "HFXO (retimed) is clocking EM01GRPCCLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
    #[inline(always)]
    pub fn hfxort(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFXORT)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select"]
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
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em01grpcclkctrl](index.html) module"]
pub struct EM01GRPCCLKCTRL_SPEC;
impl crate::RegisterSpec for EM01GRPCCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em01grpcclkctrl::R](R) reader structure"]
impl crate::Readable for EM01GRPCCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em01grpcclkctrl::W](W) writer structure"]
impl crate::Writable for EM01GRPCCLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EM01GRPCCLKCTRL to value 0x01"]
impl crate::Resettable for EM01GRPCCLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
