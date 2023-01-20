#[doc = "Register `DTCTRL` reader"]
pub struct R(crate::R<DTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTCTRL` writer"]
pub struct W(crate::W<DTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTCTRL_SPEC>;
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
impl From<crate::W<DTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTCINV` reader - DTI Complementary Output Invert."]
pub type DTCINV_R = crate::BitReader<bool>;
#[doc = "Field `DTCINV` writer - DTI Complementary Output Invert."]
pub type DTCINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCTRL_SPEC, bool, O>;
#[doc = "Field `DTIPOL` reader - DTI Inactive Polarity"]
pub type DTIPOL_R = crate::BitReader<bool>;
#[doc = "Field `DTIPOL` writer - DTI Inactive Polarity"]
pub type DTIPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DTI Complementary Output Invert."]
    #[inline(always)]
    pub fn dtcinv(&self) -> DTCINV_R {
        DTCINV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&self) -> DTIPOL_R {
        DTIPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI Complementary Output Invert."]
    #[inline(always)]
    #[must_use]
    pub fn dtcinv(&mut self) -> DTCINV_W<0> {
        DTCINV_W::new(self)
    }
    #[doc = "Bit 1 - DTI Inactive Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dtipol(&mut self) -> DTIPOL_W<1> {
        DTIPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtctrl](index.html) module"]
pub struct DTCTRL_SPEC;
impl crate::RegisterSpec for DTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtctrl::R](R) reader structure"]
impl crate::Readable for DTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtctrl::W](W) writer structure"]
impl crate::Writable for DTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTCTRL to value 0"]
impl crate::Resettable for DTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
