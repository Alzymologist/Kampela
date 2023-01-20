#[doc = "Register `IRLFCFG` reader"]
pub struct R(crate::R<IRLFCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRLFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRLFCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRLFCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRLFCFG` writer"]
pub struct W(crate::W<IRLFCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRLFCFG_SPEC>;
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
impl From<crate::W<IRLFCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRLFCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRLFEN` reader - Pulse Generator/Extender Enable"]
pub type IRLFEN_R = crate::BitReader<bool>;
#[doc = "Field `IRLFEN` writer - Pulse Generator/Extender Enable"]
pub type IRLFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRLFCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    pub fn irlfen(&self) -> IRLFEN_R {
        IRLFEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irlfen(&mut self) -> IRLFEN_W<0> {
        IRLFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irlfcfg](index.html) module"]
pub struct IRLFCFG_SPEC;
impl crate::RegisterSpec for IRLFCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irlfcfg::R](R) reader structure"]
impl crate::Readable for IRLFCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irlfcfg::W](W) writer structure"]
impl crate::Writable for IRLFCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRLFCFG to value 0"]
impl crate::Resettable for IRLFCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
