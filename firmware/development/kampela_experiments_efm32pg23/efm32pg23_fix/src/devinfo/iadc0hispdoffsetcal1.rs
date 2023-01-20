#[doc = "Register `IADC0HISPDOFFSETCAL1` reader"]
pub struct R(crate::R<IADC0HISPDOFFSETCAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IADC0HISPDOFFSETCAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IADC0HISPDOFFSETCAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IADC0HISPDOFFSETCAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFFSETANA3HISPD` reader - No Description"]
pub type OFFSETANA3HISPD_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn offsetana3hispd(&self) -> OFFSETANA3HISPD_R {
        OFFSETANA3HISPD_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "IADC High Speed Offset Calibration Info\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iadc0hispdoffsetcal1](index.html) module"]
pub struct IADC0HISPDOFFSETCAL1_SPEC;
impl crate::RegisterSpec for IADC0HISPDOFFSETCAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iadc0hispdoffsetcal1::R](R) reader structure"]
impl crate::Readable for IADC0HISPDOFFSETCAL1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IADC0HISPDOFFSETCAL1 to value 0"]
impl crate::Resettable for IADC0HISPDOFFSETCAL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
