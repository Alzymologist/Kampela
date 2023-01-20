#[doc = "Register `PPUFS` reader"]
pub struct R(crate::R<PPUFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPUFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPUFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPUFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PPUFSPERIPHID` reader - Peripheral ID"]
pub type PPUFSPERIPHID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID"]
    #[inline(always)]
    pub fn ppufsperiphid(&self) -> PPUFSPERIPHID_R {
        PPUFSPERIPHID_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppufs](index.html) module"]
pub struct PPUFS_SPEC;
impl crate::RegisterSpec for PPUFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppufs::R](R) reader structure"]
impl crate::Readable for PPUFS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PPUFS to value 0"]
impl crate::Resettable for PPUFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
