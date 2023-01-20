#[doc = "Register `TXDOUBLE` writer"]
pub struct W(crate::W<TXDOUBLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDOUBLE_SPEC>;
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
impl From<crate::W<TXDOUBLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDOUBLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA0` writer - TX Data"]
pub type TXDATA0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDOUBLE_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXDATA1` writer - TX Data"]
pub type TXDATA1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDOUBLE_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata0(&mut self) -> TXDATA0_W<0> {
        TXDATA0_W::new(self)
    }
    #[doc = "Bits 8:15 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata1(&mut self) -> TXDATA1_W<8> {
        TXDATA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdouble](index.html) module"]
pub struct TXDOUBLE_SPEC;
impl crate::RegisterSpec for TXDOUBLE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txdouble::W](W) writer structure"]
impl crate::Writable for TXDOUBLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDOUBLE to value 0"]
impl crate::Resettable for TXDOUBLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
