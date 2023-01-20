#[doc = "Register `INPUTDATABYTE` writer"]
pub struct W(crate::W<INPUTDATABYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUTDATABYTE_SPEC>;
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
impl From<crate::W<INPUTDATABYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUTDATABYTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUTDATABYTE` writer - Input Data for 8-bit"]
pub type INPUTDATABYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INPUTDATABYTE_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Input Data for 8-bit"]
    #[inline(always)]
    #[must_use]
    pub fn inputdatabyte(&mut self) -> INPUTDATABYTE_W<0> {
        INPUTDATABYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputdatabyte](index.html) module"]
pub struct INPUTDATABYTE_SPEC;
impl crate::RegisterSpec for INPUTDATABYTE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [inputdatabyte::W](W) writer structure"]
impl crate::Writable for INPUTDATABYTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTDATABYTE to value 0"]
impl crate::Resettable for INPUTDATABYTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
