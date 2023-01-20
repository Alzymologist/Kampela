#[doc = "Register `IADC0OFFSETCAL0` reader"]
pub struct R(crate::R<IADC0OFFSETCAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IADC0OFFSETCAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IADC0OFFSETCAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IADC0OFFSETCAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFFSETANABASE` reader - No Description"]
pub type OFFSETANABASE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSETANA1HIACC` reader - No Description"]
pub type OFFSETANA1HIACC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn offsetanabase(&self) -> OFFSETANABASE_R {
        OFFSETANABASE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - No Description"]
    #[inline(always)]
    pub fn offsetana1hiacc(&self) -> OFFSETANA1HIACC_R {
        OFFSETANA1HIACC_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "IADC0 Offset Calibration Info\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iadc0offsetcal0](index.html) module"]
pub struct IADC0OFFSETCAL0_SPEC;
impl crate::RegisterSpec for IADC0OFFSETCAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iadc0offsetcal0::R](R) reader structure"]
impl crate::Readable for IADC0OFFSETCAL0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IADC0OFFSETCAL0 to value 0"]
impl crate::Resettable for IADC0OFFSETCAL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
