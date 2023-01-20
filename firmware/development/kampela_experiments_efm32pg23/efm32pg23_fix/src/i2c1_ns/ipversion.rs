#[doc = "Register `IPVERSION` reader"]
pub struct R(crate::R<IPVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPVERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IPVERSION` reader - IP version ID"]
pub type IPVERSION_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IP version ID"]
    #[inline(always)]
    pub fn ipversion(&self) -> IPVERSION_R {
        IPVERSION_R::new(self.bits)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipversion](index.html) module"]
pub struct IPVERSION_SPEC;
impl crate::RegisterSpec for IPVERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipversion::R](R) reader structure"]
impl crate::Readable for IPVERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPVERSION to value 0"]
impl crate::Resettable for IPVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
