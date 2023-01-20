#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader<DEBUGRUN_A>;
#[doc = "Debug Mode Run Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBUGRUN_A {
    #[doc = "0: BURTC is frozen in debug mode"]
    X0 = 0,
    #[doc = "1: BURTC is running in debug mode"]
    X1 = 1,
}
impl From<DEBUGRUN_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUGRUN_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBUGRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUGRUN_A {
        match self.bits {
            false => DEBUGRUN_A::X0,
            true => DEBUGRUN_A::X1,
        }
    }
    #[doc = "Checks if the value of the field is `X0`"]
    #[inline(always)]
    pub fn is_x0(&self) -> bool {
        *self == DEBUGRUN_A::X0
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == DEBUGRUN_A::X1
    }
}
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, DEBUGRUN_A, O>;
impl<'a, const O: u8> DEBUGRUN_W<'a, O> {
    #[doc = "BURTC is frozen in debug mode"]
    #[inline(always)]
    pub fn x0(self) -> &'a mut W {
        self.variant(DEBUGRUN_A::X0)
    }
    #[doc = "BURTC is running in debug mode"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(DEBUGRUN_A::X1)
    }
}
#[doc = "Field `COMPTOP` reader - Compare Channel is Top Value"]
pub type COMPTOP_R = crate::BitReader<COMPTOP_A>;
#[doc = "Compare Channel is Top Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPTOP_A {
    #[doc = "0: The top value of the BURTC is 4294967295 (0xFFFFFFFF)"]
    DISABLE = 0,
    #[doc = "1: The top value of the BURTC is given by COMP"]
    ENABLE = 1,
}
impl From<COMPTOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPTOP_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPTOP_A {
        match self.bits {
            false => COMPTOP_A::DISABLE,
            true => COMPTOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COMPTOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COMPTOP_A::ENABLE
    }
}
#[doc = "Field `COMPTOP` writer - Compare Channel is Top Value"]
pub type COMPTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, COMPTOP_A, O>;
impl<'a, const O: u8> COMPTOP_W<'a, O> {
    #[doc = "The top value of the BURTC is 4294967295 (0xFFFFFFFF)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPTOP_A::DISABLE)
    }
    #[doc = "The top value of the BURTC is given by COMP"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPTOP_A::ENABLE)
    }
}
#[doc = "Field `CNTPRESC` reader - Counter prescaler value."]
pub type CNTPRESC_R = crate::FieldReader<u8, CNTPRESC_A>;
#[doc = "Counter prescaler value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTPRESC_A {
    #[doc = "0: CLK_CNT = (BURTC LF CLK)/1"]
    DIV1 = 0,
    #[doc = "1: CLK_CNT = (BURTC LF CLK)/2"]
    DIV2 = 1,
    #[doc = "2: CLK_CNT = (BURTC LF CLK)/4"]
    DIV4 = 2,
    #[doc = "3: CLK_CNT = (BURTC LF CLK)/8"]
    DIV8 = 3,
    #[doc = "4: CLK_CNT = (BURTC LF CLK)/16"]
    DIV16 = 4,
    #[doc = "5: CLK_CNT = (BURTC LF CLK)/32"]
    DIV32 = 5,
    #[doc = "6: CLK_CNT = (BURTC LF CLK)/64"]
    DIV64 = 6,
    #[doc = "7: CLK_CNT = (BURTC LF CLK)/128"]
    DIV128 = 7,
    #[doc = "8: CLK_CNT = (BURTC LF CLK)/256"]
    DIV256 = 8,
    #[doc = "9: CLK_CNT = (BURTC LF CLK)/512"]
    DIV512 = 9,
    #[doc = "10: CLK_CNT = (BURTC LF CLK)/1024"]
    DIV1024 = 10,
    #[doc = "11: CLK_CNT = (BURTC LF CLK)/2048"]
    DIV2048 = 11,
    #[doc = "12: CLK_CNT = (BURTC LF CLK)/4096"]
    DIV4096 = 12,
    #[doc = "13: CLK_CNT = (BURTC LF CLK)/8192"]
    DIV8192 = 13,
    #[doc = "14: CLK_CNT = (BURTC LF CLK)/16384"]
    DIV16384 = 14,
    #[doc = "15: CLK_CNT = (BURTC LF CLK)/32768"]
    DIV32768 = 15,
}
impl From<CNTPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTPRESC_A) -> Self {
        variant as _
    }
}
impl CNTPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTPRESC_A {
        match self.bits {
            0 => CNTPRESC_A::DIV1,
            1 => CNTPRESC_A::DIV2,
            2 => CNTPRESC_A::DIV4,
            3 => CNTPRESC_A::DIV8,
            4 => CNTPRESC_A::DIV16,
            5 => CNTPRESC_A::DIV32,
            6 => CNTPRESC_A::DIV64,
            7 => CNTPRESC_A::DIV128,
            8 => CNTPRESC_A::DIV256,
            9 => CNTPRESC_A::DIV512,
            10 => CNTPRESC_A::DIV1024,
            11 => CNTPRESC_A::DIV2048,
            12 => CNTPRESC_A::DIV4096,
            13 => CNTPRESC_A::DIV8192,
            14 => CNTPRESC_A::DIV16384,
            15 => CNTPRESC_A::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CNTPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CNTPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CNTPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CNTPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CNTPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CNTPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CNTPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CNTPRESC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == CNTPRESC_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == CNTPRESC_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == CNTPRESC_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == CNTPRESC_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == CNTPRESC_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == CNTPRESC_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == CNTPRESC_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == CNTPRESC_A::DIV32768
    }
}
#[doc = "Field `CNTPRESC` writer - Counter prescaler value."]
pub type CNTPRESC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, CNTPRESC_A, 4, O>;
impl<'a, const O: u8> CNTPRESC_W<'a, O> {
    #[doc = "CLK_CNT = (BURTC LF CLK)/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV1)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV2)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV4)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV8)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV16)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV32)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV64)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV128)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV256)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV512)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV1024)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV2048)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV4096)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV8192)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV16384)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV32768)
    }
}
impl R {
    #[doc = "Bit 0 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Channel is Top Value"]
    #[inline(always)]
    pub fn comptop(&self) -> COMPTOP_R {
        COMPTOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Counter prescaler value."]
    #[inline(always)]
    pub fn cntpresc(&self) -> CNTPRESC_R {
        CNTPRESC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<0> {
        DEBUGRUN_W::new(self)
    }
    #[doc = "Bit 1 - Compare Channel is Top Value"]
    #[inline(always)]
    #[must_use]
    pub fn comptop(&mut self) -> COMPTOP_W<1> {
        COMPTOP_W::new(self)
    }
    #[doc = "Bits 4:7 - Counter prescaler value."]
    #[inline(always)]
    #[must_use]
    pub fn cntpresc(&mut self) -> CNTPRESC_W<4> {
        CNTPRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
