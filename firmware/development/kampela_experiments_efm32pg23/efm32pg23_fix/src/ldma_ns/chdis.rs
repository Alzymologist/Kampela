#[doc = "Register `CHDIS` writer"]
pub struct W(crate::W<CHDIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHDIS_SPEC>;
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
impl From<crate::W<CHDIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHDIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHDIS` writer - DMA Channel disable"]
pub type CHDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHDIS_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - DMA Channel disable"]
    #[inline(always)]
    #[must_use]
    pub fn chdis(&mut self) -> CHDIS_W<0> {
        CHDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdis](index.html) module"]
pub struct CHDIS_SPEC;
impl crate::RegisterSpec for CHDIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chdis::W](W) writer structure"]
impl crate::Writable for CHDIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHDIS to value 0"]
impl crate::Resettable for CHDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
