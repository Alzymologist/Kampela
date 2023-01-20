#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PSTART` reader - Pending START"]
pub type PSTART_R = crate::BitReader<bool>;
#[doc = "Field `PSTOP` reader - Pending STOP"]
pub type PSTOP_R = crate::BitReader<bool>;
#[doc = "Field `PACK` reader - Pending ACK"]
pub type PACK_R = crate::BitReader<bool>;
#[doc = "Field `PNACK` reader - Pending NACK"]
pub type PNACK_R = crate::BitReader<bool>;
#[doc = "Field `PCONT` reader - Pending continue"]
pub type PCONT_R = crate::BitReader<bool>;
#[doc = "Field `PABORT` reader - Pending abort"]
pub type PABORT_R = crate::BitReader<bool>;
#[doc = "Field `TXC` reader - TX Complete"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXBL` reader - TX Buffer Level"]
pub type TXBL_R = crate::BitReader<bool>;
#[doc = "Field `RXDATAV` reader - RX Data Valid"]
pub type RXDATAV_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` reader - RX FIFO Full"]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `TXBUFCNT` reader - TX Buffer Count"]
pub type TXBUFCNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Pending START"]
    #[inline(always)]
    pub fn pstart(&self) -> PSTART_R {
        PSTART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending STOP"]
    #[inline(always)]
    pub fn pstop(&self) -> PSTOP_R {
        PSTOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending ACK"]
    #[inline(always)]
    pub fn pack(&self) -> PACK_R {
        PACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending NACK"]
    #[inline(always)]
    pub fn pnack(&self) -> PNACK_R {
        PNACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending continue"]
    #[inline(always)]
    pub fn pcont(&self) -> PCONT_R {
        PCONT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending abort"]
    #[inline(always)]
    pub fn pabort(&self) -> PABORT_R {
        PABORT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - TX Buffer Count"]
    #[inline(always)]
    pub fn txbufcnt(&self) -> TXBUFCNT_R {
        TXBUFCNT_R::new(((self.bits >> 10) & 3) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x80"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
