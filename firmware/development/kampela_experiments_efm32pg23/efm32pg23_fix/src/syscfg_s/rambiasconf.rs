#[doc = "Register `RAMBIASCONF` reader"]
pub struct R(crate::R<RAMBIASCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMBIASCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMBIASCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMBIASCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMBIASCONF` writer"]
pub struct W(crate::W<RAMBIASCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMBIASCONF_SPEC>;
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
impl From<crate::W<RAMBIASCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMBIASCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMBIASCTRL` reader - RAM Bias Control"]
pub type RAMBIASCTRL_R = crate::FieldReader<u8, RAMBIASCTRL_A>;
#[doc = "RAM Bias Control\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMBIASCTRL_A {
    #[doc = "0: None"]
    NO = 0,
    #[doc = "1: Voltage Source Bias 100mV"]
    VSB100 = 1,
    #[doc = "2: Voltage Source Bias 200mV"]
    VSB200 = 2,
    #[doc = "4: Voltage Source Bias 300mV"]
    VSB300 = 4,
    #[doc = "8: Voltage Source Bias 400mV"]
    VSB400 = 8,
}
impl From<RAMBIASCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMBIASCTRL_A) -> Self {
        variant as _
    }
}
impl RAMBIASCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAMBIASCTRL_A> {
        match self.bits {
            0 => Some(RAMBIASCTRL_A::NO),
            1 => Some(RAMBIASCTRL_A::VSB100),
            2 => Some(RAMBIASCTRL_A::VSB200),
            4 => Some(RAMBIASCTRL_A::VSB300),
            8 => Some(RAMBIASCTRL_A::VSB400),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == RAMBIASCTRL_A::NO
    }
    #[doc = "Checks if the value of the field is `VSB100`"]
    #[inline(always)]
    pub fn is_vsb100(&self) -> bool {
        *self == RAMBIASCTRL_A::VSB100
    }
    #[doc = "Checks if the value of the field is `VSB200`"]
    #[inline(always)]
    pub fn is_vsb200(&self) -> bool {
        *self == RAMBIASCTRL_A::VSB200
    }
    #[doc = "Checks if the value of the field is `VSB300`"]
    #[inline(always)]
    pub fn is_vsb300(&self) -> bool {
        *self == RAMBIASCTRL_A::VSB300
    }
    #[doc = "Checks if the value of the field is `VSB400`"]
    #[inline(always)]
    pub fn is_vsb400(&self) -> bool {
        *self == RAMBIASCTRL_A::VSB400
    }
}
#[doc = "Field `RAMBIASCTRL` writer - RAM Bias Control"]
pub type RAMBIASCTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RAMBIASCONF_SPEC, u8, RAMBIASCTRL_A, 4, O>;
impl<'a, const O: u8> RAMBIASCTRL_W<'a, O> {
    #[doc = "None"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(RAMBIASCTRL_A::NO)
    }
    #[doc = "Voltage Source Bias 100mV"]
    #[inline(always)]
    pub fn vsb100(self) -> &'a mut W {
        self.variant(RAMBIASCTRL_A::VSB100)
    }
    #[doc = "Voltage Source Bias 200mV"]
    #[inline(always)]
    pub fn vsb200(self) -> &'a mut W {
        self.variant(RAMBIASCTRL_A::VSB200)
    }
    #[doc = "Voltage Source Bias 300mV"]
    #[inline(always)]
    pub fn vsb300(self) -> &'a mut W {
        self.variant(RAMBIASCTRL_A::VSB300)
    }
    #[doc = "Voltage Source Bias 400mV"]
    #[inline(always)]
    pub fn vsb400(self) -> &'a mut W {
        self.variant(RAMBIASCTRL_A::VSB400)
    }
}
impl R {
    #[doc = "Bits 0:3 - RAM Bias Control"]
    #[inline(always)]
    pub fn rambiasctrl(&self) -> RAMBIASCTRL_R {
        RAMBIASCTRL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RAM Bias Control"]
    #[inline(always)]
    #[must_use]
    pub fn rambiasctrl(&mut self) -> RAMBIASCTRL_W<0> {
        RAMBIASCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure RAM bias configure bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rambiasconf](index.html) module"]
pub struct RAMBIASCONF_SPEC;
impl crate::RegisterSpec for RAMBIASCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rambiasconf::R](R) reader structure"]
impl crate::Readable for RAMBIASCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rambiasconf::W](W) writer structure"]
impl crate::Writable for RAMBIASCONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMBIASCONF to value 0x02"]
impl crate::Resettable for RAMBIASCONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
