#[doc = "Register `MODULENAME3` reader"]
pub struct R(crate::R<MODULENAME3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODULENAME3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODULENAME3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODULENAME3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MODCHAR13` reader - No Description"]
pub type MODCHAR13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR14` reader - No Description"]
pub type MODCHAR14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR15` reader - No Description"]
pub type MODCHAR15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR16` reader - No Description"]
pub type MODCHAR16_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar13(&self) -> MODCHAR13_R {
        MODCHAR13_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar14(&self) -> MODCHAR14_R {
        MODCHAR14_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - No Description"]
    #[inline(always)]
    pub fn modchar15(&self) -> MODCHAR15_R {
        MODCHAR15_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - No Description"]
    #[inline(always)]
    pub fn modchar16(&self) -> MODCHAR16_R {
        MODCHAR16_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Characters 13-16 of Module Name stored as a null terminated string\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modulename3](index.html) module"]
pub struct MODULENAME3_SPEC;
impl crate::RegisterSpec for MODULENAME3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modulename3::R](R) reader structure"]
impl crate::Readable for MODULENAME3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODULENAME3 to value 0xffff_ffff"]
impl crate::Resettable for MODULENAME3_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
