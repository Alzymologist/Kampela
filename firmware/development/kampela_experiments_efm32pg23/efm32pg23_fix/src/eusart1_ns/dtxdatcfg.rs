#[doc = "Register `DTXDATCFG` reader"]
pub struct R(crate::R<DTXDATCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTXDATCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTXDATCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTXDATCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTXDATCFG` writer"]
pub struct W(crate::W<DTXDATCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTXDATCFG_SPEC>;
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
impl From<crate::W<DTXDATCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTXDATCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTXDAT` reader - Default TX DATA"]
pub type DTXDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTXDAT` writer - Default TX DATA"]
pub type DTXDAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTXDATCFG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Default TX DATA"]
    #[inline(always)]
    pub fn dtxdat(&self) -> DTXDAT_R {
        DTXDAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Default TX DATA"]
    #[inline(always)]
    #[must_use]
    pub fn dtxdat(&mut self) -> DTXDAT_W<0> {
        DTXDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxdatcfg](index.html) module"]
pub struct DTXDATCFG_SPEC;
impl crate::RegisterSpec for DTXDATCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtxdatcfg::R](R) reader structure"]
impl crate::Readable for DTXDATCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtxdatcfg::W](W) writer structure"]
impl crate::Writable for DTXDATCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTXDATCFG to value 0"]
impl crate::Resettable for DTXDATCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
