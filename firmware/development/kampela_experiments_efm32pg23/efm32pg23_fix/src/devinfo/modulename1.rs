#[doc = "Register `MODULENAME1` reader"]
pub struct R(crate::R<MODULENAME1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODULENAME1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODULENAME1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODULENAME1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MODCHAR5` reader - No Description"]
pub type MODCHAR5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR6` reader - No Description"]
pub type MODCHAR6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR7` reader - No Description"]
pub type MODCHAR7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR8` reader - No Description"]
pub type MODCHAR8_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar5(&self) -> MODCHAR5_R {
        MODCHAR5_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar6(&self) -> MODCHAR6_R {
        MODCHAR6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - No Description"]
    #[inline(always)]
    pub fn modchar7(&self) -> MODCHAR7_R {
        MODCHAR7_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - No Description"]
    #[inline(always)]
    pub fn modchar8(&self) -> MODCHAR8_R {
        MODCHAR8_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Characters 5-8 of Module Name stored as a null terminated string\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modulename1](index.html) module"]
pub struct MODULENAME1_SPEC;
impl crate::RegisterSpec for MODULENAME1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modulename1::R](R) reader structure"]
impl crate::Readable for MODULENAME1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODULENAME1 to value 0xffff_ffff"]
impl crate::Resettable for MODULENAME1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
