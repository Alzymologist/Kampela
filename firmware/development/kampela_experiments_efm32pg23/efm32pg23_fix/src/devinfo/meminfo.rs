#[doc = "Register `MEMINFO` reader"]
pub struct R(crate::R<MEMINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLASHPAGESIZE` reader - Flash Page Size"]
pub type FLASHPAGESIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDPAGESIZE` reader - User Data Page Size"]
pub type UDPAGESIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DILEN` reader - Length of DI Page"]
pub type DILEN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Flash Page Size"]
    #[inline(always)]
    pub fn flashpagesize(&self) -> FLASHPAGESIZE_R {
        FLASHPAGESIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - User Data Page Size"]
    #[inline(always)]
    pub fn udpagesize(&self) -> UDPAGESIZE_R {
        UDPAGESIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Length of DI Page"]
    #[inline(always)]
    pub fn dilen(&self) -> DILEN_R {
        DILEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Flash page size and misc. chip information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [meminfo](index.html) module"]
pub struct MEMINFO_SPEC;
impl crate::RegisterSpec for MEMINFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [meminfo::R](R) reader structure"]
impl crate::Readable for MEMINFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEMINFO to value 0"]
impl crate::Resettable for MEMINFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
