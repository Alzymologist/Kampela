#[doc = "Register `INFO` reader"]
pub struct R(crate::R<INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRC` reader - CRC"]
pub type CRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRODREV` reader - Production Revision"]
pub type PRODREV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEVINFOREV` reader - DI Page Version"]
pub type DEVINFOREV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - CRC"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Production Revision"]
    #[inline(always)]
    pub fn prodrev(&self) -> PRODREV_R {
        PRODREV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DI Page Version"]
    #[inline(always)]
    pub fn devinforev(&self) -> DEVINFOREV_R {
        DEVINFOREV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Version of the device info structure being used\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [info](index.html) module"]
pub struct INFO_SPEC;
impl crate::RegisterSpec for INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [info::R](R) reader structure"]
impl crate::Readable for INFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INFO to value 0x0c00_0000"]
impl crate::Resettable for INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_0000;
}
