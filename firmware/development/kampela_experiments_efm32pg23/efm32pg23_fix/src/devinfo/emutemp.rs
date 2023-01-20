#[doc = "Register `EMUTEMP` reader"]
pub struct R(crate::R<EMUTEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMUTEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMUTEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMUTEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EMUTEMPROOM` reader - Emu Room Temperature"]
pub type EMUTEMPROOM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 2:10 - Emu Room Temperature"]
    #[inline(always)]
    pub fn emutemproom(&self) -> EMUTEMPROOM_R {
        EMUTEMPROOM_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
}
#[doc = "EMU Temperature Sensor Calibration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emutemp](index.html) module"]
pub struct EMUTEMP_SPEC;
impl crate::RegisterSpec for EMUTEMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emutemp::R](R) reader structure"]
impl crate::Readable for EMUTEMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EMUTEMP to value 0"]
impl crate::Resettable for EMUTEMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
