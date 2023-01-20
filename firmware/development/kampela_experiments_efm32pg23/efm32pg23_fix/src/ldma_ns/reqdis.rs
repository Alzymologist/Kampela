#[doc = "Register `REQDIS` reader"]
pub struct R(crate::R<REQDIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REQDIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REQDIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REQDIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REQDIS` writer"]
pub struct W(crate::W<REQDIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REQDIS_SPEC>;
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
impl From<crate::W<REQDIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REQDIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQDIS` reader - DMA Request Disables"]
pub type REQDIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REQDIS` writer - DMA Request Disables"]
pub type REQDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REQDIS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DMA Request Disables"]
    #[inline(always)]
    pub fn reqdis(&self) -> REQDIS_R {
        REQDIS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Request Disables"]
    #[inline(always)]
    #[must_use]
    pub fn reqdis(&mut self) -> REQDIS_W<0> {
        REQDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqdis](index.html) module"]
pub struct REQDIS_SPEC;
impl crate::RegisterSpec for REQDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reqdis::R](R) reader structure"]
impl crate::Readable for REQDIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reqdis::W](W) writer structure"]
impl crate::Writable for REQDIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REQDIS to value 0"]
impl crate::Resettable for REQDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
