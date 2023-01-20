#[doc = "Register `EUI48L` reader"]
pub struct R(crate::R<EUI48L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EUI48L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EUI48L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EUI48L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNIQUEID` reader - Unique ID"]
pub type UNIQUEID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OUI48L` reader - OUI48L"]
pub type OUI48L_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:23 - Unique ID"]
    #[inline(always)]
    pub fn uniqueid(&self) -> UNIQUEID_R {
        UNIQUEID_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - OUI48L"]
    #[inline(always)]
    pub fn oui48l(&self) -> OUI48L_R {
        OUI48L_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "MA-L compliant EUI48 OUI (low bits) and Unique Identifier (24-bit)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eui48l](index.html) module"]
pub struct EUI48L_SPEC;
impl crate::RegisterSpec for EUI48L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eui48l::R](R) reader structure"]
impl crate::Readable for EUI48L_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EUI48L to value 0"]
impl crate::Resettable for EUI48L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
