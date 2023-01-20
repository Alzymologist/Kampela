#[doc = "Register `PCHITS` reader"]
pub struct R(crate::R<PCHITS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCHITS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCHITS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCHITS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PCHITS` reader - Performance Counter Hits"]
pub type PCHITS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Performance Counter Hits"]
    #[inline(always)]
    pub fn pchits(&self) -> PCHITS_R {
        PCHITS_R::new(self.bits)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pchits](index.html) module"]
pub struct PCHITS_SPEC;
impl crate::RegisterSpec for PCHITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pchits::R](R) reader structure"]
impl crate::Readable for PCHITS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCHITS to value 0"]
impl crate::Resettable for PCHITS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
