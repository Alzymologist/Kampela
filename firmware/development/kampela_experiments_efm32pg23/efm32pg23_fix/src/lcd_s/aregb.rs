#[doc = "Register `AREGB` reader"]
pub struct R(crate::R<AREGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AREGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AREGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AREGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AREGB` writer"]
pub struct W(crate::W<AREGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AREGB_SPEC>;
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
impl From<crate::W<AREGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AREGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AREGB` reader - Animation Register B Data"]
pub type AREGB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AREGB` writer - Animation Register B Data"]
pub type AREGB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AREGB_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Animation Register B Data"]
    #[inline(always)]
    pub fn aregb(&self) -> AREGB_R {
        AREGB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Animation Register B Data"]
    #[inline(always)]
    #[must_use]
    pub fn aregb(&mut self) -> AREGB_W<0> {
        AREGB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aregb](index.html) module"]
pub struct AREGB_SPEC;
impl crate::RegisterSpec for AREGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aregb::R](R) reader structure"]
impl crate::Readable for AREGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aregb::W](W) writer structure"]
impl crate::Writable for AREGB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AREGB to value 0"]
impl crate::Resettable for AREGB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
