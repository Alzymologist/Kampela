#[doc = "Register `ECCERRADDR1` reader"]
pub struct R(crate::R<ECCERRADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCERRADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCERRADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCERRADDR1_SPEC>) -> Self {
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
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccerraddr1](index.html) module"]
pub struct ECCERRADDR1_SPEC;
impl crate::RegisterSpec for ECCERRADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccerraddr1::R](R) reader structure"]
impl crate::Readable for ECCERRADDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECCERRADDR1 to value 0"]
impl crate::Resettable for ECCERRADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
