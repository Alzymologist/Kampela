#[doc = "Register `NSSTATUS` reader"]
pub struct R(crate::R<NSSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SMUNSLOCK` reader - SMUNS Lock"]
pub type SMUNSLOCK_R = crate::BitReader<SMUNSLOCK_A>;
#[doc = "SMUNS Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMUNSLOCK_A {
    #[doc = "0: UNLOCKED"]
    UNLOCKED = 0,
    #[doc = "1: LOCKED"]
    LOCKED = 1,
}
impl From<SMUNSLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SMUNSLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl SMUNSLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMUNSLOCK_A {
        match self.bits {
            false => SMUNSLOCK_A::UNLOCKED,
            true => SMUNSLOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == SMUNSLOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == SMUNSLOCK_A::LOCKED
    }
}
impl R {
    #[doc = "Bit 0 - SMUNS Lock"]
    #[inline(always)]
    pub fn smunslock(&self) -> SMUNSLOCK_R {
        SMUNSLOCK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsstatus](index.html) module"]
pub struct NSSTATUS_SPEC;
impl crate::RegisterSpec for NSSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nsstatus::R](R) reader structure"]
impl crate::Readable for NSSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NSSTATUS to value 0"]
impl crate::Resettable for NSSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
