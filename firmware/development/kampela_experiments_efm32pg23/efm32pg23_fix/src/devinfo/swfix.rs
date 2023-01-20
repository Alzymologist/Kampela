#[doc = "Register `SWFIX` reader"]
pub struct R(crate::R<SWFIX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWFIX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWFIX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWFIX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSV` reader - Reserved"]
pub type RSV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(self.bits)
    }
}
#[doc = "Used to track s/w workaround info\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swfix](index.html) module"]
pub struct SWFIX_SPEC;
impl crate::RegisterSpec for SWFIX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swfix::R](R) reader structure"]
impl crate::Readable for SWFIX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SWFIX to value 0xffff_ffff"]
impl crate::Resettable for SWFIX_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
