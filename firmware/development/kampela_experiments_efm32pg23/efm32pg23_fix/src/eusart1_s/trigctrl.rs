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
impl R {
    #[doc = "Bit 0 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RXTEN_R {
        RXTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TXTEN_R {
        TXTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&self) -> AUTOTXTEN_R {
        AUTOTXTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxten(&mut self) -> RXTEN_W<0> {
        RXTEN_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txten(&mut self) -> TXTEN_W<1> {
        TXTEN_W::new(self)
    }
    #[doc = "Bit 2 - AUTOTX Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autotxten(&mut self) -> AUTOTXTEN_W<2> {
        AUTOTXTEN_W::new(self)
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
