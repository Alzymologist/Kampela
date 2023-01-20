#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POLYSEL` reader - Polynomial Select"]
pub type POLYSEL_R = crate::BitReader<POLYSEL_A>;
#[doc = "Polynomial Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLYSEL_A {
    #[doc = "0: CRC-32 (0x04C11DB7) polynomial selected"]
    CRC32 = 0,
    #[doc = "1: 16-bit CRC programmable polynomial selected"]
    CRC16 = 1,
}
impl From<POLYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: POLYSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl POLYSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLYSEL_A {
        match self.bits {
            false => POLYSEL_A::CRC32,
            true => POLYSEL_A::CRC16,
        }
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == POLYSEL_A::CRC32
    }
    #[doc = "Checks if the value of the field is `CRC16`"]
    #[inline(always)]
    pub fn is_crc16(&self) -> bool {
        *self == POLYSEL_A::CRC16
    }
}
#[doc = "Field `POLYSEL` writer - Polynomial Select"]
pub type POLYSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, POLYSEL_A, O>;
impl<'a, const O: u8> POLYSEL_W<'a, O> {
    #[doc = "CRC-32 (0x04C11DB7) polynomial selected"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut W {
        self.variant(POLYSEL_A::CRC32)
    }
    #[doc = "16-bit CRC programmable polynomial selected"]
    #[inline(always)]
    pub fn crc16(self) -> &'a mut W {
        self.variant(POLYSEL_A::CRC16)
    }
}
#[doc = "Field `BYTEMODE` reader - Byte Mode Enable"]
pub type BYTEMODE_R = crate::BitReader<bool>;
#[doc = "Field `BYTEMODE` writer - Byte Mode Enable"]
pub type BYTEMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BITREVERSE` reader - Byte-level Bit Reverse Enable"]
pub type BITREVERSE_R = crate::BitReader<BITREVERSE_A>;
#[doc = "Byte-level Bit Reverse Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BITREVERSE_A {
    #[doc = "0: No reverse"]
    NORMAL = 0,
    #[doc = "1: Reverse bit order in each byte"]
    REVERSED = 1,
}
impl From<BITREVERSE_A> for bool {
    #[inline(always)]
    fn from(variant: BITREVERSE_A) -> Self {
        variant as u8 != 0
    }
}
impl BITREVERSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BITREVERSE_A {
        match self.bits {
            false => BITREVERSE_A::NORMAL,
            true => BITREVERSE_A::REVERSED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BITREVERSE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `REVERSED`"]
    #[inline(always)]
    pub fn is_reversed(&self) -> bool {
        *self == BITREVERSE_A::REVERSED
    }
}
#[doc = "Field `BITREVERSE` writer - Byte-level Bit Reverse Enable"]
pub type BITREVERSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, BITREVERSE_A, O>;
impl<'a, const O: u8> BITREVERSE_W<'a, O> {
    #[doc = "No reverse"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BITREVERSE_A::NORMAL)
    }
    #[doc = "Reverse bit order in each byte"]
    #[inline(always)]
    pub fn reversed(self) -> &'a mut W {
        self.variant(BITREVERSE_A::REVERSED)
    }
}
#[doc = "Field `BYTEREVERSE` reader - Byte Reverse Mode"]
pub type BYTEREVERSE_R = crate::BitReader<BYTEREVERSE_A>;
#[doc = "Byte Reverse Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYTEREVERSE_A {
    #[doc = "0: No reverse: B3, B2, B1, B0"]
    NORMAL = 0,
    #[doc = "1: Reverse byte order. For 32-bit: B0, B1, B2, B3; For 16-bit: 0, 0, B0, B1"]
    REVERSED = 1,
}
impl From<BYTEREVERSE_A> for bool {
    #[inline(always)]
    fn from(variant: BYTEREVERSE_A) -> Self {
        variant as u8 != 0
    }
}
impl BYTEREVERSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYTEREVERSE_A {
        match self.bits {
            false => BYTEREVERSE_A::NORMAL,
            true => BYTEREVERSE_A::REVERSED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BYTEREVERSE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `REVERSED`"]
    #[inline(always)]
    pub fn is_reversed(&self) -> bool {
        *self == BYTEREVERSE_A::REVERSED
    }
}
#[doc = "Field `BYTEREVERSE` writer - Byte Reverse Mode"]
pub type BYTEREVERSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, BYTEREVERSE_A, O>;
impl<'a, const O: u8> BYTEREVERSE_W<'a, O> {
    #[doc = "No reverse: B3, B2, B1, B0"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BYTEREVERSE_A::NORMAL)
    }
    #[doc = "Reverse byte order. For 32-bit: B0, B1, B2, B3; For 16-bit: 0, 0, B0, B1"]
    #[inline(always)]
    pub fn reversed(self) -> &'a mut W {
        self.variant(BYTEREVERSE_A::REVERSED)
    }
}
#[doc = "Field `AUTOINIT` reader - Auto Init Enable"]
pub type AUTOINIT_R = crate::BitReader<bool>;
#[doc = "Field `AUTOINIT` writer - Auto Init Enable"]
pub type AUTOINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - Polynomial Select"]
    #[inline(always)]
    pub fn polysel(&self) -> POLYSEL_R {
        POLYSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Byte Mode Enable"]
    #[inline(always)]
    pub fn bytemode(&self) -> BYTEMODE_R {
        BYTEMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Byte-level Bit Reverse Enable"]
    #[inline(always)]
    pub fn bitreverse(&self) -> BITREVERSE_R {
        BITREVERSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Byte Reverse Mode"]
    #[inline(always)]
    pub fn bytereverse(&self) -> BYTEREVERSE_R {
        BYTEREVERSE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Auto Init Enable"]
    #[inline(always)]
    pub fn autoinit(&self) -> AUTOINIT_R {
        AUTOINIT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Polynomial Select"]
    #[inline(always)]
    #[must_use]
    pub fn polysel(&mut self) -> POLYSEL_W<4> {
        POLYSEL_W::new(self)
    }
    #[doc = "Bit 8 - Byte Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bytemode(&mut self) -> BYTEMODE_W<8> {
        BYTEMODE_W::new(self)
    }
    #[doc = "Bit 9 - Byte-level Bit Reverse Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bitreverse(&mut self) -> BITREVERSE_W<9> {
        BITREVERSE_W::new(self)
    }
    #[doc = "Bit 10 - Byte Reverse Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bytereverse(&mut self) -> BYTEREVERSE_W<10> {
        BYTEREVERSE_W::new(self)
    }
    #[doc = "Bit 13 - Auto Init Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autoinit(&mut self) -> AUTOINIT_W<13> {
        AUTOINIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
