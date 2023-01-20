#[doc = "Register `REQPEND` reader"]
pub struct R(crate::R<REQPEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REQPEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REQPEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REQPEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REQPEND` reader - DMA Requests Pending"]
pub type REQPEND_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - DMA Requests Pending"]
    #[inline(always)]
    pub fn reqpend(&self) -> REQPEND_R {
        REQPEND_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqpend](index.html) module"]
pub struct REQPEND_SPEC;
impl crate::RegisterSpec for REQPEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reqpend::R](R) reader structure"]
impl crate::Readable for REQPEND_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REQPEND to value 0"]
impl crate::Resettable for REQPEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
