#[doc = "Register `EFPIEN` reader"]
pub struct R(crate::R<EFPIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFPIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFPIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFPIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFPIEN` writer"]
pub struct W(crate::W<EFPIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFPIEN_SPEC>;
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
impl From<crate::W<EFPIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFPIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFPIEN` reader - EFP Interrupt enable"]
pub type EFPIEN_R = crate::BitReader<bool>;
#[doc = "Field `EFPIEN` writer - EFP Interrupt enable"]
pub type EFPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFPIEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EFP Interrupt enable"]
    #[inline(always)]
    pub fn efpien(&self) -> EFPIEN_R {
        EFPIEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EFP Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn efpien(&mut self) -> EFPIEN_W<0> {
        EFPIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efpien](index.html) module"]
pub struct EFPIEN_SPEC;
impl crate::RegisterSpec for EFPIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efpien::R](R) reader structure"]
impl crate::Readable for EFPIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efpien::W](W) writer structure"]
impl crate::Writable for EFPIEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFPIEN to value 0"]
impl crate::Resettable for EFPIEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
