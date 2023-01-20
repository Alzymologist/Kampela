#[doc = "Register `MSGPTR2` reader"]
pub struct R(crate::R<MSGPTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSGPTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSGPTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSGPTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSGPTR2` writer"]
pub struct W(crate::W<MSGPTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSGPTR2_SPEC>;
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
impl From<crate::W<MSGPTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSGPTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTR` reader - Pointer"]
pub type PTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PTR` writer - Pointer"]
pub type PTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSGPTR2_SPEC, u32, u32, 32, O>;
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
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msgptr2](index.html) module"]
pub struct MSGPTR2_SPEC;
impl crate::RegisterSpec for MSGPTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msgptr2::R](R) reader structure"]
impl crate::Readable for MSGPTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msgptr2::W](W) writer structure"]
impl crate::Writable for MSGPTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSGPTR2 to value 0"]
impl crate::Resettable for MSGPTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
