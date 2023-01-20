#[doc = "Register `PFMXCTRL` reader"]
pub struct R(crate::R<PFMXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFMXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFMXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFMXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFMXCTRL` writer"]
pub struct W(crate::W<PFMXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFMXCTRL_SPEC>;
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
impl From<crate::W<PFMXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFMXCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPKVAL` reader - PFMX mode Peak Current Setting"]
pub type IPKVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPKVAL` writer - PFMX mode Peak Current Setting"]
pub type IPKVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFMXCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `IPKTMAXCTRL` reader - Ton_max timeout control"]
pub type IPKTMAXCTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPKTMAXCTRL` writer - Ton_max timeout control"]
pub type IPKTMAXCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFMXCTRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:3 - PFMX mode Peak Current Setting"]
    #[inline(always)]
    pub fn ipkval(&self) -> IPKVAL_R {
        IPKVAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Ton_max timeout control"]
    #[inline(always)]
    pub fn ipktmaxctrl(&self) -> IPKTMAXCTRL_R {
        IPKTMAXCTRL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PFMX mode Peak Current Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ipkval(&mut self) -> IPKVAL_W<0> {
        IPKVAL_W::new(self)
    }
    #[doc = "Bits 8:12 - Ton_max timeout control"]
    #[inline(always)]
    #[must_use]
    pub fn ipktmaxctrl(&mut self) -> IPKTMAXCTRL_W<8> {
        IPKTMAXCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFMX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfmxctrl](index.html) module"]
pub struct PFMXCTRL_SPEC;
impl crate::RegisterSpec for PFMXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfmxctrl::R](R) reader structure"]
impl crate::Readable for PFMXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfmxctrl::W](W) writer structure"]
impl crate::Writable for PFMXCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFMXCTRL to value 0x0b0c"]
impl crate::Resettable for PFMXCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b0c;
}
