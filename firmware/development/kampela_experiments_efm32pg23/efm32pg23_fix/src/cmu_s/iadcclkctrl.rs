#[doc = "Register `IADCCLKCTRL` reader"]
pub struct R(crate::R<IADCCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IADCCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IADCCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IADCCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IADCCLKCTRL` writer"]
pub struct W(crate::W<IADCCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IADCCLKCTRL_SPEC>;
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
impl From<crate::W<IADCCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IADCCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "1: EM01GRPACLK is clocking IADCCLK"]
    EM01GRPACLK = 1,
    #[doc = "2: FSRCO is clocking IADCCLK"]
    FSRCO = 2,
    #[doc = "3: HFRCOEM23 is clocking IADCCLK"]
    HFRCOEM23 = 3,
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
            1 => Some(CLKSEL_A::EM01GRPACLK),
            2 => Some(CLKSEL_A::FSRCO),
            3 => Some(CLKSEL_A::HFRCOEM23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EM01GRPACLK`"]
    #[inline(always)]
    pub fn is_em01grpaclk(&self) -> bool {
        *self == CLKSEL_A::EM01GRPACLK
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
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IADCCLKCTRL_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "EM01GRPACLK is clocking IADCCLK"]
    #[inline(always)]
    pub fn em01grpaclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::EM01GRPACLK)
    }
    #[doc = "FSRCO is clocking IADCCLK"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::FSRCO)
    }
    #[doc = "HFRCOEM23 is clocking IADCCLK"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCOEM23)
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
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iadcclkctrl](index.html) module"]
pub struct IADCCLKCTRL_SPEC;
impl crate::RegisterSpec for IADCCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iadcclkctrl::R](R) reader structure"]
impl crate::Readable for IADCCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iadcclkctrl::W](W) writer structure"]
impl crate::Writable for IADCCLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IADCCLKCTRL to value 0x01"]
impl crate::Resettable for IADCCLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
