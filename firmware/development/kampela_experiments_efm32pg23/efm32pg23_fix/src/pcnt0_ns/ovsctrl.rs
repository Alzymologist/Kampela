#[doc = "Register `OVSCTRL` reader"]
pub struct R(crate::R<OVSCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVSCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVSCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVSCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVSCTRL` writer"]
pub struct W(crate::W<OVSCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVSCTRL_SPEC>;
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
impl From<crate::W<OVSCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVSCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTLEN` reader - Configure Filter Length for Inputs S0IN"]
pub type FILTLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTLEN` writer - Configure Filter Length for Inputs S0IN"]
pub type FILTLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OVSCTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `FLUTTERRM` reader - Flutter Remove"]
pub type FLUTTERRM_R = crate::BitReader<bool>;
#[doc = "Field `FLUTTERRM` writer - Flutter Remove"]
pub type FLUTTERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVSCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Configure Filter Length for Inputs S0IN"]
    #[inline(always)]
    pub fn filtlen(&self) -> FILTLEN_R {
        FILTLEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Flutter Remove"]
    #[inline(always)]
    pub fn flutterrm(&self) -> FLUTTERRM_R {
        FLUTTERRM_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configure Filter Length for Inputs S0IN"]
    #[inline(always)]
    #[must_use]
    pub fn filtlen(&mut self) -> FILTLEN_W<0> {
        FILTLEN_W::new(self)
    }
    #[doc = "Bit 12 - Flutter Remove"]
    #[inline(always)]
    #[must_use]
    pub fn flutterrm(&mut self) -> FLUTTERRM_W<12> {
        FLUTTERRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovsctrl](index.html) module"]
pub struct OVSCTRL_SPEC;
impl crate::RegisterSpec for OVSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ovsctrl::R](R) reader structure"]
impl crate::Readable for OVSCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ovsctrl::W](W) writer structure"]
impl crate::Writable for OVSCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OVSCTRL to value 0"]
impl crate::Resettable for OVSCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
