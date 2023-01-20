#[doc = "Register `FRAMERATE` reader"]
pub struct R(crate::R<FRAMERATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMERATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMERATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMERATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMERATE` writer"]
pub struct W(crate::W<FRAMERATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMERATE_SPEC>;
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
impl From<crate::W<FRAMERATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMERATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRDIV` reader - Frame Rate Divider"]
pub type FRDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRDIV` writer - Frame Rate Divider"]
pub type FRDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRAMERATE_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Frame Rate Divider"]
    #[inline(always)]
    pub fn frdiv(&self) -> FRDIV_R {
        FRDIV_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Frame Rate Divider"]
    #[inline(always)]
    #[must_use]
    pub fn frdiv(&mut self) -> FRDIV_W<0> {
        FRDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framerate](index.html) module"]
pub struct FRAMERATE_SPEC;
impl crate::RegisterSpec for FRAMERATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framerate::R](R) reader structure"]
impl crate::Readable for FRAMERATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framerate::W](W) writer structure"]
impl crate::Writable for FRAMERATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMERATE to value 0"]
impl crate::Resettable for FRAMERATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
