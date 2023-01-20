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
#[doc = "Field `RUNNING` reader - SYSRTC running status"]
pub type RUNNING_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSTATUS` reader - Lock Status"]
pub type LOCKSTATUS_R = crate::BitReader<LOCKSTATUS_A>;
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKSTATUS_A {
    #[doc = "0: SYSRTC registers are unlocked"]
    UNLOCKED = 0,
    #[doc = "1: SYSRTC registers are locked"]
    LOCKED = 1,
}
impl From<LOCKSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCKSTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKSTATUS_A {
        match self.bits {
            false => LOCKSTATUS_A::UNLOCKED,
            true => LOCKSTATUS_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKSTATUS_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKSTATUS_A::LOCKED
    }
}
impl R {
    #[doc = "Bit 0 - SYSRTC running status"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Status"]
    #[inline(always)]
    pub fn lockstatus(&self) -> LOCKSTATUS_R {
        LOCKSTATUS_R::new(((self.bits >> 1) & 1) != 0)
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
