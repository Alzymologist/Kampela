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
#[doc = "Field `SMULOCK` reader - SMU Lock"]
pub type SMULOCK_R = crate::BitReader<SMULOCK_A>;
#[doc = "SMU Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMULOCK_A {
    #[doc = "0: UNLOCKED"]
    UNLOCKED = 0,
    #[doc = "1: LOCKED"]
    LOCKED = 1,
}
impl From<SMULOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SMULOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl SMULOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMULOCK_A {
        match self.bits {
            false => SMULOCK_A::UNLOCKED,
            true => SMULOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == SMULOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == SMULOCK_A::LOCKED
    }
}
#[doc = "Field `SMUPRGERR` reader - SMU Programming Error"]
pub type SMUPRGERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SMU Lock"]
    #[inline(always)]
    pub fn smulock(&self) -> SMULOCK_R {
        SMULOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMU Programming Error"]
    #[inline(always)]
    pub fn smuprgerr(&self) -> SMUPRGERR_R {
        SMUPRGERR_R::new(((self.bits >> 1) & 1) != 0)
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
