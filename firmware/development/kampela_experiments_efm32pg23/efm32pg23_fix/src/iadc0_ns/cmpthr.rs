#[doc = "Register `CMPTHR` reader"]
pub struct R(crate::R<CMPTHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPTHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPTHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPTHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPTHR` writer"]
pub struct W(crate::W<CMPTHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPTHR_SPEC>;
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
impl From<crate::W<CMPTHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPTHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADLT` reader - ADC Less Than or Equal to Threshold"]
pub type ADLT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADLT` writer - ADC Less Than or Equal to Threshold"]
pub type ADLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPTHR_SPEC, u16, u16, 16, O>;
#[doc = "Field `ADGT` reader - ADC Greater Than or Equal to Threshold"]
pub type ADGT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADGT` writer - ADC Greater Than or Equal to Threshold"]
pub type ADGT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPTHR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - ADC Less Than or Equal to Threshold"]
    #[inline(always)]
    pub fn adlt(&self) -> ADLT_R {
        ADLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADC Greater Than or Equal to Threshold"]
    #[inline(always)]
    pub fn adgt(&self) -> ADGT_R {
        ADGT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC Less Than or Equal to Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn adlt(&mut self) -> ADLT_W<0> {
        ADLT_W::new(self)
    }
    #[doc = "Bits 16:31 - ADC Greater Than or Equal to Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn adgt(&mut self) -> ADGT_W<16> {
        ADGT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpthr](index.html) module"]
pub struct CMPTHR_SPEC;
impl crate::RegisterSpec for CMPTHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpthr::R](R) reader structure"]
impl crate::Readable for CMPTHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpthr::W](W) writer structure"]
impl crate::Writable for CMPTHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPTHR to value 0"]
impl crate::Resettable for CMPTHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
