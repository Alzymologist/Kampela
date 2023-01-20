#[doc = "Register `EXTIRISE` reader"]
pub struct R(crate::R<EXTIRISE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTIRISE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTIRISE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTIRISE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTIRISE` writer"]
pub struct W(crate::W<EXTIRISE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTIRISE_SPEC>;
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
impl From<crate::W<EXTIRISE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTIRISE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTIRISE` reader - EXT Int Rise"]
pub type EXTIRISE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EXTIRISE` writer - EXT Int Rise"]
pub type EXTIRISE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTIRISE_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - EXT Int Rise"]
    #[inline(always)]
    pub fn extirise(&self) -> EXTIRISE_R {
        EXTIRISE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - EXT Int Rise"]
    #[inline(always)]
    #[must_use]
    pub fn extirise(&mut self) -> EXTIRISE_W<0> {
        EXTIRISE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Rising Edge Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extirise](index.html) module"]
pub struct EXTIRISE_SPEC;
impl crate::RegisterSpec for EXTIRISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extirise::R](R) reader structure"]
impl crate::Readable for EXTIRISE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extirise::W](W) writer structure"]
impl crate::Writable for EXTIRISE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTIRISE to value 0"]
impl crate::Resettable for EXTIRISE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
