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
#[doc = "Field `MASTER` reader - SPI Main Mode"]
pub type MASTER_R = crate::BitReader<bool>;
#[doc = "Field `RXBLOCK` reader - Block Incoming Data"]
pub type RXBLOCK_R = crate::BitReader<bool>;
#[doc = "Field `TXTRI` reader - Transmitter Tristated"]
pub type TXTRI_R = crate::BitReader<bool>;
#[doc = "Field `TXC` reader - TX Complete"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXBL` reader - TX Buffer Level"]
pub type TXBL_R = crate::BitReader<bool>;
#[doc = "Field `RXDATAV` reader - RX Data Valid"]
pub type RXDATAV_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` reader - RX FIFO Full"]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `TXBDRIGHT` reader - TX Buffer Expects Double Right Data"]
pub type TXBDRIGHT_R = crate::BitReader<bool>;
#[doc = "Field `TXBSRIGHT` reader - TX Buffer Expects Single Right Data"]
pub type TXBSRIGHT_R = crate::BitReader<bool>;
#[doc = "Field `RXDATAVRIGHT` reader - RX Data Right"]
pub type RXDATAVRIGHT_R = crate::BitReader<bool>;
#[doc = "Field `RXFULLRIGHT` reader - RX Full of Right Data"]
pub type RXFULLRIGHT_R = crate::BitReader<bool>;
#[doc = "Field `TXIDLE` reader - TX Idle"]
pub type TXIDLE_R = crate::BitReader<bool>;
#[doc = "Field `TIMERRESTARTED` reader - The USART Timer restarted itself"]
pub type TIMERRESTARTED_R = crate::BitReader<bool>;
#[doc = "Field `TXBUFCNT` reader - TX Buffer Count"]
pub type TXBUFCNT_R = crate::FieldReader<u8, u8>;
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
    #[doc = "Bit 2 - SPI Main Mode"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 2) & 1) != 0)
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
    #[doc = "Bit 6 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX Buffer Expects Double Right Data"]
    #[inline(always)]
    pub fn txbdright(&self) -> TXBDRIGHT_R {
        TXBDRIGHT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TX Buffer Expects Single Right Data"]
    #[inline(always)]
    pub fn txbsright(&self) -> TXBSRIGHT_R {
        TXBSRIGHT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX Data Right"]
    #[inline(always)]
    pub fn rxdatavright(&self) -> RXDATAVRIGHT_R {
        RXDATAVRIGHT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX Full of Right Data"]
    #[inline(always)]
    pub fn rxfullright(&self) -> RXFULLRIGHT_R {
        RXFULLRIGHT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX Idle"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The USART Timer restarted itself"]
    #[inline(always)]
    pub fn timerrestarted(&self) -> TIMERRESTARTED_R {
        TIMERRESTARTED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - TX Buffer Count"]
    #[inline(always)]
    pub fn txbufcnt(&self) -> TXBUFCNT_R {
        TXBUFCNT_R::new(((self.bits >> 16) & 3) as u8)
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
#[doc = "`reset()` method sets STATUS to value 0x2040"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x2040;
}
