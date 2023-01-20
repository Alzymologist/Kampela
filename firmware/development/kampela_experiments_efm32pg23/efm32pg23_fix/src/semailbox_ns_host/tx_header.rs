#[doc = "Register `TX_HEADER` writer"]
pub struct W(crate::W<TX_HEADER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_HEADER_SPEC>;
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
impl From<crate::W<TX_HEADER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_HEADER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXHEADER` writer - TXHEADER"]
pub type TXHEADER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_HEADER_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - TXHEADER"]
    #[inline(always)]
    #[must_use]
    pub fn txheader(&mut self) -> TXHEADER_W<0> {
        TXHEADER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A write access to this register will be mapped to the TX FIFO (only for header).\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_header](index.html) module"]
pub struct TX_HEADER_SPEC;
impl crate::RegisterSpec for TX_HEADER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tx_header::W](W) writer structure"]
impl crate::Writable for TX_HEADER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_HEADER to value 0"]
impl crate::Resettable for TX_HEADER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
