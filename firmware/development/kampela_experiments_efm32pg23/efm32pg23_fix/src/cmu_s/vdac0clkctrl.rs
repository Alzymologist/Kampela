#[doc = "Register `VDAC0CLKCTRL` reader"]
pub struct R(crate::R<VDAC0CLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDAC0CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDAC0CLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDAC0CLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDAC0CLKCTRL` writer"]
pub struct W(crate::W<VDAC0CLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDAC0CLKCTRL_SPEC>;
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
impl From<crate::W<VDAC0CLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDAC0CLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: VDAC is not clocked"]
    DISABLED = 0,
    #[doc = "1: EM01GRPACLK is clocking VDAC"]
    EM01GRPACLK = 1,
    #[doc = "2: EM23GRPACLK is clocking VDAC"]
    EM23GRPACLK = 2,
    #[doc = "3: FSRCO is clocking VDAC"]
    FSRCO = 3,
    #[doc = "4: HFRCOEM23 is clocking VDAC"]
    HFRCOEM23 = 4,
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
            1 => Some(CLKSEL_A::EM01GRPACLK),
            2 => Some(CLKSEL_A::EM23GRPACLK),
            3 => Some(CLKSEL_A::FSRCO),
            4 => Some(CLKSEL_A::HFRCOEM23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `EM01GRPACLK`"]
    #[inline(always)]
    pub fn is_em01grpaclk(&self) -> bool {
        *self == CLKSEL_A::EM01GRPACLK
    }
    #[doc = "Checks if the value of the field is `EM23GRPACLK`"]
    #[inline(always)]
    pub fn is_em23grpaclk(&self) -> bool {
        *self == CLKSEL_A::EM23GRPACLK
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
    crate::FieldWriter<'a, u32, VDAC0CLKCTRL_SPEC, u8, CLKSEL_A, 3, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "VDAC is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKSEL_A::DISABLED)
    }
    #[doc = "EM01GRPACLK is clocking VDAC"]
    #[inline(always)]
    pub fn em01grpaclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::EM01GRPACLK)
    }
    #[doc = "EM23GRPACLK is clocking VDAC"]
    #[inline(always)]
    pub fn em23grpaclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::EM23GRPACLK)
    }
    #[doc = "FSRCO is clocking VDAC"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::FSRCO)
    }
    #[doc = "HFRCOEM23 is clocking VDAC"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCOEM23)
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
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdac0clkctrl](index.html) module"]
pub struct VDAC0CLKCTRL_SPEC;
impl crate::RegisterSpec for VDAC0CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdac0clkctrl::R](R) reader structure"]
impl crate::Readable for VDAC0CLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdac0clkctrl::W](W) writer structure"]
impl crate::Writable for VDAC0CLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VDAC0CLKCTRL to value 0x01"]
impl crate::Resettable for VDAC0CLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
