#[doc = "Register `CHBUSY` reader"]
pub struct R(crate::R<CHBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - Channels Busy"]
pub type BUSY_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Channels Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chbusy](index.html) module"]
pub struct CHBUSY_SPEC;
impl crate::RegisterSpec for CHBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chbusy::R](R) reader structure"]
impl crate::Readable for CHBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHBUSY to value 0"]
impl crate::Resettable for CHBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
