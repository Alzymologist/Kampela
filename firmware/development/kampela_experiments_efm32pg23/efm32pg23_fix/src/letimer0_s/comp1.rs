#[doc = "Register `COMP1` reader"]
pub struct R(crate::R<COMP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP1` writer"]
pub struct W(crate::W<COMP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_SPEC>;
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
impl From<crate::W<COMP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1` reader - Compare Value 1"]
pub type COMP1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMP1` writer - Compare Value 1"]
pub type COMP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Compare Value 1"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Compare Value 1"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> COMP1_W<0> {
        COMP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1](index.html) module"]
pub struct COMP1_SPEC;
impl crate::RegisterSpec for COMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1::R](R) reader structure"]
impl crate::Readable for COMP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp1::W](W) writer structure"]
impl crate::Writable for COMP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP1 to value 0"]
impl crate::Resettable for COMP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
