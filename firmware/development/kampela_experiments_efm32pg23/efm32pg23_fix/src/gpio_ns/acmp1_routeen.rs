#[doc = "Register `ACMP1_ROUTEEN` reader"]
pub struct R(crate::R<ACMP1_ROUTEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMP1_ROUTEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACMP1_ROUTEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACMP1_ROUTEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACMP1_ROUTEEN` writer"]
pub struct W(crate::W<ACMP1_ROUTEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMP1_ROUTEEN_SPEC>;
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
impl From<crate::W<ACMP1_ROUTEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACMP1_ROUTEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACMPOUTPEN` reader - ACMPOUT pin enable control bit"]
pub type ACMPOUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `ACMPOUTPEN` writer - ACMPOUT pin enable control bit"]
pub type ACMPOUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMP1_ROUTEEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ACMPOUT pin enable control bit"]
    #[inline(always)]
    pub fn acmpoutpen(&self) -> ACMPOUTPEN_R {
        ACMPOUTPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACMPOUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn acmpoutpen(&mut self) -> ACMPOUTPEN_W<0> {
        ACMPOUTPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ACMP1 pin enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmp1_routeen](index.html) module"]
pub struct ACMP1_ROUTEEN_SPEC;
impl crate::RegisterSpec for ACMP1_ROUTEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acmp1_routeen::R](R) reader structure"]
impl crate::Readable for ACMP1_ROUTEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acmp1_routeen::W](W) writer structure"]
impl crate::Writable for ACMP1_ROUTEEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACMP1_ROUTEEN to value 0"]
impl crate::Resettable for ACMP1_ROUTEEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
