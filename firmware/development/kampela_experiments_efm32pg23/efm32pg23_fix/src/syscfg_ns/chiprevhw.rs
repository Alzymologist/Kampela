#[doc = "Register `CHIPREVHW` reader"]
pub struct R(crate::R<CHIPREVHW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIPREVHW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIPREVHW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIPREVHW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHIPREVHW` writer"]
pub struct W(crate::W<CHIPREVHW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHIPREVHW_SPEC>;
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
impl From<crate::W<CHIPREVHW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHIPREVHW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAJOR` reader - Hardwired Chip Revision Major value"]
pub type MAJOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJOR` writer - Hardwired Chip Revision Major value"]
pub type MAJOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHIPREVHW_SPEC, u8, u8, 6, O>;
#[doc = "Field `FAMILY` reader - Hardwired Chip Family value"]
pub type FAMILY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAMILY` writer - Hardwired Chip Family value"]
pub type FAMILY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHIPREVHW_SPEC, u8, u8, 6, O>;
#[doc = "Field `MINOR` reader - Hardwired Chip Revision Minor value"]
pub type MINOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINOR` writer - Hardwired Chip Revision Minor value"]
pub type MINOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHIPREVHW_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:5 - Hardwired Chip Revision Major value"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Hardwired Chip Family value"]
    #[inline(always)]
    pub fn family(&self) -> FAMILY_R {
        FAMILY_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - Hardwired Chip Revision Minor value"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Hardwired Chip Revision Major value"]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MAJOR_W<0> {
        MAJOR_W::new(self)
    }
    #[doc = "Bits 6:11 - Hardwired Chip Family value"]
    #[inline(always)]
    #[must_use]
    pub fn family(&mut self) -> FAMILY_W<6> {
        FAMILY_W::new(self)
    }
    #[doc = "Bits 12:19 - Hardwired Chip Revision Minor value"]
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
#[doc = "Read to get the hard-wired chip revision.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chiprevhw](index.html) module"]
pub struct CHIPREVHW_SPEC;
impl crate::RegisterSpec for CHIPREVHW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chiprevhw::R](R) reader structure"]
impl crate::Readable for CHIPREVHW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chiprevhw::W](W) writer structure"]
impl crate::Writable for CHIPREVHW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHIPREVHW to value 0x0e01"]
impl crate::Resettable for CHIPREVHW_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e01;
}
