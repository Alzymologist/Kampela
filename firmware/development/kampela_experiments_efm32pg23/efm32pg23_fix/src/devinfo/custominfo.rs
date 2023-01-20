#[doc = "Register `CUSTOMINFO` reader"]
pub struct R(crate::R<CUSTOMINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CUSTOMINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CUSTOMINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CUSTOMINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTNO` reader - Part Number"]
pub type PARTNO_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - Part Number"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Custom information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [custominfo](index.html) module"]
pub struct CUSTOMINFO_SPEC;
impl crate::RegisterSpec for CUSTOMINFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [custominfo::R](R) reader structure"]
impl crate::Readable for CUSTOMINFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CUSTOMINFO to value 0"]
impl crate::Resettable for CUSTOMINFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
