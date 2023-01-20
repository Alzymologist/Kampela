#[doc = "Register `HFRCOEM23CAL17` reader"]
pub struct R(crate::R<HFRCOEM23CAL17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFRCOEM23CAL17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFRCOEM23CAL17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFRCOEM23CAL17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TUNING` reader - No Description"]
pub type TUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FINETUNING` reader - No Description"]
pub type FINETUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LDOHP` reader - No Description"]
pub type LDOHP_R = crate::BitReader<bool>;
#[doc = "Field `FREQRANGE` reader - No Description"]
pub type FREQRANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPBIAS` reader - No Description"]
pub type CMPBIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` reader - No Description"]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPSEL` reader - No Description"]
pub type CMPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IREFTC` reader - No Description"]
pub type IREFTC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - No Description"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - No Description"]
    #[inline(always)]
    pub fn finetuning(&self) -> FINETUNING_R {
        FINETUNING_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - No Description"]
    #[inline(always)]
    pub fn ldohp(&self) -> LDOHP_R {
        LDOHP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - No Description"]
    #[inline(always)]
    pub fn freqrange(&self) -> FREQRANGE_R {
        FREQRANGE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - No Description"]
    #[inline(always)]
    pub fn cmpbias(&self) -> CMPBIAS_R {
        CMPBIAS_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:25 - No Description"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - No Description"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CMPSEL_R {
        CMPSEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - No Description"]
    #[inline(always)]
    pub fn ireftc(&self) -> IREFTC_R {
        IREFTC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "HFRCOEM23 Calibration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfrcoem23cal17](index.html) module"]
pub struct HFRCOEM23CAL17_SPEC;
impl crate::RegisterSpec for HFRCOEM23CAL17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfrcoem23cal17::R](R) reader structure"]
impl crate::Readable for HFRCOEM23CAL17_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFRCOEM23CAL17 to value 0"]
impl crate::Resettable for HFRCOEM23CAL17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
