#[doc = "Register `SYNC_PEEK` reader"]
pub struct R(crate::R<SYNC_PEEK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNC_PEEK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNC_PEEK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNC_PEEK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0VAL` reader - Channel Value"]
pub type CH0VAL_R = crate::BitReader<bool>;
#[doc = "Field `CH1VAL` reader - Channel Value"]
pub type CH1VAL_R = crate::BitReader<bool>;
#[doc = "Field `CH2VAL` reader - Channel Value"]
pub type CH2VAL_R = crate::BitReader<bool>;
#[doc = "Field `CH3VAL` reader - Channel Value"]
pub type CH3VAL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel Value"]
    #[inline(always)]
    pub fn ch0val(&self) -> CH0VAL_R {
        CH0VAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Value"]
    #[inline(always)]
    pub fn ch1val(&self) -> CH1VAL_R {
        CH1VAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Value"]
    #[inline(always)]
    pub fn ch2val(&self) -> CH2VAL_R {
        CH2VAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Value"]
    #[inline(always)]
    pub fn ch3val(&self) -> CH3VAL_R {
        CH3VAL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync_peek](index.html) module"]
pub struct SYNC_PEEK_SPEC;
impl crate::RegisterSpec for SYNC_PEEK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sync_peek::R](R) reader structure"]
impl crate::Readable for SYNC_PEEK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNC_PEEK to value 0"]
impl crate::Resettable for SYNC_PEEK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
