#[doc = "Register `REP0` reader"]
pub struct R(crate::R<REP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REP0` writer"]
pub struct W(crate::W<REP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REP0_SPEC>;
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
impl From<crate::W<REP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REP0` reader - Repeat Counter 0"]
pub type REP0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REP0` writer - Repeat Counter 0"]
pub type REP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REP0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Repeat Counter 0"]
    #[inline(always)]
    pub fn rep0(&self) -> REP0_R {
        REP0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repeat Counter 0"]
    #[inline(always)]
    #[must_use]
    pub fn rep0(&mut self) -> REP0_W<0> {
        REP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rep0](index.html) module"]
pub struct REP0_SPEC;
impl crate::RegisterSpec for REP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rep0::R](R) reader structure"]
impl crate::Readable for REP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rep0::W](W) writer structure"]
impl crate::Writable for REP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REP0 to value 0"]
impl crate::Resettable for REP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
