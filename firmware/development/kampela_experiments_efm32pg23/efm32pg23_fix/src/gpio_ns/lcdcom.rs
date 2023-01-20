#[doc = "Register `LCDCOM` reader"]
pub struct R(crate::R<LCDCOM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCOM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCOM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCOM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCOM` writer"]
pub struct W(crate::W<LCDCOM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCOM_SPEC>;
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
impl From<crate::W<LCDCOM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCOM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDCOMALLOC` reader - LCD Common Allocation"]
pub type LCDCOMALLOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCDCOMALLOC` writer - LCD Common Allocation"]
pub type LCDCOMALLOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCDCOM_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - LCD Common Allocation"]
    #[inline(always)]
    pub fn lcdcomalloc(&self) -> LCDCOMALLOC_R {
        LCDCOMALLOC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LCD Common Allocation"]
    #[inline(always)]
    #[must_use]
    pub fn lcdcomalloc(&mut self) -> LCDCOMALLOC_W<0> {
        LCDCOMALLOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Common Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcom](index.html) module"]
pub struct LCDCOM_SPEC;
impl crate::RegisterSpec for LCDCOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdcom::R](R) reader structure"]
impl crate::Readable for LCDCOM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcom::W](W) writer structure"]
impl crate::Writable for LCDCOM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCDCOM to value 0"]
impl crate::Resettable for LCDCOM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
