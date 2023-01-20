#[doc = "Register `GRP0_CMP0VALUE` reader"]
pub struct R(crate::R<GRP0_CMP0VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRP0_CMP0VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRP0_CMP0VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRP0_CMP0VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GRP0_CMP0VALUE` writer"]
pub struct W(crate::W<GRP0_CMP0VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRP0_CMP0VALUE_SPEC>;
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
impl From<crate::W<GRP0_CMP0VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRP0_CMP0VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP0VALUE` reader - Compare 0 Value"]
pub type CMP0VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMP0VALUE` writer - Compare 0 Value"]
pub type CMP0VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GRP0_CMP0VALUE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Compare 0 Value"]
    #[inline(always)]
    pub fn cmp0value(&self) -> CMP0VALUE_R {
        CMP0VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare 0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0value(&mut self) -> CMP0VALUE_W<0> {
        CMP0VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grp0_cmp0value](index.html) module"]
pub struct GRP0_CMP0VALUE_SPEC;
impl crate::RegisterSpec for GRP0_CMP0VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grp0_cmp0value::R](R) reader structure"]
impl crate::Readable for GRP0_CMP0VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [grp0_cmp0value::W](W) writer structure"]
impl crate::Writable for GRP0_CMP0VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GRP0_CMP0VALUE to value 0"]
impl crate::Resettable for GRP0_CMP0VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
