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
#[doc = "Field `TXC` reader - TX Complete Interrupt Flag"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - TX Complete Interrupt Flag"]
pub type TXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TXFL` reader - TX FIFO Level Interrupt Flag"]
pub type TXFL_R = crate::BitReader<bool>;
#[doc = "Field `TXFL` writer - TX FIFO Level Interrupt Flag"]
pub type TXFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXFL` reader - RX FIFO Level Interrupt Flag"]
pub type RXFL_R = crate::BitReader<bool>;
#[doc = "Field `RXFL` writer - RX FIFO Level Interrupt Flag"]
pub type RXFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXFULL` reader - RX FIFO Full Interrupt Flag"]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` writer - RX FIFO Full Interrupt Flag"]
pub type RXFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXOF` reader - RX FIFO Overflow Interrupt Flag"]
pub type RXOF_R = crate::BitReader<bool>;
#[doc = "Field `RXOF` writer - RX FIFO Overflow Interrupt Flag"]
pub type RXOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXUF` reader - RX FIFO Underflow Interrupt Flag"]
pub type RXUF_R = crate::BitReader<bool>;
#[doc = "Field `RXUF` writer - RX FIFO Underflow Interrupt Flag"]
pub type RXUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TXOF` reader - TX FIFO Overflow Interrupt Flag"]
pub type TXOF_R = crate::BitReader<bool>;
#[doc = "Field `TXOF` writer - TX FIFO Overflow Interrupt Flag"]
pub type TXOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TXUF` reader - TX FIFO Underflow Interrupt Flag"]
pub type TXUF_R = crate::BitReader<bool>;
#[doc = "Field `TXUF` writer - TX FIFO Underflow Interrupt Flag"]
pub type TXUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `PERR` reader - Parity Error Interrupt Flag"]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `PERR` writer - Parity Error Interrupt Flag"]
pub type PERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `FERR` reader - Framing Error Interrupt Flag"]
pub type FERR_R = crate::BitReader<bool>;
#[doc = "Field `FERR` writer - Framing Error Interrupt Flag"]
pub type FERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `MPAF` reader - Multi-Processor Address Frame Interrupt"]
pub type MPAF_R = crate::BitReader<bool>;
#[doc = "Field `MPAF` writer - Multi-Processor Address Frame Interrupt"]
pub type MPAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `LOADERR` reader - Load Error Interrupt Flag"]
pub type LOADERR_R = crate::BitReader<bool>;
#[doc = "Field `LOADERR` writer - Load Error Interrupt Flag"]
pub type LOADERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `CCF` reader - Collision Check Fail Interrupt Flag"]
pub type CCF_R = crate::BitReader<bool>;
#[doc = "Field `CCF` writer - Collision Check Fail Interrupt Flag"]
pub type CCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TXIDLE` reader - TX Idle Interrupt Flag"]
pub type TXIDLE_R = crate::BitReader<bool>;
#[doc = "Field `TXIDLE` writer - TX Idle Interrupt Flag"]
pub type TXIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `STARTF` reader - Start Frame Interrupt Flag"]
pub type STARTF_R = crate::BitReader<bool>;
#[doc = "Field `STARTF` writer - Start Frame Interrupt Flag"]
pub type STARTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SIGF` reader - Signal Frame Interrupt Flag"]
pub type SIGF_R = crate::BitReader<bool>;
#[doc = "Field `SIGF` writer - Signal Frame Interrupt Flag"]
pub type SIGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `AUTOBAUDDONE` reader - Auto Baud Complete Interrupt Flag"]
pub type AUTOBAUDDONE_R = crate::BitReader<bool>;
#[doc = "Field `AUTOBAUDDONE` writer - Auto Baud Complete Interrupt Flag"]
pub type AUTOBAUDDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXTO` reader - RX Timeout Interrupt Flag"]
pub type RXTO_R = crate::BitReader<bool>;
#[doc = "Field `RXTO` writer - RX Timeout Interrupt Flag"]
pub type RXTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Level Interrupt Flag"]
    #[inline(always)]
    pub fn txfl(&self) -> TXFL_R {
        TXFL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Level Interrupt Flag"]
    #[inline(always)]
    pub fn rxfl(&self) -> RXFL_R {
        RXFL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&self) -> TXUF_R {
        TXUF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Address Frame Interrupt"]
    #[inline(always)]
    pub fn mpaf(&self) -> MPAF_R {
        MPAF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Load Error Interrupt Flag"]
    #[inline(always)]
    pub fn loaderr(&self) -> LOADERR_R {
        LOADERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Collision Check Fail Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX Idle Interrupt Flag"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 18 - Start Frame Interrupt Flag"]
    #[inline(always)]
    pub fn startf(&self) -> STARTF_R {
        STARTF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Signal Frame Interrupt Flag"]
    #[inline(always)]
    pub fn sigf(&self) -> SIGF_R {
        SIGF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto Baud Complete Interrupt Flag"]
    #[inline(always)]
    pub fn autobauddone(&self) -> AUTOBAUDDONE_R {
        AUTOBAUDDONE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RX Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn rxto(&self) -> RXTO_R {
        RXTO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<0> {
        TXC_W::new(self)
    }
    #[doc = "Bit 1 - TX FIFO Level Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txfl(&mut self) -> TXFL_W<1> {
        TXFL_W::new(self)
    }
    #[doc = "Bit 2 - RX FIFO Level Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxfl(&mut self) -> RXFL_W<2> {
        RXFL_W::new(self)
    }
    #[doc = "Bit 3 - RX FIFO Full Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<3> {
        RXFULL_W::new(self)
    }
    #[doc = "Bit 4 - RX FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RXOF_W<4> {
        RXOF_W::new(self)
    }
    #[doc = "Bit 5 - RX FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<5> {
        RXUF_W::new(self)
    }
    #[doc = "Bit 6 - TX FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<6> {
        TXOF_W::new(self)
    }
    #[doc = "Bit 7 - TX FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txuf(&mut self) -> TXUF_W<7> {
        TXUF_W::new(self)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<8> {
        PERR_W::new(self)
    }
    #[doc = "Bit 9 - Framing Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<9> {
        FERR_W::new(self)
    }
    #[doc = "Bit 10 - Multi-Processor Address Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mpaf(&mut self) -> MPAF_W<10> {
        MPAF_W::new(self)
    }
    #[doc = "Bit 11 - Load Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn loaderr(&mut self) -> LOADERR_W<11> {
        LOADERR_W::new(self)
    }
    #[doc = "Bit 12 - Collision Check Fail Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CCF_W<12> {
        CCF_W::new(self)
    }
    #[doc = "Bit 13 - TX Idle Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txidle(&mut self) -> TXIDLE_W<13> {
        TXIDLE_W::new(self)
    }
    #[doc = "Bit 18 - Start Frame Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn startf(&mut self) -> STARTF_W<18> {
        STARTF_W::new(self)
    }
    #[doc = "Bit 19 - Signal Frame Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sigf(&mut self) -> SIGF_W<19> {
        SIGF_W::new(self)
    }
    #[doc = "Bit 24 - Auto Baud Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn autobauddone(&mut self) -> AUTOBAUDDONE_W<24> {
        AUTOBAUDDONE_W::new(self)
    }
    #[doc = "Bit 25 - RX Timeout Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxto(&mut self) -> RXTO_W<25> {
        RXTO_W::new(self)
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
