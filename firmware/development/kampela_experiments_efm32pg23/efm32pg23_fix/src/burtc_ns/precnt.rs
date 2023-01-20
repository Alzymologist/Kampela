#[doc = "Register `PRECNT` reader"]
pub struct R(crate::R<PRECNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRECNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRECNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRECNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRECNT` writer"]
pub struct W(crate::W<PRECNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRECNT_SPEC>;
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
impl From<crate::W<PRECNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRECNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRECNT` reader - Pre-Counter Value"]
pub type PRECNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRECNT` writer - Pre-Counter Value"]
pub type PRECNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRECNT_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    pub fn precnt(&self) -> PRECNT_R {
        PRECNT_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn precnt(&mut self) -> PRECNT_W<0> {
        PRECNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [precnt](index.html) module"]
pub struct PRECNT_SPEC;
impl crate::RegisterSpec for PRECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [precnt::R](R) reader structure"]
impl crate::Readable for PRECNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [precnt::W](W) writer structure"]
impl crate::Writable for PRECNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRECNT to value 0"]
impl crate::Resettable for PRECNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
