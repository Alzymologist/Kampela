#[doc = "Register `TIMING` reader"]
pub struct R(crate::R<TIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMING` writer"]
pub struct W(crate::W<TIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMING_SPEC>;
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
impl From<crate::W<TIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDELAY` reader - TX frame start delay"]
pub type TXDELAY_R = crate::FieldReader<u8, TXDELAY_A>;
#[doc = "TX frame start delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXDELAY_A {
    #[doc = "0: Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    DISABLE = 0,
    #[doc = "1: Start of transmission is delayed for 1 baud-times"]
    ONE = 1,
    #[doc = "2: Start of transmission is delayed for 2 baud-times"]
    TWO = 2,
    #[doc = "3: Start of transmission is delayed for 3 baud-times"]
    THREE = 3,
    #[doc = "4: Start of transmission is delayed for 7 baud-times"]
    SEVEN = 4,
    #[doc = "5: Start of transmission is delayed for TCMPVAL0 baud-times"]
    TCMP0 = 5,
    #[doc = "6: Start of transmission is delayed for TCMPVAL1 baud-times"]
    TCMP1 = 6,
    #[doc = "7: Start of transmission is delayed for TCMPVAL2 baud-times"]
    TCMP2 = 7,
}
impl From<TXDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: TXDELAY_A) -> Self {
        variant as _
    }
}
impl TXDELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDELAY_A {
        match self.bits {
            0 => TXDELAY_A::DISABLE,
            1 => TXDELAY_A::ONE,
            2 => TXDELAY_A::TWO,
            3 => TXDELAY_A::THREE,
            4 => TXDELAY_A::SEVEN,
            5 => TXDELAY_A::TCMP0,
            6 => TXDELAY_A::TCMP1,
            7 => TXDELAY_A::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXDELAY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == TXDELAY_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == TXDELAY_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == TXDELAY_A::THREE
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == TXDELAY_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == TXDELAY_A::TCMP0
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == TXDELAY_A::TCMP1
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == TXDELAY_A::TCMP2
    }
}
#[doc = "Field `TXDELAY` writer - TX frame start delay"]
pub type TXDELAY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMING_SPEC, u8, TXDELAY_A, 3, O>;
impl<'a, const O: u8> TXDELAY_W<'a, O> {
    #[doc = "Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXDELAY_A::DISABLE)
    }
    #[doc = "Start of transmission is delayed for 1 baud-times"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(TXDELAY_A::ONE)
    }
    #[doc = "Start of transmission is delayed for 2 baud-times"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(TXDELAY_A::TWO)
    }
    #[doc = "Start of transmission is delayed for 3 baud-times"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(TXDELAY_A::THREE)
    }
    #[doc = "Start of transmission is delayed for 7 baud-times"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(TXDELAY_A::SEVEN)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(TXDELAY_A::TCMP0)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(TXDELAY_A::TCMP1)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(TXDELAY_A::TCMP2)
    }
}
#[doc = "Field `CSSETUP` reader - Chip Select Setup"]
pub type CSSETUP_R = crate::FieldReader<u8, CSSETUP_A>;
#[doc = "Chip Select Setup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSSETUP_A {
    #[doc = "0: CS is not asserted before start of transmission"]
    ZERO = 0,
    #[doc = "1: CS is asserted for 1 baud-times before start of transmission"]
    ONE = 1,
    #[doc = "2: CS is asserted for 2 baud-times before start of transmission"]
    TWO = 2,
    #[doc = "3: CS is asserted for 3 baud-times before start of transmission"]
    THREE = 3,
    #[doc = "4: CS is asserted for 7 baud-times before start of transmission"]
    SEVEN = 4,
    #[doc = "5: CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    TCMP0 = 5,
    #[doc = "6: CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    TCMP1 = 6,
    #[doc = "7: CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    TCMP2 = 7,
}
impl From<CSSETUP_A> for u8 {
    #[inline(always)]
    fn from(variant: CSSETUP_A) -> Self {
        variant as _
    }
}
impl CSSETUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSETUP_A {
        match self.bits {
            0 => CSSETUP_A::ZERO,
            1 => CSSETUP_A::ONE,
            2 => CSSETUP_A::TWO,
            3 => CSSETUP_A::THREE,
            4 => CSSETUP_A::SEVEN,
            5 => CSSETUP_A::TCMP0,
            6 => CSSETUP_A::TCMP1,
            7 => CSSETUP_A::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSSETUP_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSSETUP_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSSETUP_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CSSETUP_A::THREE
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == CSSETUP_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == CSSETUP_A::TCMP0
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == CSSETUP_A::TCMP1
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == CSSETUP_A::TCMP2
    }
}
#[doc = "Field `CSSETUP` writer - Chip Select Setup"]
pub type CSSETUP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMING_SPEC, u8, CSSETUP_A, 3, O>;
impl<'a, const O: u8> CSSETUP_W<'a, O> {
    #[doc = "CS is not asserted before start of transmission"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CSSETUP_A::ZERO)
    }
    #[doc = "CS is asserted for 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CSSETUP_A::ONE)
    }
    #[doc = "CS is asserted for 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(CSSETUP_A::TWO)
    }
    #[doc = "CS is asserted for 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(CSSETUP_A::THREE)
    }
    #[doc = "CS is asserted for 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(CSSETUP_A::SEVEN)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(CSSETUP_A::TCMP0)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(CSSETUP_A::TCMP1)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(CSSETUP_A::TCMP2)
    }
}
#[doc = "Field `ICS` reader - Inter-character spacing"]
pub type ICS_R = crate::FieldReader<u8, ICS_A>;
#[doc = "Inter-character spacing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICS_A {
    #[doc = "0: There is no space between charcters"]
    ZERO = 0,
    #[doc = "1: Create a space of 1 baud-times before start of transmission"]
    ONE = 1,
    #[doc = "2: Create a space of 2 baud-times before start of transmission"]
    TWO = 2,
    #[doc = "3: Create a space of 3 baud-times before start of transmission"]
    THREE = 3,
    #[doc = "4: Create a space of 7 baud-times before start of transmission"]
    SEVEN = 4,
    #[doc = "5: Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    TCMP0 = 5,
    #[doc = "6: Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    TCMP1 = 6,
    #[doc = "7: Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    TCMP2 = 7,
}
impl From<ICS_A> for u8 {
    #[inline(always)]
    fn from(variant: ICS_A) -> Self {
        variant as _
    }
}
impl ICS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICS_A {
        match self.bits {
            0 => ICS_A::ZERO,
            1 => ICS_A::ONE,
            2 => ICS_A::TWO,
            3 => ICS_A::THREE,
            4 => ICS_A::SEVEN,
            5 => ICS_A::TCMP0,
            6 => ICS_A::TCMP1,
            7 => ICS_A::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ICS_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ICS_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == ICS_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == ICS_A::THREE
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == ICS_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == ICS_A::TCMP0
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == ICS_A::TCMP1
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == ICS_A::TCMP2
    }
}
#[doc = "Field `ICS` writer - Inter-character spacing"]
pub type ICS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TIMING_SPEC, u8, ICS_A, 3, O>;
impl<'a, const O: u8> ICS_W<'a, O> {
    #[doc = "There is no space between charcters"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(ICS_A::ZERO)
    }
    #[doc = "Create a space of 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ICS_A::ONE)
    }
    #[doc = "Create a space of 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(ICS_A::TWO)
    }
    #[doc = "Create a space of 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(ICS_A::THREE)
    }
    #[doc = "Create a space of 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(ICS_A::SEVEN)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(ICS_A::TCMP0)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(ICS_A::TCMP1)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(ICS_A::TCMP2)
    }
}
#[doc = "Field `CSHOLD` reader - Chip Select Hold"]
pub type CSHOLD_R = crate::FieldReader<u8, CSHOLD_A>;
#[doc = "Chip Select Hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSHOLD_A {
    #[doc = "0: Disable CS being asserted after the end of transmission"]
    ZERO = 0,
    #[doc = "1: CS is asserted for 1 baud-times after the end of transmission"]
    ONE = 1,
    #[doc = "2: CS is asserted for 2 baud-times after the end of transmission"]
    TWO = 2,
    #[doc = "3: CS is asserted for 3 baud-times after the end of transmission"]
    THREE = 3,
    #[doc = "4: CS is asserted for 7 baud-times after the end of transmission"]
    SEVEN = 4,
    #[doc = "5: CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    TCMP0 = 5,
    #[doc = "6: CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    TCMP1 = 6,
    #[doc = "7: CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    TCMP2 = 7,
}
impl From<CSHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: CSHOLD_A) -> Self {
        variant as _
    }
}
impl CSHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSHOLD_A {
        match self.bits {
            0 => CSHOLD_A::ZERO,
            1 => CSHOLD_A::ONE,
            2 => CSHOLD_A::TWO,
            3 => CSHOLD_A::THREE,
            4 => CSHOLD_A::SEVEN,
            5 => CSHOLD_A::TCMP0,
            6 => CSHOLD_A::TCMP1,
            7 => CSHOLD_A::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSHOLD_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSHOLD_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSHOLD_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CSHOLD_A::THREE
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == CSHOLD_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == CSHOLD_A::TCMP0
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == CSHOLD_A::TCMP1
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == CSHOLD_A::TCMP2
    }
}
#[doc = "Field `CSHOLD` writer - Chip Select Hold"]
pub type CSHOLD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMING_SPEC, u8, CSHOLD_A, 3, O>;
impl<'a, const O: u8> CSHOLD_W<'a, O> {
    #[doc = "Disable CS being asserted after the end of transmission"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CSHOLD_A::ZERO)
    }
    #[doc = "CS is asserted for 1 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CSHOLD_A::ONE)
    }
    #[doc = "CS is asserted for 2 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(CSHOLD_A::TWO)
    }
    #[doc = "CS is asserted for 3 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(CSHOLD_A::THREE)
    }
    #[doc = "CS is asserted for 7 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(CSHOLD_A::SEVEN)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(CSHOLD_A::TCMP0)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(CSHOLD_A::TCMP1)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(CSHOLD_A::TCMP2)
    }
}
impl R {
    #[doc = "Bits 16:18 - TX frame start delay"]
    #[inline(always)]
    pub fn txdelay(&self) -> TXDELAY_R {
        TXDELAY_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Chip Select Setup"]
    #[inline(always)]
    pub fn cssetup(&self) -> CSSETUP_R {
        CSSETUP_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Inter-character spacing"]
    #[inline(always)]
    pub fn ics(&self) -> ICS_R {
        ICS_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Chip Select Hold"]
    #[inline(always)]
    pub fn cshold(&self) -> CSHOLD_R {
        CSHOLD_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - TX frame start delay"]
    #[inline(always)]
    #[must_use]
    pub fn txdelay(&mut self) -> TXDELAY_W<16> {
        TXDELAY_W::new(self)
    }
    #[doc = "Bits 20:22 - Chip Select Setup"]
    #[inline(always)]
    #[must_use]
    pub fn cssetup(&mut self) -> CSSETUP_W<20> {
        CSSETUP_W::new(self)
    }
    #[doc = "Bits 24:26 - Inter-character spacing"]
    #[inline(always)]
    #[must_use]
    pub fn ics(&mut self) -> ICS_W<24> {
        ICS_W::new(self)
    }
    #[doc = "Bits 28:30 - Chip Select Hold"]
    #[inline(always)]
    #[must_use]
    pub fn cshold(&mut self) -> CSHOLD_W<28> {
        CSHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timing](index.html) module"]
pub struct TIMING_SPEC;
impl crate::RegisterSpec for TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timing::R](R) reader structure"]
impl crate::Readable for TIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timing::W](W) writer structure"]
impl crate::Writable for TIMING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMING to value 0"]
impl crate::Resettable for TIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
