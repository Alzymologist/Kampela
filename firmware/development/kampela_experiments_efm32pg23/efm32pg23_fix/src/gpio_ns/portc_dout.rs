#[doc = "Register `PORTC_DOUT` reader"]
pub struct R(crate::R<PORTC_DOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTC_DOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTC_DOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTC_DOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTC_DOUT` writer"]
pub struct W(crate::W<PORTC_DOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTC_DOUT_SPEC>;
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
impl From<crate::W<PORTC_DOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTC_DOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT` reader - Data output"]
pub type DOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DOUT` writer - Data output"]
pub type DOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORTC_DOUT_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Data output"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Data output"]
    #[inline(always)]
    #[must_use]
    pub fn dout(&mut self) -> DOUT_W<0> {
        DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data out\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portc_dout](index.html) module"]
pub struct PORTC_DOUT_SPEC;
impl crate::RegisterSpec for PORTC_DOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portc_dout::R](R) reader structure"]
impl crate::Readable for PORTC_DOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portc_dout::W](W) writer structure"]
impl crate::Writable for PORTC_DOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTC_DOUT to value 0"]
impl crate::Resettable for PORTC_DOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
