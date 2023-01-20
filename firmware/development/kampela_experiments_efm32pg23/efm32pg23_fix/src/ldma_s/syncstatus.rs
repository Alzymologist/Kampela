#[doc = "Register `SYNCSTATUS` reader"]
pub struct R(crate::R<SYNCSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYNCTRIG` reader - sync trig status"]
pub type SYNCTRIG_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - sync trig status"]
    #[inline(always)]
    pub fn synctrig(&self) -> SYNCTRIG_R {
        SYNCTRIG_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncstatus](index.html) module"]
pub struct SYNCSTATUS_SPEC;
impl crate::RegisterSpec for SYNCSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncstatus::R](R) reader structure"]
impl crate::Readable for SYNCSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCSTATUS to value 0"]
impl crate::Resettable for SYNCSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
