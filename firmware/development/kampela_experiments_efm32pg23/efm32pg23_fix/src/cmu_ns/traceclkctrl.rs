#[doc = "Register `TRACECLKCTRL` reader"]
pub struct R(crate::R<TRACECLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACECLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACECLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACECLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACECLKCTRL` writer"]
pub struct W(crate::W<TRACECLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACECLKCTRL_SPEC>;
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
impl From<crate::W<TRACECLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACECLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC` reader - TRACECLK Prescaler"]
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
#[doc = "TRACECLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: TRACECLK is SYSCLK divided by 1"]
    DIV1 = 0,
    #[doc = "1: TRACECLK is SYSCLK divided by 2"]
    DIV2 = 1,
    #[doc = "3: TRACECLK is SYSCLK divided by 4"]
    DIV4 = 3,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::DIV1),
            1 => Some(PRESC_A::DIV2),
            3 => Some(PRESC_A::DIV4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::DIV4
    }
}
#[doc = "Field `PRESC` writer - TRACECLK Prescaler"]
pub type PRESC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRACECLKCTRL_SPEC, u8, PRESC_A, 2, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "TRACECLK is SYSCLK divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESC_A::DIV1)
    }
    #[doc = "TRACECLK is SYSCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::DIV2)
    }
    #[doc = "TRACECLK is SYSCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::DIV4)
    }
}
impl R {
    #[doc = "Bits 4:5 - TRACECLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - TRACECLK Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<4> {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceclkctrl](index.html) module"]
pub struct TRACECLKCTRL_SPEC;
impl crate::RegisterSpec for TRACECLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [traceclkctrl::R](R) reader structure"]
impl crate::Readable for TRACECLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [traceclkctrl::W](W) writer structure"]
impl crate::Writable for TRACECLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRACECLKCTRL to value 0"]
impl crate::Resettable for TRACECLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
