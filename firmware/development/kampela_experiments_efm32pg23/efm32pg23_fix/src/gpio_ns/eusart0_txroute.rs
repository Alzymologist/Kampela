#[doc = "Register `EUSART0_TXROUTE` reader"]
pub struct R(crate::R<EUSART0_TXROUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EUSART0_TXROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EUSART0_TXROUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EUSART0_TXROUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EUSART0_TXROUTE` writer"]
pub struct W(crate::W<EUSART0_TXROUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EUSART0_TXROUTE_SPEC>;
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
impl From<crate::W<EUSART0_TXROUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EUSART0_TXROUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORT` reader - TX port select register"]
pub type PORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT` writer - TX port select register"]
pub type PORT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EUSART0_TXROUTE_SPEC, u8, u8, 2, O>;
#[doc = "Field `PIN` reader - TX pin select register"]
pub type PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN` writer - TX pin select register"]
pub type PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EUSART0_TXROUTE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - TX port select register"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - TX pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TX port select register"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PORT_W<0> {
        PORT_W::new(self)
    }
    #[doc = "Bits 16:19 - TX pin select register"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PIN_W<16> {
        PIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX port/pin select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eusart0_txroute](index.html) module"]
pub struct EUSART0_TXROUTE_SPEC;
impl crate::RegisterSpec for EUSART0_TXROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eusart0_txroute::R](R) reader structure"]
impl crate::Readable for EUSART0_TXROUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eusart0_txroute::W](W) writer structure"]
impl crate::Writable for EUSART0_TXROUTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EUSART0_TXROUTE to value 0"]
impl crate::Resettable for EUSART0_TXROUTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
