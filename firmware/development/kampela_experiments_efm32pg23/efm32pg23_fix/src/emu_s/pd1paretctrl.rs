#[doc = "Register `PD1PARETCTRL` reader"]
pub struct R(crate::R<PD1PARETCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD1PARETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD1PARETCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD1PARETCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD1PARETCTRL` writer"]
pub struct W(crate::W<PD1PARETCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD1PARETCTRL_SPEC>;
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
impl From<crate::W<PD1PARETCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD1PARETCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD1PARETDIS` reader - Disable PD1 Partial Retention"]
pub type PD1PARETDIS_R = crate::FieldReader<u16, PD1PARETDIS_A>;
#[doc = "Disable PD1 Partial Retention\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PD1PARETDIS_A {
    #[doc = "1: Retain associated registers when in EM2/3"]
    PERIPHNORETAIN = 1,
}
impl From<PD1PARETDIS_A> for u16 {
    #[inline(always)]
    fn from(variant: PD1PARETDIS_A) -> Self {
        variant as _
    }
}
impl PD1PARETDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD1PARETDIS_A> {
        match self.bits {
            1 => Some(PD1PARETDIS_A::PERIPHNORETAIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPHNORETAIN`"]
    #[inline(always)]
    pub fn is_periphnoretain(&self) -> bool {
        *self == PD1PARETDIS_A::PERIPHNORETAIN
    }
}
#[doc = "Field `PD1PARETDIS` writer - Disable PD1 Partial Retention"]
pub type PD1PARETDIS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PD1PARETCTRL_SPEC, u16, PD1PARETDIS_A, 16, O>;
impl<'a, const O: u8> PD1PARETDIS_W<'a, O> {
    #[doc = "Retain associated registers when in EM2/3"]
    #[inline(always)]
    pub fn periphnoretain(self) -> &'a mut W {
        self.variant(PD1PARETDIS_A::PERIPHNORETAIN)
    }
}
impl R {
    #[doc = "Bits 0:15 - Disable PD1 Partial Retention"]
    #[inline(always)]
    pub fn pd1paretdis(&self) -> PD1PARETDIS_R {
        PD1PARETDIS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Disable PD1 Partial Retention"]
    #[inline(always)]
    #[must_use]
    pub fn pd1paretdis(&mut self) -> PD1PARETDIS_W<0> {
        PD1PARETDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd1paretctrl](index.html) module"]
pub struct PD1PARETCTRL_SPEC;
impl crate::RegisterSpec for PD1PARETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd1paretctrl::R](R) reader structure"]
impl crate::Readable for PD1PARETCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd1paretctrl::W](W) writer structure"]
impl crate::Writable for PD1PARETCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PD1PARETCTRL to value 0"]
impl crate::Resettable for PD1PARETCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
