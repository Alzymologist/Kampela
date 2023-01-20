#[doc = "Register `GRP0_SYNCBUSY` reader"]
pub struct R(crate::R<GRP0_SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRP0_SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRP0_SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRP0_SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTRL` reader - Sync busy for CTRL register"]
pub type CTRL_R = crate::BitReader<bool>;
#[doc = "Field `CMP0VALUE` reader - Sync busy for CMP0VALUE register"]
pub type CMP0VALUE_R = crate::BitReader<bool>;
#[doc = "Field `CMP1VALUE` reader - Sync busy for CMP1VALUE register"]
pub type CMP1VALUE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Sync busy for CTRL register"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sync busy for CMP0VALUE register"]
    #[inline(always)]
    pub fn cmp0value(&self) -> CMP0VALUE_R {
        CMP0VALUE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sync busy for CMP1VALUE register"]
    #[inline(always)]
    pub fn cmp1value(&self) -> CMP1VALUE_R {
        CMP1VALUE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grp0_syncbusy](index.html) module"]
pub struct GRP0_SYNCBUSY_SPEC;
impl crate::RegisterSpec for GRP0_SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grp0_syncbusy::R](R) reader structure"]
impl crate::Readable for GRP0_SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GRP0_SYNCBUSY to value 0"]
impl crate::Resettable for GRP0_SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
