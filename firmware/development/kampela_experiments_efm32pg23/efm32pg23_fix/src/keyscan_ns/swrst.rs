#[doc = "Register `SWRST` reader"]
pub struct R(crate::R<SWRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWRST` writer"]
pub struct W(crate::W<SWRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRST_SPEC>;
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
impl From<crate::W<SWRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` writer - Software reset command"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_SPEC, bool, O>;
#[doc = "Field `RESETTING` reader - Software reset busy status"]
pub type RESETTING_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Software reset busy status"]
    #[inline(always)]
    pub fn resetting(&self) -> RESETTING_R {
        RESETTING_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset command"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst](index.html) module"]
pub struct SWRST_SPEC;
impl crate::RegisterSpec for SWRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swrst::R](R) reader structure"]
impl crate::Readable for SWRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swrst::W](W) writer structure"]
impl crate::Writable for SWRST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWRST to value 0"]
impl crate::Resettable for SWRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
