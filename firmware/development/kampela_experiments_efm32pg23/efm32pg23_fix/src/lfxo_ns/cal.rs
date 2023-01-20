#[doc = "Register `CAL` reader"]
pub struct R(crate::R<CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL` writer"]
pub struct W(crate::W<CAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_SPEC>;
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
impl From<crate::W<CAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTUNE` reader - Internal Capacitance Tuning"]
pub type CAPTUNE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAPTUNE` writer - Internal Capacitance Tuning"]
pub type CAPTUNE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 7, O>;
#[doc = "Field `GAIN` reader - LFXO Startup Gain"]
pub type GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN` writer - LFXO Startup Gain"]
pub type GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:6 - Internal Capacitance Tuning"]
    #[inline(always)]
    pub fn captune(&self) -> CAPTUNE_R {
        CAPTUNE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - LFXO Startup Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Internal Capacitance Tuning"]
    #[inline(always)]
    #[must_use]
    pub fn captune(&mut self) -> CAPTUNE_W<0> {
        CAPTUNE_W::new(self)
    }
    #[doc = "Bits 8:9 - LFXO Startup Gain"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<8> {
        GAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Do not write to this register unless CALBSY in SYNCBUSY register is low.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal](index.html) module"]
pub struct CAL_SPEC;
impl crate::RegisterSpec for CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal::R](R) reader structure"]
impl crate::Readable for CAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal::W](W) writer structure"]
impl crate::Writable for CAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL to value 0x0200"]
impl crate::Resettable for CAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
