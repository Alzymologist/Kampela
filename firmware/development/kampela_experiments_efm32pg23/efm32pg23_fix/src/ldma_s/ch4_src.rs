#[doc = "Register `CH4_SRC` reader"]
pub struct R(crate::R<CH4_SRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4_SRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4_SRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4_SRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH4_SRC` writer"]
pub struct W(crate::W<CH4_SRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH4_SRC_SPEC>;
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
impl From<crate::W<CH4_SRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH4_SRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCADDR` reader - Source Data Address"]
pub type SRCADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SRCADDR` writer - Source Data Address"]
pub type SRCADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH4_SRC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn srcaddr(&self) -> SRCADDR_R {
        SRCADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    #[must_use]
    pub fn srcaddr(&mut self) -> SRCADDR_W<0> {
        SRCADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_src](index.html) module"]
pub struct CH4_SRC_SPEC;
impl crate::RegisterSpec for CH4_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4_src::R](R) reader structure"]
impl crate::Readable for CH4_SRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch4_src::W](W) writer structure"]
impl crate::Writable for CH4_SRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH4_SRC to value 0"]
impl crate::Resettable for CH4_SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
