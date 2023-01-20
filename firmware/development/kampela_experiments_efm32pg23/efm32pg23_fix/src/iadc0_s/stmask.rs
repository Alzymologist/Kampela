#[doc = "Register `STMASK` reader"]
pub struct R(crate::R<STMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STMASK` reader - Scan Table Mask"]
pub type STMASK_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Scan Table Mask"]
    #[inline(always)]
    pub fn stmask(&self) -> STMASK_R {
        STMASK_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Scan Table Mask\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmask](index.html) module"]
pub struct STMASK_SPEC;
impl crate::RegisterSpec for STMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stmask::R](R) reader structure"]
impl crate::Readable for STMASK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STMASK to value 0"]
impl crate::Resettable for STMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
