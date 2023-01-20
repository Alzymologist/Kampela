#[doc = "Register `CH2_REQSEL` reader"]
pub struct R(crate::R<CH2_REQSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2_REQSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2_REQSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2_REQSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2_REQSEL` writer"]
pub struct W(crate::W<CH2_REQSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2_REQSEL_SPEC>;
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
impl From<crate::W<CH2_REQSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2_REQSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SIGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2_REQSEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SOURCESEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SOURCESEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2_REQSEL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<0> {
        SIGSEL_W::new(self)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SOURCESEL_W<16> {
        SOURCESEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_reqsel](index.html) module"]
pub struct CH2_REQSEL_SPEC;
impl crate::RegisterSpec for CH2_REQSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2_reqsel::R](R) reader structure"]
impl crate::Readable for CH2_REQSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2_reqsel::W](W) writer structure"]
impl crate::Writable for CH2_REQSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2_REQSEL to value 0"]
impl crate::Resettable for CH2_REQSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
