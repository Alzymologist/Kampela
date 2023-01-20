#[doc = "Register `RSTCAUSE` reader"]
pub struct R(crate::R<RSTCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `POR` reader - Power On Reset"]
pub type POR_R = crate::BitReader<bool>;
#[doc = "Field `PIN` reader - Pin Reset"]
pub type PIN_R = crate::BitReader<bool>;
#[doc = "Field `EM4` reader - EM4 Wakeup Reset"]
pub type EM4_R = crate::BitReader<bool>;
#[doc = "Field `WDOG0` reader - Watchdog 0 Reset"]
pub type WDOG0_R = crate::BitReader<bool>;
#[doc = "Field `WDOG1` reader - Watchdog 1 Reset"]
pub type WDOG1_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUP` reader - M33 Core Lockup Reset"]
pub type LOCKUP_R = crate::BitReader<bool>;
#[doc = "Field `SYSREQ` reader - M33 Core Sys Reset"]
pub type SYSREQ_R = crate::BitReader<bool>;
#[doc = "Field `DVDDBOD` reader - HVBOD Reset"]
pub type DVDDBOD_R = crate::BitReader<bool>;
#[doc = "Field `DVDDLEBOD` reader - LEBOD Reset"]
pub type DVDDLEBOD_R = crate::BitReader<bool>;
#[doc = "Field `DECBOD` reader - LVBOD Reset"]
pub type DECBOD_R = crate::BitReader<bool>;
#[doc = "Field `AVDDBOD` reader - LEBOD1 Reset"]
pub type AVDDBOD_R = crate::BitReader<bool>;
#[doc = "Field `IOVDD0BOD` reader - LEBOD2 Reset"]
pub type IOVDD0BOD_R = crate::BitReader<bool>;
#[doc = "Field `SETAMPER` reader - SE Tamper event Reset"]
pub type SETAMPER_R = crate::BitReader<bool>;
#[doc = "Field `VREGIN` reader - DCDC VREGIN comparator"]
pub type VREGIN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin Reset"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EM4 Wakeup Reset"]
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog 0 Reset"]
    #[inline(always)]
    pub fn wdog0(&self) -> WDOG0_R {
        WDOG0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog 1 Reset"]
    #[inline(always)]
    pub fn wdog1(&self) -> WDOG1_R {
        WDOG1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - M33 Core Lockup Reset"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - M33 Core Sys Reset"]
    #[inline(always)]
    pub fn sysreq(&self) -> SYSREQ_R {
        SYSREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HVBOD Reset"]
    #[inline(always)]
    pub fn dvddbod(&self) -> DVDDBOD_R {
        DVDDBOD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LEBOD Reset"]
    #[inline(always)]
    pub fn dvddlebod(&self) -> DVDDLEBOD_R {
        DVDDLEBOD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LVBOD Reset"]
    #[inline(always)]
    pub fn decbod(&self) -> DECBOD_R {
        DECBOD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LEBOD1 Reset"]
    #[inline(always)]
    pub fn avddbod(&self) -> AVDDBOD_R {
        AVDDBOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LEBOD2 Reset"]
    #[inline(always)]
    pub fn iovdd0bod(&self) -> IOVDD0BOD_R {
        IOVDD0BOD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - SE Tamper event Reset"]
    #[inline(always)]
    pub fn setamper(&self) -> SETAMPER_R {
        SETAMPER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 31 - DCDC VREGIN comparator"]
    #[inline(always)]
    pub fn vregin(&self) -> VREGIN_R {
        VREGIN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcause](index.html) module"]
pub struct RSTCAUSE_SPEC;
impl crate::RegisterSpec for RSTCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcause::R](R) reader structure"]
impl crate::Readable for RSTCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTCAUSE to value 0"]
impl crate::Resettable for RSTCAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
