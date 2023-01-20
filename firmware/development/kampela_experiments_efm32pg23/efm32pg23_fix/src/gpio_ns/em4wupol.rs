#[doc = "Register `EM4WUPOL` reader"]
pub struct R(crate::R<EM4WUPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM4WUPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM4WUPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM4WUPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM4WUPOL` writer"]
pub struct W(crate::W<EM4WUPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM4WUPOL_SPEC>;
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
impl From<crate::W<EM4WUPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM4WUPOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM4WUPOL` reader - EM4 Wake-Up Polarity"]
pub type EM4WUPOL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EM4WUPOL` writer - EM4 Wake-Up Polarity"]
pub type EM4WUPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EM4WUPOL_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 16:27 - EM4 Wake-Up Polarity"]
    #[inline(always)]
    pub fn em4wupol(&self) -> EM4WUPOL_R {
        EM4WUPOL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - EM4 Wake-Up Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn em4wupol(&mut self) -> EM4WUPOL_W<16> {
        EM4WUPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em4wupol](index.html) module"]
pub struct EM4WUPOL_SPEC;
impl crate::RegisterSpec for EM4WUPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em4wupol::R](R) reader structure"]
impl crate::Readable for EM4WUPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em4wupol::W](W) writer structure"]
impl crate::Writable for EM4WUPOL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EM4WUPOL to value 0"]
impl crate::Resettable for EM4WUPOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
