#[doc = "Register `BMPUFSADDR` reader"]
pub struct R(crate::R<BMPUFSADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMPUFSADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMPUFSADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMPUFSADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BMPUFSADDR` reader - Fault Address"]
pub type BMPUFSADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fault Address"]
    #[inline(always)]
    pub fn bmpufsaddr(&self) -> BMPUFSADDR_R {
        BMPUFSADDR_R::new(self.bits)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmpufsaddr](index.html) module"]
pub struct BMPUFSADDR_SPEC;
impl crate::RegisterSpec for BMPUFSADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmpufsaddr::R](R) reader structure"]
impl crate::Readable for BMPUFSADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BMPUFSADDR to value 0"]
impl crate::Resettable for BMPUFSADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
