#[doc = "Register `INPUTDATAHWORD` writer"]
pub struct W(crate::W<INPUTDATAHWORD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUTDATAHWORD_SPEC>;
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
impl From<crate::W<INPUTDATAHWORD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUTDATAHWORD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUTDATAHWORD` writer - Input Data for 16-bit"]
pub type INPUTDATAHWORD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INPUTDATAHWORD_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - Input Data for 16-bit"]
    #[inline(always)]
    #[must_use]
    pub fn inputdatahword(&mut self) -> INPUTDATAHWORD_W<0> {
        INPUTDATAHWORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputdatahword](index.html) module"]
pub struct INPUTDATAHWORD_SPEC;
impl crate::RegisterSpec for INPUTDATAHWORD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [inputdatahword::W](W) writer structure"]
impl crate::Writable for INPUTDATAHWORD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTDATAHWORD to value 0"]
impl crate::Resettable for INPUTDATAHWORD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
