#[doc = "Register `MODULENAME4` reader"]
pub struct R(crate::R<MODULENAME4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODULENAME4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODULENAME4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODULENAME4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MODCHAR17` reader - No Description"]
pub type MODCHAR17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR18` reader - No Description"]
pub type MODCHAR18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR19` reader - No Description"]
pub type MODCHAR19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR20` reader - No Description"]
pub type MODCHAR20_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar17(&self) -> MODCHAR17_R {
        MODCHAR17_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar18(&self) -> MODCHAR18_R {
        MODCHAR18_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - No Description"]
    #[inline(always)]
    pub fn modchar19(&self) -> MODCHAR19_R {
        MODCHAR19_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - No Description"]
    #[inline(always)]
    pub fn modchar20(&self) -> MODCHAR20_R {
        MODCHAR20_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Characters 17-20 of Module Name stored as a null terminated string\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modulename4](index.html) module"]
pub struct MODULENAME4_SPEC;
impl crate::RegisterSpec for MODULENAME4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modulename4::R](R) reader structure"]
impl crate::Readable for MODULENAME4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODULENAME4 to value 0xffff_ffff"]
impl crate::Resettable for MODULENAME4_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
