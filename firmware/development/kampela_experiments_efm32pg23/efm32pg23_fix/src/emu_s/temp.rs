#[doc = "Register `TEMP` reader"]
pub struct R(crate::R<TEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEMPLSB` reader - Temperature measured decimal part"]
pub type TEMPLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEMP` reader - Temperature measured"]
pub type TEMP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TEMPAVG` reader - Averaged Temperature"]
pub type TEMPAVG_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1 - Temperature measured decimal part"]
    #[inline(always)]
    pub fn templsb(&self) -> TEMPLSB_R {
        TEMPLSB_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:10 - Temperature measured"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:26 - Averaged Temperature"]
    #[inline(always)]
    pub fn tempavg(&self) -> TEMPAVG_R {
        TEMPAVG_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp](index.html) module"]
pub struct TEMP_SPEC;
impl crate::RegisterSpec for TEMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [temp::R](R) reader structure"]
impl crate::Readable for TEMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TEMP to value 0"]
impl crate::Resettable for TEMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
