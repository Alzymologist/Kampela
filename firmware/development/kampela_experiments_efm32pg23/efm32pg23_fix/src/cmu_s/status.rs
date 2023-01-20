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
#[doc = "Field `CALRDY` reader - Calibration Ready"]
pub type CALRDY_R = crate::BitReader<bool>;
#[doc = "Field `WDOGLOCK` reader - Configuration Lock Status for WDOG"]
pub type WDOGLOCK_R = crate::BitReader<WDOGLOCK_A>;
#[doc = "Configuration Lock Status for WDOG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDOGLOCK_A {
    #[doc = "0: WDOG configuration lock is unlocked"]
    UNLOCKED = 0,
    #[doc = "1: WDOG configuration lock is locked"]
    LOCKED = 1,
}
impl From<WDOGLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: WDOGLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl WDOGLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOGLOCK_A {
        match self.bits {
            false => WDOGLOCK_A::UNLOCKED,
            true => WDOGLOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == WDOGLOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == WDOGLOCK_A::LOCKED
    }
}
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
    #[doc = "Bit 0 - Calibration Ready"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 30 - Configuration Lock Status for WDOG"]
    #[inline(always)]
    pub fn wdoglock(&self) -> WDOGLOCK_R {
        WDOGLOCK_R::new(((self.bits >> 30) & 1) != 0)
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
