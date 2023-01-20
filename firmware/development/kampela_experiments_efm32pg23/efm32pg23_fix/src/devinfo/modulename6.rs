#[doc = "Register `MODULENAME6` reader"]
pub struct R(crate::R<MODULENAME6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODULENAME6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODULENAME6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODULENAME6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MODCHAR25` reader - No Description"]
pub type MODCHAR25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCHAR26` reader - No Description"]
pub type MODCHAR26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSV` reader - No Description"]
pub type RSV_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar25(&self) -> MODCHAR25_R {
        MODCHAR25_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar26(&self) -> MODCHAR26_R {
        MODCHAR26_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - No Description"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Characters 25-26 of Module Name stored as a null terminated string\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modulename6](index.html) module"]
pub struct MODULENAME6_SPEC;
impl crate::RegisterSpec for MODULENAME6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modulename6::R](R) reader structure"]
impl crate::Readable for MODULENAME6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODULENAME6 to value 0xffff_ffff"]
impl crate::Resettable for MODULENAME6_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
