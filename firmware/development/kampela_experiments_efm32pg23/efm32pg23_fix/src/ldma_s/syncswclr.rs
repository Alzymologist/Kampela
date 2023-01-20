#[doc = "Register `SYNCSWCLR` writer"]
pub struct W(crate::W<SYNCSWCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCSWCLR_SPEC>;
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
impl From<crate::W<SYNCSWCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCSWCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCSWCLR` writer - DMA SYNC Software Trigger Clear"]
pub type SYNCSWCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNCSWCLR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - DMA SYNC Software Trigger Clear"]
    #[inline(always)]
    #[must_use]
    pub fn syncswclr(&mut self) -> SYNCSWCLR_W<0> {
        SYNCSWCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncswclr](index.html) module"]
pub struct SYNCSWCLR_SPEC;
impl crate::RegisterSpec for SYNCSWCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [syncswclr::W](W) writer structure"]
impl crate::Writable for SYNCSWCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCSWCLR to value 0"]
impl crate::Resettable for SYNCSWCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
