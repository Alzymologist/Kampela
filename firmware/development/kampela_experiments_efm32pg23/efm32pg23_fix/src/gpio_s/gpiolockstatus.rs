#[doc = "Register `GPIOLOCKSTATUS` reader"]
pub struct R(crate::R<GPIOLOCKSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOLOCKSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOLOCKSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOLOCKSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCK` reader - GPIO LOCK status"]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "GPIO LOCK status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: Registers are unlocked"]
    UNLOCKED = 0,
    #[doc = "1: Registers are locked"]
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
    #[doc = "Bit 0 - GPIO LOCK status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiolockstatus](index.html) module"]
pub struct GPIOLOCKSTATUS_SPEC;
impl crate::RegisterSpec for GPIOLOCKSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiolockstatus::R](R) reader structure"]
impl crate::Readable for GPIOLOCKSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOLOCKSTATUS to value 0"]
impl crate::Resettable for GPIOLOCKSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
