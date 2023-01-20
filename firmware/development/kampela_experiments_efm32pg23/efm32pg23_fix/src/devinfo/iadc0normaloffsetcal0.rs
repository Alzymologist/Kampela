#[doc = "Register `IADC0NORMALOFFSETCAL0` reader"]
pub struct R(crate::R<IADC0NORMALOFFSETCAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IADC0NORMALOFFSETCAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IADC0NORMALOFFSETCAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IADC0NORMALOFFSETCAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFFSETANA1NORM` reader - No Description"]
pub type OFFSETANA1NORM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSETANA2NORM` reader - No Description"]
pub type OFFSETANA2NORM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn offsetana1norm(&self) -> OFFSETANA1NORM_R {
        OFFSETANA1NORM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - No Description"]
    #[inline(always)]
    pub fn offsetana2norm(&self) -> OFFSETANA2NORM_R {
        OFFSETANA2NORM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "IADC0 Normal Offset Calibration Info\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iadc0normaloffsetcal0](index.html) module"]
pub struct IADC0NORMALOFFSETCAL0_SPEC;
impl crate::RegisterSpec for IADC0NORMALOFFSETCAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iadc0normaloffsetcal0::R](R) reader structure"]
impl crate::Readable for IADC0NORMALOFFSETCAL0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IADC0NORMALOFFSETCAL0 to value 0"]
impl crate::Resettable for IADC0NORMALOFFSETCAL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
