#[doc = "Register `ESAUMRB45` reader"]
pub struct R(crate::R<ESAUMRB45_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESAUMRB45_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESAUMRB45_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESAUMRB45_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESAUMRB45` writer"]
pub struct W(crate::W<ESAUMRB45_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESAUMRB45_SPEC>;
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
impl From<crate::W<ESAUMRB45_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESAUMRB45_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESAUMRB45` reader - Moveable Region Boundary"]
pub type ESAUMRB45_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ESAUMRB45` writer - Moveable Region Boundary"]
pub type ESAUMRB45_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ESAUMRB45_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    pub fn esaumrb45(&self) -> ESAUMRB45_R {
        ESAUMRB45_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    #[must_use]
    pub fn esaumrb45(&mut self) -> ESAUMRB45_W<12> {
        ESAUMRB45_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esaumrb45](index.html) module"]
pub struct ESAUMRB45_SPEC;
impl crate::RegisterSpec for ESAUMRB45_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esaumrb45::R](R) reader structure"]
impl crate::Readable for ESAUMRB45_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esaumrb45::W](W) writer structure"]
impl crate::Writable for ESAUMRB45_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESAUMRB45 to value 0x0200_0000"]
impl crate::Resettable for ESAUMRB45_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0000;
}
