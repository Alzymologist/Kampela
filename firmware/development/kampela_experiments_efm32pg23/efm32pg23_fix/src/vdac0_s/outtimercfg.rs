#[doc = "Register `OUTTIMERCFG` reader"]
pub struct R(crate::R<OUTTIMERCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTTIMERCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTTIMERCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTTIMERCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTTIMERCFG` writer"]
pub struct W(crate::W<OUTTIMERCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTTIMERCFG_SPEC>;
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
impl From<crate::W<OUTTIMERCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTTIMERCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0OUTHOLDTIME` reader - CH0 Output Hold Time"]
pub type CH0OUTHOLDTIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH0OUTHOLDTIME` writer - CH0 Output Hold Time"]
pub type CH0OUTHOLDTIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTTIMERCFG_SPEC, u16, u16, 10, O>;
#[doc = "Field `CH1OUTHOLDTIME` reader - CH1 Output Hold Time"]
pub type CH1OUTHOLDTIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH1OUTHOLDTIME` writer - CH1 Output Hold Time"]
pub type CH1OUTHOLDTIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTTIMERCFG_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - CH0 Output Hold Time"]
    #[inline(always)]
    pub fn ch0outholdtime(&self) -> CH0OUTHOLDTIME_R {
        CH0OUTHOLDTIME_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 15:24 - CH1 Output Hold Time"]
    #[inline(always)]
    pub fn ch1outholdtime(&self) -> CH1OUTHOLDTIME_R {
        CH1OUTHOLDTIME_R::new(((self.bits >> 15) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - CH0 Output Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn ch0outholdtime(&mut self) -> CH0OUTHOLDTIME_W<0> {
        CH0OUTHOLDTIME_W::new(self)
    }
    #[doc = "Bits 15:24 - CH1 Output Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn ch1outholdtime(&mut self) -> CH1OUTHOLDTIME_W<15> {
        CH1OUTHOLDTIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outtimercfg](index.html) module"]
pub struct OUTTIMERCFG_SPEC;
impl crate::RegisterSpec for OUTTIMERCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outtimercfg::R](R) reader structure"]
impl crate::Readable for OUTTIMERCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outtimercfg::W](W) writer structure"]
impl crate::Writable for OUTTIMERCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTTIMERCFG to value 0"]
impl crate::Resettable for OUTTIMERCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
