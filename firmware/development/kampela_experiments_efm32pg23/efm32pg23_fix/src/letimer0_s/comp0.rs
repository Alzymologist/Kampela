#[doc = "Register `COMP0` reader"]
pub struct R(crate::R<COMP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP0` writer"]
pub struct W(crate::W<COMP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP0_SPEC>;
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
impl From<crate::W<COMP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP0` reader - Compare Value 0"]
pub type COMP0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMP0` writer - Compare Value 0"]
pub type COMP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP0_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Compare Value 0"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Compare Value 0"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> COMP0_W<0> {
        COMP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp0](index.html) module"]
pub struct COMP0_SPEC;
impl crate::RegisterSpec for COMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp0::R](R) reader structure"]
impl crate::Readable for COMP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp0::W](W) writer structure"]
impl crate::Writable for COMP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP0 to value 0"]
impl crate::Resettable for COMP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
