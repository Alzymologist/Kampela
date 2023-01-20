#[doc = "Register `MODXOCAL` reader"]
pub struct R(crate::R<MODXOCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODXOCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODXOCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODXOCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HFXOCTUNEXIANA` reader - No Description"]
pub type HFXOCTUNEXIANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HFXOCTUNEXOANA` reader - No Description"]
pub type HFXOCTUNEXOANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LFXOCAPTUNE` reader - No Description"]
pub type LFXOCAPTUNE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn hfxoctunexiana(&self) -> HFXOCTUNEXIANA_R {
        HFXOCTUNEXIANA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn hfxoctunexoana(&self) -> HFXOCTUNEXOANA_R {
        HFXOCTUNEXOANA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - No Description"]
    #[inline(always)]
    pub fn lfxocaptune(&self) -> LFXOCAPTUNE_R {
        LFXOCAPTUNE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Module Crystal Oscillator Calibration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modxocal](index.html) module"]
pub struct MODXOCAL_SPEC;
impl crate::RegisterSpec for MODXOCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modxocal::R](R) reader structure"]
impl crate::Readable for MODXOCAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODXOCAL to value 0x007f_ffff"]
impl crate::Resettable for MODXOCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x007f_ffff;
}
