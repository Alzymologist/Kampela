#[doc = "Register `CH1F` writer"]
pub struct W(crate::W<CH1F_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1F_SPEC>;
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
impl From<crate::W<CH1F_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1F_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Channel 1 Data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1F_SPEC, u16, u16, 12, O>;
impl W {
    #[doc = "Bits 0:11 - Channel 1 Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1f](index.html) module"]
pub struct CH1F_SPEC;
impl crate::RegisterSpec for CH1F_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ch1f::W](W) writer structure"]
impl crate::Writable for CH1F_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1F to value 0"]
impl crate::Resettable for CH1F_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
