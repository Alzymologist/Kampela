#[doc = "Register `SINGLEFIFODATA` reader"]
pub struct R(crate::R<SINGLEFIFODATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLEFIFODATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLEFIFODATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLEFIFODATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Single FIFO Read Data"]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Single FIFO Read Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Read the oldest valid data from the single FIFO and pop the FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlefifodata](index.html) module"]
pub struct SINGLEFIFODATA_SPEC;
impl crate::RegisterSpec for SINGLEFIFODATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singlefifodata::R](R) reader structure"]
impl crate::Readable for SINGLEFIFODATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SINGLEFIFODATA to value 0"]
impl crate::Resettable for SINGLEFIFODATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
