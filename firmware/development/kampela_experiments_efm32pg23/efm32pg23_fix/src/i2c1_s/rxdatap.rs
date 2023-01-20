#[doc = "Register `RXDATAP` reader"]
pub struct R(crate::R<RXDATAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDATAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDATAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDATAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATAP` reader - RX Data Peek"]
pub type RXDATAP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX Data Peek"]
    #[inline(always)]
    pub fn rxdatap(&self) -> RXDATAP_R {
        RXDATAP_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdatap](index.html) module"]
pub struct RXDATAP_SPEC;
impl crate::RegisterSpec for RXDATAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdatap::R](R) reader structure"]
impl crate::Readable for RXDATAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDATAP to value 0"]
impl crate::Resettable for RXDATAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
