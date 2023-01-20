#[doc = "Register `PCMISSES` reader"]
pub struct R(crate::R<PCMISSES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCMISSES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCMISSES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCMISSES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PCMISSES` reader - Performance Counter Misses"]
pub type PCMISSES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Performance Counter Misses"]
    #[inline(always)]
    pub fn pcmisses(&self) -> PCMISSES_R {
        PCMISSES_R::new(self.bits)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmisses](index.html) module"]
pub struct PCMISSES_SPEC;
impl crate::RegisterSpec for PCMISSES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcmisses::R](R) reader structure"]
impl crate::Readable for PCMISSES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCMISSES to value 0"]
impl crate::Resettable for PCMISSES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
