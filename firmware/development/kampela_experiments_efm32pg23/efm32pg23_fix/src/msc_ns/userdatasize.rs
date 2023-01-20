#[doc = "Register `USERDATASIZE` reader"]
pub struct R(crate::R<USERDATASIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USERDATASIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USERDATASIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USERDATASIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USERDATASIZE` reader - User Data Size"]
pub type USERDATASIZE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - User Data Size"]
    #[inline(always)]
    pub fn userdatasize(&self) -> USERDATASIZE_R {
        USERDATASIZE_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userdatasize](index.html) module"]
pub struct USERDATASIZE_SPEC;
impl crate::RegisterSpec for USERDATASIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [userdatasize::R](R) reader structure"]
impl crate::Readable for USERDATASIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USERDATASIZE to value 0x04"]
impl crate::Resettable for USERDATASIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
