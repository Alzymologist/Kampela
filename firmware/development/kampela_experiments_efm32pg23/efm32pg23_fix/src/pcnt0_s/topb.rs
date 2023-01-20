#[doc = "Register `TOPB` reader"]
pub struct R(crate::R<TOPB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOPB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOPB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOPB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOPB` writer"]
pub struct W(crate::W<TOPB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOPB_SPEC>;
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
impl From<crate::W<TOPB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOPB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOPB` reader - Counter Top Buffer Register"]
pub type TOPB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOPB` writer - Counter Top Buffer Register"]
pub type TOPB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOPB_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Counter Top Buffer Register"]
    #[inline(always)]
    pub fn topb(&self) -> TOPB_R {
        TOPB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Top Buffer Register"]
    #[inline(always)]
    #[must_use]
    pub fn topb(&mut self) -> TOPB_W<0> {
        TOPB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [topb](index.html) module"]
pub struct TOPB_SPEC;
impl crate::RegisterSpec for TOPB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [topb::R](R) reader structure"]
impl crate::Readable for TOPB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [topb::W](W) writer structure"]
impl crate::Writable for TOPB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOPB to value 0xff"]
impl crate::Resettable for TOPB_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
