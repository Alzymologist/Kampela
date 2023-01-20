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
#[doc = "Field `BUSY` reader - Erase/Write Busy"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `LOCKED` reader - Access Locked"]
pub type LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `INVADDR` reader - Invalid Write Address or Erase Page"]
pub type INVADDR_R = crate::BitReader<bool>;
#[doc = "Field `WDATAREADY` reader - WDATA Write Ready"]
pub type WDATAREADY_R = crate::BitReader<bool>;
#[doc = "Field `ERASEABORTED` reader - The Current Flash Erase Operation Aborte"]
pub type ERASEABORTED_R = crate::BitReader<bool>;
#[doc = "Field `PENDING` reader - Write command is in queue"]
pub type PENDING_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` reader - Write command timeout flag"]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `RANGEPARTIAL` reader - EraseRange with skipped locked pages"]
pub type RANGEPARTIAL_R = crate::BitReader<bool>;
#[doc = "Field `REGLOCK` reader - Register Lock Status"]
pub type REGLOCK_R = crate::BitReader<REGLOCK_A>;
#[doc = "Register Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLOCK_A {
    #[doc = "0: UNLOCKED"]
    UNLOCKED = 0,
    #[doc = "1: LOCKED"]
    LOCKED = 1,
}
impl From<REGLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: REGLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl REGLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGLOCK_A {
        match self.bits {
            false => REGLOCK_A::UNLOCKED,
            true => REGLOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == REGLOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == REGLOCK_A::LOCKED
    }
}
#[doc = "Field `PWRON` reader - Flash power on status"]
pub type PWRON_R = crate::BitReader<bool>;
#[doc = "Field `WREADY` reader - Flash Write Ready"]
pub type WREADY_R = crate::BitReader<bool>;
#[doc = "Field `PWRUPCKBDFAILCOUNT` reader - Flash power up checkerboard pattern chec"]
pub type PWRUPCKBDFAILCOUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Erase/Write Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Access Locked"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invalid Write Address or Erase Page"]
    #[inline(always)]
    pub fn invaddr(&self) -> INVADDR_R {
        INVADDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WDATA Write Ready"]
    #[inline(always)]
    pub fn wdataready(&self) -> WDATAREADY_R {
        WDATAREADY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The Current Flash Erase Operation Aborte"]
    #[inline(always)]
    pub fn eraseaborted(&self) -> ERASEABORTED_R {
        ERASEABORTED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write command is in queue"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write command timeout flag"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EraseRange with skipped locked pages"]
    #[inline(always)]
    pub fn rangepartial(&self) -> RANGEPARTIAL_R {
        RANGEPARTIAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Register Lock Status"]
    #[inline(always)]
    pub fn reglock(&self) -> REGLOCK_R {
        REGLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Flash power on status"]
    #[inline(always)]
    pub fn pwron(&self) -> PWRON_R {
        PWRON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Flash Write Ready"]
    #[inline(always)]
    pub fn wready(&self) -> WREADY_R {
        WREADY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Flash power up checkerboard pattern chec"]
    #[inline(always)]
    pub fn pwrupckbdfailcount(&self) -> PWRUPCKBDFAILCOUNT_R {
        PWRUPCKBDFAILCOUNT_R::new(((self.bits >> 28) & 0x0f) as u8)
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
#[doc = "`reset()` method sets STATUS to value 0x0800_0008"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800_0008;
}
