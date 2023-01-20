#[doc = "Register `RXDOUBLEX` reader"]
pub struct R(crate::R<RXDOUBLEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDOUBLEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDOUBLEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDOUBLEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA0` reader - RX Data 0"]
pub type RXDATA0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERR0` reader - Data Parity Error 0"]
pub type PERR0_R = crate::BitReader<bool>;
#[doc = "Field `FERR0` reader - Data Framing Error 0"]
pub type FERR0_R = crate::BitReader<bool>;
#[doc = "Field `RXDATA1` reader - RX Data 1"]
pub type RXDATA1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERR1` reader - Data Parity Error 1"]
pub type PERR1_R = crate::BitReader<bool>;
#[doc = "Field `FERR1` reader - Data Framing Error 1"]
pub type FERR1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:8 - RX Data 0"]
    #[inline(always)]
    pub fn rxdata0(&self) -> RXDATA0_R {
        RXDATA0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Data Parity Error 0"]
    #[inline(always)]
    pub fn perr0(&self) -> PERR0_R {
        PERR0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Framing Error 0"]
    #[inline(always)]
    pub fn ferr0(&self) -> FERR0_R {
        FERR0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - RX Data 1"]
    #[inline(always)]
    pub fn rxdata1(&self) -> RXDATA1_R {
        RXDATA1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Data Parity Error 1"]
    #[inline(always)]
    pub fn perr1(&self) -> PERR1_R {
        PERR1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data Framing Error 1"]
    #[inline(always)]
    pub fn ferr1(&self) -> FERR1_R {
        FERR1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdoublex](index.html) module"]
pub struct RXDOUBLEX_SPEC;
impl crate::RegisterSpec for RXDOUBLEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdoublex::R](R) reader structure"]
impl crate::Readable for RXDOUBLEX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDOUBLEX to value 0"]
impl crate::Resettable for RXDOUBLEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
