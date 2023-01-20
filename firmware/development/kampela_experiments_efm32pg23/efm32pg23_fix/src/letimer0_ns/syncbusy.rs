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
#[doc = "Field `CNT` reader - Sync busy for CNT"]
pub type CNT_R = crate::BitReader<bool>;
#[doc = "Field `TOP` reader - Sync busy for TOP"]
pub type TOP_R = crate::BitReader<bool>;
#[doc = "Field `REP0` reader - Sync busy for REP0"]
pub type REP0_R = crate::BitReader<bool>;
#[doc = "Field `REP1` reader - Sync busy for REP1"]
pub type REP1_R = crate::BitReader<bool>;
#[doc = "Field `START` reader - Sync busy for START"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `STOP` reader - Sync busy for STOP"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `CLEAR` reader - Sync busy for CLEAR"]
pub type CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `CTO0` reader - Sync busy for CTO0"]
pub type CTO0_R = crate::BitReader<bool>;
#[doc = "Field `CTO1` reader - Sync busy for CTO1"]
pub type CTO1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Sync busy for CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Sync busy for TOP"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sync busy for REP0"]
    #[inline(always)]
    pub fn rep0(&self) -> REP0_R {
        REP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sync busy for REP1"]
    #[inline(always)]
    pub fn rep1(&self) -> REP1_R {
        REP1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sync busy for START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sync busy for STOP"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sync busy for CLEAR"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sync busy for CTO0"]
    #[inline(always)]
    pub fn cto0(&self) -> CTO0_R {
        CTO0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sync busy for CTO1"]
    #[inline(always)]
    pub fn cto1(&self) -> CTO1_R {
        CTO1_R::new(((self.bits >> 9) & 1) != 0)
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
