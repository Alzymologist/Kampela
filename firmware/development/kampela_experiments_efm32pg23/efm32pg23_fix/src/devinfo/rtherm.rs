#[doc = "Register `RTHERM` reader"]
pub struct R(crate::R<RTHERM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTHERM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTHERM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTHERM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTHERM` reader - No Description"]
pub type RTHERM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn rtherm(&self) -> RTHERM_R {
        RTHERM_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RTHERM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtherm](index.html) module"]
pub struct RTHERM_SPEC;
impl crate::RegisterSpec for RTHERM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtherm::R](R) reader structure"]
impl crate::Readable for RTHERM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTHERM to value 0"]
impl crate::Resettable for RTHERM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
