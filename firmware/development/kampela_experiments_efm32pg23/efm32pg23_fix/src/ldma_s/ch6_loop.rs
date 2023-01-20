#[doc = "Register `CH6_LOOP` reader"]
pub struct R(crate::R<CH6_LOOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH6_LOOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH6_LOOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH6_LOOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH6_LOOP` writer"]
pub struct W(crate::W<CH6_LOOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH6_LOOP_SPEC>;
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
impl From<crate::W<CH6_LOOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH6_LOOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOOPCNT` reader - Linked Structure Sequence Loop Counter"]
pub type LOOPCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOOPCNT` writer - Linked Structure Sequence Loop Counter"]
pub type LOOPCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH6_LOOP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    pub fn loopcnt(&self) -> LOOPCNT_R {
        LOOPCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    #[must_use]
    pub fn loopcnt(&mut self) -> LOOPCNT_W<0> {
        LOOPCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_loop](index.html) module"]
pub struct CH6_LOOP_SPEC;
impl crate::RegisterSpec for CH6_LOOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch6_loop::R](R) reader structure"]
impl crate::Readable for CH6_LOOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch6_loop::W](W) writer structure"]
impl crate::Writable for CH6_LOOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH6_LOOP to value 0"]
impl crate::Resettable for CH6_LOOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
