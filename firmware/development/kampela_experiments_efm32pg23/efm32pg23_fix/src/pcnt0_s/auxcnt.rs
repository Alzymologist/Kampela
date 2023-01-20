#[doc = "Register `AUXCNT` reader"]
pub struct R(crate::R<AUXCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AUXCNT` reader - Auxiliary Counter Value"]
pub type AUXCNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Auxiliary Counter Value"]
    #[inline(always)]
    pub fn auxcnt(&self) -> AUXCNT_R {
        AUXCNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxcnt](index.html) module"]
pub struct AUXCNT_SPEC;
impl crate::RegisterSpec for AUXCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxcnt::R](R) reader structure"]
impl crate::Readable for AUXCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AUXCNT to value 0"]
impl crate::Resettable for AUXCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
