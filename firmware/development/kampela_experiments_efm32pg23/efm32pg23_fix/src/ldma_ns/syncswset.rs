#[doc = "Register `SYNCSWSET` writer"]
pub struct W(crate::W<SYNCSWSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCSWSET_SPEC>;
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
impl From<crate::W<SYNCSWSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCSWSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCSWSET` writer - DMA SYNC Software Trigger Set"]
pub type SYNCSWSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNCSWSET_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - DMA SYNC Software Trigger Set"]
    #[inline(always)]
    #[must_use]
    pub fn syncswset(&mut self) -> SYNCSWSET_W<0> {
        SYNCSWSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncswset](index.html) module"]
pub struct SYNCSWSET_SPEC;
impl crate::RegisterSpec for SYNCSWSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [syncswset::W](W) writer structure"]
impl crate::Writable for SYNCSWSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCSWSET to value 0"]
impl crate::Resettable for SYNCSWSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
