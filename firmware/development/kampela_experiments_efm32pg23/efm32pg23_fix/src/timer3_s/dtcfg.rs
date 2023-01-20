#[doc = "Register `DTCFG` reader"]
pub struct R(crate::R<DTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTCFG` writer"]
pub struct W(crate::W<DTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTCFG_SPEC>;
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
impl From<crate::W<DTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTEN` reader - DTI Enable"]
pub type DTEN_R = crate::BitReader<bool>;
#[doc = "Field `DTEN` writer - DTI Enable"]
pub type DTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCFG_SPEC, bool, O>;
#[doc = "Field `DTDAS` reader - DTI Automatic Start-up Functionality"]
pub type DTDAS_R = crate::BitReader<DTDAS_A>;
#[doc = "DTI Automatic Start-up Functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTDAS_A {
    #[doc = "0: No DTI restart on debugger exit"]
    NORESTART = 0,
    #[doc = "1: DTI restart on debugger exit"]
    RESTART = 1,
}
impl From<DTDAS_A> for bool {
    #[inline(always)]
    fn from(variant: DTDAS_A) -> Self {
        variant as u8 != 0
    }
}
impl DTDAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTDAS_A {
        match self.bits {
            false => DTDAS_A::NORESTART,
            true => DTDAS_A::RESTART,
        }
    }
    #[doc = "Checks if the value of the field is `NORESTART`"]
    #[inline(always)]
    pub fn is_norestart(&self) -> bool {
        *self == DTDAS_A::NORESTART
    }
    #[doc = "Checks if the value of the field is `RESTART`"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == DTDAS_A::RESTART
    }
}
#[doc = "Field `DTDAS` writer - DTI Automatic Start-up Functionality"]
pub type DTDAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCFG_SPEC, DTDAS_A, O>;
impl<'a, const O: u8> DTDAS_W<'a, O> {
    #[doc = "No DTI restart on debugger exit"]
    #[inline(always)]
    pub fn norestart(self) -> &'a mut W {
        self.variant(DTDAS_A::NORESTART)
    }
    #[doc = "DTI restart on debugger exit"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut W {
        self.variant(DTDAS_A::RESTART)
    }
}
#[doc = "Field `DTAR` reader - DTI Always Run"]
pub type DTAR_R = crate::BitReader<bool>;
#[doc = "Field `DTAR` writer - DTI Always Run"]
pub type DTAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCFG_SPEC, bool, O>;
#[doc = "Field `DTFATS` reader - DTI Fault Action on Timer Stop"]
pub type DTFATS_R = crate::BitReader<bool>;
#[doc = "Field `DTFATS` writer - DTI Fault Action on Timer Stop"]
pub type DTFATS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCFG_SPEC, bool, O>;
#[doc = "Field `DTPRSEN` reader - DTI PRS Source Enable"]
pub type DTPRSEN_R = crate::BitReader<bool>;
#[doc = "Field `DTPRSEN` writer - DTI PRS Source Enable"]
pub type DTPRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&self) -> DTDAS_R {
        DTDAS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline(always)]
    pub fn dtar(&self) -> DTAR_R {
        DTAR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline(always)]
    pub fn dtfats(&self) -> DTFATS_R {
        DTFATS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&self) -> DTPRSEN_R {
        DTPRSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<0> {
        DTEN_W::new(self)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    #[must_use]
    pub fn dtdas(&mut self) -> DTDAS_W<1> {
        DTDAS_W::new(self)
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline(always)]
    #[must_use]
    pub fn dtar(&mut self) -> DTAR_W<9> {
        DTAR_W::new(self)
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline(always)]
    #[must_use]
    pub fn dtfats(&mut self) -> DTFATS_W<10> {
        DTFATS_W::new(self)
    }
    #[doc = "Bit 11 - DTI PRS Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtprsen(&mut self) -> DTPRSEN_W<11> {
        DTPRSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtcfg](index.html) module"]
pub struct DTCFG_SPEC;
impl crate::RegisterSpec for DTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtcfg::R](R) reader structure"]
impl crate::Readable for DTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtcfg::W](W) writer structure"]
impl crate::Writable for DTCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTCFG to value 0"]
impl crate::Resettable for DTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
