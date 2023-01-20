#[doc = "Register `SYSCLKCTRL` reader"]
pub struct R(crate::R<SYSCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCLKCTRL` writer"]
pub struct W(crate::W<SYSCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCLKCTRL_SPEC>;
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
impl From<crate::W<SYSCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "1: FSRCO is clocking SYSCLK"]
    FSRCO = 1,
    #[doc = "2: HFRCODPLL is clocking SYSCLK"]
    HFRCODPLL = 2,
    #[doc = "3: HFXO is clocking SYSCLK"]
    HFXO = 3,
    #[doc = "4: CLKIN0 is clocking SYSCLK"]
    CLKIN0 = 4,
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
            1 => Some(CLKSEL_A::FSRCO),
            2 => Some(CLKSEL_A::HFRCODPLL),
            3 => Some(CLKSEL_A::HFXO),
            4 => Some(CLKSEL_A::CLKIN0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FSRCO`"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == CLKSEL_A::FSRCO
    }
    #[doc = "Checks if the value of the field is `HFRCODPLL`"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == CLKSEL_A::HFRCODPLL
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `CLKIN0`"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == CLKSEL_A::CLKIN0
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSCLKCTRL_SPEC, u8, CLKSEL_A, 3, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "FSRCO is clocking SYSCLK"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::FSRCO)
    }
    #[doc = "HFRCODPLL is clocking SYSCLK"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCODPLL)
    }
    #[doc = "HFXO is clocking SYSCLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFXO)
    }
    #[doc = "CLKIN0 is clocking SYSCLK"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut W {
        self.variant(CLKSEL_A::CLKIN0)
    }
}
#[doc = "Field `PCLKPRESC` reader - PCLK Prescaler"]
pub type PCLKPRESC_R = crate::BitReader<PCLKPRESC_A>;
#[doc = "PCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCLKPRESC_A {
    #[doc = "0: PCLK is HCLK divided by 1"]
    DIV1 = 0,
    #[doc = "1: PCLK is HCLK divided by 2"]
    DIV2 = 1,
}
impl From<PCLKPRESC_A> for bool {
    #[inline(always)]
    fn from(variant: PCLKPRESC_A) -> Self {
        variant as u8 != 0
    }
}
impl PCLKPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLKPRESC_A {
        match self.bits {
            false => PCLKPRESC_A::DIV1,
            true => PCLKPRESC_A::DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PCLKPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PCLKPRESC_A::DIV2
    }
}
#[doc = "Field `PCLKPRESC` writer - PCLK Prescaler"]
pub type PCLKPRESC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCLKCTRL_SPEC, PCLKPRESC_A, O>;
impl<'a, const O: u8> PCLKPRESC_W<'a, O> {
    #[doc = "PCLK is HCLK divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PCLKPRESC_A::DIV1)
    }
    #[doc = "PCLK is HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PCLKPRESC_A::DIV2)
    }
}
#[doc = "Field `HCLKPRESC` reader - HCLK Prescaler"]
pub type HCLKPRESC_R = crate::FieldReader<u8, HCLKPRESC_A>;
#[doc = "HCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HCLKPRESC_A {
    #[doc = "0: HCLK is SYSCLK divided by 1"]
    DIV1 = 0,
    #[doc = "1: HCLK is SYSCLK divided by 2"]
    DIV2 = 1,
    #[doc = "3: HCLK is SYSCLK divided by 4"]
    DIV4 = 3,
    #[doc = "7: HCLK is SYSCLK divided by 8"]
    DIV8 = 7,
    #[doc = "15: HCLK is SYSCLK divided by 16"]
    DIV16 = 15,
}
impl From<HCLKPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: HCLKPRESC_A) -> Self {
        variant as _
    }
}
impl HCLKPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HCLKPRESC_A> {
        match self.bits {
            0 => Some(HCLKPRESC_A::DIV1),
            1 => Some(HCLKPRESC_A::DIV2),
            3 => Some(HCLKPRESC_A::DIV4),
            7 => Some(HCLKPRESC_A::DIV8),
            15 => Some(HCLKPRESC_A::DIV16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HCLKPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HCLKPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HCLKPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HCLKPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HCLKPRESC_A::DIV16
    }
}
#[doc = "Field `HCLKPRESC` writer - HCLK Prescaler"]
pub type HCLKPRESC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSCLKCTRL_SPEC, u8, HCLKPRESC_A, 4, O>;
impl<'a, const O: u8> HCLKPRESC_W<'a, O> {
    #[doc = "HCLK is SYSCLK divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HCLKPRESC_A::DIV1)
    }
    #[doc = "HCLK is SYSCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HCLKPRESC_A::DIV2)
    }
    #[doc = "HCLK is SYSCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HCLKPRESC_A::DIV4)
    }
    #[doc = "HCLK is SYSCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HCLKPRESC_A::DIV8)
    }
    #[doc = "HCLK is SYSCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HCLKPRESC_A::DIV16)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 10 - PCLK Prescaler"]
    #[inline(always)]
    pub fn pclkpresc(&self) -> PCLKPRESC_R {
        PCLKPRESC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:15 - HCLK Prescaler"]
    #[inline(always)]
    pub fn hclkpresc(&self) -> HCLKPRESC_R {
        HCLKPRESC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 10 - PCLK Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pclkpresc(&mut self) -> PCLKPRESC_W<10> {
        PCLKPRESC_W::new(self)
    }
    #[doc = "Bits 12:15 - HCLK Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hclkpresc(&mut self) -> HCLKPRESC_W<12> {
        HCLKPRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysclkctrl](index.html) module"]
pub struct SYSCLKCTRL_SPEC;
impl crate::RegisterSpec for SYSCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysclkctrl::R](R) reader structure"]
impl crate::Readable for SYSCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysclkctrl::W](W) writer structure"]
impl crate::Writable for SYSCLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCLKCTRL to value 0x01"]
impl crate::Resettable for SYSCLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
