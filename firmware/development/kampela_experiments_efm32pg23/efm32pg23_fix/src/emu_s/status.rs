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
#[doc = "Field `LOCK` reader - Lock status"]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "Lock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: All EMU lockable registers are unlocked."]
    UNLOCKED = 0,
    #[doc = "1: All EMU lockable registers are locked."]
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
#[doc = "Field `FIRSTTEMPDONE` reader - First Temp done"]
pub type FIRSTTEMPDONE_R = crate::BitReader<bool>;
#[doc = "Field `TEMPACTIVE` reader - Temp active"]
pub type TEMPACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `TEMPAVGACTIVE` reader - Temp Average active"]
pub type TEMPAVGACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `VSCALEBUSY` reader - Vscale busy"]
pub type VSCALEBUSY_R = crate::BitReader<bool>;
#[doc = "Field `VSCALEFAILED` reader - Vscale failed"]
pub type VSCALEFAILED_R = crate::BitReader<bool>;
#[doc = "Field `VSCALE` reader - Vscale status"]
pub type VSCALE_R = crate::FieldReader<u8, VSCALE_A>;
#[doc = "Vscale status\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VSCALE_A {
    #[doc = "0: Voltage scaling set to 0.9v"]
    VSCALE0 = 0,
    #[doc = "1: Voltage scaling set to 1.0v"]
    VSCALE1 = 1,
    #[doc = "2: Voltage scaling set to 1.1v"]
    VSCALE2 = 2,
}
impl From<VSCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: VSCALE_A) -> Self {
        variant as _
    }
}
impl VSCALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VSCALE_A> {
        match self.bits {
            0 => Some(VSCALE_A::VSCALE0),
            1 => Some(VSCALE_A::VSCALE1),
            2 => Some(VSCALE_A::VSCALE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VSCALE0`"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == VSCALE_A::VSCALE0
    }
    #[doc = "Checks if the value of the field is `VSCALE1`"]
    #[inline(always)]
    pub fn is_vscale1(&self) -> bool {
        *self == VSCALE_A::VSCALE1
    }
    #[doc = "Checks if the value of the field is `VSCALE2`"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == VSCALE_A::VSCALE2
    }
}
#[doc = "Field `EM4IORET` reader - EM4 IO retention status"]
pub type EM4IORET_R = crate::BitReader<bool>;
#[doc = "Field `EM2ENTERED` reader - EM2 entered"]
pub type EM2ENTERED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Lock status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - First Temp done"]
    #[inline(always)]
    pub fn firsttempdone(&self) -> FIRSTTEMPDONE_R {
        FIRSTTEMPDONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Temp active"]
    #[inline(always)]
    pub fn tempactive(&self) -> TEMPACTIVE_R {
        TEMPACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Temp Average active"]
    #[inline(always)]
    pub fn tempavgactive(&self) -> TEMPAVGACTIVE_R {
        TEMPAVGACTIVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Vscale busy"]
    #[inline(always)]
    pub fn vscalebusy(&self) -> VSCALEBUSY_R {
        VSCALEBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Vscale failed"]
    #[inline(always)]
    pub fn vscalefailed(&self) -> VSCALEFAILED_R {
        VSCALEFAILED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Vscale status"]
    #[inline(always)]
    pub fn vscale(&self) -> VSCALE_R {
        VSCALE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 12 - EM4 IO retention status"]
    #[inline(always)]
    pub fn em4ioret(&self) -> EM4IORET_R {
        EM4IORET_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - EM2 entered"]
    #[inline(always)]
    pub fn em2entered(&self) -> EM2ENTERED_R {
        EM2ENTERED_R::new(((self.bits >> 14) & 1) != 0)
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
#[doc = "`reset()` method sets STATUS to value 0x80"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
