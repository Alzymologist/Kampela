#[doc = "Register `CH5_CFG` reader"]
pub struct R(crate::R<CH5_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH5_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH5_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH5_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH5_CFG` writer"]
pub struct W(crate::W<CH5_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH5_CFG_SPEC>;
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
impl From<crate::W<CH5_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH5_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARBSLOTS` reader - Arbitration Slot Number Select"]
pub type ARBSLOTS_R = crate::FieldReader<u8, ARBSLOTS_A>;
#[doc = "Arbitration Slot Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBSLOTS_A {
    #[doc = "0: One arbitration slot selected"]
    ONE = 0,
    #[doc = "1: Two arbitration slots selected"]
    TWO = 1,
    #[doc = "2: Four arbitration slots selected"]
    FOUR = 2,
    #[doc = "3: Eight arbitration slots selected"]
    EIGHT = 3,
}
impl From<ARBSLOTS_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBSLOTS_A) -> Self {
        variant as _
    }
}
impl ARBSLOTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBSLOTS_A {
        match self.bits {
            0 => ARBSLOTS_A::ONE,
            1 => ARBSLOTS_A::TWO,
            2 => ARBSLOTS_A::FOUR,
            3 => ARBSLOTS_A::EIGHT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ARBSLOTS_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == ARBSLOTS_A::TWO
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == ARBSLOTS_A::FOUR
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == ARBSLOTS_A::EIGHT
    }
}
#[doc = "Field `ARBSLOTS` writer - Arbitration Slot Number Select"]
pub type ARBSLOTS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CH5_CFG_SPEC, u8, ARBSLOTS_A, 2, O>;
impl<'a, const O: u8> ARBSLOTS_W<'a, O> {
    #[doc = "One arbitration slot selected"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ARBSLOTS_A::ONE)
    }
    #[doc = "Two arbitration slots selected"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(ARBSLOTS_A::TWO)
    }
    #[doc = "Four arbitration slots selected"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(ARBSLOTS_A::FOUR)
    }
    #[doc = "Eight arbitration slots selected"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(ARBSLOTS_A::EIGHT)
    }
}
#[doc = "Field `SRCINCSIGN` reader - Source Address Increment Sign"]
pub type SRCINCSIGN_R = crate::BitReader<SRCINCSIGN_A>;
#[doc = "Source Address Increment Sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRCINCSIGN_A {
    #[doc = "0: Increment source address"]
    POSITIVE = 0,
    #[doc = "1: Decrement source address"]
    NEGATIVE = 1,
}
impl From<SRCINCSIGN_A> for bool {
    #[inline(always)]
    fn from(variant: SRCINCSIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRCINCSIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCINCSIGN_A {
        match self.bits {
            false => SRCINCSIGN_A::POSITIVE,
            true => SRCINCSIGN_A::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == SRCINCSIGN_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SRCINCSIGN_A::NEGATIVE
    }
}
#[doc = "Field `SRCINCSIGN` writer - Source Address Increment Sign"]
pub type SRCINCSIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH5_CFG_SPEC, SRCINCSIGN_A, O>;
impl<'a, const O: u8> SRCINCSIGN_W<'a, O> {
    #[doc = "Increment source address"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(SRCINCSIGN_A::POSITIVE)
    }
    #[doc = "Decrement source address"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(SRCINCSIGN_A::NEGATIVE)
    }
}
#[doc = "Field `DSTINCSIGN` reader - Destination Address Increment Sign"]
pub type DSTINCSIGN_R = crate::BitReader<DSTINCSIGN_A>;
#[doc = "Destination Address Increment Sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSTINCSIGN_A {
    #[doc = "0: Increment destination address"]
    POSITIVE = 0,
    #[doc = "1: Decrement destination address"]
    NEGATIVE = 1,
}
impl From<DSTINCSIGN_A> for bool {
    #[inline(always)]
    fn from(variant: DSTINCSIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl DSTINCSIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSTINCSIGN_A {
        match self.bits {
            false => DSTINCSIGN_A::POSITIVE,
            true => DSTINCSIGN_A::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DSTINCSIGN_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DSTINCSIGN_A::NEGATIVE
    }
}
#[doc = "Field `DSTINCSIGN` writer - Destination Address Increment Sign"]
pub type DSTINCSIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH5_CFG_SPEC, DSTINCSIGN_A, O>;
impl<'a, const O: u8> DSTINCSIGN_W<'a, O> {
    #[doc = "Increment destination address"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(DSTINCSIGN_A::POSITIVE)
    }
    #[doc = "Decrement destination address"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(DSTINCSIGN_A::NEGATIVE)
    }
}
impl R {
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline(always)]
    pub fn arbslots(&self) -> ARBSLOTS_R {
        ARBSLOTS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline(always)]
    pub fn srcincsign(&self) -> SRCINCSIGN_R {
        SRCINCSIGN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline(always)]
    pub fn dstincsign(&self) -> DSTINCSIGN_R {
        DSTINCSIGN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline(always)]
    #[must_use]
    pub fn arbslots(&mut self) -> ARBSLOTS_W<16> {
        ARBSLOTS_W::new(self)
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline(always)]
    #[must_use]
    pub fn srcincsign(&mut self) -> SRCINCSIGN_W<20> {
        SRCINCSIGN_W::new(self)
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline(always)]
    #[must_use]
    pub fn dstincsign(&mut self) -> DSTINCSIGN_W<21> {
        DSTINCSIGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_cfg](index.html) module"]
pub struct CH5_CFG_SPEC;
impl crate::RegisterSpec for CH5_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch5_cfg::R](R) reader structure"]
impl crate::Readable for CH5_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch5_cfg::W](W) writer structure"]
impl crate::Writable for CH5_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH5_CFG to value 0"]
impl crate::Resettable for CH5_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
