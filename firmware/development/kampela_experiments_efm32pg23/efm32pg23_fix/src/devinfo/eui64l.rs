#[doc = "Register `EUI64L` reader"]
pub struct R(crate::R<EUI64L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EUI64L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EUI64L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EUI64L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNIQUEL` reader - UNIQUEL"]
pub type UNIQUEL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - UNIQUEL"]
    #[inline(always)]
    pub fn uniquel(&self) -> UNIQUEL_R {
        UNIQUEL_R::new(self.bits)
    }
}
#[doc = "MA-L compliant EUI64 Unique Identifier (low bits)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eui64l](index.html) module"]
pub struct EUI64L_SPEC;
impl crate::RegisterSpec for EUI64L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eui64l::R](R) reader structure"]
impl crate::Readable for EUI64L_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EUI64L to value 0"]
impl crate::Resettable for EUI64L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
