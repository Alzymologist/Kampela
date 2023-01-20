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
#[doc = "Field `BYPSW` reader - Bypass Switch is currently enabled"]
pub type BYPSW_R = crate::BitReader<bool>;
#[doc = "Field `WARM` reader - DCDC Warmup Done"]
pub type WARM_R = crate::BitReader<bool>;
#[doc = "Field `RUNNING` reader - DCDC is running"]
pub type RUNNING_R = crate::BitReader<bool>;
#[doc = "Field `VREGIN` reader - VREGIN comparator status"]
pub type VREGIN_R = crate::BitReader<bool>;
#[doc = "Field `BYPCMPOUT` reader - Bypass Comparator Output"]
pub type BYPCMPOUT_R = crate::BitReader<bool>;
#[doc = "Field `PPMODE` reader - DCDC in pulse-pairing mode"]
pub type PPMODE_R = crate::BitReader<bool>;
#[doc = "Field `PFMXMODE` reader - DCDC in PFMX mode"]
pub type PFMXMODE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Bypass Switch is currently enabled"]
    #[inline(always)]
    pub fn bypsw(&self) -> BYPSW_R {
        BYPSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCDC Warmup Done"]
    #[inline(always)]
    pub fn warm(&self) -> WARM_R {
        WARM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCDC is running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VREGIN comparator status"]
    #[inline(always)]
    pub fn vregin(&self) -> VREGIN_R {
        VREGIN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass Comparator Output"]
    #[inline(always)]
    pub fn bypcmpout(&self) -> BYPCMPOUT_R {
        BYPCMPOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DCDC in pulse-pairing mode"]
    #[inline(always)]
    pub fn ppmode(&self) -> PPMODE_R {
        PPMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DCDC in PFMX mode"]
    #[inline(always)]
    pub fn pfmxmode(&self) -> PFMXMODE_R {
        PFMXMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "DCDC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
