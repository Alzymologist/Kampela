#[doc = "Register `OUTCTRL` reader"]
pub struct R(crate::R<OUTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTCTRL` writer"]
pub struct W(crate::W<OUTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTCTRL_SPEC>;
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
impl From<crate::W<OUTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAINOUTENCH0` reader - CH0 Main Output Enable"]
pub type MAINOUTENCH0_R = crate::BitReader<bool>;
#[doc = "Field `MAINOUTENCH0` writer - CH0 Main Output Enable"]
pub type MAINOUTENCH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTCTRL_SPEC, bool, O>;
#[doc = "Field `MAINOUTENCH1` reader - CH1 Main Output Enable"]
pub type MAINOUTENCH1_R = crate::BitReader<bool>;
#[doc = "Field `MAINOUTENCH1` writer - CH1 Main Output Enable"]
pub type MAINOUTENCH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTCTRL_SPEC, bool, O>;
#[doc = "Field `AUXOUTENCH0` reader - CH0 Alternative Output Enable"]
pub type AUXOUTENCH0_R = crate::BitReader<bool>;
#[doc = "Field `AUXOUTENCH0` writer - CH0 Alternative Output Enable"]
pub type AUXOUTENCH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTCTRL_SPEC, bool, O>;
#[doc = "Field `AUXOUTENCH1` reader - CH1 Alternative Output Enable"]
pub type AUXOUTENCH1_R = crate::BitReader<bool>;
#[doc = "Field `AUXOUTENCH1` writer - CH1 Alternative Output Enable"]
pub type AUXOUTENCH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTCTRL_SPEC, bool, O>;
#[doc = "Field `SHORTCH0` reader - CH1 Main and Alternative Output Short"]
pub type SHORTCH0_R = crate::BitReader<bool>;
#[doc = "Field `SHORTCH0` writer - CH1 Main and Alternative Output Short"]
pub type SHORTCH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTCTRL_SPEC, bool, O>;
#[doc = "Field `SHORTCH1` reader - CH0 Main and Alternative Output Short"]
pub type SHORTCH1_R = crate::BitReader<bool>;
#[doc = "Field `SHORTCH1` writer - CH0 Main and Alternative Output Short"]
pub type SHORTCH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTCTRL_SPEC, bool, O>;
#[doc = "Field `ABUSPORTSELCH0` reader - CH0 ABUS Port Select"]
pub type ABUSPORTSELCH0_R = crate::FieldReader<u8, ABUSPORTSELCH0_A>;
#[doc = "CH0 ABUS Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABUSPORTSELCH0_A {
    #[doc = "0: No GPIO Selected for CH0 ABUS Output"]
    NONE = 0,
    #[doc = "1: Port A Selected"]
    PORTA = 1,
    #[doc = "2: Port B Selected"]
    PORTB = 2,
    #[doc = "3: Port C Selected"]
    PORTC = 3,
    #[doc = "4: Port D Selected"]
    PORTD = 4,
}
impl From<ABUSPORTSELCH0_A> for u8 {
    #[inline(always)]
    fn from(variant: ABUSPORTSELCH0_A) -> Self {
        variant as _
    }
}
impl ABUSPORTSELCH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ABUSPORTSELCH0_A> {
        match self.bits {
            0 => Some(ABUSPORTSELCH0_A::NONE),
            1 => Some(ABUSPORTSELCH0_A::PORTA),
            2 => Some(ABUSPORTSELCH0_A::PORTB),
            3 => Some(ABUSPORTSELCH0_A::PORTC),
            4 => Some(ABUSPORTSELCH0_A::PORTD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ABUSPORTSELCH0_A::NONE
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == ABUSPORTSELCH0_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == ABUSPORTSELCH0_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == ABUSPORTSELCH0_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == ABUSPORTSELCH0_A::PORTD
    }
}
#[doc = "Field `ABUSPORTSELCH0` writer - CH0 ABUS Port Select"]
pub type ABUSPORTSELCH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTCTRL_SPEC, u8, ABUSPORTSELCH0_A, 3, O>;
impl<'a, const O: u8> ABUSPORTSELCH0_W<'a, O> {
    #[doc = "No GPIO Selected for CH0 ABUS Output"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ABUSPORTSELCH0_A::NONE)
    }
    #[doc = "Port A Selected"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(ABUSPORTSELCH0_A::PORTA)
    }
    #[doc = "Port B Selected"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(ABUSPORTSELCH0_A::PORTB)
    }
    #[doc = "Port C Selected"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(ABUSPORTSELCH0_A::PORTC)
    }
    #[doc = "Port D Selected"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(ABUSPORTSELCH0_A::PORTD)
    }
}
#[doc = "Field `ABUSPINSELCH0` reader - CH0 ABUS Pin Select"]
pub type ABUSPINSELCH0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABUSPINSELCH0` writer - CH0 ABUS Pin Select"]
pub type ABUSPINSELCH0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTCTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `ABUSPORTSELCH1` reader - CH1 ABUS Port Select"]
pub type ABUSPORTSELCH1_R = crate::FieldReader<u8, ABUSPORTSELCH1_A>;
#[doc = "CH1 ABUS Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABUSPORTSELCH1_A {
    #[doc = "0: No GPIO Selected for CH1 ABUS Output"]
    NONE = 0,
    #[doc = "1: Port A Selected"]
    PORTA = 1,
    #[doc = "2: Port B Selected"]
    PORTB = 2,
    #[doc = "3: Port C Selected"]
    PORTC = 3,
    #[doc = "4: Port D Selected"]
    PORTD = 4,
}
impl From<ABUSPORTSELCH1_A> for u8 {
    #[inline(always)]
    fn from(variant: ABUSPORTSELCH1_A) -> Self {
        variant as _
    }
}
impl ABUSPORTSELCH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ABUSPORTSELCH1_A> {
        match self.bits {
            0 => Some(ABUSPORTSELCH1_A::NONE),
            1 => Some(ABUSPORTSELCH1_A::PORTA),
            2 => Some(ABUSPORTSELCH1_A::PORTB),
            3 => Some(ABUSPORTSELCH1_A::PORTC),
            4 => Some(ABUSPORTSELCH1_A::PORTD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ABUSPORTSELCH1_A::NONE
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == ABUSPORTSELCH1_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == ABUSPORTSELCH1_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == ABUSPORTSELCH1_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == ABUSPORTSELCH1_A::PORTD
    }
}
#[doc = "Field `ABUSPORTSELCH1` writer - CH1 ABUS Port Select"]
pub type ABUSPORTSELCH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTCTRL_SPEC, u8, ABUSPORTSELCH1_A, 3, O>;
impl<'a, const O: u8> ABUSPORTSELCH1_W<'a, O> {
    #[doc = "No GPIO Selected for CH1 ABUS Output"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ABUSPORTSELCH1_A::NONE)
    }
    #[doc = "Port A Selected"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(ABUSPORTSELCH1_A::PORTA)
    }
    #[doc = "Port B Selected"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(ABUSPORTSELCH1_A::PORTB)
    }
    #[doc = "Port C Selected"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(ABUSPORTSELCH1_A::PORTC)
    }
    #[doc = "Port D Selected"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(ABUSPORTSELCH1_A::PORTD)
    }
}
#[doc = "Field `ABUSPINSELCH1` reader - CH1 ABUS Pin Select"]
pub type ABUSPINSELCH1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABUSPINSELCH1` writer - CH1 ABUS Pin Select"]
pub type ABUSPINSELCH1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTCTRL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - CH0 Main Output Enable"]
    #[inline(always)]
    pub fn mainoutench0(&self) -> MAINOUTENCH0_R {
        MAINOUTENCH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Main Output Enable"]
    #[inline(always)]
    pub fn mainoutench1(&self) -> MAINOUTENCH1_R {
        MAINOUTENCH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CH0 Alternative Output Enable"]
    #[inline(always)]
    pub fn auxoutench0(&self) -> AUXOUTENCH0_R {
        AUXOUTENCH0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH1 Alternative Output Enable"]
    #[inline(always)]
    pub fn auxoutench1(&self) -> AUXOUTENCH1_R {
        AUXOUTENCH1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH1 Main and Alternative Output Short"]
    #[inline(always)]
    pub fn shortch0(&self) -> SHORTCH0_R {
        SHORTCH0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH0 Main and Alternative Output Short"]
    #[inline(always)]
    pub fn shortch1(&self) -> SHORTCH1_R {
        SHORTCH1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:14 - CH0 ABUS Port Select"]
    #[inline(always)]
    pub fn abusportselch0(&self) -> ABUSPORTSELCH0_R {
        ABUSPORTSELCH0_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:20 - CH0 ABUS Pin Select"]
    #[inline(always)]
    pub fn abuspinselch0(&self) -> ABUSPINSELCH0_R {
        ABUSPINSELCH0_R::new(((self.bits >> 15) & 0x3f) as u8)
    }
    #[doc = "Bits 22:24 - CH1 ABUS Port Select"]
    #[inline(always)]
    pub fn abusportselch1(&self) -> ABUSPORTSELCH1_R {
        ABUSPORTSELCH1_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:30 - CH1 ABUS Pin Select"]
    #[inline(always)]
    pub fn abuspinselch1(&self) -> ABUSPINSELCH1_R {
        ABUSPINSELCH1_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Main Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mainoutench0(&mut self) -> MAINOUTENCH0_W<0> {
        MAINOUTENCH0_W::new(self)
    }
    #[doc = "Bit 1 - CH1 Main Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mainoutench1(&mut self) -> MAINOUTENCH1_W<1> {
        MAINOUTENCH1_W::new(self)
    }
    #[doc = "Bit 4 - CH0 Alternative Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auxoutench0(&mut self) -> AUXOUTENCH0_W<4> {
        AUXOUTENCH0_W::new(self)
    }
    #[doc = "Bit 5 - CH1 Alternative Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auxoutench1(&mut self) -> AUXOUTENCH1_W<5> {
        AUXOUTENCH1_W::new(self)
    }
    #[doc = "Bit 8 - CH1 Main and Alternative Output Short"]
    #[inline(always)]
    #[must_use]
    pub fn shortch0(&mut self) -> SHORTCH0_W<8> {
        SHORTCH0_W::new(self)
    }
    #[doc = "Bit 9 - CH0 Main and Alternative Output Short"]
    #[inline(always)]
    #[must_use]
    pub fn shortch1(&mut self) -> SHORTCH1_W<9> {
        SHORTCH1_W::new(self)
    }
    #[doc = "Bits 12:14 - CH0 ABUS Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn abusportselch0(&mut self) -> ABUSPORTSELCH0_W<12> {
        ABUSPORTSELCH0_W::new(self)
    }
    #[doc = "Bits 15:20 - CH0 ABUS Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn abuspinselch0(&mut self) -> ABUSPINSELCH0_W<15> {
        ABUSPINSELCH0_W::new(self)
    }
    #[doc = "Bits 22:24 - CH1 ABUS Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn abusportselch1(&mut self) -> ABUSPORTSELCH1_W<22> {
        ABUSPORTSELCH1_W::new(self)
    }
    #[doc = "Bits 25:30 - CH1 ABUS Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn abuspinselch1(&mut self) -> ABUSPINSELCH1_W<25> {
        ABUSPINSELCH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outctrl](index.html) module"]
pub struct OUTCTRL_SPEC;
impl crate::RegisterSpec for OUTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outctrl::R](R) reader structure"]
impl crate::Readable for OUTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outctrl::W](W) writer structure"]
impl crate::Writable for OUTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTCTRL to value 0"]
impl crate::Resettable for OUTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
