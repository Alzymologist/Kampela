#[doc = "Register `MODULENAME0` reader"]
pub struct R(crate::R<MODULENAME0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODULENAME0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODULENAME0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODULENAME0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MODCHAR1` reader - No Description"]
pub type MODCHAR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR2` reader - No Description"]
pub type MODCHAR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR3` reader - No Description"]
pub type MODCHAR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR4` reader - No Description"]
pub type MODCHAR4_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar1(&self) -> MODCHAR1_R {
        MODCHAR1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar2(&self) -> MODCHAR2_R {
        MODCHAR2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - No Description"]
    #[inline(always)]
    pub fn modchar3(&self) -> MODCHAR3_R {
        MODCHAR3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - No Description"]
    #[inline(always)]
    pub fn modchar4(&self) -> MODCHAR4_R {
        MODCHAR4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Characters 1-4 of Module Name stored as a null terminated string\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modulename0](index.html) module"]
pub struct MODULENAME0_SPEC;
impl crate::RegisterSpec for MODULENAME0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modulename0::R](R) reader structure"]
impl crate::Readable for MODULENAME0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODULENAME0 to value 0xffff_ffff"]
impl crate::Resettable for MODULENAME0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
