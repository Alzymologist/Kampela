#[doc = "Register `CC1_ICF` reader"]
pub struct R(crate::R<CC1_ICF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC1_ICF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC1_ICF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC1_ICF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ICF` reader - Input Capture FIFO"]
pub type ICF_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input Capture FIFO"]
    #[inline(always)]
    pub fn icf(&self) -> ICF_R {
        ICF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc1_icf](index.html) module"]
pub struct CC1_ICF_SPEC;
impl crate::RegisterSpec for CC1_ICF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc1_icf::R](R) reader structure"]
impl crate::Readable for CC1_ICF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CC1_ICF to value 0"]
impl crate::Resettable for CC1_ICF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
