#[doc = "Register `DATAREV` reader"]
pub struct R(crate::R<DATAREV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAREV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAREV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAREV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATAREV` reader - Data Reverse Value"]
pub type DATAREV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Reverse Value"]
    #[inline(always)]
    pub fn datarev(&self) -> DATAREV_R {
        DATAREV_R::new(self.bits)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datarev](index.html) module"]
pub struct DATAREV_SPEC;
impl crate::RegisterSpec for DATAREV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datarev::R](R) reader structure"]
impl crate::Readable for DATAREV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATAREV to value 0"]
impl crate::Resettable for DATAREV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
