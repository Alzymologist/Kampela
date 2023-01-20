#[doc = "Register `MSIZE` reader"]
pub struct R(crate::R<MSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLASH` reader - Flash Size"]
pub type FLASH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SRAM` reader - Sram Size"]
pub type SRAM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:26 - Sram Size"]
    #[inline(always)]
    pub fn sram(&self) -> SRAM_R {
        SRAM_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[doc = "Flash and SRAM Memory size in kB\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msize](index.html) module"]
pub struct MSIZE_SPEC;
impl crate::RegisterSpec for MSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msize::R](R) reader structure"]
impl crate::Readable for MSIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSIZE to value 0"]
impl crate::Resettable for MSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
