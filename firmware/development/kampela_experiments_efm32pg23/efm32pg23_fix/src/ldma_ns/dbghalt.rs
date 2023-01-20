#[doc = "Register `DBGHALT` reader"]
pub struct R(crate::R<DBGHALT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGHALT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGHALT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGHALT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGHALT` writer"]
pub struct W(crate::W<DBGHALT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGHALT_SPEC>;
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
impl From<crate::W<DBGHALT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGHALT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGHALT` reader - DMA Debug Halt"]
pub type DBGHALT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBGHALT` writer - DMA Debug Halt"]
pub type DBGHALT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBGHALT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DMA Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Debug Halt"]
    #[inline(always)]
    #[must_use]
    pub fn dbghalt(&mut self) -> DBGHALT_W<0> {
        DBGHALT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbghalt](index.html) module"]
pub struct DBGHALT_SPEC;
impl crate::RegisterSpec for DBGHALT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbghalt::R](R) reader structure"]
impl crate::Readable for DBGHALT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbghalt::W](W) writer structure"]
impl crate::Writable for DBGHALT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGHALT to value 0"]
impl crate::Resettable for DBGHALT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
