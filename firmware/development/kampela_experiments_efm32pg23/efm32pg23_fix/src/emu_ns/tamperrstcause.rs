#[doc = "Register `TAMPERRSTCAUSE` reader"]
pub struct R(crate::R<TAMPERRSTCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMPERRSTCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMPERRSTCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMPERRSTCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAMPERRST` reader - Tamper reset vector"]
pub type TAMPERRST_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tamper reset vector"]
    #[inline(always)]
    pub fn tamperrst(&self) -> TAMPERRST_R {
        TAMPERRST_R::new(self.bits)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamperrstcause](index.html) module"]
pub struct TAMPERRSTCAUSE_SPEC;
impl crate::RegisterSpec for TAMPERRSTCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamperrstcause::R](R) reader structure"]
impl crate::Readable for TAMPERRSTCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAMPERRSTCAUSE to value 0"]
impl crate::Resettable for TAMPERRSTCAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
