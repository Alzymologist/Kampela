#[doc = "Register `EXTIFALL` reader"]
pub struct R(crate::R<EXTIFALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTIFALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTIFALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTIFALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTIFALL` writer"]
pub struct W(crate::W<EXTIFALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTIFALL_SPEC>;
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
impl From<crate::W<EXTIFALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTIFALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTIFALL` reader - EXT Int FALL"]
pub type EXTIFALL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EXTIFALL` writer - EXT Int FALL"]
pub type EXTIFALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTIFALL_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - EXT Int FALL"]
    #[inline(always)]
    pub fn extifall(&self) -> EXTIFALL_R {
        EXTIFALL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - EXT Int FALL"]
    #[inline(always)]
    #[must_use]
    pub fn extifall(&mut self) -> EXTIFALL_W<0> {
        EXTIFALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Falling Edge Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extifall](index.html) module"]
pub struct EXTIFALL_SPEC;
impl crate::RegisterSpec for EXTIFALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extifall::R](R) reader structure"]
impl crate::Readable for EXTIFALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extifall::W](W) writer structure"]
impl crate::Writable for EXTIFALL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTIFALL to value 0"]
impl crate::Resettable for EXTIFALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
