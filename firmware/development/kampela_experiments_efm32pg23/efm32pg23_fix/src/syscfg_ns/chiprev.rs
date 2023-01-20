#[doc = "Register `CHIPREV` reader"]
pub struct R(crate::R<CHIPREV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIPREV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIPREV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIPREV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHIPREV` writer"]
pub struct W(crate::W<CHIPREV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHIPREV_SPEC>;
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
impl From<crate::W<CHIPREV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHIPREV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAJOR` reader - Chip Revision Major value"]
pub type MAJOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJOR` writer - Chip Revision Major value"]
pub type MAJOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHIPREV_SPEC, u8, u8, 6, O>;
#[doc = "Field `FAMILY` reader - Chip Family value"]
pub type FAMILY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAMILY` writer - Chip Family value"]
pub type FAMILY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHIPREV_SPEC, u8, u8, 6, O>;
#[doc = "Field `MINOR` reader - Chip Revision Minor value"]
pub type MINOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINOR` writer - Chip Revision Minor value"]
pub type MINOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHIPREV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:5 - Chip Revision Major value"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Chip Family value"]
    #[inline(always)]
    pub fn family(&self) -> FAMILY_R {
        FAMILY_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - Chip Revision Minor value"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Chip Revision Major value"]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MAJOR_W<0> {
        MAJOR_W::new(self)
    }
    #[doc = "Bits 6:11 - Chip Family value"]
    #[inline(always)]
    #[must_use]
    pub fn family(&mut self) -> FAMILY_W<6> {
        FAMILY_W::new(self)
    }
    #[doc = "Bits 12:19 - Chip Revision Minor value"]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MINOR_W<12> {
        MINOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read to get the chip revision programmed by feature configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chiprev](index.html) module"]
pub struct CHIPREV_SPEC;
impl crate::RegisterSpec for CHIPREV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chiprev::R](R) reader structure"]
impl crate::Readable for CHIPREV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chiprev::W](W) writer structure"]
impl crate::Writable for CHIPREV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHIPREV to value 0"]
impl crate::Resettable for CHIPREV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
