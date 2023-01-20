#[doc = "Register `EFPIF` reader"]
pub struct R(crate::R<EFPIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFPIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFPIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFPIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFPIF` writer"]
pub struct W(crate::W<EFPIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFPIF_SPEC>;
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
impl From<crate::W<EFPIF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFPIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFPIF` reader - EFP Interrupt Flag"]
pub type EFPIF_R = crate::BitReader<bool>;
#[doc = "Field `EFPIF` writer - EFP Interrupt Flag"]
pub type EFPIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFPIF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EFP Interrupt Flag"]
    #[inline(always)]
    pub fn efpif(&self) -> EFPIF_R {
        EFPIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EFP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn efpif(&mut self) -> EFPIF_W<0> {
        EFPIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efpif](index.html) module"]
pub struct EFPIF_SPEC;
impl crate::RegisterSpec for EFPIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efpif::R](R) reader structure"]
impl crate::Readable for EFPIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efpif::W](W) writer structure"]
impl crate::Writable for EFPIF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFPIF to value 0"]
impl crate::Resettable for EFPIF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
