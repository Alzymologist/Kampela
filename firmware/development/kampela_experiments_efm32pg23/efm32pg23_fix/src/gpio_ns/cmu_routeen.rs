#[doc = "Register `CMU_ROUTEEN` reader"]
pub struct R(crate::R<CMU_ROUTEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMU_ROUTEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMU_ROUTEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMU_ROUTEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMU_ROUTEEN` writer"]
pub struct W(crate::W<CMU_ROUTEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMU_ROUTEEN_SPEC>;
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
impl From<crate::W<CMU_ROUTEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMU_ROUTEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKOUT0PEN` reader - CLKOUT0 pin enable control bit"]
pub type CLKOUT0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKOUT0PEN` writer - CLKOUT0 pin enable control bit"]
pub type CLKOUT0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMU_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CLKOUT1PEN` reader - CLKOUT1 pin enable control bit"]
pub type CLKOUT1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKOUT1PEN` writer - CLKOUT1 pin enable control bit"]
pub type CLKOUT1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMU_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CLKOUT2PEN` reader - CLKOUT2 pin enable control bit"]
pub type CLKOUT2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKOUT2PEN` writer - CLKOUT2 pin enable control bit"]
pub type CLKOUT2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMU_ROUTEEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CLKOUT0 pin enable control bit"]
    #[inline(always)]
    pub fn clkout0pen(&self) -> CLKOUT0PEN_R {
        CLKOUT0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CLKOUT1 pin enable control bit"]
    #[inline(always)]
    pub fn clkout1pen(&self) -> CLKOUT1PEN_R {
        CLKOUT1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CLKOUT2 pin enable control bit"]
    #[inline(always)]
    pub fn clkout2pen(&self) -> CLKOUT2PEN_R {
        CLKOUT2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLKOUT0 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn clkout0pen(&mut self) -> CLKOUT0PEN_W<0> {
        CLKOUT0PEN_W::new(self)
    }
    #[doc = "Bit 1 - CLKOUT1 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn clkout1pen(&mut self) -> CLKOUT1PEN_W<1> {
        CLKOUT1PEN_W::new(self)
    }
    #[doc = "Bit 2 - CLKOUT2 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn clkout2pen(&mut self) -> CLKOUT2PEN_W<2> {
        CLKOUT2PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMU pin enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmu_routeen](index.html) module"]
pub struct CMU_ROUTEEN_SPEC;
impl crate::RegisterSpec for CMU_ROUTEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmu_routeen::R](R) reader structure"]
impl crate::Readable for CMU_ROUTEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmu_routeen::W](W) writer structure"]
impl crate::Writable for CMU_ROUTEEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMU_ROUTEEN to value 0"]
impl crate::Resettable for CMU_ROUTEEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
