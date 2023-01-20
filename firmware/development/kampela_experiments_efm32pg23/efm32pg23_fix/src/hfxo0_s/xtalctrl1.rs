#[doc = "Register `XTALCTRL1` reader"]
pub struct R(crate::R<XTALCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTALCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTALCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTALCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTALCTRL1` writer"]
pub struct W(crate::W<XTALCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTALCTRL1_SPEC>;
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
impl From<crate::W<XTALCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTALCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTUNEXIBUFOUTANA` reader - BUFOUT Tuning Capacitance on XI"]
pub type CTUNEXIBUFOUTANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTUNEXIBUFOUTANA` writer - BUFOUT Tuning Capacitance on XI"]
pub type CTUNEXIBUFOUTANA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTALCTRL1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - BUFOUT Tuning Capacitance on XI"]
    #[inline(always)]
    pub fn ctunexibufoutana(&self) -> CTUNEXIBUFOUTANA_R {
        CTUNEXIBUFOUTANA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BUFOUT Tuning Capacitance on XI"]
    #[inline(always)]
    #[must_use]
    pub fn ctunexibufoutana(&mut self) -> CTUNEXIBUFOUTANA_W<0> {
        CTUNEXIBUFOUTANA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalctrl1](index.html) module"]
pub struct XTALCTRL1_SPEC;
impl crate::RegisterSpec for XTALCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtalctrl1::R](R) reader structure"]
impl crate::Readable for XTALCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtalctrl1::W](W) writer structure"]
impl crate::Writable for XTALCTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTALCTRL1 to value 0x3c"]
impl crate::Resettable for XTALCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3c;
}
