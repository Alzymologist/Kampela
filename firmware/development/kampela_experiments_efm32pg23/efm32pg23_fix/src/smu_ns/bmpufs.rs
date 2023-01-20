#[doc = "Register `BMPUFS` reader"]
pub struct R(crate::R<BMPUFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMPUFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMPUFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMPUFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BMPUFSMASTERID` reader - Bus Manager ID"]
pub type BMPUFSMASTERID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Bus Manager ID"]
    #[inline(always)]
    pub fn bmpufsmasterid(&self) -> BMPUFSMASTERID_R {
        BMPUFSMASTERID_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmpufs](index.html) module"]
pub struct BMPUFS_SPEC;
impl crate::RegisterSpec for BMPUFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmpufs::R](R) reader structure"]
impl crate::Readable for BMPUFS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BMPUFS to value 0"]
impl crate::Resettable for BMPUFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
