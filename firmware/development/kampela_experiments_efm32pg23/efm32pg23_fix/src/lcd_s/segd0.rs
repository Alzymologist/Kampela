#[doc = "Register `SEGD0` reader"]
pub struct R(crate::R<SEGD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD0` writer"]
pub struct W(crate::W<SEGD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD0_SPEC>;
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
impl From<crate::W<SEGD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD0` reader - COM0 Segment Data Low"]
pub type SEGD0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEGD0` writer - COM0 Segment Data Low"]
pub type SEGD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEGD0_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - COM0 Segment Data Low"]
    #[inline(always)]
    pub fn segd0(&self) -> SEGD0_R {
        SEGD0_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - COM0 Segment Data Low"]
    #[inline(always)]
    #[must_use]
    pub fn segd0(&mut self) -> SEGD0_W<0> {
        SEGD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd0](index.html) module"]
pub struct SEGD0_SPEC;
impl crate::RegisterSpec for SEGD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd0::R](R) reader structure"]
impl crate::Readable for SEGD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd0::W](W) writer structure"]
impl crate::Writable for SEGD0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD0 to value 0"]
impl crate::Resettable for SEGD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
