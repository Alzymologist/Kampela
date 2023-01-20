#[doc = "Register `TX_STATUS` reader"]
pub struct R(crate::R<TX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REMBYTES` reader - REMBYTES"]
pub type REMBYTES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MSGINFO` reader - MSGINFO"]
pub type MSGINFO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXINT` reader - TXINT"]
pub type TXINT_R = crate::BitReader<bool>;
#[doc = "Field `TXFULL` reader - TXFULL"]
pub type TXFULL_R = crate::BitReader<bool>;
#[doc = "Field `TXERROR` reader - TXERROR"]
pub type TXERROR_R = crate::BitReader<bool>;
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
    #[doc = "Bit 20 - TXINT"]
    #[inline(always)]
    pub fn txint(&self) -> TXINT_R {
        TXINT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TXFULL"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - TXERROR"]
    #[inline(always)]
    pub fn txerror(&self) -> TXERROR_R {
        TXERROR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "TX Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_status](index.html) module"]
pub struct TX_STATUS_SPEC;
impl crate::RegisterSpec for TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_status::R](R) reader structure"]
impl crate::Readable for TX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_STATUS to value 0"]
impl crate::Resettable for TX_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
