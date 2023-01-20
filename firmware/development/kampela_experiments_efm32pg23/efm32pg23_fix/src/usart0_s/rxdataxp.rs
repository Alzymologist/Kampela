#[doc = "Register `RXDATAXP` reader"]
pub struct R(crate::R<RXDATAXP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDATAXP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDATAXP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDATAXP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATAP` reader - RX Data Peek"]
pub type RXDATAP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERRP` reader - Data Parity Error Peek"]
pub type PERRP_R = crate::BitReader<bool>;
#[doc = "Field `FERRP` reader - Data Framing Error Peek"]
pub type FERRP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:8 - RX Data Peek"]
    #[inline(always)]
    pub fn rxdatap(&self) -> RXDATAP_R {
        RXDATAP_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Data Parity Error Peek"]
    #[inline(always)]
    pub fn perrp(&self) -> PERRP_R {
        PERRP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Framing Error Peek"]
    #[inline(always)]
    pub fn ferrp(&self) -> FERRP_R {
        FERRP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdataxp](index.html) module"]
pub struct RXDATAXP_SPEC;
impl crate::RegisterSpec for RXDATAXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdataxp::R](R) reader structure"]
impl crate::Readable for RXDATAXP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDATAXP to value 0"]
impl crate::Resettable for RXDATAXP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
