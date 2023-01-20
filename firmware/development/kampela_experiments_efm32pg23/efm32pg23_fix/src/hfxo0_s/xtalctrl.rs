#[doc = "Register `XTALCTRL` reader"]
pub struct R(crate::R<XTALCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTALCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTALCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTALCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTALCTRL` writer"]
pub struct W(crate::W<XTALCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTALCTRL_SPEC>;
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
impl From<crate::W<XTALCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTALCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COREBIASANA` reader - Core Bias Current"]
pub type COREBIASANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COREBIASANA` writer - Core Bias Current"]
pub type COREBIASANA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTALCTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTUNEXIANA` reader - Tuning Capacitance on XI"]
pub type CTUNEXIANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTUNEXIANA` writer - Tuning Capacitance on XI"]
pub type CTUNEXIANA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTALCTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTUNEXOANA` reader - Tuning Capacitance on XO"]
pub type CTUNEXOANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTUNEXOANA` writer - Tuning Capacitance on XO"]
pub type CTUNEXOANA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTALCTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTUNEFIXANA` reader - Fixed Tuning Capacitance"]
pub type CTUNEFIXANA_R = crate::FieldReader<u8, CTUNEFIXANA_A>;
#[doc = "Fixed Tuning Capacitance\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTUNEFIXANA_A {
    #[doc = "0: Remove fixed capacitance on XI and XO nodes"]
    NONE = 0,
    #[doc = "1: Adds fixed capacitance on XI node"]
    XI = 1,
    #[doc = "2: Adds fixed capacitance on XO node"]
    XO = 2,
    #[doc = "3: Adds fixed capacitance on both XI and XO nodes"]
    BOTH = 3,
}
impl From<CTUNEFIXANA_A> for u8 {
    #[inline(always)]
    fn from(variant: CTUNEFIXANA_A) -> Self {
        variant as _
    }
}
impl CTUNEFIXANA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTUNEFIXANA_A {
        match self.bits {
            0 => CTUNEFIXANA_A::NONE,
            1 => CTUNEFIXANA_A::XI,
            2 => CTUNEFIXANA_A::XO,
            3 => CTUNEFIXANA_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CTUNEFIXANA_A::NONE
    }
    #[doc = "Checks if the value of the field is `XI`"]
    #[inline(always)]
    pub fn is_xi(&self) -> bool {
        *self == CTUNEFIXANA_A::XI
    }
    #[doc = "Checks if the value of the field is `XO`"]
    #[inline(always)]
    pub fn is_xo(&self) -> bool {
        *self == CTUNEFIXANA_A::XO
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CTUNEFIXANA_A::BOTH
    }
}
#[doc = "Field `CTUNEFIXANA` writer - Fixed Tuning Capacitance"]
pub type CTUNEFIXANA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, XTALCTRL_SPEC, u8, CTUNEFIXANA_A, 2, O>;
impl<'a, const O: u8> CTUNEFIXANA_W<'a, O> {
    #[doc = "Remove fixed capacitance on XI and XO nodes"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CTUNEFIXANA_A::NONE)
    }
    #[doc = "Adds fixed capacitance on XI node"]
    #[inline(always)]
    pub fn xi(self) -> &'a mut W {
        self.variant(CTUNEFIXANA_A::XI)
    }
    #[doc = "Adds fixed capacitance on XO node"]
    #[inline(always)]
    pub fn xo(self) -> &'a mut W {
        self.variant(CTUNEFIXANA_A::XO)
    }
    #[doc = "Adds fixed capacitance on both XI and XO nodes"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(CTUNEFIXANA_A::BOTH)
    }
}
#[doc = "Field `COREDGENANA` reader - Core Degeneration"]
pub type COREDGENANA_R = crate::FieldReader<u8, COREDGENANA_A>;
#[doc = "Core Degeneration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COREDGENANA_A {
    #[doc = "0: Do not apply core degeneration resistence"]
    NONE = 0,
    #[doc = "1: Apply 33 ohm core degeneration resistence"]
    DGEN33 = 1,
    #[doc = "2: Apply 50 ohm core degeneration resistence"]
    DGEN50 = 2,
    #[doc = "3: Apply 100 ohm core degeneration resistence"]
    DGEN100 = 3,
}
impl From<COREDGENANA_A> for u8 {
    #[inline(always)]
    fn from(variant: COREDGENANA_A) -> Self {
        variant as _
    }
}
impl COREDGENANA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COREDGENANA_A {
        match self.bits {
            0 => COREDGENANA_A::NONE,
            1 => COREDGENANA_A::DGEN33,
            2 => COREDGENANA_A::DGEN50,
            3 => COREDGENANA_A::DGEN100,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == COREDGENANA_A::NONE
    }
    #[doc = "Checks if the value of the field is `DGEN33`"]
    #[inline(always)]
    pub fn is_dgen33(&self) -> bool {
        *self == COREDGENANA_A::DGEN33
    }
    #[doc = "Checks if the value of the field is `DGEN50`"]
    #[inline(always)]
    pub fn is_dgen50(&self) -> bool {
        *self == COREDGENANA_A::DGEN50
    }
    #[doc = "Checks if the value of the field is `DGEN100`"]
    #[inline(always)]
    pub fn is_dgen100(&self) -> bool {
        *self == COREDGENANA_A::DGEN100
    }
}
#[doc = "Field `COREDGENANA` writer - Core Degeneration"]
pub type COREDGENANA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, XTALCTRL_SPEC, u8, COREDGENANA_A, 2, O>;
impl<'a, const O: u8> COREDGENANA_W<'a, O> {
    #[doc = "Do not apply core degeneration resistence"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(COREDGENANA_A::NONE)
    }
    #[doc = "Apply 33 ohm core degeneration resistence"]
    #[inline(always)]
    pub fn dgen33(self) -> &'a mut W {
        self.variant(COREDGENANA_A::DGEN33)
    }
    #[doc = "Apply 50 ohm core degeneration resistence"]
    #[inline(always)]
    pub fn dgen50(self) -> &'a mut W {
        self.variant(COREDGENANA_A::DGEN50)
    }
    #[doc = "Apply 100 ohm core degeneration resistence"]
    #[inline(always)]
    pub fn dgen100(self) -> &'a mut W {
        self.variant(COREDGENANA_A::DGEN100)
    }
}
#[doc = "Field `SKIPCOREBIASOPT` reader - Skip Core Bias Optimization"]
pub type SKIPCOREBIASOPT_R = crate::BitReader<bool>;
#[doc = "Field `SKIPCOREBIASOPT` writer - Skip Core Bias Optimization"]
pub type SKIPCOREBIASOPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTALCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Core Bias Current"]
    #[inline(always)]
    pub fn corebiasana(&self) -> COREBIASANA_R {
        COREBIASANA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Tuning Capacitance on XI"]
    #[inline(always)]
    pub fn ctunexiana(&self) -> CTUNEXIANA_R {
        CTUNEXIANA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Tuning Capacitance on XO"]
    #[inline(always)]
    pub fn ctunexoana(&self) -> CTUNEXOANA_R {
        CTUNEXOANA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Fixed Tuning Capacitance"]
    #[inline(always)]
    pub fn ctunefixana(&self) -> CTUNEFIXANA_R {
        CTUNEFIXANA_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Core Degeneration"]
    #[inline(always)]
    pub fn coredgenana(&self) -> COREDGENANA_R {
        COREDGENANA_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 31 - Skip Core Bias Optimization"]
    #[inline(always)]
    pub fn skipcorebiasopt(&self) -> SKIPCOREBIASOPT_R {
        SKIPCOREBIASOPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Core Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn corebiasana(&mut self) -> COREBIASANA_W<0> {
        COREBIASANA_W::new(self)
    }
    #[doc = "Bits 8:15 - Tuning Capacitance on XI"]
    #[inline(always)]
    #[must_use]
    pub fn ctunexiana(&mut self) -> CTUNEXIANA_W<8> {
        CTUNEXIANA_W::new(self)
    }
    #[doc = "Bits 16:23 - Tuning Capacitance on XO"]
    #[inline(always)]
    #[must_use]
    pub fn ctunexoana(&mut self) -> CTUNEXOANA_W<16> {
        CTUNEXOANA_W::new(self)
    }
    #[doc = "Bits 24:25 - Fixed Tuning Capacitance"]
    #[inline(always)]
    #[must_use]
    pub fn ctunefixana(&mut self) -> CTUNEFIXANA_W<24> {
        CTUNEFIXANA_W::new(self)
    }
    #[doc = "Bits 26:27 - Core Degeneration"]
    #[inline(always)]
    #[must_use]
    pub fn coredgenana(&mut self) -> COREDGENANA_W<26> {
        COREDGENANA_W::new(self)
    }
    #[doc = "Bit 31 - Skip Core Bias Optimization"]
    #[inline(always)]
    #[must_use]
    pub fn skipcorebiasopt(&mut self) -> SKIPCOREBIASOPT_W<31> {
        SKIPCOREBIASOPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalctrl](index.html) module"]
pub struct XTALCTRL_SPEC;
impl crate::RegisterSpec for XTALCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtalctrl::R](R) reader structure"]
impl crate::Readable for XTALCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtalctrl::W](W) writer structure"]
impl crate::Writable for XTALCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTALCTRL to value 0x033c_3c3c"]
impl crate::Resettable for XTALCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x033c_3c3c;
}
