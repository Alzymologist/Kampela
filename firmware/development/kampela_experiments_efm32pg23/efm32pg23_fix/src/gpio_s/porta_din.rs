#[doc = "Register `PORTA_DIN` reader"]
pub struct R(crate::R<PORTA_DIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTA_DIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTA_DIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTA_DIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN` reader - Data input"]
pub type DIN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Data input"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "data in\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [porta_din](index.html) module"]
pub struct PORTA_DIN_SPEC;
impl crate::RegisterSpec for PORTA_DIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [porta_din::R](R) reader structure"]
impl crate::Readable for PORTA_DIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PORTA_DIN to value 0"]
impl crate::Resettable for PORTA_DIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
