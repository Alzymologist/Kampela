#[doc = "Register `CONFIGURATION` reader"]
pub struct R(crate::R<CONFIGURATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIGURATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIGURATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIGURATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIGURATION` writer"]
pub struct W(crate::W<CONFIGURATION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIGURATION_SPEC>;
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
impl From<crate::W<CONFIGURATION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIGURATION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINTEN` reader - TXINTEN"]
pub type TXINTEN_R = crate::BitReader<bool>;
#[doc = "Field `TXINTEN` writer - TXINTEN"]
pub type TXINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIGURATION_SPEC, bool, O>;
#[doc = "Field `RXINTEN` reader - RXINTEN"]
pub type RXINTEN_R = crate::BitReader<bool>;
#[doc = "Field `RXINTEN` writer - RXINTEN"]
pub type RXINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIGURATION_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TXINTEN"]
    #[inline(always)]
    pub fn txinten(&self) -> TXINTEN_R {
        TXINTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXINTEN"]
    #[inline(always)]
    pub fn rxinten(&self) -> RXINTEN_R {
        RXINTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXINTEN"]
    #[inline(always)]
    #[must_use]
    pub fn txinten(&mut self) -> TXINTEN_W<0> {
        TXINTEN_W::new(self)
    }
    #[doc = "Bit 1 - RXINTEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxinten(&mut self) -> RXINTEN_W<1> {
        RXINTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [configuration](index.html) module"]
pub struct CONFIGURATION_SPEC;
impl crate::RegisterSpec for CONFIGURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [configuration::R](R) reader structure"]
impl crate::Readable for CONFIGURATION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [configuration::W](W) writer structure"]
impl crate::Writable for CONFIGURATION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIGURATION to value 0"]
impl crate::Resettable for CONFIGURATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
