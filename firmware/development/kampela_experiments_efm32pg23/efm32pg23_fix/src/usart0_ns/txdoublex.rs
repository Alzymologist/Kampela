#[doc = "Register `TXDOUBLEX` writer"]
pub struct W(crate::W<TXDOUBLEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDOUBLEX_SPEC>;
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
impl From<crate::W<TXDOUBLEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDOUBLEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA0` writer - TX Data"]
pub type TXDATA0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDOUBLEX_SPEC, u16, u16, 9, O>;
#[doc = "Field `UBRXAT0` writer - Unblock RX After Transmission"]
pub type UBRXAT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, O>;
#[doc = "Field `TXTRIAT0` writer - Set TXTRI After Transmission"]
pub type TXTRIAT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, O>;
#[doc = "Field `TXBREAK0` writer - Transmit Data As Break"]
pub type TXBREAK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, O>;
#[doc = "Field `TXDISAT0` writer - Clear TXEN After Transmission"]
pub type TXDISAT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, O>;
#[doc = "Field `RXENAT0` writer - Enable RX After Transmission"]
pub type RXENAT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, O>;
#[doc = "Field `TXDATA1` writer - TX Data"]
pub type TXDATA1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDOUBLEX_SPEC, u16, u16, 9, O>;
#[doc = "Field `UBRXAT1` writer - Unblock RX After Transmission"]
pub type UBRXAT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, O>;
#[doc = "Field `TXTRIAT1` writer - Set TXTRI After Transmission"]
pub type TXTRIAT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, O>;
#[doc = "Field `TXBREAK1` writer - Transmit Data As Break"]
pub type TXBREAK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, O>;
#[doc = "Field `TXDISAT1` writer - Clear TXEN After Transmission"]
pub type TXDISAT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, O>;
#[doc = "Field `RXENAT1` writer - Enable RX After Transmission"]
pub type RXENAT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata0(&mut self) -> TXDATA0_W<0> {
        TXDATA0_W::new(self)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ubrxat0(&mut self) -> UBRXAT0_W<11> {
        UBRXAT0_W::new(self)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txtriat0(&mut self) -> TXTRIAT0_W<12> {
        TXTRIAT0_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Data As Break"]
    #[inline(always)]
    #[must_use]
    pub fn txbreak0(&mut self) -> TXBREAK0_W<13> {
        TXBREAK0_W::new(self)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txdisat0(&mut self) -> TXDISAT0_W<14> {
        TXDISAT0_W::new(self)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn rxenat0(&mut self) -> RXENAT0_W<15> {
        RXENAT0_W::new(self)
    }
    #[doc = "Bits 16:24 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata1(&mut self) -> TXDATA1_W<16> {
        TXDATA1_W::new(self)
    }
    #[doc = "Bit 27 - Unblock RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ubrxat1(&mut self) -> UBRXAT1_W<27> {
        UBRXAT1_W::new(self)
    }
    #[doc = "Bit 28 - Set TXTRI After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txtriat1(&mut self) -> TXTRIAT1_W<28> {
        TXTRIAT1_W::new(self)
    }
    #[doc = "Bit 29 - Transmit Data As Break"]
    #[inline(always)]
    #[must_use]
    pub fn txbreak1(&mut self) -> TXBREAK1_W<29> {
        TXBREAK1_W::new(self)
    }
    #[doc = "Bit 30 - Clear TXEN After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txdisat1(&mut self) -> TXDISAT1_W<30> {
        TXDISAT1_W::new(self)
    }
    #[doc = "Bit 31 - Enable RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn rxenat1(&mut self) -> RXENAT1_W<31> {
        RXENAT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdoublex](index.html) module"]
pub struct TXDOUBLEX_SPEC;
impl crate::RegisterSpec for TXDOUBLEX_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txdoublex::W](W) writer structure"]
impl crate::Writable for TXDOUBLEX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDOUBLEX to value 0"]
impl crate::Resettable for TXDOUBLEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
