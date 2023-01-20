#[doc = "Register `EUI64H` reader"]
pub struct R(crate::R<EUI64H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EUI64H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EUI64H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EUI64H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNIQUEH` reader - UNIQUEH"]
pub type UNIQUEH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUI64` reader - OUI64"]
pub type OUI64_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:7 - UNIQUEH"]
    #[inline(always)]
    pub fn uniqueh(&self) -> UNIQUEH_R {
        UNIQUEH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - OUI64"]
    #[inline(always)]
    pub fn oui64(&self) -> OUI64_R {
        OUI64_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "MA-L compliant EUI64 OUI and Unique Identifier (high bits)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eui64h](index.html) module"]
pub struct EUI64H_SPEC;
impl crate::RegisterSpec for EUI64H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eui64h::R](R) reader structure"]
impl crate::Readable for EUI64H_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EUI64H to value 0"]
impl crate::Resettable for EUI64H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
