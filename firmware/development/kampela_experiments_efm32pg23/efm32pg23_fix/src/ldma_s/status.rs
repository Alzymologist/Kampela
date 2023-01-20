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
#[doc = "Field `ANYBUSY` reader - Any DMA Channel Busy"]
pub type ANYBUSY_R = crate::BitReader<bool>;
#[doc = "Field `ANYREQ` reader - Any DMA Channel Request Pending"]
pub type ANYREQ_R = crate::BitReader<bool>;
#[doc = "Field `CHGRANT` reader - Granted Channel Number"]
pub type CHGRANT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHERROR` reader - Errant Channel Number"]
pub type CHERROR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOLEVEL` reader - FIFO Level"]
pub type FIFOLEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHNUM` reader - Number of Channels"]
pub type CHNUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Any DMA Channel Busy"]
    #[inline(always)]
    pub fn anybusy(&self) -> ANYBUSY_R {
        ANYBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Any DMA Channel Request Pending"]
    #[inline(always)]
    pub fn anyreq(&self) -> ANYREQ_R {
        ANYREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Granted Channel Number"]
    #[inline(always)]
    pub fn chgrant(&self) -> CHGRANT_R {
        CHGRANT_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Errant Channel Number"]
    #[inline(always)]
    pub fn cherror(&self) -> CHERROR_R {
        CHERROR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - FIFO Level"]
    #[inline(always)]
    pub fn fifolevel(&self) -> FIFOLEVEL_R {
        FIFOLEVEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Number of Channels"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new(((self.bits >> 24) & 0x1f) as u8)
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
#[doc = "`reset()` method sets STATUS to value 0x1f10_0000"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f10_0000;
}
