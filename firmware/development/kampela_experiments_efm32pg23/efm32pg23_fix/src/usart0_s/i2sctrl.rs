#[doc = "Register `I2SCTRL` reader"]
pub struct R(crate::R<I2SCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCTRL` writer"]
pub struct W(crate::W<I2SCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCTRL_SPEC>;
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
impl From<crate::W<I2SCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable I2S Mode"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable I2S Mode"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCTRL_SPEC, bool, O>;
#[doc = "Field `MONO` reader - Stero or Mono"]
pub type MONO_R = crate::BitReader<bool>;
#[doc = "Field `MONO` writer - Stero or Mono"]
pub type MONO_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCTRL_SPEC, bool, O>;
#[doc = "Field `JUSTIFY` reader - Justification of I2S Data"]
pub type JUSTIFY_R = crate::BitReader<JUSTIFY_A>;
#[doc = "Justification of I2S Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JUSTIFY_A {
    #[doc = "0: Data is left-justified"]
    LEFT = 0,
    #[doc = "1: Data is right-justified"]
    RIGHT = 1,
}
impl From<JUSTIFY_A> for bool {
    #[inline(always)]
    fn from(variant: JUSTIFY_A) -> Self {
        variant as u8 != 0
    }
}
impl JUSTIFY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JUSTIFY_A {
        match self.bits {
            false => JUSTIFY_A::LEFT,
            true => JUSTIFY_A::RIGHT,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == JUSTIFY_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == JUSTIFY_A::RIGHT
    }
}
#[doc = "Field `JUSTIFY` writer - Justification of I2S Data"]
pub type JUSTIFY_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCTRL_SPEC, JUSTIFY_A, O>;
impl<'a, const O: u8> JUSTIFY_W<'a, O> {
    #[doc = "Data is left-justified"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(JUSTIFY_A::LEFT)
    }
    #[doc = "Data is right-justified"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(JUSTIFY_A::RIGHT)
    }
}
#[doc = "Field `DMASPLIT` reader - Separate DMA Request For Left/Right Data"]
pub type DMASPLIT_R = crate::BitReader<bool>;
#[doc = "Field `DMASPLIT` writer - Separate DMA Request For Left/Right Data"]
pub type DMASPLIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCTRL_SPEC, bool, O>;
#[doc = "Field `DELAY` reader - Delay on I2S data"]
pub type DELAY_R = crate::BitReader<bool>;
#[doc = "Field `DELAY` writer - Delay on I2S data"]
pub type DELAY_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCTRL_SPEC, bool, O>;
#[doc = "Field `FORMAT` reader - I2S Word Format"]
pub type FORMAT_R = crate::FieldReader<u8, FORMAT_A>;
#[doc = "I2S Word Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "0: 32-bit word, 32-bit data"]
    W32D32 = 0,
    #[doc = "1: 32-bit word, 32-bit data with 8 lsb masked"]
    W32D24M = 1,
    #[doc = "2: 32-bit word, 24-bit data"]
    W32D24 = 2,
    #[doc = "3: 32-bit word, 16-bit data"]
    W32D16 = 3,
    #[doc = "4: 32-bit word, 8-bit data"]
    W32D8 = 4,
    #[doc = "5: 16-bit word, 16-bit data"]
    W16D16 = 5,
    #[doc = "6: 16-bit word, 8-bit data"]
    W16D8 = 6,
    #[doc = "7: 8-bit word, 8-bit data"]
    W8D8 = 7,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
impl FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORMAT_A {
        match self.bits {
            0 => FORMAT_A::W32D32,
            1 => FORMAT_A::W32D24M,
            2 => FORMAT_A::W32D24,
            3 => FORMAT_A::W32D16,
            4 => FORMAT_A::W32D8,
            5 => FORMAT_A::W16D16,
            6 => FORMAT_A::W16D8,
            7 => FORMAT_A::W8D8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `W32D32`"]
    #[inline(always)]
    pub fn is_w32d32(&self) -> bool {
        *self == FORMAT_A::W32D32
    }
    #[doc = "Checks if the value of the field is `W32D24M`"]
    #[inline(always)]
    pub fn is_w32d24m(&self) -> bool {
        *self == FORMAT_A::W32D24M
    }
    #[doc = "Checks if the value of the field is `W32D24`"]
    #[inline(always)]
    pub fn is_w32d24(&self) -> bool {
        *self == FORMAT_A::W32D24
    }
    #[doc = "Checks if the value of the field is `W32D16`"]
    #[inline(always)]
    pub fn is_w32d16(&self) -> bool {
        *self == FORMAT_A::W32D16
    }
    #[doc = "Checks if the value of the field is `W32D8`"]
    #[inline(always)]
    pub fn is_w32d8(&self) -> bool {
        *self == FORMAT_A::W32D8
    }
    #[doc = "Checks if the value of the field is `W16D16`"]
    #[inline(always)]
    pub fn is_w16d16(&self) -> bool {
        *self == FORMAT_A::W16D16
    }
    #[doc = "Checks if the value of the field is `W16D8`"]
    #[inline(always)]
    pub fn is_w16d8(&self) -> bool {
        *self == FORMAT_A::W16D8
    }
    #[doc = "Checks if the value of the field is `W8D8`"]
    #[inline(always)]
    pub fn is_w8d8(&self) -> bool {
        *self == FORMAT_A::W8D8
    }
}
#[doc = "Field `FORMAT` writer - I2S Word Format"]
pub type FORMAT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I2SCTRL_SPEC, u8, FORMAT_A, 3, O>;
impl<'a, const O: u8> FORMAT_W<'a, O> {
    #[doc = "32-bit word, 32-bit data"]
    #[inline(always)]
    pub fn w32d32(self) -> &'a mut W {
        self.variant(FORMAT_A::W32D32)
    }
    #[doc = "32-bit word, 32-bit data with 8 lsb masked"]
    #[inline(always)]
    pub fn w32d24m(self) -> &'a mut W {
        self.variant(FORMAT_A::W32D24M)
    }
    #[doc = "32-bit word, 24-bit data"]
    #[inline(always)]
    pub fn w32d24(self) -> &'a mut W {
        self.variant(FORMAT_A::W32D24)
    }
    #[doc = "32-bit word, 16-bit data"]
    #[inline(always)]
    pub fn w32d16(self) -> &'a mut W {
        self.variant(FORMAT_A::W32D16)
    }
    #[doc = "32-bit word, 8-bit data"]
    #[inline(always)]
    pub fn w32d8(self) -> &'a mut W {
        self.variant(FORMAT_A::W32D8)
    }
    #[doc = "16-bit word, 16-bit data"]
    #[inline(always)]
    pub fn w16d16(self) -> &'a mut W {
        self.variant(FORMAT_A::W16D16)
    }
    #[doc = "16-bit word, 8-bit data"]
    #[inline(always)]
    pub fn w16d8(self) -> &'a mut W {
        self.variant(FORMAT_A::W16D8)
    }
    #[doc = "8-bit word, 8-bit data"]
    #[inline(always)]
    pub fn w8d8(self) -> &'a mut W {
        self.variant(FORMAT_A::W8D8)
    }
}
impl R {
    #[doc = "Bit 0 - Enable I2S Mode"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stero or Mono"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Justification of I2S Data"]
    #[inline(always)]
    pub fn justify(&self) -> JUSTIFY_R {
        JUSTIFY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Separate DMA Request For Left/Right Data"]
    #[inline(always)]
    pub fn dmasplit(&self) -> DMASPLIT_R {
        DMASPLIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Delay on I2S data"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - I2S Word Format"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable I2S Mode"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Stero or Mono"]
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MONO_W<1> {
        MONO_W::new(self)
    }
    #[doc = "Bit 2 - Justification of I2S Data"]
    #[inline(always)]
    #[must_use]
    pub fn justify(&mut self) -> JUSTIFY_W<2> {
        JUSTIFY_W::new(self)
    }
    #[doc = "Bit 3 - Separate DMA Request For Left/Right Data"]
    #[inline(always)]
    #[must_use]
    pub fn dmasplit(&mut self) -> DMASPLIT_W<3> {
        DMASPLIT_W::new(self)
    }
    #[doc = "Bit 4 - Delay on I2S data"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<4> {
        DELAY_W::new(self)
    }
    #[doc = "Bits 8:10 - I2S Word Format"]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<8> {
        FORMAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sctrl](index.html) module"]
pub struct I2SCTRL_SPEC;
impl crate::RegisterSpec for I2SCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sctrl::R](R) reader structure"]
impl crate::Readable for I2SCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sctrl::W](W) writer structure"]
impl crate::Writable for I2SCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCTRL to value 0"]
impl crate::Resettable for I2SCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
