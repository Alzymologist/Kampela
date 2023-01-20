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
#[doc = "Field `SINGLEQEN` reader - Single Queue Enabled"]
pub type SINGLEQEN_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEQUEUEPENDING` reader - Single Queue Pending"]
pub type SINGLEQUEUEPENDING_R = crate::BitReader<bool>;
#[doc = "Field `SCANQEN` reader - Scan Queued Enabled"]
pub type SCANQEN_R = crate::BitReader<bool>;
#[doc = "Field `SCANQUEUEPENDING` reader - Scan Queue Pending"]
pub type SCANQUEUEPENDING_R = crate::BitReader<bool>;
#[doc = "Field `CONVERTING` reader - Converting"]
pub type CONVERTING_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEFIFODV` reader - SINGLEFIFO Data Valid"]
pub type SINGLEFIFODV_R = crate::BitReader<bool>;
#[doc = "Field `SCANFIFODV` reader - SCANFIFO Data Valid"]
pub type SCANFIFODV_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEFIFOFLUSHING` reader - The Single FIFO is flushing"]
pub type SINGLEFIFOFLUSHING_R = crate::BitReader<bool>;
#[doc = "Field `SCANFIFOFLUSHING` reader - The Scan FIFO is flushing"]
pub type SCANFIFOFLUSHING_R = crate::BitReader<bool>;
#[doc = "Field `TIMERACTIVE` reader - Timer Active"]
pub type TIMERACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEWRITEPENDING` reader - SINGLE write pending"]
pub type SINGLEWRITEPENDING_R = crate::BitReader<bool>;
#[doc = "Field `MASKREQWRITEPENDING` reader - MASKREQ write pending"]
pub type MASKREQWRITEPENDING_R = crate::BitReader<bool>;
#[doc = "Field `SYNCBUSY` reader - SYNCBUSY"]
pub type SYNCBUSY_R = crate::BitReader<bool>;
#[doc = "Field `ADCWARM` reader - ADCWARM"]
pub type ADCWARM_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Single Queue Enabled"]
    #[inline(always)]
    pub fn singleqen(&self) -> SINGLEQEN_R {
        SINGLEQEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single Queue Pending"]
    #[inline(always)]
    pub fn singlequeuepending(&self) -> SINGLEQUEUEPENDING_R {
        SINGLEQUEUEPENDING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Queued Enabled"]
    #[inline(always)]
    pub fn scanqen(&self) -> SCANQEN_R {
        SCANQEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan Queue Pending"]
    #[inline(always)]
    pub fn scanqueuepending(&self) -> SCANQUEUEPENDING_R {
        SCANQUEUEPENDING_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Converting"]
    #[inline(always)]
    pub fn converting(&self) -> CONVERTING_R {
        CONVERTING_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SINGLEFIFO Data Valid"]
    #[inline(always)]
    pub fn singlefifodv(&self) -> SINGLEFIFODV_R {
        SINGLEFIFODV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCANFIFO Data Valid"]
    #[inline(always)]
    pub fn scanfifodv(&self) -> SCANFIFODV_R {
        SCANFIFODV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - The Single FIFO is flushing"]
    #[inline(always)]
    pub fn singlefifoflushing(&self) -> SINGLEFIFOFLUSHING_R {
        SINGLEFIFOFLUSHING_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The Scan FIFO is flushing"]
    #[inline(always)]
    pub fn scanfifoflushing(&self) -> SCANFIFOFLUSHING_R {
        SCANFIFOFLUSHING_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer Active"]
    #[inline(always)]
    pub fn timeractive(&self) -> TIMERACTIVE_R {
        TIMERACTIVE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - SINGLE write pending"]
    #[inline(always)]
    pub fn singlewritepending(&self) -> SINGLEWRITEPENDING_R {
        SINGLEWRITEPENDING_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MASKREQ write pending"]
    #[inline(always)]
    pub fn maskreqwritepending(&self) -> MASKREQWRITEPENDING_R {
        MASKREQWRITEPENDING_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - SYNCBUSY"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SYNCBUSY_R {
        SYNCBUSY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - ADCWARM"]
    #[inline(always)]
    pub fn adcwarm(&self) -> ADCWARM_R {
        ADCWARM_R::new(((self.bits >> 30) & 1) != 0)
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
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
