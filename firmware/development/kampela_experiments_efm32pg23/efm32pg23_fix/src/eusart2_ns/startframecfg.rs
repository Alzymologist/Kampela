#[doc = "Register `STARTFRAMECFG` reader"]
pub struct R(crate::R<STARTFRAMECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTFRAMECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTFRAMECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTFRAMECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTFRAMECFG` writer"]
pub struct W(crate::W<STARTFRAMECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTFRAMECFG_SPEC>;
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
impl From<crate::W<STARTFRAMECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTFRAMECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTFRAME` reader - Start Frame"]
pub type STARTFRAME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STARTFRAME` writer - Start Frame"]
pub type STARTFRAME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STARTFRAMECFG_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Start Frame"]
    #[inline(always)]
    pub fn startframe(&self) -> STARTFRAME_R {
        STARTFRAME_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Start Frame"]
    #[inline(always)]
    #[must_use]
    pub fn startframe(&mut self) -> STARTFRAME_W<0> {
        STARTFRAME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startframecfg](index.html) module"]
pub struct STARTFRAMECFG_SPEC;
impl crate::RegisterSpec for STARTFRAMECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [startframecfg::R](R) reader structure"]
impl crate::Readable for STARTFRAMECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [startframecfg::W](W) writer structure"]
impl crate::Writable for STARTFRAMECFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STARTFRAMECFG to value 0"]
impl crate::Resettable for STARTFRAMECFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
