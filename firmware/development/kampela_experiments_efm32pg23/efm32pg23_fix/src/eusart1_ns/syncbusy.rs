#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIV` reader - SYNCBUSY for DIV in CLKDIV"]
pub type DIV_R = crate::BitReader<bool>;
#[doc = "Field `RXTEN` reader - SYNCBUSY for RXTEN in TRIGCTRL"]
pub type RXTEN_R = crate::BitReader<bool>;
#[doc = "Field `TXTEN` reader - SYNCBUSY for TXTEN in TRIGCTRL"]
pub type TXTEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` reader - SYNCBUSY for RXEN in CMD"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXDIS` reader - SYNCBUSY for RXDIS in CMD"]
pub type RXDIS_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` reader - SYNCBUSY for TXEN in CMD"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXDIS` reader - SYNCBUSY for TXDIS in CMD"]
pub type TXDIS_R = crate::BitReader<bool>;
#[doc = "Field `RXBLOCKEN` reader - SYNCBUSY for RXBLOCKEN in CMD"]
pub type RXBLOCKEN_R = crate::BitReader<bool>;
#[doc = "Field `RXBLOCKDIS` reader - SYNCBUSY for RXBLOCKDIS in CMD"]
pub type RXBLOCKDIS_R = crate::BitReader<bool>;
#[doc = "Field `TXTRIEN` reader - SYNCBUSY for TXTRIEN in CMD"]
pub type TXTRIEN_R = crate::BitReader<bool>;
#[doc = "Field `TXTRIDIS` reader - SYNCBUSY in TXTRIDIS in CMD"]
pub type TXTRIDIS_R = crate::BitReader<bool>;
#[doc = "Field `AUTOTXTEN` reader - SYNCBUSY for AUTOTXTEN in TRIGCTRL"]
pub type AUTOTXTEN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SYNCBUSY for DIV in CLKDIV"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNCBUSY for RXTEN in TRIGCTRL"]
    #[inline(always)]
    pub fn rxten(&self) -> RXTEN_R {
        RXTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYNCBUSY for TXTEN in TRIGCTRL"]
    #[inline(always)]
    pub fn txten(&self) -> TXTEN_R {
        TXTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SYNCBUSY for RXEN in CMD"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SYNCBUSY for RXDIS in CMD"]
    #[inline(always)]
    pub fn rxdis(&self) -> RXDIS_R {
        RXDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYNCBUSY for TXEN in CMD"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SYNCBUSY for TXDIS in CMD"]
    #[inline(always)]
    pub fn txdis(&self) -> TXDIS_R {
        TXDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SYNCBUSY for RXBLOCKEN in CMD"]
    #[inline(always)]
    pub fn rxblocken(&self) -> RXBLOCKEN_R {
        RXBLOCKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SYNCBUSY for RXBLOCKDIS in CMD"]
    #[inline(always)]
    pub fn rxblockdis(&self) -> RXBLOCKDIS_R {
        RXBLOCKDIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SYNCBUSY for TXTRIEN in CMD"]
    #[inline(always)]
    pub fn txtrien(&self) -> TXTRIEN_R {
        TXTRIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SYNCBUSY in TXTRIDIS in CMD"]
    #[inline(always)]
    pub fn txtridis(&self) -> TXTRIDIS_R {
        TXTRIDIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SYNCBUSY for AUTOTXTEN in TRIGCTRL"]
    #[inline(always)]
    pub fn autotxten(&self) -> AUTOTXTEN_R {
        AUTOTXTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
