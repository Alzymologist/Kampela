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
#[doc = "Field `BIAS` reader - Bias Configuration"]
pub type BIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIAS` writer - Bias Configuration"]
pub type BIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `HYST` reader - Hysteresis mode"]
pub type HYST_R = crate::FieldReader<u8, HYST_A>;
#[doc = "Hysteresis mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYST_A {
    #[doc = "0: Hysteresis disabled"]
    DISABLED = 0,
    #[doc = "1: 10mV symmetrical hysteresis"]
    SYM10MV = 1,
    #[doc = "2: 20mV symmetrical hysteresis"]
    SYM20MV = 2,
    #[doc = "3: 30mV symmetrical hysteresis"]
    SYM30MV = 3,
    #[doc = "4: 10mV hysteresis on positive edge transitions"]
    POS10MV = 4,
    #[doc = "5: 20mV hysteresis on positive edge transitions"]
    POS20MV = 5,
    #[doc = "6: 30mV hysteresis on positive edge transitions"]
    POS30MV = 6,
    #[doc = "8: 10mV hysteresis on negative edge transitions"]
    NEG10MV = 8,
    #[doc = "9: 20mV hysteresis on negative edge transitions"]
    NEG20MV = 9,
    #[doc = "10: 30mV hysteresis on negative edge transitions"]
    NEG30MV = 10,
}
impl From<HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as _
    }
}
impl HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HYST_A> {
        match self.bits {
            0 => Some(HYST_A::DISABLED),
            1 => Some(HYST_A::SYM10MV),
            2 => Some(HYST_A::SYM20MV),
            3 => Some(HYST_A::SYM30MV),
            4 => Some(HYST_A::POS10MV),
            5 => Some(HYST_A::POS20MV),
            6 => Some(HYST_A::POS30MV),
            8 => Some(HYST_A::NEG10MV),
            9 => Some(HYST_A::NEG20MV),
            10 => Some(HYST_A::NEG30MV),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HYST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `SYM10MV`"]
    #[inline(always)]
    pub fn is_sym10mv(&self) -> bool {
        *self == HYST_A::SYM10MV
    }
    #[doc = "Checks if the value of the field is `SYM20MV`"]
    #[inline(always)]
    pub fn is_sym20mv(&self) -> bool {
        *self == HYST_A::SYM20MV
    }
    #[doc = "Checks if the value of the field is `SYM30MV`"]
    #[inline(always)]
    pub fn is_sym30mv(&self) -> bool {
        *self == HYST_A::SYM30MV
    }
    #[doc = "Checks if the value of the field is `POS10MV`"]
    #[inline(always)]
    pub fn is_pos10mv(&self) -> bool {
        *self == HYST_A::POS10MV
    }
    #[doc = "Checks if the value of the field is `POS20MV`"]
    #[inline(always)]
    pub fn is_pos20mv(&self) -> bool {
        *self == HYST_A::POS20MV
    }
    #[doc = "Checks if the value of the field is `POS30MV`"]
    #[inline(always)]
    pub fn is_pos30mv(&self) -> bool {
        *self == HYST_A::POS30MV
    }
    #[doc = "Checks if the value of the field is `NEG10MV`"]
    #[inline(always)]
    pub fn is_neg10mv(&self) -> bool {
        *self == HYST_A::NEG10MV
    }
    #[doc = "Checks if the value of the field is `NEG20MV`"]
    #[inline(always)]
    pub fn is_neg20mv(&self) -> bool {
        *self == HYST_A::NEG20MV
    }
    #[doc = "Checks if the value of the field is `NEG30MV`"]
    #[inline(always)]
    pub fn is_neg30mv(&self) -> bool {
        *self == HYST_A::NEG30MV
    }
}
#[doc = "Field `HYST` writer - Hysteresis mode"]
pub type HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, HYST_A, 4, O>;
impl<'a, const O: u8> HYST_W<'a, O> {
    #[doc = "Hysteresis disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HYST_A::DISABLED)
    }
    #[doc = "10mV symmetrical hysteresis"]
    #[inline(always)]
    pub fn sym10mv(self) -> &'a mut W {
        self.variant(HYST_A::SYM10MV)
    }
    #[doc = "20mV symmetrical hysteresis"]
    #[inline(always)]
    pub fn sym20mv(self) -> &'a mut W {
        self.variant(HYST_A::SYM20MV)
    }
    #[doc = "30mV symmetrical hysteresis"]
    #[inline(always)]
    pub fn sym30mv(self) -> &'a mut W {
        self.variant(HYST_A::SYM30MV)
    }
    #[doc = "10mV hysteresis on positive edge transitions"]
    #[inline(always)]
    pub fn pos10mv(self) -> &'a mut W {
        self.variant(HYST_A::POS10MV)
    }
    #[doc = "20mV hysteresis on positive edge transitions"]
    #[inline(always)]
    pub fn pos20mv(self) -> &'a mut W {
        self.variant(HYST_A::POS20MV)
    }
    #[doc = "30mV hysteresis on positive edge transitions"]
    #[inline(always)]
    pub fn pos30mv(self) -> &'a mut W {
        self.variant(HYST_A::POS30MV)
    }
    #[doc = "10mV hysteresis on negative edge transitions"]
    #[inline(always)]
    pub fn neg10mv(self) -> &'a mut W {
        self.variant(HYST_A::NEG10MV)
    }
    #[doc = "20mV hysteresis on negative edge transitions"]
    #[inline(always)]
    pub fn neg20mv(self) -> &'a mut W {
        self.variant(HYST_A::NEG20MV)
    }
    #[doc = "30mV hysteresis on negative edge transitions"]
    #[inline(always)]
    pub fn neg30mv(self) -> &'a mut W {
        self.variant(HYST_A::NEG30MV)
    }
}
#[doc = "Field `INPUTRANGE` reader - Input Range"]
pub type INPUTRANGE_R = crate::BitReader<INPUTRANGE_A>;
#[doc = "Input Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUTRANGE_A {
    #[doc = "0: Use this setting when the input to the comparator core can be from 0 to AVDD."]
    FULL = 0,
    #[doc = "1: It is recommended to use this setting when the input to the comparator core will always be less than AVDD-0.7V."]
    REDUCED = 1,
}
impl From<INPUTRANGE_A> for bool {
    #[inline(always)]
    fn from(variant: INPUTRANGE_A) -> Self {
        variant as u8 != 0
    }
}
impl INPUTRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUTRANGE_A {
        match self.bits {
            false => INPUTRANGE_A::FULL,
            true => INPUTRANGE_A::REDUCED,
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == INPUTRANGE_A::FULL
    }
    #[doc = "Checks if the value of the field is `REDUCED`"]
    #[inline(always)]
    pub fn is_reduced(&self) -> bool {
        *self == INPUTRANGE_A::REDUCED
    }
}
#[doc = "Field `INPUTRANGE` writer - Input Range"]
pub type INPUTRANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, INPUTRANGE_A, O>;
impl<'a, const O: u8> INPUTRANGE_W<'a, O> {
    #[doc = "Use this setting when the input to the comparator core can be from 0 to AVDD."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(INPUTRANGE_A::FULL)
    }
    #[doc = "It is recommended to use this setting when the input to the comparator core will always be less than AVDD-0.7V."]
    #[inline(always)]
    pub fn reduced(self) -> &'a mut W {
        self.variant(INPUTRANGE_A::REDUCED)
    }
}
#[doc = "Field `ACCURACY` reader - ACMP accuracy mode"]
pub type ACCURACY_R = crate::BitReader<ACCURACY_A>;
#[doc = "ACMP accuracy mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACCURACY_A {
    #[doc = "0: ACMP operates in low-accuracy mode but consumes less current."]
    LOW = 0,
    #[doc = "1: ACMP operates in high-accuracy mode but consumes more current."]
    HIGH = 1,
}
impl From<ACCURACY_A> for bool {
    #[inline(always)]
    fn from(variant: ACCURACY_A) -> Self {
        variant as u8 != 0
    }
}
impl ACCURACY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCURACY_A {
        match self.bits {
            false => ACCURACY_A::LOW,
            true => ACCURACY_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ACCURACY_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ACCURACY_A::HIGH
    }
}
#[doc = "Field `ACCURACY` writer - ACMP accuracy mode"]
pub type ACCURACY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, ACCURACY_A, O>;
impl<'a, const O: u8> ACCURACY_W<'a, O> {
    #[doc = "ACMP operates in low-accuracy mode but consumes less current."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ACCURACY_A::LOW)
    }
    #[doc = "ACMP operates in high-accuracy mode but consumes more current."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ACCURACY_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:2 - Bias Configuration"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - Hysteresis mode"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input Range"]
    #[inline(always)]
    pub fn inputrange(&self) -> INPUTRANGE_R {
        INPUTRANGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ACMP accuracy mode"]
    #[inline(always)]
    pub fn accuracy(&self) -> ACCURACY_R {
        ACCURACY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bias Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn bias(&mut self) -> BIAS_W<0> {
        BIAS_W::new(self)
    }
    #[doc = "Bits 8:11 - Hysteresis mode"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<8> {
        HYST_W::new(self)
    }
    #[doc = "Bit 16 - Input Range"]
    #[inline(always)]
    #[must_use]
    pub fn inputrange(&mut self) -> INPUTRANGE_W<16> {
        INPUTRANGE_W::new(self)
    }
    #[doc = "Bit 17 - ACMP accuracy mode"]
    #[inline(always)]
    #[must_use]
    pub fn accuracy(&mut self) -> ACCURACY_W<17> {
        ACCURACY_W::new(self)
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
#[doc = "`reset()` method sets CFG to value 0x04"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
