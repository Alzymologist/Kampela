#[doc = "Register `TOPBUFF` reader"]
pub struct R(crate::R<TOPBUFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOPBUFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOPBUFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOPBUFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOPBUFF` writer"]
pub struct W(crate::W<TOPBUFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOPBUFF_SPEC>;
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
impl From<crate::W<TOPBUFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOPBUFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOPBUFF` reader - Buffered Counter TOP Value"]
pub type TOPBUFF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TOPBUFF` writer - Buffered Counter TOP Value"]
pub type TOPBUFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOPBUFF_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Buffered Counter TOP Value"]
    #[inline(always)]
    pub fn topbuff(&self) -> TOPBUFF_R {
        TOPBUFF_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Buffered Counter TOP Value"]
    #[inline(always)]
    #[must_use]
    pub fn topbuff(&mut self) -> TOPBUFF_W<0> {
        TOPBUFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [topbuff](index.html) module"]
pub struct TOPBUFF_SPEC;
impl crate::RegisterSpec for TOPBUFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [topbuff::R](R) reader structure"]
impl crate::Readable for TOPBUFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [topbuff::W](W) writer structure"]
impl crate::Writable for TOPBUFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOPBUFF to value 0"]
impl crate::Resettable for TOPBUFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
