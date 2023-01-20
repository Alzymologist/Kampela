#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `START` reader - Sync busy for START"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `STOP` reader - Sync busy for STOP"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `PRECNT` reader - Sync busy for PRECNT"]
pub type PRECNT_R = crate::BitReader<bool>;
#[doc = "Field `CNT` reader - Sync busy for CNT"]
pub type CNT_R = crate::BitReader<bool>;
#[doc = "Field `COMP` reader - Sync busy for COMP"]
pub type COMP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Sync busy for START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sync busy for STOP"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sync busy for PRECNT"]
    #[inline(always)]
    pub fn precnt(&self) -> PRECNT_R {
        PRECNT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sync busy for CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sync busy for COMP"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
