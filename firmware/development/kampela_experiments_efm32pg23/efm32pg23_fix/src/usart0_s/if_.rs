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
#[doc = "Field `TXBL` reader - TX Buffer Level Interrupt Flag"]
pub type TXBL_R = crate::BitReader<bool>;
#[doc = "Field `TXBL` writer - TX Buffer Level Interrupt Flag"]
pub type TXBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXDATAV` reader - RX Data Valid Interrupt Flag"]
pub type RXDATAV_R = crate::BitReader<bool>;
#[doc = "Field `RXDATAV` writer - RX Data Valid Interrupt Flag"]
pub type RXDATAV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXFULL` reader - RX Buffer Full Interrupt Flag"]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` writer - RX Buffer Full Interrupt Flag"]
pub type RXFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXOF` reader - RX Overflow Interrupt Flag"]
pub type RXOF_R = crate::BitReader<bool>;
#[doc = "Field `RXOF` writer - RX Overflow Interrupt Flag"]
pub type RXOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXUF` reader - RX Underflow Interrupt Flag"]
pub type RXUF_R = crate::BitReader<bool>;
#[doc = "Field `RXUF` writer - RX Underflow Interrupt Flag"]
pub type RXUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TXOF` reader - TX Overflow Interrupt Flag"]
pub type TXOF_R = crate::BitReader<bool>;
#[doc = "Field `TXOF` writer - TX Overflow Interrupt Flag"]
pub type TXOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TXUF` reader - TX Underflow Interrupt Flag"]
pub type TXUF_R = crate::BitReader<bool>;
#[doc = "Field `TXUF` writer - TX Underflow Interrupt Flag"]
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
#[doc = "Field `SSM` reader - Chip-Select In Main Mode Interrupt Flag"]
pub type SSM_R = crate::BitReader<bool>;
#[doc = "Field `SSM` writer - Chip-Select In Main Mode Interrupt Flag"]
pub type SSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `CCF` reader - Collision Check Fail Interrupt Flag"]
pub type CCF_R = crate::BitReader<bool>;
#[doc = "Field `CCF` writer - Collision Check Fail Interrupt Flag"]
pub type CCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TXIDLE` reader - TX Idle Interrupt Flag"]
pub type TXIDLE_R = crate::BitReader<bool>;
#[doc = "Field `TXIDLE` writer - TX Idle Interrupt Flag"]
pub type TXIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TCMP0` reader - Timer comparator 0 Interrupt Flag"]
pub type TCMP0_R = crate::BitReader<bool>;
#[doc = "Field `TCMP0` writer - Timer comparator 0 Interrupt Flag"]
pub type TCMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TCMP1` reader - Timer comparator 1 Interrupt Flag"]
pub type TCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TCMP1` writer - Timer comparator 1 Interrupt Flag"]
pub type TCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TCMP2` reader - Timer comparator 2 Interrupt Flag"]
pub type TCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TCMP2` writer - Timer comparator 2 Interrupt Flag"]
pub type TCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Underflow Interrupt Flag"]
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
    #[doc = "Bit 11 - Chip-Select In Main Mode Interrupt Flag"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - Timer comparator 0 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp0(&self) -> TCMP0_R {
        TCMP0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer comparator 1 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp1(&self) -> TCMP1_R {
        TCMP1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer comparator 2 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp2(&self) -> TCMP2_R {
        TCMP2_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<0> {
        TXC_W::new(self)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txbl(&mut self) -> TXBL_W<1> {
        TXBL_W::new(self)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxdatav(&mut self) -> RXDATAV_W<2> {
        RXDATAV_W::new(self)
    }
    #[doc = "Bit 3 - RX Buffer Full Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<3> {
        RXFULL_W::new(self)
    }
    #[doc = "Bit 4 - RX Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RXOF_W<4> {
        RXOF_W::new(self)
    }
    #[doc = "Bit 5 - RX Underflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<5> {
        RXUF_W::new(self)
    }
    #[doc = "Bit 6 - TX Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<6> {
        TXOF_W::new(self)
    }
    #[doc = "Bit 7 - TX Underflow Interrupt Flag"]
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
    #[doc = "Bit 11 - Chip-Select In Main Mode Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<11> {
        SSM_W::new(self)
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
    #[doc = "Bit 14 - Timer comparator 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmp0(&mut self) -> TCMP0_W<14> {
        TCMP0_W::new(self)
    }
    #[doc = "Bit 15 - Timer comparator 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmp1(&mut self) -> TCMP1_W<15> {
        TCMP1_W::new(self)
    }
    #[doc = "Bit 16 - Timer comparator 2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmp2(&mut self) -> TCMP2_W<16> {
        TCMP2_W::new(self)
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
#[doc = "`reset()` method sets IF to value 0x02"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
