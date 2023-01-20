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
#[doc = "Field `ASTATE` reader - Current Animation State"]
pub type ASTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLINK` reader - Blink State"]
pub type BLINK_R = crate::BitReader<bool>;
#[doc = "Field `LOADBUSY` reader - Load Synchronization is busy"]
pub type LOADBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Current Animation State"]
    #[inline(always)]
    pub fn astate(&self) -> ASTATE_R {
        ASTATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Blink State"]
    #[inline(always)]
    pub fn blink(&self) -> BLINK_R {
        BLINK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Load Synchronization is busy"]
    #[inline(always)]
    pub fn loadbusy(&self) -> LOADBUSY_R {
        LOADBUSY_R::new(((self.bits >> 11) & 1) != 0)
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
