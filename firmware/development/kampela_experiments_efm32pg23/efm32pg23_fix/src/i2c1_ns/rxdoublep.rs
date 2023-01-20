#[doc = "Register `RXDOUBLEP` reader"]
pub struct R(crate::R<RXDOUBLEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDOUBLEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDOUBLEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDOUBLEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATAP0` reader - RX Data 0 Peek"]
pub type RXDATAP0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXDATAP1` reader - RX Data 1 Peek"]
pub type RXDATAP1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX Data 0 Peek"]
    #[inline(always)]
    pub fn rxdatap0(&self) -> RXDATAP0_R {
        RXDATAP0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Data 1 Peek"]
    #[inline(always)]
    pub fn rxdatap1(&self) -> RXDATAP1_R {
        RXDATAP1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdoublep](index.html) module"]
pub struct RXDOUBLEP_SPEC;
impl crate::RegisterSpec for RXDOUBLEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdoublep::R](R) reader structure"]
impl crate::Readable for RXDOUBLEP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDOUBLEP to value 0"]
impl crate::Resettable for RXDOUBLEP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
