#[doc = "Register `RXDOUBLE` reader"]
pub struct R(crate::R<RXDOUBLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDOUBLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDOUBLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDOUBLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA0` reader - RX Data 0"]
pub type RXDATA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXDATA1` reader - RX Data 1"]
pub type RXDATA1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX Data 0"]
    #[inline(always)]
    pub fn rxdata0(&self) -> RXDATA0_R {
        RXDATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Data 1"]
    #[inline(always)]
    pub fn rxdata1(&self) -> RXDATA1_R {
        RXDATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdouble](index.html) module"]
pub struct RXDOUBLE_SPEC;
impl crate::RegisterSpec for RXDOUBLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdouble::R](R) reader structure"]
impl crate::Readable for RXDOUBLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDOUBLE to value 0"]
impl crate::Resettable for RXDOUBLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
