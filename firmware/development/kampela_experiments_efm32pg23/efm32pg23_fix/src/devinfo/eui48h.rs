#[doc = "Register `EUI48H` reader"]
pub struct R(crate::R<EUI48H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EUI48H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EUI48H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EUI48H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUI48H` reader - OUI48H"]
pub type OUI48H_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - OUI48H"]
    #[inline(always)]
    pub fn oui48h(&self) -> OUI48H_R {
        OUI48H_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MA-L compliant EUI48 OUI (high bits)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eui48h](index.html) module"]
pub struct EUI48H_SPEC;
impl crate::RegisterSpec for EUI48H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eui48h::R](R) reader structure"]
impl crate::Readable for EUI48H_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EUI48H to value 0xffff_0000"]
impl crate::Resettable for EUI48H_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
