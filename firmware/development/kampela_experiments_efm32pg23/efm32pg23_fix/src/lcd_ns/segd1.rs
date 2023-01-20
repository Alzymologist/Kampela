#[doc = "Register `SEGD1` reader"]
pub struct R(crate::R<SEGD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD1` writer"]
pub struct W(crate::W<SEGD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD1_SPEC>;
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
impl From<crate::W<SEGD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD1` reader - COM1 Segment Data Low"]
pub type SEGD1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEGD1` writer - COM1 Segment Data Low"]
pub type SEGD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEGD1_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - COM1 Segment Data Low"]
    #[inline(always)]
    pub fn segd1(&self) -> SEGD1_R {
        SEGD1_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - COM1 Segment Data Low"]
    #[inline(always)]
    #[must_use]
    pub fn segd1(&mut self) -> SEGD1_W<0> {
        SEGD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd1](index.html) module"]
pub struct SEGD1_SPEC;
impl crate::RegisterSpec for SEGD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd1::R](R) reader structure"]
impl crate::Readable for SEGD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd1::W](W) writer structure"]
impl crate::Writable for SEGD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD1 to value 0"]
impl crate::Resettable for SEGD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
