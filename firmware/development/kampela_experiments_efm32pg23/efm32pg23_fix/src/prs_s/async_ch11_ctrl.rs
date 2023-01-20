#[doc = "Register `ASYNC_CH11_CTRL` reader"]
pub struct R(crate::R<ASYNC_CH11_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNC_CH11_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNC_CH11_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNC_CH11_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNC_CH11_CTRL` writer"]
pub struct W(crate::W<ASYNC_CH11_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNC_CH11_CTRL_SPEC>;
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
impl From<crate::W<ASYNC_CH11_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNC_CH11_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SIGSEL_R = crate::FieldReader<u8, SIGSEL_A>;
#[doc = "Signal Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIGSEL_A {
    #[doc = "0: NONE"]
    NONE = 0,
}
impl From<SIGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SIGSEL_A) -> Self {
        variant as _
    }
}
impl SIGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIGSEL_A> {
        match self.bits {
            0 => Some(SIGSEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SIGSEL_A::NONE
    }
}
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SIGSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ASYNC_CH11_CTRL_SPEC, u8, SIGSEL_A, 3, O>;
impl<'a, const O: u8> SIGSEL_W<'a, O> {
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SIGSEL_A::NONE)
    }
}
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SOURCESEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SOURCESEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ASYNC_CH11_CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `FNSEL` reader - Function Select"]
pub type FNSEL_R = crate::FieldReader<u8, FNSEL_A>;
#[doc = "Function Select\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FNSEL_A {
    #[doc = "0: Logical 0"]
    LOGICAL_ZERO = 0,
    #[doc = "1: A NOR B"]
    A_NOR_B = 1,
    #[doc = "2: (!A) AND B"]
    NOT_A_AND_B = 2,
    #[doc = "3: !A"]
    NOT_A = 3,
    #[doc = "4: A AND (!B)"]
    A_AND_NOT_B = 4,
    #[doc = "5: !B"]
    NOT_B = 5,
    #[doc = "6: A XOR B"]
    A_XOR_B = 6,
    #[doc = "7: A NAND B"]
    A_NAND_B = 7,
    #[doc = "8: A AND B"]
    A_AND_B = 8,
    #[doc = "9: A XNOR B"]
    A_XNOR_B = 9,
    #[doc = "10: B"]
    B = 10,
    #[doc = "11: (!A) OR B"]
    NOT_A_OR_B = 11,
    #[doc = "12: A"]
    A = 12,
    #[doc = "13: A OR (!B)"]
    A_OR_NOT_B = 13,
    #[doc = "14: A OR B"]
    A_OR_B = 14,
    #[doc = "15: Logical 1"]
    LOGICAL_ONE = 15,
}
impl From<FNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FNSEL_A) -> Self {
        variant as _
    }
}
impl FNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FNSEL_A {
        match self.bits {
            0 => FNSEL_A::LOGICAL_ZERO,
            1 => FNSEL_A::A_NOR_B,
            2 => FNSEL_A::NOT_A_AND_B,
            3 => FNSEL_A::NOT_A,
            4 => FNSEL_A::A_AND_NOT_B,
            5 => FNSEL_A::NOT_B,
            6 => FNSEL_A::A_XOR_B,
            7 => FNSEL_A::A_NAND_B,
            8 => FNSEL_A::A_AND_B,
            9 => FNSEL_A::A_XNOR_B,
            10 => FNSEL_A::B,
            11 => FNSEL_A::NOT_A_OR_B,
            12 => FNSEL_A::A,
            13 => FNSEL_A::A_OR_NOT_B,
            14 => FNSEL_A::A_OR_B,
            15 => FNSEL_A::LOGICAL_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOGICAL_ZERO`"]
    #[inline(always)]
    pub fn is_logical_zero(&self) -> bool {
        *self == FNSEL_A::LOGICAL_ZERO
    }
    #[doc = "Checks if the value of the field is `A_NOR_B`"]
    #[inline(always)]
    pub fn is_a_nor_b(&self) -> bool {
        *self == FNSEL_A::A_NOR_B
    }
    #[doc = "Checks if the value of the field is `NOT_A_AND_B`"]
    #[inline(always)]
    pub fn is_not_a_and_b(&self) -> bool {
        *self == FNSEL_A::NOT_A_AND_B
    }
    #[doc = "Checks if the value of the field is `NOT_A`"]
    #[inline(always)]
    pub fn is_not_a(&self) -> bool {
        *self == FNSEL_A::NOT_A
    }
    #[doc = "Checks if the value of the field is `A_AND_NOT_B`"]
    #[inline(always)]
    pub fn is_a_and_not_b(&self) -> bool {
        *self == FNSEL_A::A_AND_NOT_B
    }
    #[doc = "Checks if the value of the field is `NOT_B`"]
    #[inline(always)]
    pub fn is_not_b(&self) -> bool {
        *self == FNSEL_A::NOT_B
    }
    #[doc = "Checks if the value of the field is `A_XOR_B`"]
    #[inline(always)]
    pub fn is_a_xor_b(&self) -> bool {
        *self == FNSEL_A::A_XOR_B
    }
    #[doc = "Checks if the value of the field is `A_NAND_B`"]
    #[inline(always)]
    pub fn is_a_nand_b(&self) -> bool {
        *self == FNSEL_A::A_NAND_B
    }
    #[doc = "Checks if the value of the field is `A_AND_B`"]
    #[inline(always)]
    pub fn is_a_and_b(&self) -> bool {
        *self == FNSEL_A::A_AND_B
    }
    #[doc = "Checks if the value of the field is `A_XNOR_B`"]
    #[inline(always)]
    pub fn is_a_xnor_b(&self) -> bool {
        *self == FNSEL_A::A_XNOR_B
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == FNSEL_A::B
    }
    #[doc = "Checks if the value of the field is `NOT_A_OR_B`"]
    #[inline(always)]
    pub fn is_not_a_or_b(&self) -> bool {
        *self == FNSEL_A::NOT_A_OR_B
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == FNSEL_A::A
    }
    #[doc = "Checks if the value of the field is `A_OR_NOT_B`"]
    #[inline(always)]
    pub fn is_a_or_not_b(&self) -> bool {
        *self == FNSEL_A::A_OR_NOT_B
    }
    #[doc = "Checks if the value of the field is `A_OR_B`"]
    #[inline(always)]
    pub fn is_a_or_b(&self) -> bool {
        *self == FNSEL_A::A_OR_B
    }
    #[doc = "Checks if the value of the field is `LOGICAL_ONE`"]
    #[inline(always)]
    pub fn is_logical_one(&self) -> bool {
        *self == FNSEL_A::LOGICAL_ONE
    }
}
#[doc = "Field `FNSEL` writer - Function Select"]
pub type FNSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ASYNC_CH11_CTRL_SPEC, u8, FNSEL_A, 4, O>;
impl<'a, const O: u8> FNSEL_W<'a, O> {
    #[doc = "Logical 0"]
    #[inline(always)]
    pub fn logical_zero(self) -> &'a mut W {
        self.variant(FNSEL_A::LOGICAL_ZERO)
    }
    #[doc = "A NOR B"]
    #[inline(always)]
    pub fn a_nor_b(self) -> &'a mut W {
        self.variant(FNSEL_A::A_NOR_B)
    }
    #[doc = "(!A) AND B"]
    #[inline(always)]
    pub fn not_a_and_b(self) -> &'a mut W {
        self.variant(FNSEL_A::NOT_A_AND_B)
    }
    #[doc = "!A"]
    #[inline(always)]
    pub fn not_a(self) -> &'a mut W {
        self.variant(FNSEL_A::NOT_A)
    }
    #[doc = "A AND (!B)"]
    #[inline(always)]
    pub fn a_and_not_b(self) -> &'a mut W {
        self.variant(FNSEL_A::A_AND_NOT_B)
    }
    #[doc = "!B"]
    #[inline(always)]
    pub fn not_b(self) -> &'a mut W {
        self.variant(FNSEL_A::NOT_B)
    }
    #[doc = "A XOR B"]
    #[inline(always)]
    pub fn a_xor_b(self) -> &'a mut W {
        self.variant(FNSEL_A::A_XOR_B)
    }
    #[doc = "A NAND B"]
    #[inline(always)]
    pub fn a_nand_b(self) -> &'a mut W {
        self.variant(FNSEL_A::A_NAND_B)
    }
    #[doc = "A AND B"]
    #[inline(always)]
    pub fn a_and_b(self) -> &'a mut W {
        self.variant(FNSEL_A::A_AND_B)
    }
    #[doc = "A XNOR B"]
    #[inline(always)]
    pub fn a_xnor_b(self) -> &'a mut W {
        self.variant(FNSEL_A::A_XNOR_B)
    }
    #[doc = "B"]
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(FNSEL_A::B)
    }
    #[doc = "(!A) OR B"]
    #[inline(always)]
    pub fn not_a_or_b(self) -> &'a mut W {
        self.variant(FNSEL_A::NOT_A_OR_B)
    }
    #[doc = "A"]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(FNSEL_A::A)
    }
    #[doc = "A OR (!B)"]
    #[inline(always)]
    pub fn a_or_not_b(self) -> &'a mut W {
        self.variant(FNSEL_A::A_OR_NOT_B)
    }
    #[doc = "A OR B"]
    #[inline(always)]
    pub fn a_or_b(self) -> &'a mut W {
        self.variant(FNSEL_A::A_OR_B)
    }
    #[doc = "Logical 1"]
    #[inline(always)]
    pub fn logical_one(self) -> &'a mut W {
        self.variant(FNSEL_A::LOGICAL_ONE)
    }
}
#[doc = "Field `AUXSEL` reader - Aux Select"]
pub type AUXSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUXSEL` writer - Aux Select"]
pub type AUXSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ASYNC_CH11_CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - Function Select"]
    #[inline(always)]
    pub fn fnsel(&self) -> FNSEL_R {
        FNSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Aux Select"]
    #[inline(always)]
    pub fn auxsel(&self) -> AUXSEL_R {
        AUXSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<0> {
        SIGSEL_W::new(self)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SOURCESEL_W<8> {
        SOURCESEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn fnsel(&mut self) -> FNSEL_W<16> {
        FNSEL_W::new(self)
    }
    #[doc = "Bits 24:27 - Aux Select"]
    #[inline(always)]
    #[must_use]
    pub fn auxsel(&mut self) -> AUXSEL_W<24> {
        AUXSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [async_ch11_ctrl](index.html) module"]
pub struct ASYNC_CH11_CTRL_SPEC;
impl crate::RegisterSpec for ASYNC_CH11_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [async_ch11_ctrl::R](R) reader structure"]
impl crate::Readable for ASYNC_CH11_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [async_ch11_ctrl::W](W) writer structure"]
impl crate::Writable for ASYNC_CH11_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNC_CH11_CTRL to value 0x000c_0000"]
impl crate::Resettable for ASYNC_CH11_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x000c_0000;
}
