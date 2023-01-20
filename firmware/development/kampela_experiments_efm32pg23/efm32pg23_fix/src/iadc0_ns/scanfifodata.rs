#[doc = "Register `SCANFIFODATA` reader"]
pub struct R(crate::R<SCANFIFODATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANFIFODATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANFIFODATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANFIFODATA_SPEC>) -> Self {
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
#[doc = "Read the oldest valid data from the scan FIFO and pop the FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanfifodata](index.html) module"]
pub struct SCANFIFODATA_SPEC;
impl crate::RegisterSpec for SCANFIFODATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scanfifodata::R](R) reader structure"]
impl crate::Readable for SCANFIFODATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCANFIFODATA to value 0"]
impl crate::Resettable for SCANFIFODATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
