#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RXBLOCKEN` writer - Receiver Block Enable"]
pub type RXBLOCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RXBLOCKDIS` writer - Receiver Block Disable"]
pub type RXBLOCKDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TXTRIEN` writer - Transmitter Tristate Enable"]
pub type TXTRIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TXTRIDIS` writer - Transmitter Tristate Disable"]
pub type TXTRIDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CLEARTX` writer - Clear TX FIFO"]
pub type CLEARTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<0> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RXDIS_W<1> {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 2 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<2> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TXDIS_W<3> {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 4 - Receiver Block Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxblocken(&mut self) -> RXBLOCKEN_W<4> {
        RXBLOCKEN_W::new(self)
    }
    #[doc = "Bit 5 - Receiver Block Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxblockdis(&mut self) -> RXBLOCKDIS_W<5> {
        RXBLOCKDIS_W::new(self)
    }
    #[doc = "Bit 6 - Transmitter Tristate Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txtrien(&mut self) -> TXTRIEN_W<6> {
        TXTRIEN_W::new(self)
    }
    #[doc = "Bit 7 - Transmitter Tristate Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txtridis(&mut self) -> TXTRIDIS_W<7> {
        TXTRIDIS_W::new(self)
    }
    #[doc = "Bit 8 - Clear TX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn cleartx(&mut self) -> CLEARTX_W<8> {
        CLEARTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
