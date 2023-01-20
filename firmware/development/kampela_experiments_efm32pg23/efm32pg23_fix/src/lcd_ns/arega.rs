#[doc = "Register `AREGA` reader"]
pub struct R(crate::R<AREGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AREGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AREGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AREGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AREGA` writer"]
pub struct W(crate::W<AREGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AREGA_SPEC>;
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
impl From<crate::W<AREGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AREGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AREGA` reader - Animation Register A Data"]
pub type AREGA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AREGA` writer - Animation Register A Data"]
pub type AREGA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AREGA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Animation Register A Data"]
    #[inline(always)]
    pub fn arega(&self) -> AREGA_R {
        AREGA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Animation Register A Data"]
    #[inline(always)]
    #[must_use]
    pub fn arega(&mut self) -> AREGA_W<0> {
        AREGA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arega](index.html) module"]
pub struct AREGA_SPEC;
impl crate::RegisterSpec for AREGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arega::R](R) reader structure"]
impl crate::Readable for AREGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arega::W](W) writer structure"]
impl crate::Writable for AREGA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AREGA to value 0"]
impl crate::Resettable for AREGA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
