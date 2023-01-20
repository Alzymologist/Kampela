#[doc = "Register `RXDATAX` reader"]
pub struct R(crate::R<RXDATAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDATAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDATAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDATAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - RX Data"]
pub type RXDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERR` reader - Data Parity Error"]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `FERR` reader - Data Framing Error"]
pub type FERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:8 - RX Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Data Parity Error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Framing Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdatax](index.html) module"]
pub struct RXDATAX_SPEC;
impl crate::RegisterSpec for RXDATAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdatax::R](R) reader structure"]
impl crate::Readable for RXDATAX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDATAX to value 0"]
impl crate::Resettable for RXDATAX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
