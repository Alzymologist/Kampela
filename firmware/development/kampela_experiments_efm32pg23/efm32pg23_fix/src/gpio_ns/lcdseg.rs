#[doc = "Register `LCDSEG` reader"]
pub struct R(crate::R<LCDSEG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDSEG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDSEG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDSEG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDSEG` writer"]
pub struct W(crate::W<LCDSEG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDSEG_SPEC>;
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
impl From<crate::W<LCDSEG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDSEG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDSEGALLOC` reader - LCD Segment Allocation"]
pub type LCDSEGALLOC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LCDSEGALLOC` writer - LCD Segment Allocation"]
pub type LCDSEGALLOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCDSEG_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - LCD Segment Allocation"]
    #[inline(always)]
    pub fn lcdsegalloc(&self) -> LCDSEGALLOC_R {
        LCDSEGALLOC_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - LCD Segment Allocation"]
    #[inline(always)]
    #[must_use]
    pub fn lcdsegalloc(&mut self) -> LCDSEGALLOC_W<0> {
        LCDSEGALLOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Segment Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdseg](index.html) module"]
pub struct LCDSEG_SPEC;
impl crate::RegisterSpec for LCDSEG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdseg::R](R) reader structure"]
impl crate::Readable for LCDSEG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdseg::W](W) writer structure"]
impl crate::Writable for LCDSEG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCDSEG to value 0"]
impl crate::Resettable for LCDSEG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
