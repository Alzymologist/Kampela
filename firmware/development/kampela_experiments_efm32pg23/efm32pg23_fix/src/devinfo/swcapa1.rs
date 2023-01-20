#[doc = "Register `SWCAPA1` reader"]
pub struct R(crate::R<SWCAPA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWCAPA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWCAPA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWCAPA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFMCUEN` reader - RF-MCU"]
pub type RFMCUEN_R = crate::BitReader<bool>;
#[doc = "Field `NCPEN` reader - NCP"]
pub type NCPEN_R = crate::BitReader<bool>;
#[doc = "Field `GWEN` reader - Gateway"]
pub type GWEN_R = crate::BitReader<bool>;
#[doc = "Field `XOUT` reader - XOUT"]
pub type XOUT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RF-MCU"]
    #[inline(always)]
    pub fn rfmcuen(&self) -> RFMCUEN_R {
        RFMCUEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NCP"]
    #[inline(always)]
    pub fn ncpen(&self) -> NCPEN_R {
        NCPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gateway"]
    #[inline(always)]
    pub fn gwen(&self) -> GWEN_R {
        GWEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XOUT"]
    #[inline(always)]
    pub fn xout(&self) -> XOUT_R {
        XOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Software Capability Vector 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swcapa1](index.html) module"]
pub struct SWCAPA1_SPEC;
impl crate::RegisterSpec for SWCAPA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swcapa1::R](R) reader structure"]
impl crate::Readable for SWCAPA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SWCAPA1 to value 0"]
impl crate::Resettable for SWCAPA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
