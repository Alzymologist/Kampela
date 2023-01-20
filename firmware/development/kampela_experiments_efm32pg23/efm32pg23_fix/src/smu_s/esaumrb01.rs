#[doc = "Register `ESAUMRB01` reader"]
pub struct R(crate::R<ESAUMRB01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESAUMRB01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESAUMRB01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESAUMRB01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESAUMRB01` writer"]
pub struct W(crate::W<ESAUMRB01_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESAUMRB01_SPEC>;
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
impl From<crate::W<ESAUMRB01_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESAUMRB01_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESAUMRB01` reader - Moveable Region Boundary"]
pub type ESAUMRB01_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ESAUMRB01` writer - Moveable Region Boundary"]
pub type ESAUMRB01_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ESAUMRB01_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    pub fn esaumrb01(&self) -> ESAUMRB01_R {
        ESAUMRB01_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    #[must_use]
    pub fn esaumrb01(&mut self) -> ESAUMRB01_W<12> {
        ESAUMRB01_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esaumrb01](index.html) module"]
pub struct ESAUMRB01_SPEC;
impl crate::RegisterSpec for ESAUMRB01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esaumrb01::R](R) reader structure"]
impl crate::Readable for ESAUMRB01_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esaumrb01::W](W) writer structure"]
impl crate::Writable for ESAUMRB01_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESAUMRB01 to value 0x0a00_0000"]
impl crate::Resettable for ESAUMRB01_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a00_0000;
}
