#[doc = "Register `PCAHITS` reader"]
pub struct R(crate::R<PCAHITS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCAHITS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCAHITS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCAHITS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PCAHITS` reader - Performance Counter Advanced Hits"]
pub type PCAHITS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Performance Counter Advanced Hits"]
    #[inline(always)]
    pub fn pcahits(&self) -> PCAHITS_R {
        PCAHITS_R::new(self.bits)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcahits](index.html) module"]
pub struct PCAHITS_SPEC;
impl crate::RegisterSpec for PCAHITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcahits::R](R) reader structure"]
impl crate::Readable for PCAHITS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCAHITS to value 0"]
impl crate::Resettable for PCAHITS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
