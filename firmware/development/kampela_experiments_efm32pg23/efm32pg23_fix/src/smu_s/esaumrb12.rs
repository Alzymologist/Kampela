#[doc = "Register `ESAUMRB12` reader"]
pub struct R(crate::R<ESAUMRB12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESAUMRB12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESAUMRB12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESAUMRB12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESAUMRB12` writer"]
pub struct W(crate::W<ESAUMRB12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESAUMRB12_SPEC>;
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
impl From<crate::W<ESAUMRB12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESAUMRB12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESAUMRB12` reader - Moveable Region Boundary"]
pub type ESAUMRB12_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ESAUMRB12` writer - Moveable Region Boundary"]
pub type ESAUMRB12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ESAUMRB12_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    pub fn esaumrb12(&self) -> ESAUMRB12_R {
        ESAUMRB12_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    #[must_use]
    pub fn esaumrb12(&mut self) -> ESAUMRB12_W<12> {
        ESAUMRB12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esaumrb12](index.html) module"]
pub struct ESAUMRB12_SPEC;
impl crate::RegisterSpec for ESAUMRB12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esaumrb12::R](R) reader structure"]
impl crate::Readable for ESAUMRB12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esaumrb12::W](W) writer structure"]
impl crate::Writable for ESAUMRB12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESAUMRB12 to value 0x0c00_0000"]
impl crate::Resettable for ESAUMRB12_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_0000;
}
