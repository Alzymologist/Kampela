#[doc = "Register `MASKREQ` reader"]
pub struct R(crate::R<MASKREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASKREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASKREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASKREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASKREQ` writer"]
pub struct W(crate::W<MASKREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASKREQ_SPEC>;
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
impl From<crate::W<MASKREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASKREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASKREQ` reader - Scan Queue Mask Request"]
pub type MASKREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MASKREQ` writer - Scan Queue Mask Request"]
pub type MASKREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASKREQ_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Scan Queue Mask Request"]
    #[inline(always)]
    pub fn maskreq(&self) -> MASKREQ_R {
        MASKREQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan Queue Mask Request"]
    #[inline(always)]
    #[must_use]
    pub fn maskreq(&mut self) -> MASKREQ_W<0> {
        MASKREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maskreq](index.html) module"]
pub struct MASKREQ_SPEC;
impl crate::RegisterSpec for MASKREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maskreq::R](R) reader structure"]
impl crate::Readable for MASKREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maskreq::W](W) writer structure"]
impl crate::Writable for MASKREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASKREQ to value 0"]
impl crate::Resettable for MASKREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
