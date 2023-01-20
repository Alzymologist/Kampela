#[doc = "Register `RFIMLCDCTRL` reader"]
pub struct R(crate::R<RFIMLCDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIMLCDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIMLCDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIMLCDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIMLCDCTRL` writer"]
pub struct W(crate::W<RFIMLCDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIMLCDCTRL_SPEC>;
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
impl From<crate::W<RFIMLCDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIMLCDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDCPXOEN` reader - LCD Charge Pump XO Clock Enable"]
pub type LCDCPXOEN_R = crate::BitReader<bool>;
#[doc = "Field `LCDCPXOEN` writer - LCD Charge Pump XO Clock Enable"]
pub type LCDCPXOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIMLCDCTRL_SPEC, bool, O>;
#[doc = "Field `LCDCPXOSEL` reader - LCD Charge Pump XO Select"]
pub type LCDCPXOSEL_R = crate::BitReader<LCDCPXOSEL_A>;
#[doc = "LCD Charge Pump XO Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCPXOSEL_A {
    #[doc = "0: Internal LCD CP 10Mhz RC oscillator"]
    INTRCO = 0,
    #[doc = "1: HFXO divided 4 clock"]
    HFXODIV = 1,
}
impl From<LCDCPXOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCPXOSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCPXOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCPXOSEL_A {
        match self.bits {
            false => LCDCPXOSEL_A::INTRCO,
            true => LCDCPXOSEL_A::HFXODIV,
        }
    }
    #[doc = "Checks if the value of the field is `INTRCO`"]
    #[inline(always)]
    pub fn is_intrco(&self) -> bool {
        *self == LCDCPXOSEL_A::INTRCO
    }
    #[doc = "Checks if the value of the field is `HFXODIV`"]
    #[inline(always)]
    pub fn is_hfxodiv(&self) -> bool {
        *self == LCDCPXOSEL_A::HFXODIV
    }
}
#[doc = "Field `LCDCPXOSEL` writer - LCD Charge Pump XO Select"]
pub type LCDCPXOSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIMLCDCTRL_SPEC, LCDCPXOSEL_A, O>;
impl<'a, const O: u8> LCDCPXOSEL_W<'a, O> {
    #[doc = "Internal LCD CP 10Mhz RC oscillator"]
    #[inline(always)]
    pub fn intrco(self) -> &'a mut W {
        self.variant(LCDCPXOSEL_A::INTRCO)
    }
    #[doc = "HFXO divided 4 clock"]
    #[inline(always)]
    pub fn hfxodiv(self) -> &'a mut W {
        self.variant(LCDCPXOSEL_A::HFXODIV)
    }
}
#[doc = "Field `LCDCPXORETIMEEN` reader - LCD Charge Pump XO Retime Enable"]
pub type LCDCPXORETIMEEN_R = crate::BitReader<bool>;
#[doc = "Field `LCDCPXORETIMEEN` writer - LCD Charge Pump XO Retime Enable"]
pub type LCDCPXORETIMEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIMLCDCTRL_SPEC, bool, O>;
#[doc = "Field `LCDLOWNOISE` reader - LCD Low Noise"]
pub type LCDLOWNOISE_R = crate::BitReader<LCDLOWNOISE_A>;
#[doc = "LCD Low Noise\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDLOWNOISE_A {
    #[doc = "0: Normal operation"]
    NORMAL = 0,
    #[doc = "1: slows down slew rate to reduce RF interference at a cost of additional power consumption"]
    SLOW = 1,
}
impl From<LCDLOWNOISE_A> for bool {
    #[inline(always)]
    fn from(variant: LCDLOWNOISE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDLOWNOISE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDLOWNOISE_A {
        match self.bits {
            false => LCDLOWNOISE_A::NORMAL,
            true => LCDLOWNOISE_A::SLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LCDLOWNOISE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == LCDLOWNOISE_A::SLOW
    }
}
#[doc = "Field `LCDLOWNOISE` writer - LCD Low Noise"]
pub type LCDLOWNOISE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RFIMLCDCTRL_SPEC, LCDLOWNOISE_A, O>;
impl<'a, const O: u8> LCDLOWNOISE_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LCDLOWNOISE_A::NORMAL)
    }
    #[doc = "slows down slew rate to reduce RF interference at a cost of additional power consumption"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(LCDLOWNOISE_A::SLOW)
    }
}
#[doc = "Field `LCDCMPDOUT` reader - LCD Comparator Dout"]
pub type LCDCMPDOUT_R = crate::BitReader<bool>;
#[doc = "Field `LCDCMPDOUT` writer - LCD Comparator Dout"]
pub type LCDCMPDOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIMLCDCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LCD Charge Pump XO Clock Enable"]
    #[inline(always)]
    pub fn lcdcpxoen(&self) -> LCDCPXOEN_R {
        LCDCPXOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD Charge Pump XO Select"]
    #[inline(always)]
    pub fn lcdcpxosel(&self) -> LCDCPXOSEL_R {
        LCDCPXOSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD Charge Pump XO Retime Enable"]
    #[inline(always)]
    pub fn lcdcpxoretimeen(&self) -> LCDCPXORETIMEEN_R {
        LCDCPXORETIMEEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD Low Noise"]
    #[inline(always)]
    pub fn lcdlownoise(&self) -> LCDLOWNOISE_R {
        LCDLOWNOISE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD Comparator Dout"]
    #[inline(always)]
    pub fn lcdcmpdout(&self) -> LCDCMPDOUT_R {
        LCDCMPDOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Charge Pump XO Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcdcpxoen(&mut self) -> LCDCPXOEN_W<0> {
        LCDCPXOEN_W::new(self)
    }
    #[doc = "Bit 1 - LCD Charge Pump XO Select"]
    #[inline(always)]
    #[must_use]
    pub fn lcdcpxosel(&mut self) -> LCDCPXOSEL_W<1> {
        LCDCPXOSEL_W::new(self)
    }
    #[doc = "Bit 2 - LCD Charge Pump XO Retime Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcdcpxoretimeen(&mut self) -> LCDCPXORETIMEEN_W<2> {
        LCDCPXORETIMEEN_W::new(self)
    }
    #[doc = "Bit 3 - LCD Low Noise"]
    #[inline(always)]
    #[must_use]
    pub fn lcdlownoise(&mut self) -> LCDLOWNOISE_W<3> {
        LCDLOWNOISE_W::new(self)
    }
    #[doc = "Bit 4 - LCD Comparator Dout"]
    #[inline(always)]
    #[must_use]
    pub fn lcdcmpdout(&mut self) -> LCDCMPDOUT_W<4> {
        LCDCMPDOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfimlcdctrl](index.html) module"]
pub struct RFIMLCDCTRL_SPEC;
impl crate::RegisterSpec for RFIMLCDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfimlcdctrl::R](R) reader structure"]
impl crate::Readable for RFIMLCDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfimlcdctrl::W](W) writer structure"]
impl crate::Writable for RFIMLCDCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFIMLCDCTRL to value 0"]
impl crate::Resettable for RFIMLCDCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
