#[doc = "Register `IADC0GAIN0` reader"]
pub struct R(crate::R<IADC0GAIN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IADC0GAIN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IADC0GAIN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IADC0GAIN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GAINCANA1` reader - No Description"]
pub type GAINCANA1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GAINCANA2` reader - No Description"]
pub type GAINCANA2_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn gaincana1(&self) -> GAINCANA1_R {
        GAINCANA1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - No Description"]
    #[inline(always)]
    pub fn gaincana2(&self) -> GAINCANA2_R {
        GAINCANA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "IADC0 Gain Calibration Info\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iadc0gain0](index.html) module"]
pub struct IADC0GAIN0_SPEC;
impl crate::RegisterSpec for IADC0GAIN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iadc0gain0::R](R) reader structure"]
impl crate::Readable for IADC0GAIN0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IADC0GAIN0 to value 0"]
impl crate::Resettable for IADC0GAIN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
