#[doc = "Register `LOCKSTATUS` reader"]
pub struct R(crate::R<LOCKSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCK` reader - Lock Status"]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: Unlocked State"]
    UNLOCKED = 0,
    #[doc = "1: LOCKED STATE"]
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
    #[doc = "Bit 0 - Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockstatus](index.html) module"]
pub struct LOCKSTATUS_SPEC;
impl crate::RegisterSpec for LOCKSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockstatus::R](R) reader structure"]
impl crate::Readable for LOCKSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOCKSTATUS to value 0"]
impl crate::Resettable for LOCKSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
