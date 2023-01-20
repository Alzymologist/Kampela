#[doc = "Register `CC1_ICOF` reader"]
pub struct R(crate::R<CC1_ICOF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC1_ICOF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC1_ICOF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC1_ICOF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ICOF` reader - Input Capture FIFO Overflow"]
pub type ICOF_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Input Capture FIFO Overflow"]
    #[inline(always)]
    pub fn icof(&self) -> ICOF_R {
        ICOF_R::new(self.bits)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc1_icof](index.html) module"]
pub struct CC1_ICOF_SPEC;
impl crate::RegisterSpec for CC1_ICOF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc1_icof::R](R) reader structure"]
impl crate::Readable for CC1_ICOF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CC1_ICOF to value 0"]
impl crate::Resettable for CC1_ICOF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
