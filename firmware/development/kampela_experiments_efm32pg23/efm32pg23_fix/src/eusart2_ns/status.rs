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
#[doc = "Field `RXENS` reader - Receiver Enable Status"]
pub type RXENS_R = crate::BitReader<bool>;
#[doc = "Field `TXENS` reader - Transmitter Enable Status"]
pub type TXENS_R = crate::BitReader<bool>;
#[doc = "Field `RXBLOCK` reader - Block Incoming Data"]
pub type RXBLOCK_R = crate::BitReader<bool>;
#[doc = "Field `TXTRI` reader - Transmitter Tristated"]
pub type TXTRI_R = crate::BitReader<bool>;
#[doc = "Field `TXC` reader - TX Complete"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXFL` reader - TX FIFO Level"]
pub type TXFL_R = crate::BitReader<bool>;
#[doc = "Field `RXFL` reader - RX FIFO Level"]
pub type RXFL_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` reader - RX FIFO Full"]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXIDLE` reader - RX Idle"]
pub type RXIDLE_R = crate::BitReader<bool>;
#[doc = "Field `TXIDLE` reader - TX Idle"]
pub type TXIDLE_R = crate::BitReader<bool>;
#[doc = "Field `TXFCNT` reader - Valid entries in TX FIFO"]
pub type TXFCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUTOBAUDDONE` reader - Auto Baud Rate Detection Completed"]
pub type AUTOBAUDDONE_R = crate::BitReader<bool>;
#[doc = "Field `CLEARTXBUSY` reader - TX FIFO Clear Busy"]
pub type CLEARTXBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Receiver Enable Status"]
    #[inline(always)]
    pub fn rxens(&self) -> RXENS_R {
        RXENS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Enable Status"]
    #[inline(always)]
    pub fn txens(&self) -> TXENS_R {
        TXENS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Block Incoming Data"]
    #[inline(always)]
    pub fn rxblock(&self) -> RXBLOCK_R {
        RXBLOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmitter Tristated"]
    #[inline(always)]
    pub fn txtri(&self) -> TXTRI_R {
        TXTRI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX FIFO Level"]
    #[inline(always)]
    pub fn txfl(&self) -> TXFL_R {
        TXFL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX FIFO Level"]
    #[inline(always)]
    pub fn rxfl(&self) -> RXFL_R {
        RXFL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - RX Idle"]
    #[inline(always)]
    pub fn rxidle(&self) -> RXIDLE_R {
        RXIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX Idle"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Valid entries in TX FIFO"]
    #[inline(always)]
    pub fn txfcnt(&self) -> TXFCNT_R {
        TXFCNT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Auto Baud Rate Detection Completed"]
    #[inline(always)]
    pub fn autobauddone(&self) -> AUTOBAUDDONE_R {
        AUTOBAUDDONE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TX FIFO Clear Busy"]
    #[inline(always)]
    pub fn cleartxbusy(&self) -> CLEARTXBUSY_R {
        CLEARTXBUSY_R::new(((self.bits >> 25) & 1) != 0)
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
#[doc = "`reset()` method sets STATUS to value 0x3040"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x3040;
}
