#[doc = "Register `CALTEMP` reader"]
pub struct R(crate::R<CALTEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALTEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALTEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALTEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEMP` reader - Cal Temp"]
pub type TEMP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Cal Temp"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Calibration Temperature Information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caltemp](index.html) module"]
pub struct CALTEMP_SPEC;
impl crate::RegisterSpec for CALTEMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [caltemp::R](R) reader structure"]
impl crate::Readable for CALTEMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CALTEMP to value 0"]
impl crate::Resettable for CALTEMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
