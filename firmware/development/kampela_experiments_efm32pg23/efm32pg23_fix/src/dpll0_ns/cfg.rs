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
#[doc = "Field `MODE` reader - Operating Mode Control"]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Operating Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: Frequency Lock Mode"]
    FLL = 0,
    #[doc = "1: Phase Lock Mode"]
    PLL = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::FLL,
            true => MODE_A::PLL,
        }
    }
    #[doc = "Checks if the value of the field is `FLL`"]
    #[inline(always)]
    pub fn is_fll(&self) -> bool {
        *self == MODE_A::FLL
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MODE_A::PLL
    }
}
#[doc = "Field `MODE` writer - Operating Mode Control"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Frequency Lock Mode"]
    #[inline(always)]
    pub fn fll(self) -> &'a mut W {
        self.variant(MODE_A::FLL)
    }
    #[doc = "Phase Lock Mode"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MODE_A::PLL)
    }
}
#[doc = "Field `EDGESEL` reader - Reference Edge Select"]
pub type EDGESEL_R = crate::BitReader<bool>;
#[doc = "Field `EDGESEL` writer - Reference Edge Select"]
pub type EDGESEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `AUTORECOVER` reader - Automatic Recovery Control"]
pub type AUTORECOVER_R = crate::BitReader<bool>;
#[doc = "Field `AUTORECOVER` writer - Automatic Recovery Control"]
pub type AUTORECOVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DITHEN` reader - Dither Enable Control"]
pub type DITHEN_R = crate::BitReader<bool>;
#[doc = "Field `DITHEN` writer - Dither Enable Control"]
pub type DITHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Operating Mode Control"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reference Edge Select"]
    #[inline(always)]
    pub fn edgesel(&self) -> EDGESEL_R {
        EDGESEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic Recovery Control"]
    #[inline(always)]
    pub fn autorecover(&self) -> AUTORECOVER_R {
        AUTORECOVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Dither Enable Control"]
    #[inline(always)]
    pub fn dithen(&self) -> DITHEN_R {
        DITHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operating Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 1 - Reference Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn edgesel(&mut self) -> EDGESEL_W<1> {
        EDGESEL_W::new(self)
    }
    #[doc = "Bit 2 - Automatic Recovery Control"]
    #[inline(always)]
    #[must_use]
    pub fn autorecover(&mut self) -> AUTORECOVER_W<2> {
        AUTORECOVER_W::new(self)
    }
    #[doc = "Bit 6 - Dither Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn dithen(&mut self) -> DITHEN_W<6> {
        DITHEN_W::new(self)
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
