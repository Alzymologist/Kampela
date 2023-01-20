#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTRL` reader - CTRL Sync Busy Status"]
pub type CTRL_R = crate::BitReader<bool>;
#[doc = "Field `EM01CTRL0` reader - EM01CTRL0 Sync Busy Status"]
pub type EM01CTRL0_R = crate::BitReader<bool>;
#[doc = "Field `EM01CTRL1` reader - EM01CTRL1 Sync Bust Status"]
pub type EM01CTRL1_R = crate::BitReader<bool>;
#[doc = "Field `EM23CTRL0` reader - EM23CTRL0 Sync Busy Status"]
pub type EM23CTRL0_R = crate::BitReader<bool>;
#[doc = "Field `PFMXCTRL` reader - PFMXCTRL Sync Busy Status"]
pub type PFMXCTRL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - CTRL Sync Busy Status"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EM01CTRL0 Sync Busy Status"]
    #[inline(always)]
    pub fn em01ctrl0(&self) -> EM01CTRL0_R {
        EM01CTRL0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EM01CTRL1 Sync Bust Status"]
    #[inline(always)]
    pub fn em01ctrl1(&self) -> EM01CTRL1_R {
        EM01CTRL1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EM23CTRL0 Sync Busy Status"]
    #[inline(always)]
    pub fn em23ctrl0(&self) -> EM23CTRL0_R {
        EM23CTRL0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - PFMXCTRL Sync Busy Status"]
    #[inline(always)]
    pub fn pfmxctrl(&self) -> PFMXCTRL_R {
        PFMXCTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Syncbusy Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
