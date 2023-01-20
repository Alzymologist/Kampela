#[doc = "Register `SADDRMASK` reader"]
pub struct R(crate::R<SADDRMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SADDRMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SADDRMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SADDRMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SADDRMASK` writer"]
pub struct W(crate::W<SADDRMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SADDRMASK_SPEC>;
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
impl From<crate::W<SADDRMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SADDRMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADDRMASK` reader - Follower Address Mask"]
pub type SADDRMASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SADDRMASK` writer - Follower Address Mask"]
pub type SADDRMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SADDRMASK_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 1:7 - Follower Address Mask"]
    #[inline(always)]
    pub fn saddrmask(&self) -> SADDRMASK_R {
        SADDRMASK_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Follower Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn saddrmask(&mut self) -> SADDRMASK_W<1> {
        SADDRMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddrmask](index.html) module"]
pub struct SADDRMASK_SPEC;
impl crate::RegisterSpec for SADDRMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saddrmask::R](R) reader structure"]
impl crate::Readable for SADDRMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saddrmask::W](W) writer structure"]
impl crate::Writable for SADDRMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SADDRMASK to value 0"]
impl crate::Resettable for SADDRMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
