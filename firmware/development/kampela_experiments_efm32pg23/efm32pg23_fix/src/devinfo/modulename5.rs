#[doc = "Register `MODULENAME5` reader"]
pub struct R(crate::R<MODULENAME5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODULENAME5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODULENAME5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODULENAME5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MODCHAR21` reader - No Description"]
pub type MODCHAR21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR22` reader - No Description"]
pub type MODCHAR22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR23` reader - No Description"]
pub type MODCHAR23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR24` reader - No Description"]
pub type MODCHAR24_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar21(&self) -> MODCHAR21_R {
        MODCHAR21_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar22(&self) -> MODCHAR22_R {
        MODCHAR22_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - No Description"]
    #[inline(always)]
    pub fn modchar23(&self) -> MODCHAR23_R {
        MODCHAR23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - No Description"]
    #[inline(always)]
    pub fn modchar24(&self) -> MODCHAR24_R {
        MODCHAR24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Characters 21-24 of Module Name stored as a null terminated string\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modulename5](index.html) module"]
pub struct MODULENAME5_SPEC;
impl crate::RegisterSpec for MODULENAME5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modulename5::R](R) reader structure"]
impl crate::Readable for MODULENAME5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODULENAME5 to value 0xffff_ffff"]
impl crate::Resettable for MODULENAME5_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
