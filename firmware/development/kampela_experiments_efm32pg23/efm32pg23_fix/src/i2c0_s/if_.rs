#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - START condition Interrupt Flag"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - START condition Interrupt Flag"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RSTART` reader - Repeated START condition Interrupt Flag"]
pub type RSTART_R = crate::BitReader<bool>;
#[doc = "Field `RSTART` writer - Repeated START condition Interrupt Flag"]
pub type RSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ADDR` reader - Address Interrupt Flag"]
pub type ADDR_R = crate::BitReader<bool>;
#[doc = "Field `ADDR` writer - Address Interrupt Flag"]
pub type ADDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TXC` reader - Transfer Completed Interrupt Flag"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - Transfer Completed Interrupt Flag"]
pub type TXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TXBL` reader - Transmit Buffer Level Interrupt Flag"]
pub type TXBL_R = crate::BitReader<bool>;
#[doc = "Field `TXBL` writer - Transmit Buffer Level Interrupt Flag"]
pub type TXBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXDATAV` reader - Receive Data Valid Interrupt Flag"]
pub type RXDATAV_R = crate::BitReader<bool>;
#[doc = "Field `RXDATAV` writer - Receive Data Valid Interrupt Flag"]
pub type RXDATAV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ACK` reader - Acknowledge Received Interrupt Flag"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - Acknowledge Received Interrupt Flag"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `NACK` reader - Not Acknowledge Received Interrupt Flag"]
pub type NACK_R = crate::BitReader<bool>;
#[doc = "Field `NACK` writer - Not Acknowledge Received Interrupt Flag"]
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `MSTOP` reader - Leader STOP Condition Interrupt Flag"]
pub type MSTOP_R = crate::BitReader<bool>;
#[doc = "Field `MSTOP` writer - Leader STOP Condition Interrupt Flag"]
pub type MSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ARBLOST` reader - Arbitration Lost Interrupt Flag"]
pub type ARBLOST_R = crate::BitReader<bool>;
#[doc = "Field `ARBLOST` writer - Arbitration Lost Interrupt Flag"]
pub type ARBLOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `BUSERR` reader - Bus Error Interrupt Flag"]
pub type BUSERR_R = crate::BitReader<bool>;
#[doc = "Field `BUSERR` writer - Bus Error Interrupt Flag"]
pub type BUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `BUSHOLD` reader - Bus Held Interrupt Flag"]
pub type BUSHOLD_R = crate::BitReader<bool>;
#[doc = "Field `BUSHOLD` writer - Bus Held Interrupt Flag"]
pub type BUSHOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TXOF` reader - Transmit Buffer Overflow Interrupt Flag"]
pub type TXOF_R = crate::BitReader<bool>;
#[doc = "Field `TXOF` writer - Transmit Buffer Overflow Interrupt Flag"]
pub type TXOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXUF` reader - Receive Buffer Underflow Interrupt Flag"]
pub type RXUF_R = crate::BitReader<bool>;
#[doc = "Field `RXUF` writer - Receive Buffer Underflow Interrupt Flag"]
pub type RXUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `BITO` reader - Bus Idle Timeout Interrupt Flag"]
pub type BITO_R = crate::BitReader<bool>;
#[doc = "Field `BITO` writer - Bus Idle Timeout Interrupt Flag"]
pub type BITO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `CLTO` reader - Clock Low Timeout Interrupt Flag"]
pub type CLTO_R = crate::BitReader<bool>;
#[doc = "Field `CLTO` writer - Clock Low Timeout Interrupt Flag"]
pub type CLTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SSTOP` reader - Follower STOP condition Interrupt Flag"]
pub type SSTOP_R = crate::BitReader<bool>;
#[doc = "Field `SSTOP` writer - Follower STOP condition Interrupt Flag"]
pub type SSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXFULL` reader - Receive Buffer Full Interrupt Flag"]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` writer - Receive Buffer Full Interrupt Flag"]
pub type RXFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `CLERR` reader - Clock Low Error Interrupt Flag"]
pub type CLERR_R = crate::BitReader<bool>;
#[doc = "Field `CLERR` writer - Clock Low Error Interrupt Flag"]
pub type CLERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SCLERR` reader - SCL Error Interrupt Flag"]
pub type SCLERR_R = crate::BitReader<bool>;
#[doc = "Field `SCLERR` writer - SCL Error Interrupt Flag"]
pub type SCLERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SDAERR` reader - SDA Error Interrupt Flag"]
pub type SDAERR_R = crate::BitReader<bool>;
#[doc = "Field `SDAERR` writer - SDA Error Interrupt Flag"]
pub type SDAERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - START condition Interrupt Flag"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeated START condition Interrupt Flag"]
    #[inline(always)]
    pub fn rstart(&self) -> RSTART_R {
        RSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Address Interrupt Flag"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer Completed Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Not Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Leader STOP Condition Interrupt Flag"]
    #[inline(always)]
    pub fn mstop(&self) -> MSTOP_R {
        MSTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Flag"]
    #[inline(always)]
    pub fn arblost(&self) -> ARBLOST_R {
        ARBLOST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Held Interrupt Flag"]
    #[inline(always)]
    pub fn bushold(&self) -> BUSHOLD_R {
        BUSHOLD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Buffer Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bus Idle Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn bito(&self) -> BITO_R {
        BITO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock Low Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn clto(&self) -> CLTO_R {
        CLTO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Follower STOP condition Interrupt Flag"]
    #[inline(always)]
    pub fn sstop(&self) -> SSTOP_R {
        SSTOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Low Error Interrupt Flag"]
    #[inline(always)]
    pub fn clerr(&self) -> CLERR_R {
        CLERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCL Error Interrupt Flag"]
    #[inline(always)]
    pub fn sclerr(&self) -> SCLERR_R {
        SCLERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SDA Error Interrupt Flag"]
    #[inline(always)]
    pub fn sdaerr(&self) -> SDAERR_R {
        SDAERR_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - START condition Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Repeated START condition Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rstart(&mut self) -> RSTART_W<1> {
        RSTART_W::new(self)
    }
    #[doc = "Bit 2 - Address Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<2> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 3 - Transfer Completed Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<3> {
        TXC_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Buffer Level Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txbl(&mut self) -> TXBL_W<4> {
        TXBL_W::new(self)
    }
    #[doc = "Bit 5 - Receive Data Valid Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxdatav(&mut self) -> RXDATAV_W<5> {
        RXDATAV_W::new(self)
    }
    #[doc = "Bit 6 - Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<6> {
        ACK_W::new(self)
    }
    #[doc = "Bit 7 - Not Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<7> {
        NACK_W::new(self)
    }
    #[doc = "Bit 8 - Leader STOP Condition Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mstop(&mut self) -> MSTOP_W<8> {
        MSTOP_W::new(self)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn arblost(&mut self) -> ARBLOST_W<9> {
        ARBLOST_W::new(self)
    }
    #[doc = "Bit 10 - Bus Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BUSERR_W<10> {
        BUSERR_W::new(self)
    }
    #[doc = "Bit 11 - Bus Held Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bushold(&mut self) -> BUSHOLD_W<11> {
        BUSHOLD_W::new(self)
    }
    #[doc = "Bit 12 - Transmit Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<12> {
        TXOF_W::new(self)
    }
    #[doc = "Bit 13 - Receive Buffer Underflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<13> {
        RXUF_W::new(self)
    }
    #[doc = "Bit 14 - Bus Idle Timeout Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bito(&mut self) -> BITO_W<14> {
        BITO_W::new(self)
    }
    #[doc = "Bit 15 - Clock Low Timeout Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clto(&mut self) -> CLTO_W<15> {
        CLTO_W::new(self)
    }
    #[doc = "Bit 16 - Follower STOP condition Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sstop(&mut self) -> SSTOP_W<16> {
        SSTOP_W::new(self)
    }
    #[doc = "Bit 17 - Receive Buffer Full Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<17> {
        RXFULL_W::new(self)
    }
    #[doc = "Bit 18 - Clock Low Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clerr(&mut self) -> CLERR_W<18> {
        CLERR_W::new(self)
    }
    #[doc = "Bit 19 - SCL Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sclerr(&mut self) -> SCLERR_W<19> {
        SCLERR_W::new(self)
    }
    #[doc = "Bit 20 - SDA Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sdaerr(&mut self) -> SDAERR_W<20> {
        SDAERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
