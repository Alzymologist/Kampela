#[doc = "Register `EUSART0CLKCTRL` reader"]
pub struct R(crate::R<EUSART0CLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EUSART0CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EUSART0CLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EUSART0CLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EUSART0CLKCTRL` writer"]
pub struct W(crate::W<EUSART0CLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EUSART0CLKCTRL_SPEC>;
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
impl From<crate::W<EUSART0CLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EUSART0CLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: EUSART0 is not clocked"]
    DISABLED = 0,
    #[doc = "1: EM01GRPCCLK is clocking EUSART0"]
    EM01GRPCCLK = 1,
    #[doc = "2: HFRCOEM23 is clocking EUSART0"]
    HFRCOEM23 = 2,
    #[doc = "3: LFRCO is clocking EUSART0"]
    LFRCO = 3,
    #[doc = "4: LFXO is clocking EUSART0"]
    LFXO = 4,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::DISABLED),
            1 => Some(CLKSEL_A::EM01GRPCCLK),
            2 => Some(CLKSEL_A::HFRCOEM23),
            3 => Some(CLKSEL_A::LFRCO),
            4 => Some(CLKSEL_A::LFXO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `EM01GRPCCLK`"]
    #[inline(always)]
    pub fn is_em01grpcclk(&self) -> bool {
        *self == CLKSEL_A::EM01GRPCCLK
    }
    #[doc = "Checks if the value of the field is `HFRCOEM23`"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == CLKSEL_A::HFRCOEM23
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKSEL_A::LFXO
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EUSART0CLKCTRL_SPEC, u8, CLKSEL_A, 3, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "EUSART0 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKSEL_A::DISABLED)
    }
    #[doc = "EM01GRPCCLK is clocking EUSART0"]
    #[inline(always)]
    pub fn em01grpcclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::EM01GRPCCLK)
    }
    #[doc = "HFRCOEM23 is clocking EUSART0"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCOEM23)
    }
    #[doc = "LFRCO is clocking EUSART0"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFRCO)
    }
    #[doc = "LFXO is clocking EUSART0"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFXO)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eusart0clkctrl](index.html) module"]
pub struct EUSART0CLKCTRL_SPEC;
impl crate::RegisterSpec for EUSART0CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eusart0clkctrl::R](R) reader structure"]
impl crate::Readable for EUSART0CLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eusart0clkctrl::W](W) writer structure"]
impl crate::Writable for EUSART0CLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EUSART0CLKCTRL to value 0x01"]
impl crate::Resettable for EUSART0CLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
