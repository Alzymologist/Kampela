#[doc = "Register `IADC0GAIN1` reader"]
pub struct R(crate::R<IADC0GAIN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IADC0GAIN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IADC0GAIN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IADC0GAIN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GAINCANA3` reader - No Description"]
pub type GAINCANA3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GAINCANA4` reader - No Description"]
pub type GAINCANA4_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn gaincana3(&self) -> GAINCANA3_R {
        GAINCANA3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - No Description"]
    #[inline(always)]
    pub fn gaincana4(&self) -> GAINCANA4_R {
        GAINCANA4_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "IADC0 Gain Calibration Info\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iadc0gain1](index.html) module"]
pub struct IADC0GAIN1_SPEC;
impl crate::RegisterSpec for IADC0GAIN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iadc0gain1::R](R) reader structure"]
impl crate::Readable for IADC0GAIN1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IADC0GAIN1 to value 0"]
impl crate::Resettable for IADC0GAIN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
