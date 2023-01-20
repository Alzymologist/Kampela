#[doc = "Register `MSGPTR1` reader"]
pub struct R(crate::R<MSGPTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSGPTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSGPTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSGPTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSGPTR1` writer"]
pub struct W(crate::W<MSGPTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSGPTR1_SPEC>;
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
impl From<crate::W<MSGPTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSGPTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTR` reader - Pointer"]
pub type PTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PTR` writer - Pointer"]
pub type PTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSGPTR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Pointer"]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn ptr(&mut self) -> PTR_W<0> {
        PTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msgptr1](index.html) module"]
pub struct MSGPTR1_SPEC;
impl crate::RegisterSpec for MSGPTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msgptr1::R](R) reader structure"]
impl crate::Readable for MSGPTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msgptr1::W](W) writer structure"]
impl crate::Writable for MSGPTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSGPTR1 to value 0"]
impl crate::Resettable for MSGPTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
