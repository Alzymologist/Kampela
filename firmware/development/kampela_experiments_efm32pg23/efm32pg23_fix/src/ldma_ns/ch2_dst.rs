#[doc = "Register `CH2_DST` reader"]
pub struct R(crate::R<CH2_DST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2_DST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2_DST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2_DST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2_DST` writer"]
pub struct W(crate::W<CH2_DST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2_DST_SPEC>;
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
impl From<crate::W<CH2_DST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2_DST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTADDR` reader - Destination Data Address"]
pub type DSTADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSTADDR` writer - Destination Data Address"]
pub type DSTADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2_DST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    pub fn dstaddr(&self) -> DSTADDR_R {
        DSTADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    #[must_use]
    pub fn dstaddr(&mut self) -> DSTADDR_W<0> {
        DSTADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_dst](index.html) module"]
pub struct CH2_DST_SPEC;
impl crate::RegisterSpec for CH2_DST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2_dst::R](R) reader structure"]
impl crate::Readable for CH2_DST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2_dst::W](W) writer structure"]
impl crate::Writable for CH2_DST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2_DST to value 0"]
impl crate::Resettable for CH2_DST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
