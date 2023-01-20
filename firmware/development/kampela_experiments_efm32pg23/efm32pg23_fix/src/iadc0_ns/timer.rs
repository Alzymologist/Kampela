#[doc = "Register `TIMER` reader"]
pub struct R(crate::R<TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER` writer"]
pub struct W(crate::W<TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_SPEC>;
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
impl From<crate::W<TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER` reader - Timer Period"]
pub type TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER` writer - Timer Period"]
pub type TIMER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timer Period"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Period"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<0> {
        TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer](index.html) module"]
pub struct TIMER_SPEC;
impl crate::RegisterSpec for TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer::R](R) reader structure"]
impl crate::Readable for TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer::W](W) writer structure"]
impl crate::Writable for TIMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER to value 0"]
impl crate::Resettable for TIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
