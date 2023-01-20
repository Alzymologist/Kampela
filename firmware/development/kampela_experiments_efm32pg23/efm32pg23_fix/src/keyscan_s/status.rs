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
#[doc = "Field `ROW` reader - Row detection"]
pub type ROW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RUNNING` reader - Running"]
pub type RUNNING_R = crate::BitReader<bool>;
#[doc = "Field `COL` reader - Column Latched"]
pub type COL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NOKEY` reader - No Key pressed status"]
pub type NOKEY_R = crate::BitReader<bool>;
#[doc = "Field `SYNCBUSY` reader - Sync Busy"]
pub type SYNCBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - Row detection"]
    #[inline(always)]
    pub fn row(&self) -> ROW_R {
        ROW_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Column Latched"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - No Key pressed status"]
    #[inline(always)]
    pub fn nokey(&self) -> NOKEY_R {
        NOKEY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Sync Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SYNCBUSY_R {
        SYNCBUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x4000_0000"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
