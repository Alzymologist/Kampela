#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDY` reader - Ready Status"]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `COREBIASOPTRDY` reader - Core Bias Optimization Ready"]
pub type COREBIASOPTRDY_R = crate::BitReader<bool>;
#[doc = "Field `PRSRDY` reader - PRS Ready Status"]
pub type PRSRDY_R = crate::BitReader<bool>;
#[doc = "Field `BUFOUTRDY` reader - BUFOUT Ready Status"]
pub type BUFOUTRDY_R = crate::BitReader<bool>;
#[doc = "Field `BUFOUTFROZEN` reader - BUFOUT Frozen"]
pub type BUFOUTFROZEN_R = crate::BitReader<bool>;
#[doc = "Field `ENS` reader - Enabled Status"]
pub type ENS_R = crate::BitReader<bool>;
#[doc = "Field `HWREQ` reader - Oscillator Requested by Digital Clock"]
pub type HWREQ_R = crate::BitReader<bool>;
#[doc = "Field `ISWARM` reader - Oscillator Is Kept Warm"]
pub type ISWARM_R = crate::BitReader<bool>;
#[doc = "Field `PRSHWREQ` reader - Oscillator Requested by PRS Request"]
pub type PRSHWREQ_R = crate::BitReader<bool>;
#[doc = "Field `BUFOUTHWREQ` reader - Oscillator Requested by BUFOUT Request"]
pub type BUFOUTHWREQ_R = crate::BitReader<bool>;
#[doc = "Field `SYNCBUSY` reader - Sync Busy"]
pub type SYNCBUSY_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` reader - Configuration Lock Status"]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "Configuration Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: Configuration lock is unlocked"]
    UNLOCKED = 0,
    #[doc = "1: Configuration lock is locked"]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
impl R {
    #[doc = "Bit 0 - Ready Status"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core Bias Optimization Ready"]
    #[inline(always)]
    pub fn corebiasoptrdy(&self) -> COREBIASOPTRDY_R {
        COREBIASOPTRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PRS Ready Status"]
    #[inline(always)]
    pub fn prsrdy(&self) -> PRSRDY_R {
        PRSRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUFOUT Ready Status"]
    #[inline(always)]
    pub fn bufoutrdy(&self) -> BUFOUTRDY_R {
        BUFOUTRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - BUFOUT Frozen"]
    #[inline(always)]
    pub fn bufoutfrozen(&self) -> BUFOUTFROZEN_R {
        BUFOUTFROZEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enabled Status"]
    #[inline(always)]
    pub fn ens(&self) -> ENS_R {
        ENS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Oscillator Requested by Digital Clock"]
    #[inline(always)]
    pub fn hwreq(&self) -> HWREQ_R {
        HWREQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Oscillator Is Kept Warm"]
    #[inline(always)]
    pub fn iswarm(&self) -> ISWARM_R {
        ISWARM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Oscillator Requested by PRS Request"]
    #[inline(always)]
    pub fn prshwreq(&self) -> PRSHWREQ_R {
        PRSHWREQ_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Oscillator Requested by BUFOUT Request"]
    #[inline(always)]
    pub fn bufouthwreq(&self) -> BUFOUTHWREQ_R {
        BUFOUTHWREQ_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Sync Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SYNCBUSY_R {
        SYNCBUSY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configuration Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
