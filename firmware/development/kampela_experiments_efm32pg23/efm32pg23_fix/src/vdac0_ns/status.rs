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
#[doc = "Field `CH0ENS` reader - Channel 0 Enabled Status"]
pub type CH0ENS_R = crate::BitReader<bool>;
#[doc = "Field `CH1ENS` reader - Channel 1 Enabled Status"]
pub type CH1ENS_R = crate::BitReader<bool>;
#[doc = "Field `CH0WARM` reader - Channel 0 Warmed Status"]
pub type CH0WARM_R = crate::BitReader<bool>;
#[doc = "Field `CH1WARM` reader - Channel 1 Warmed Status"]
pub type CH1WARM_R = crate::BitReader<bool>;
#[doc = "Field `CH0FIFOFULL` reader - Channel 0 FIFO Full Status"]
pub type CH0FIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `CH1FIFOFULL` reader - Channel 1 FIFO Full Status"]
pub type CH1FIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `CH0FIFOCNT` reader - Channel 0 FIFO Valid Count"]
pub type CH0FIFOCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1FIFOCNT` reader - Channel 1 FIFO Valid Count"]
pub type CH1FIFOCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0CURRENTSTATE` reader - Channel 0 Current Status"]
pub type CH0CURRENTSTATE_R = crate::BitReader<bool>;
#[doc = "Field `CH1CURRENTSTATE` reader - Channel 1 Current Status"]
pub type CH1CURRENTSTATE_R = crate::BitReader<bool>;
#[doc = "Field `CH0FIFOEMPTY` reader - Channel 0 FIFO Empty Status"]
pub type CH0FIFOEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `CH1FIFOEMPTY` reader - Channel 1 FIFO Empty Status"]
pub type CH1FIFOEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `CH0FIFOFLBUSY` reader - CH0 FIFO Flush Sync Busy"]
pub type CH0FIFOFLBUSY_R = crate::BitReader<bool>;
#[doc = "Field `CH1FIFOFLBUSY` reader - CH1 FIFO Flush Sync Busy"]
pub type CH1FIFOFLBUSY_R = crate::BitReader<bool>;
#[doc = "Field `ABUSINPUTCONFLICT` reader - ABUS Input Conflict Status"]
pub type ABUSINPUTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `SINEACTIVE` reader - Sine Wave Output Status on Channel"]
pub type SINEACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `ABUSALLOCERR` reader - ABUS Allocation Error Status"]
pub type ABUSALLOCERR_R = crate::BitReader<bool>;
#[doc = "Field `SYNCBUSY` reader - Sync Busy Combined"]
pub type SYNCBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Enabled Status"]
    #[inline(always)]
    pub fn ch0ens(&self) -> CH0ENS_R {
        CH0ENS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Enabled Status"]
    #[inline(always)]
    pub fn ch1ens(&self) -> CH1ENS_R {
        CH1ENS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Warmed Status"]
    #[inline(always)]
    pub fn ch0warm(&self) -> CH0WARM_R {
        CH0WARM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Warmed Status"]
    #[inline(always)]
    pub fn ch1warm(&self) -> CH1WARM_R {
        CH1WARM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 FIFO Full Status"]
    #[inline(always)]
    pub fn ch0fifofull(&self) -> CH0FIFOFULL_R {
        CH0FIFOFULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 FIFO Full Status"]
    #[inline(always)]
    pub fn ch1fifofull(&self) -> CH1FIFOFULL_R {
        CH1FIFOFULL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Channel 0 FIFO Valid Count"]
    #[inline(always)]
    pub fn ch0fifocnt(&self) -> CH0FIFOCNT_R {
        CH0FIFOCNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 1 FIFO Valid Count"]
    #[inline(always)]
    pub fn ch1fifocnt(&self) -> CH1FIFOCNT_R {
        CH1FIFOCNT_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 19 - Channel 0 Current Status"]
    #[inline(always)]
    pub fn ch0currentstate(&self) -> CH0CURRENTSTATE_R {
        CH0CURRENTSTATE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 1 Current Status"]
    #[inline(always)]
    pub fn ch1currentstate(&self) -> CH1CURRENTSTATE_R {
        CH1CURRENTSTATE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 0 FIFO Empty Status"]
    #[inline(always)]
    pub fn ch0fifoempty(&self) -> CH0FIFOEMPTY_R {
        CH0FIFOEMPTY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 1 FIFO Empty Status"]
    #[inline(always)]
    pub fn ch1fifoempty(&self) -> CH1FIFOEMPTY_R {
        CH1FIFOEMPTY_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - CH0 FIFO Flush Sync Busy"]
    #[inline(always)]
    pub fn ch0fifoflbusy(&self) -> CH0FIFOFLBUSY_R {
        CH0FIFOFLBUSY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CH1 FIFO Flush Sync Busy"]
    #[inline(always)]
    pub fn ch1fifoflbusy(&self) -> CH1FIFOFLBUSY_R {
        CH1FIFOFLBUSY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ABUS Input Conflict Status"]
    #[inline(always)]
    pub fn abusinputconflict(&self) -> ABUSINPUTCONFLICT_R {
        ABUSINPUTCONFLICT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Sine Wave Output Status on Channel"]
    #[inline(always)]
    pub fn sineactive(&self) -> SINEACTIVE_R {
        SINEACTIVE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ABUS Allocation Error Status"]
    #[inline(always)]
    pub fn abusallocerr(&self) -> ABUSALLOCERR_R {
        ABUSALLOCERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Sync Busy Combined"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SYNCBUSY_R {
        SYNCBUSY_R::new(((self.bits >> 31) & 1) != 0)
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
