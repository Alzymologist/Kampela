#[doc = "Register `CALCMD` writer"]
pub struct W(crate::W<CALCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALCMD_SPEC>;
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
impl From<crate::W<CALCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALSTART` writer - Calibration Start"]
pub type CALSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALCMD_SPEC, bool, O>;
#[doc = "Field `CALSTOP` writer - Calibration Stop"]
pub type CALSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALCMD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn calstart(&mut self) -> CALSTART_W<0> {
        CALSTART_W::new(self)
    }
    #[doc = "Bit 1 - Calibration Stop"]
    #[inline(always)]
    #[must_use]
    pub fn calstop(&mut self) -> CALSTOP_W<1> {
        CALSTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calcmd](index.html) module"]
pub struct CALCMD_SPEC;
impl crate::RegisterSpec for CALCMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [calcmd::W](W) writer structure"]
impl crate::Writable for CALCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALCMD to value 0"]
impl crate::Resettable for CALCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
