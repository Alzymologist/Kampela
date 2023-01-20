#[doc = "Register `SREG1` reader"]
pub struct R(crate::R<SREG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SREG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SREG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SREG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SREG1` writer"]
pub struct W(crate::W<SREG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SREG1_SPEC>;
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
impl From<crate::W<SREG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SREG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRATCH` reader - Scratch Register"]
pub type SCRATCH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCRATCH` writer - Scratch Register"]
pub type SCRATCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SREG1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Scratch Register"]
    #[inline(always)]
    pub fn scratch(&self) -> SCRATCH_R {
        SCRATCH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scratch Register"]
    #[inline(always)]
    #[must_use]
    pub fn scratch(&mut self) -> SCRATCH_W<0> {
        SCRATCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Used for SIMCTRL Data Access in Verification Environment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sreg1](index.html) module"]
pub struct SREG1_SPEC;
impl crate::RegisterSpec for SREG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sreg1::R](R) reader structure"]
impl crate::Readable for SREG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sreg1::W](W) writer structure"]
impl crate::Writable for SREG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SREG1 to value 0"]
impl crate::Resettable for SREG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
