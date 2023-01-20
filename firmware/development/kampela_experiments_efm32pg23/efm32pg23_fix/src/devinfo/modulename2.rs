#[doc = "Register `MODULENAME2` reader"]
pub struct R(crate::R<MODULENAME2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODULENAME2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODULENAME2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODULENAME2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MODCHAR9` reader - No Description"]
pub type MODCHAR9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR10` reader - No Description"]
pub type MODCHAR10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR11` reader - No Description"]
pub type MODCHAR11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR12` reader - No Description"]
pub type MODCHAR12_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar9(&self) -> MODCHAR9_R {
        MODCHAR9_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar10(&self) -> MODCHAR10_R {
        MODCHAR10_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - No Description"]
    #[inline(always)]
    pub fn modchar11(&self) -> MODCHAR11_R {
        MODCHAR11_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - No Description"]
    #[inline(always)]
    pub fn modchar12(&self) -> MODCHAR12_R {
        MODCHAR12_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Characters 9-12 of Module Name stored as a null terminated string\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modulename2](index.html) module"]
pub struct MODULENAME2_SPEC;
impl crate::RegisterSpec for MODULENAME2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modulename2::R](R) reader structure"]
impl crate::Readable for MODULENAME2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODULENAME2 to value 0xffff_ffff"]
impl crate::Resettable for MODULENAME2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
