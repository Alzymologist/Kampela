#[doc = "Register `PORTD_DIN` reader"]
pub struct R(crate::R<PORTD_DIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTD_DIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTD_DIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTD_DIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN` reader - Data input"]
pub type DIN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Data input"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "data in\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portd_din](index.html) module"]
pub struct PORTD_DIN_SPEC;
impl crate::RegisterSpec for PORTD_DIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portd_din::R](R) reader structure"]
impl crate::Readable for PORTD_DIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PORTD_DIN to value 0"]
impl crate::Resettable for PORTD_DIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
