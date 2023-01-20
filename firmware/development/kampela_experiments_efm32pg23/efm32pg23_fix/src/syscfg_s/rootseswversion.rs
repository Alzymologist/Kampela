#[doc = "Register `ROOTSESWVERSION` reader"]
pub struct R(crate::R<ROOTSESWVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROOTSESWVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROOTSESWVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROOTSESWVERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROOTSESWVERSION` writer"]
pub struct W(crate::W<ROOTSESWVERSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROOTSESWVERSION_SPEC>;
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
impl From<crate::W<ROOTSESWVERSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROOTSESWVERSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWVERSION` reader - SW Version"]
pub type SWVERSION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SWVERSION` writer - SW Version"]
pub type SWVERSION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROOTSESWVERSION_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - SW Version"]
    #[inline(always)]
    pub fn swversion(&self) -> SWVERSION_R {
        SWVERSION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SW Version"]
    #[inline(always)]
    #[must_use]
    pub fn swversion(&mut self) -> SWVERSION_W<0> {
        SWVERSION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SE Software version\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rootseswversion](index.html) module"]
pub struct ROOTSESWVERSION_SPEC;
impl crate::RegisterSpec for ROOTSESWVERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rootseswversion::R](R) reader structure"]
impl crate::Readable for ROOTSESWVERSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rootseswversion::W](W) writer structure"]
impl crate::Writable for ROOTSESWVERSION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROOTSESWVERSION to value 0"]
impl crate::Resettable for ROOTSESWVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
