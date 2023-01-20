#[doc = "Register `SCANFIFOSTAT` reader"]
pub struct R(crate::R<SCANFIFOSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANFIFOSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANFIFOSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANFIFOSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOREADCNT` reader - FIFO Read Count"]
pub type FIFOREADCNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - FIFO Read Count"]
    #[inline(always)]
    pub fn fiforeadcnt(&self) -> FIFOREADCNT_R {
        FIFOREADCNT_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Scan FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanfifostat](index.html) module"]
pub struct SCANFIFOSTAT_SPEC;
impl crate::RegisterSpec for SCANFIFOSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scanfifostat::R](R) reader structure"]
impl crate::Readable for SCANFIFOSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCANFIFOSTAT to value 0"]
impl crate::Resettable for SCANFIFOSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
