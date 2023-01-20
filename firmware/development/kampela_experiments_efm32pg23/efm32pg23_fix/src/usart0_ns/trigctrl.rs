#[doc = "Register `TRIGCTRL` reader"]
pub struct R(crate::R<TRIGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIGCTRL` writer"]
pub struct W(crate::W<TRIGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIGCTRL_SPEC>;
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
impl From<crate::W<TRIGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXTEN` reader - Receive Trigger Enable"]
pub type RXTEN_R = crate::BitReader<bool>;
#[doc = "Field `RXTEN` writer - Receive Trigger Enable"]
pub type RXTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, O>;
#[doc = "Field `TXTEN` reader - Transmit Trigger Enable"]
pub type TXTEN_R = crate::BitReader<bool>;
#[doc = "Field `TXTEN` writer - Transmit Trigger Enable"]
pub type TXTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, O>;
#[doc = "Field `AUTOTXTEN` reader - AUTOTX Trigger Enable"]
pub type AUTOTXTEN_R = crate::BitReader<bool>;
#[doc = "Field `AUTOTXTEN` writer - AUTOTX Trigger Enable"]
pub type AUTOTXTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, O>;
#[doc = "Field `TXARX0EN` reader - Enable Transmit Trigger after RX End of"]
pub type TXARX0EN_R = crate::BitReader<bool>;
#[doc = "Field `TXARX0EN` writer - Enable Transmit Trigger after RX End of"]
pub type TXARX0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, O>;
#[doc = "Field `TXARX1EN` reader - Enable Transmit Trigger after RX End of"]
pub type TXARX1EN_R = crate::BitReader<bool>;
#[doc = "Field `TXARX1EN` writer - Enable Transmit Trigger after RX End of"]
pub type TXARX1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, O>;
#[doc = "Field `TXARX2EN` reader - Enable Transmit Trigger after RX End of"]
pub type TXARX2EN_R = crate::BitReader<bool>;
#[doc = "Field `TXARX2EN` writer - Enable Transmit Trigger after RX End of"]
pub type TXARX2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, O>;
#[doc = "Field `RXATX0EN` reader - Enable Receive Trigger after TX end of f"]
pub type RXATX0EN_R = crate::BitReader<bool>;
#[doc = "Field `RXATX0EN` writer - Enable Receive Trigger after TX end of f"]
pub type RXATX0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, O>;
#[doc = "Field `RXATX1EN` reader - Enable Receive Trigger after TX end of f"]
pub type RXATX1EN_R = crate::BitReader<bool>;
#[doc = "Field `RXATX1EN` writer - Enable Receive Trigger after TX end of f"]
pub type RXATX1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, O>;
#[doc = "Field `RXATX2EN` reader - Enable Receive Trigger after TX end of f"]
pub type RXATX2EN_R = crate::BitReader<bool>;
#[doc = "Field `RXATX2EN` writer - Enable Receive Trigger after TX end of f"]
pub type RXATX2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RXTEN_R {
        RXTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TXTEN_R {
        TXTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&self) -> AUTOTXTEN_R {
        AUTOTXTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Transmit Trigger after RX End of"]
    #[inline(always)]
    pub fn txarx0en(&self) -> TXARX0EN_R {
        TXARX0EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Transmit Trigger after RX End of"]
    #[inline(always)]
    pub fn txarx1en(&self) -> TXARX1EN_R {
        TXARX1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Transmit Trigger after RX End of"]
    #[inline(always)]
    pub fn txarx2en(&self) -> TXARX2EN_R {
        TXARX2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Receive Trigger after TX end of f"]
    #[inline(always)]
    pub fn rxatx0en(&self) -> RXATX0EN_R {
        RXATX0EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Receive Trigger after TX end of f"]
    #[inline(always)]
    pub fn rxatx1en(&self) -> RXATX1EN_R {
        RXATX1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Receive Trigger after TX end of f"]
    #[inline(always)]
    pub fn rxatx2en(&self) -> RXATX2EN_R {
        RXATX2EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxten(&mut self) -> RXTEN_W<4> {
        RXTEN_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txten(&mut self) -> TXTEN_W<5> {
        TXTEN_W::new(self)
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autotxten(&mut self) -> AUTOTXTEN_W<6> {
        AUTOTXTEN_W::new(self)
    }
    #[doc = "Bit 7 - Enable Transmit Trigger after RX End of"]
    #[inline(always)]
    #[must_use]
    pub fn txarx0en(&mut self) -> TXARX0EN_W<7> {
        TXARX0EN_W::new(self)
    }
    #[doc = "Bit 8 - Enable Transmit Trigger after RX End of"]
    #[inline(always)]
    #[must_use]
    pub fn txarx1en(&mut self) -> TXARX1EN_W<8> {
        TXARX1EN_W::new(self)
    }
    #[doc = "Bit 9 - Enable Transmit Trigger after RX End of"]
    #[inline(always)]
    #[must_use]
    pub fn txarx2en(&mut self) -> TXARX2EN_W<9> {
        TXARX2EN_W::new(self)
    }
    #[doc = "Bit 10 - Enable Receive Trigger after TX end of f"]
    #[inline(always)]
    #[must_use]
    pub fn rxatx0en(&mut self) -> RXATX0EN_W<10> {
        RXATX0EN_W::new(self)
    }
    #[doc = "Bit 11 - Enable Receive Trigger after TX end of f"]
    #[inline(always)]
    #[must_use]
    pub fn rxatx1en(&mut self) -> RXATX1EN_W<11> {
        RXATX1EN_W::new(self)
    }
    #[doc = "Bit 12 - Enable Receive Trigger after TX end of f"]
    #[inline(always)]
    #[must_use]
    pub fn rxatx2en(&mut self) -> RXATX2EN_W<12> {
        RXATX2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigctrl](index.html) module"]
pub struct TRIGCTRL_SPEC;
impl crate::RegisterSpec for TRIGCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigctrl::R](R) reader structure"]
impl crate::Readable for TRIGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trigctrl::W](W) writer structure"]
impl crate::Writable for TRIGCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIGCTRL to value 0"]
impl crate::Resettable for TRIGCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
