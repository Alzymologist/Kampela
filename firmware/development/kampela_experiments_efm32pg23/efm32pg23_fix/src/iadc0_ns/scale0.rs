#[doc = "Register `SCALE0` reader"]
pub struct R(crate::R<SCALE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCALE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCALE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCALE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCALE0` writer"]
pub struct W(crate::W<SCALE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCALE0_SPEC>;
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
impl From<crate::W<SCALE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCALE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET` reader - Offset"]
pub type OFFSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OFFSET` writer - Offset"]
pub type OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCALE0_SPEC, u32, u32, 18, O>;
#[doc = "Field `GAIN13LSB` reader - Gain 13 LSBs"]
pub type GAIN13LSB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GAIN13LSB` writer - Gain 13 LSBs"]
pub type GAIN13LSB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCALE0_SPEC, u16, u16, 13, O>;
#[doc = "Field `GAIN3MSB` reader - Gain 3 MSBs"]
pub type GAIN3MSB_R = crate::BitReader<GAIN3MSB_A>;
#[doc = "Gain 3 MSBs\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAIN3MSB_A {
    #[doc = "0: Upper 3 bits of gain = 011 (0.75x)"]
    GAIN011 = 0,
    #[doc = "1: Upper 3 bits of gain = 100 (1.00x)"]
    GAIN100 = 1,
}
impl From<GAIN3MSB_A> for bool {
    #[inline(always)]
    fn from(variant: GAIN3MSB_A) -> Self {
        variant as u8 != 0
    }
}
impl GAIN3MSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN3MSB_A {
        match self.bits {
            false => GAIN3MSB_A::GAIN011,
            true => GAIN3MSB_A::GAIN100,
        }
    }
    #[doc = "Checks if the value of the field is `GAIN011`"]
    #[inline(always)]
    pub fn is_gain011(&self) -> bool {
        *self == GAIN3MSB_A::GAIN011
    }
    #[doc = "Checks if the value of the field is `GAIN100`"]
    #[inline(always)]
    pub fn is_gain100(&self) -> bool {
        *self == GAIN3MSB_A::GAIN100
    }
}
#[doc = "Field `GAIN3MSB` writer - Gain 3 MSBs"]
pub type GAIN3MSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCALE0_SPEC, GAIN3MSB_A, O>;
impl<'a, const O: u8> GAIN3MSB_W<'a, O> {
    #[doc = "Upper 3 bits of gain = 011 (0.75x)"]
    #[inline(always)]
    pub fn gain011(self) -> &'a mut W {
        self.variant(GAIN3MSB_A::GAIN011)
    }
    #[doc = "Upper 3 bits of gain = 100 (1.00x)"]
    #[inline(always)]
    pub fn gain100(self) -> &'a mut W {
        self.variant(GAIN3MSB_A::GAIN100)
    }
}
impl R {
    #[doc = "Bits 0:17 - Offset"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:30 - Gain 13 LSBs"]
    #[inline(always)]
    pub fn gain13lsb(&self) -> GAIN13LSB_R {
        GAIN13LSB_R::new(((self.bits >> 18) & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - Gain 3 MSBs"]
    #[inline(always)]
    pub fn gain3msb(&self) -> GAIN3MSB_R {
        GAIN3MSB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Offset"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<0> {
        OFFSET_W::new(self)
    }
    #[doc = "Bits 18:30 - Gain 13 LSBs"]
    #[inline(always)]
    #[must_use]
    pub fn gain13lsb(&mut self) -> GAIN13LSB_W<18> {
        GAIN13LSB_W::new(self)
    }
    #[doc = "Bit 31 - Gain 3 MSBs"]
    #[inline(always)]
    #[must_use]
    pub fn gain3msb(&mut self) -> GAIN3MSB_W<31> {
        GAIN3MSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scale\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scale0](index.html) module"]
pub struct SCALE0_SPEC;
impl crate::RegisterSpec for SCALE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scale0::R](R) reader structure"]
impl crate::Readable for SCALE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scale0::W](W) writer structure"]
impl crate::Writable for SCALE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCALE0 to value 0x8002_c000"]
impl crate::Resettable for SCALE0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8002_c000;
}
