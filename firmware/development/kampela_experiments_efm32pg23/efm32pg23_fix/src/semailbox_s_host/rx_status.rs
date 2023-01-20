#[doc = "Register `RX_STATUS` reader"]
pub struct R(crate::R<RX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REMBYTES` reader - REMBYTES"]
pub type REMBYTES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MSGINFO` reader - MSGINFO"]
pub type MSGINFO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXINT` reader - RXINT"]
pub type RXINT_R = crate::BitReader<bool>;
#[doc = "Field `RXEMPTY` reader - RXEMPTY"]
pub type RXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RXHDR` reader - RXHDR"]
pub type RXHDR_R = crate::BitReader<bool>;
#[doc = "Field `RXERROR` reader - RXERROR"]
pub type RXERROR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - REMBYTES"]
    #[inline(always)]
    pub fn rembytes(&self) -> REMBYTES_R {
        REMBYTES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - MSGINFO"]
    #[inline(always)]
    pub fn msginfo(&self) -> MSGINFO_R {
        MSGINFO_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - RXINT"]
    #[inline(always)]
    pub fn rxint(&self) -> RXINT_R {
        RXINT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RXEMPTY"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - RXHDR"]
    #[inline(always)]
    pub fn rxhdr(&self) -> RXHDR_R {
        RXHDR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RXERROR"]
    #[inline(always)]
    pub fn rxerror(&self) -> RXERROR_R {
        RXERROR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "RX Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_status](index.html) module"]
pub struct RX_STATUS_SPEC;
impl crate::RegisterSpec for RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_status::R](R) reader structure"]
impl crate::Readable for RX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_STATUS to value 0"]
impl crate::Resettable for RX_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
