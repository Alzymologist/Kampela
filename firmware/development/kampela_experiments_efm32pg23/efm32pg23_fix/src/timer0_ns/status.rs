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
#[doc = "Field `RUNNING` reader - Running"]
pub type RUNNING_R = crate::BitReader<bool>;
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Counting up"]
    UP = 0,
    #[doc = "1: Counting down"]
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
#[doc = "Field `TIMERLOCKSTATUS` reader - Timer lock status"]
pub type TIMERLOCKSTATUS_R = crate::BitReader<TIMERLOCKSTATUS_A>;
#[doc = "Timer lock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMERLOCKSTATUS_A {
    #[doc = "0: TIMER registers are unlocked"]
    UNLOCKED = 0,
    #[doc = "1: TIMER registers are locked"]
    LOCKED = 1,
}
impl From<TIMERLOCKSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: TIMERLOCKSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMERLOCKSTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMERLOCKSTATUS_A {
        match self.bits {
            false => TIMERLOCKSTATUS_A::UNLOCKED,
            true => TIMERLOCKSTATUS_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == TIMERLOCKSTATUS_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == TIMERLOCKSTATUS_A::LOCKED
    }
}
#[doc = "Field `DTILOCKSTATUS` reader - DTI lock status"]
pub type DTILOCKSTATUS_R = crate::BitReader<DTILOCKSTATUS_A>;
#[doc = "DTI lock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTILOCKSTATUS_A {
    #[doc = "0: DTI registers are unlocked"]
    UNLOCKED = 0,
    #[doc = "1: DTI registers are locked"]
    LOCKED = 1,
}
impl From<DTILOCKSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DTILOCKSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl DTILOCKSTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTILOCKSTATUS_A {
        match self.bits {
            false => DTILOCKSTATUS_A::UNLOCKED,
            true => DTILOCKSTATUS_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTILOCKSTATUS_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTILOCKSTATUS_A::LOCKED
    }
}
#[doc = "Field `SYNCBUSY` reader - Sync Busy"]
pub type SYNCBUSY_R = crate::BitReader<bool>;
#[doc = "Field `OCBV0` reader - Output Compare Buffer Valid"]
pub type OCBV0_R = crate::BitReader<bool>;
#[doc = "Field `OCBV1` reader - Output Compare Buffer Valid"]
pub type OCBV1_R = crate::BitReader<bool>;
#[doc = "Field `OCBV2` reader - Output Compare Buffer Valid"]
pub type OCBV2_R = crate::BitReader<bool>;
#[doc = "Field `ICFEMPTY0` reader - Input capture fifo empty"]
pub type ICFEMPTY0_R = crate::BitReader<bool>;
#[doc = "Field `ICFEMPTY1` reader - Input capture fifo empty"]
pub type ICFEMPTY1_R = crate::BitReader<bool>;
#[doc = "Field `ICFEMPTY2` reader - Input capture fifo empty"]
pub type ICFEMPTY2_R = crate::BitReader<bool>;
#[doc = "Field `CCPOL0` reader - Compare/Capture Polarity"]
pub type CCPOL0_R = crate::BitReader<CCPOL0_A>;
#[doc = "Compare/Capture Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPOL0_A {
    #[doc = "0: CCx polarity low level/rising edge"]
    LOWRISE = 0,
    #[doc = "1: CCx polarity high level/falling edge"]
    HIGHFALL = 1,
}
impl From<CCPOL0_A> for bool {
    #[inline(always)]
    fn from(variant: CCPOL0_A) -> Self {
        variant as u8 != 0
    }
}
impl CCPOL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPOL0_A {
        match self.bits {
            false => CCPOL0_A::LOWRISE,
            true => CCPOL0_A::HIGHFALL,
        }
    }
    #[doc = "Checks if the value of the field is `LOWRISE`"]
    #[inline(always)]
    pub fn is_lowrise(&self) -> bool {
        *self == CCPOL0_A::LOWRISE
    }
    #[doc = "Checks if the value of the field is `HIGHFALL`"]
    #[inline(always)]
    pub fn is_highfall(&self) -> bool {
        *self == CCPOL0_A::HIGHFALL
    }
}
#[doc = "Field `CCPOL1` reader - Compare/Capture Polarity"]
pub type CCPOL1_R = crate::BitReader<CCPOL1_A>;
#[doc = "Compare/Capture Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPOL1_A {
    #[doc = "0: CCx polarity low level/rising edge"]
    LOWRISE = 0,
    #[doc = "1: CCx polarity high level/falling edge"]
    HIGHFALL = 1,
}
impl From<CCPOL1_A> for bool {
    #[inline(always)]
    fn from(variant: CCPOL1_A) -> Self {
        variant as u8 != 0
    }
}
impl CCPOL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPOL1_A {
        match self.bits {
            false => CCPOL1_A::LOWRISE,
            true => CCPOL1_A::HIGHFALL,
        }
    }
    #[doc = "Checks if the value of the field is `LOWRISE`"]
    #[inline(always)]
    pub fn is_lowrise(&self) -> bool {
        *self == CCPOL1_A::LOWRISE
    }
    #[doc = "Checks if the value of the field is `HIGHFALL`"]
    #[inline(always)]
    pub fn is_highfall(&self) -> bool {
        *self == CCPOL1_A::HIGHFALL
    }
}
#[doc = "Field `CCPOL2` reader - Compare/Capture Polarity"]
pub type CCPOL2_R = crate::BitReader<CCPOL2_A>;
#[doc = "Compare/Capture Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPOL2_A {
    #[doc = "0: CCx polarity low level/rising edge"]
    LOWRISE = 0,
    #[doc = "1: CCx polarity high level/falling edge"]
    HIGHFALL = 1,
}
impl From<CCPOL2_A> for bool {
    #[inline(always)]
    fn from(variant: CCPOL2_A) -> Self {
        variant as u8 != 0
    }
}
impl CCPOL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPOL2_A {
        match self.bits {
            false => CCPOL2_A::LOWRISE,
            true => CCPOL2_A::HIGHFALL,
        }
    }
    #[doc = "Checks if the value of the field is `LOWRISE`"]
    #[inline(always)]
    pub fn is_lowrise(&self) -> bool {
        *self == CCPOL2_A::LOWRISE
    }
    #[doc = "Checks if the value of the field is `HIGHFALL`"]
    #[inline(always)]
    pub fn is_highfall(&self) -> bool {
        *self == CCPOL2_A::HIGHFALL
    }
}
impl R {
    #[doc = "Bit 0 - Running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TOP Buffer Valid"]
    #[inline(always)]
    pub fn topbv(&self) -> TOPBV_R {
        TOPBV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer lock status"]
    #[inline(always)]
    pub fn timerlockstatus(&self) -> TIMERLOCKSTATUS_R {
        TIMERLOCKSTATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DTI lock status"]
    #[inline(always)]
    pub fn dtilockstatus(&self) -> DTILOCKSTATUS_R {
        DTILOCKSTATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sync Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SYNCBUSY_R {
        SYNCBUSY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Compare Buffer Valid"]
    #[inline(always)]
    pub fn ocbv0(&self) -> OCBV0_R {
        OCBV0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Compare Buffer Valid"]
    #[inline(always)]
    pub fn ocbv1(&self) -> OCBV1_R {
        OCBV1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output Compare Buffer Valid"]
    #[inline(always)]
    pub fn ocbv2(&self) -> OCBV2_R {
        OCBV2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Input capture fifo empty"]
    #[inline(always)]
    pub fn icfempty0(&self) -> ICFEMPTY0_R {
        ICFEMPTY0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input capture fifo empty"]
    #[inline(always)]
    pub fn icfempty1(&self) -> ICFEMPTY1_R {
        ICFEMPTY1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input capture fifo empty"]
    #[inline(always)]
    pub fn icfempty2(&self) -> ICFEMPTY2_R {
        ICFEMPTY2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Compare/Capture Polarity"]
    #[inline(always)]
    pub fn ccpol0(&self) -> CCPOL0_R {
        CCPOL0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Compare/Capture Polarity"]
    #[inline(always)]
    pub fn ccpol1(&self) -> CCPOL1_R {
        CCPOL1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Compare/Capture Polarity"]
    #[inline(always)]
    pub fn ccpol2(&self) -> CCPOL2_R {
        CCPOL2_R::new(((self.bits >> 26) & 1) != 0)
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
