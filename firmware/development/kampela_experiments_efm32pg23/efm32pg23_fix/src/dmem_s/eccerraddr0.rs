#[doc = "Register `ECCERRADDR0` reader"]
pub struct R(crate::R<ECCERRADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCERRADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCERRADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCERRADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - ECC Error Address"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC Error Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccerraddr0](index.html) module"]
pub struct ECCERRADDR0_SPEC;
impl crate::RegisterSpec for ECCERRADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccerraddr0::R](R) reader structure"]
impl crate::Readable for ECCERRADDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECCERRADDR0 to value 0"]
impl crate::Resettable for ECCERRADDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
