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
#[doc = "Field `DIR` reader - Current Counter Direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Current Counter Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Up counter mode (clockwise in EXTCLKQUAD mode with the EDGE bit in PCNTn_CTRL set to 0)."]
    UP = 0,
    #[doc = "1: Down counter mode."]
    DOWN = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::UP,
            true => DIR_A::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == DIR_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == DIR_A::DOWN
    }
}
#[doc = "Field `TOPBV` reader - TOP Buffer Valid"]
pub type TOPBV_R = crate::BitReader<bool>;
#[doc = "Field `PCNTLOCKSTATUS` reader - Lock Status"]
pub type PCNTLOCKSTATUS_R = crate::BitReader<PCNTLOCKSTATUS_A>;
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCNTLOCKSTATUS_A {
    #[doc = "0: PCNT registers are unlocked"]
    UNLOCKED = 0,
    #[doc = "1: PCNT registers are locked"]
    LOCKED = 1,
}
impl From<PCNTLOCKSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: PCNTLOCKSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl PCNTLOCKSTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCNTLOCKSTATUS_A {
        match self.bits {
            false => PCNTLOCKSTATUS_A::UNLOCKED,
            true => PCNTLOCKSTATUS_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PCNTLOCKSTATUS_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PCNTLOCKSTATUS_A::LOCKED
    }
}
#[doc = "Field `CNTRUNNING` reader - Main Counter running status"]
pub type CNTRUNNING_R = crate::BitReader<bool>;
#[doc = "Field `AUXCNTRUNNING` reader - Aux Counter running status"]
pub type AUXCNTRUNNING_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Current Counter Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TOP Buffer Valid"]
    #[inline(always)]
    pub fn topbv(&self) -> TOPBV_R {
        TOPBV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Status"]
    #[inline(always)]
    pub fn pcntlockstatus(&self) -> PCNTLOCKSTATUS_R {
        PCNTLOCKSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Main Counter running status"]
    #[inline(always)]
    pub fn cntrunning(&self) -> CNTRUNNING_R {
        CNTRUNNING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Aux Counter running status"]
    #[inline(always)]
    pub fn auxcntrunning(&self) -> AUXCNTRUNNING_R {
        AUXCNTRUNNING_R::new(((self.bits >> 4) & 1) != 0)
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
