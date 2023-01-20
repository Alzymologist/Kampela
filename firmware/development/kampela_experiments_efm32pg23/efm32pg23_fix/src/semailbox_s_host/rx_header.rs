#[doc = "Register `RX_HEADER` reader"]
pub struct R(crate::R<RX_HEADER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_HEADER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_HEADER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_HEADER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXHEADER` reader - RXHEADER"]
pub type RXHEADER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RXHEADER"]
    #[inline(always)]
    pub fn rxheader(&self) -> RXHEADER_R {
        RXHEADER_R::new(self.bits)
    }
}
#[doc = "A read access to this register will be mapped to the RX FIFO (only for the header).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_header](index.html) module"]
pub struct RX_HEADER_SPEC;
impl crate::RegisterSpec for RX_HEADER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_header::R](R) reader structure"]
impl crate::Readable for RX_HEADER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_HEADER to value 0"]
impl crate::Resettable for RX_HEADER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
