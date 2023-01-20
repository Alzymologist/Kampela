#[doc = "Register `TIMECMP1` reader"]
pub struct R(crate::R<TIMECMP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMECMP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMECMP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMECMP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMECMP1` writer"]
pub struct W(crate::W<TIMECMP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMECMP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TIMECMP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMECMP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCMPVAL` reader - Timer comparator 1."]
pub type TCMPVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCMPVAL` writer - Timer comparator 1."]
pub type TCMPVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMECMP1_SPEC, u8, u8, 8, O>;
#[doc = "Field `TSTART` reader - Timer start source"]
pub type TSTART_R = crate::FieldReader<u8, TSTART_A>;
#[doc = "Timer start source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTART_A {
    #[doc = "0: Comparator 1 is disabled"]
    DISABLE = 0,
    #[doc = "1: Comparator 1 and timer are started at TX end of frame"]
    TXEOF = 1,
    #[doc = "2: Comparator 1 and timer are started at TX Complete"]
    TXC = 2,
    #[doc = "3: Comparator 1 and timer are started at RX going going Active (default: low)"]
    RXACT = 3,
    #[doc = "4: Comparator 1 and timer are started at RX end of frame"]
    RXEOF = 4,
}
impl From<TSTART_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTART_A) -> Self {
        variant as _
    }
}
impl TSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSTART_A> {
        match self.bits {
            0 => Some(TSTART_A::DISABLE),
            1 => Some(TSTART_A::TXEOF),
            2 => Some(TSTART_A::TXC),
            3 => Some(TSTART_A::RXACT),
            4 => Some(TSTART_A::RXEOF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTART_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `TXEOF`"]
    #[inline(always)]
    pub fn is_txeof(&self) -> bool {
        *self == TSTART_A::TXEOF
    }
    #[doc = "Checks if the value of the field is `TXC`"]
    #[inline(always)]
    pub fn is_txc(&self) -> bool {
        *self == TSTART_A::TXC
    }
    #[doc = "Checks if the value of the field is `RXACT`"]
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == TSTART_A::RXACT
    }
    #[doc = "Checks if the value of the field is `RXEOF`"]
    #[inline(always)]
    pub fn is_rxeof(&self) -> bool {
        *self == TSTART_A::RXEOF
    }
}
#[doc = "Field `TSTART` writer - Timer start source"]
pub type TSTART_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMECMP1_SPEC, u8, TSTART_A, 3, O>;
impl<'a, const O: u8> TSTART_W<'a, O> {
    #[doc = "Comparator 1 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTART_A::DISABLE)
    }
    #[doc = "Comparator 1 and timer are started at TX end of frame"]
    #[inline(always)]
    pub fn txeof(self) -> &'a mut W {
        self.variant(TSTART_A::TXEOF)
    }
    #[doc = "Comparator 1 and timer are started at TX Complete"]
    #[inline(always)]
    pub fn txc(self) -> &'a mut W {
        self.variant(TSTART_A::TXC)
    }
    #[doc = "Comparator 1 and timer are started at RX going going Active (default: low)"]
    #[inline(always)]
    pub fn rxact(self) -> &'a mut W {
        self.variant(TSTART_A::RXACT)
    }
    #[doc = "Comparator 1 and timer are started at RX end of frame"]
    #[inline(always)]
    pub fn rxeof(self) -> &'a mut W {
        self.variant(TSTART_A::RXEOF)
    }
}
#[doc = "Field `TSTOP` reader - Source used to disable comparator 1"]
pub type TSTOP_R = crate::FieldReader<u8, TSTOP_A>;
#[doc = "Source used to disable comparator 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTOP_A {
    #[doc = "0: Comparator 1 is disabled when the counter equals TCMPVAL and triggers a TCMP1 event"]
    TCMP1 = 0,
    #[doc = "1: Comparator 1 is disabled at TX start TX Engine"]
    TXST = 1,
    #[doc = "2: Comparator 1 is disabled on RX going going Active (default: low)"]
    RXACT = 2,
    #[doc = "3: Comparator 1 is disabled on RX going Inactive"]
    RXACTN = 3,
}
impl From<TSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTOP_A) -> Self {
        variant as _
    }
}
impl TSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSTOP_A> {
        match self.bits {
            0 => Some(TSTOP_A::TCMP1),
            1 => Some(TSTOP_A::TXST),
            2 => Some(TSTOP_A::RXACT),
            3 => Some(TSTOP_A::RXACTN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == TSTOP_A::TCMP1
    }
    #[doc = "Checks if the value of the field is `TXST`"]
    #[inline(always)]
    pub fn is_txst(&self) -> bool {
        *self == TSTOP_A::TXST
    }
    #[doc = "Checks if the value of the field is `RXACT`"]
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == TSTOP_A::RXACT
    }
    #[doc = "Checks if the value of the field is `RXACTN`"]
    #[inline(always)]
    pub fn is_rxactn(&self) -> bool {
        *self == TSTOP_A::RXACTN
    }
}
#[doc = "Field `TSTOP` writer - Source used to disable comparator 1"]
pub type TSTOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMECMP1_SPEC, u8, TSTOP_A, 3, O>;
impl<'a, const O: u8> TSTOP_W<'a, O> {
    #[doc = "Comparator 1 is disabled when the counter equals TCMPVAL and triggers a TCMP1 event"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(TSTOP_A::TCMP1)
    }
    #[doc = "Comparator 1 is disabled at TX start TX Engine"]
    #[inline(always)]
    pub fn txst(self) -> &'a mut W {
        self.variant(TSTOP_A::TXST)
    }
    #[doc = "Comparator 1 is disabled on RX going going Active (default: low)"]
    #[inline(always)]
    pub fn rxact(self) -> &'a mut W {
        self.variant(TSTOP_A::RXACT)
    }
    #[doc = "Comparator 1 is disabled on RX going Inactive"]
    #[inline(always)]
    pub fn rxactn(self) -> &'a mut W {
        self.variant(TSTOP_A::RXACTN)
    }
}
#[doc = "Field `RESTARTEN` reader - Restart Timer on TCMP1"]
pub type RESTARTEN_R = crate::BitReader<RESTARTEN_A>;
#[doc = "Restart Timer on TCMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESTARTEN_A {
    #[doc = "0: Disable the timer restarting on TCMP1"]
    DISABLE = 0,
    #[doc = "1: Enable the timer restarting on TCMP1"]
    ENABLE = 1,
}
impl From<RESTARTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RESTARTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RESTARTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESTARTEN_A {
        match self.bits {
            false => RESTARTEN_A::DISABLE,
            true => RESTARTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RESTARTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RESTARTEN_A::ENABLE
    }
}
#[doc = "Field `RESTARTEN` writer - Restart Timer on TCMP1"]
pub type RESTARTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMECMP1_SPEC, RESTARTEN_A, O>;
impl<'a, const O: u8> RESTARTEN_W<'a, O> {
    #[doc = "Disable the timer restarting on TCMP1"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESTARTEN_A::DISABLE)
    }
    #[doc = "Enable the timer restarting on TCMP1"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RESTARTEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:7 - Timer comparator 1."]
    #[inline(always)]
    pub fn tcmpval(&self) -> TCMPVAL_R {
        TCMPVAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Timer start source"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Source used to disable comparator 1"]
    #[inline(always)]
    pub fn tstop(&self) -> TSTOP_R {
        TSTOP_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Restart Timer on TCMP1"]
    #[inline(always)]
    pub fn restarten(&self) -> RESTARTEN_R {
        RESTARTEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer comparator 1."]
    #[inline(always)]
    #[must_use]
    pub fn tcmpval(&mut self) -> TCMPVAL_W<0> {
        TCMPVAL_W::new(self)
    }
    #[doc = "Bits 16:18 - Timer start source"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TSTART_W<16> {
        TSTART_W::new(self)
    }
    #[doc = "Bits 20:22 - Source used to disable comparator 1"]
    #[inline(always)]
    #[must_use]
    pub fn tstop(&mut self) -> TSTOP_W<20> {
        TSTOP_W::new(self)
    }
    #[doc = "Bit 24 - Restart Timer on TCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn restarten(&mut self) -> RESTARTEN_W<24> {
        RESTARTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timecmp1](index.html) module"]
pub struct TIMECMP1_SPEC;
impl crate::RegisterSpec for TIMECMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timecmp1::R](R) reader structure"]
impl crate::Readable for TIMECMP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timecmp1::W](W) writer structure"]
impl crate::Writable for TIMECMP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMECMP1 to value 0"]
impl crate::Resettable for TIMECMP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
