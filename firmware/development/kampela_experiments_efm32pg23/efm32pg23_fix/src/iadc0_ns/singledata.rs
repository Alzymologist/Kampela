#[doc = "Register `SINGLEDATA` reader"]
pub struct R(crate::R<SINGLEDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLEDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLEDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLEDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Data"]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "latest single queue conversion data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singledata](index.html) module"]
pub struct SINGLEDATA_SPEC;
impl crate::RegisterSpec for SINGLEDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singledata::R](R) reader structure"]
impl crate::Readable for SINGLEDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SINGLEDATA to value 0"]
impl crate::Resettable for SINGLEDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
