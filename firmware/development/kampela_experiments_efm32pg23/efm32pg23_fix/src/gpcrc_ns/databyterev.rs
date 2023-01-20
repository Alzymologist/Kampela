#[doc = "Register `DATABYTEREV` reader"]
pub struct R(crate::R<DATABYTEREV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATABYTEREV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATABYTEREV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATABYTEREV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATABYTEREV` reader - Data Byte Reverse Value"]
pub type DATABYTEREV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Byte Reverse Value"]
    #[inline(always)]
    pub fn databyterev(&self) -> DATABYTEREV_R {
        DATABYTEREV_R::new(self.bits)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [databyterev](index.html) module"]
pub struct DATABYTEREV_SPEC;
impl crate::RegisterSpec for DATABYTEREV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [databyterev::R](R) reader structure"]
impl crate::Readable for DATABYTEREV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATABYTEREV to value 0"]
impl crate::Resettable for DATABYTEREV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
