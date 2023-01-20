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
#[doc = "Field `CLRSRC` reader - WDOG Clear Source"]
pub type CLRSRC_R = crate::BitReader<CLRSRC_A>;
#[doc = "WDOG Clear Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRSRC_A {
    #[doc = "0: A write to the clear bit will clear the WDOG counter"]
    SW = 0,
    #[doc = "1: A rising edge on the PRS Source 0 will clear the WDOG counter"]
    PRSSRC0 = 1,
}
impl From<CLRSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CLRSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRSRC_A {
        match self.bits {
            false => CLRSRC_A::SW,
            true => CLRSRC_A::PRSSRC0,
        }
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == CLRSRC_A::SW
    }
    #[doc = "Checks if the value of the field is `PRSSRC0`"]
    #[inline(always)]
    pub fn is_prssrc0(&self) -> bool {
        *self == CLRSRC_A::PRSSRC0
    }
}
#[doc = "Field `CLRSRC` writer - WDOG Clear Source"]
pub type CLRSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CLRSRC_A, O>;
impl<'a, const O: u8> CLRSRC_W<'a, O> {
    #[doc = "A write to the clear bit will clear the WDOG counter"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut W {
        self.variant(CLRSRC_A::SW)
    }
    #[doc = "A rising edge on the PRS Source 0 will clear the WDOG counter"]
    #[inline(always)]
    pub fn prssrc0(self) -> &'a mut W {
        self.variant(CLRSRC_A::PRSSRC0)
    }
}
#[doc = "Field `EM1RUN` reader - EM1 Run"]
pub type EM1RUN_R = crate::BitReader<EM1RUN_A>;
#[doc = "EM1 Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM1RUN_A {
    #[doc = "0: WDOG timer is frozen in EM1."]
    DISABLE = 0,
    #[doc = "1: WDOG timer is running in EM1."]
    ENABLE = 1,
}
impl From<EM1RUN_A> for bool {
    #[inline(always)]
    fn from(variant: EM1RUN_A) -> Self {
        variant as u8 != 0
    }
}
impl EM1RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EM1RUN_A {
        match self.bits {
            false => EM1RUN_A::DISABLE,
            true => EM1RUN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EM1RUN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EM1RUN_A::ENABLE
    }
}
#[doc = "Field `EM1RUN` writer - EM1 Run"]
pub type EM1RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, EM1RUN_A, O>;
impl<'a, const O: u8> EM1RUN_W<'a, O> {
    #[doc = "WDOG timer is frozen in EM1."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EM1RUN_A::DISABLE)
    }
    #[doc = "WDOG timer is running in EM1."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EM1RUN_A::ENABLE)
    }
}
#[doc = "Field `EM2RUN` reader - EM2 Run"]
pub type EM2RUN_R = crate::BitReader<EM2RUN_A>;
#[doc = "EM2 Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM2RUN_A {
    #[doc = "0: WDOG timer is frozen in EM2."]
    DISABLE = 0,
    #[doc = "1: WDOG timer is running in EM2."]
    ENABLE = 1,
}
impl From<EM2RUN_A> for bool {
    #[inline(always)]
    fn from(variant: EM2RUN_A) -> Self {
        variant as u8 != 0
    }
}
impl EM2RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EM2RUN_A {
        match self.bits {
            false => EM2RUN_A::DISABLE,
            true => EM2RUN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EM2RUN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EM2RUN_A::ENABLE
    }
}
#[doc = "Field `EM2RUN` writer - EM2 Run"]
pub type EM2RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, EM2RUN_A, O>;
impl<'a, const O: u8> EM2RUN_W<'a, O> {
    #[doc = "WDOG timer is frozen in EM2."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EM2RUN_A::DISABLE)
    }
    #[doc = "WDOG timer is running in EM2."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EM2RUN_A::ENABLE)
    }
}
#[doc = "Field `EM3RUN` reader - EM3 Run"]
pub type EM3RUN_R = crate::BitReader<EM3RUN_A>;
#[doc = "EM3 Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM3RUN_A {
    #[doc = "0: WDOG timer is frozen in EM3."]
    DISABLE = 0,
    #[doc = "1: WDOG timer is running in EM3."]
    ENABLE = 1,
}
impl From<EM3RUN_A> for bool {
    #[inline(always)]
    fn from(variant: EM3RUN_A) -> Self {
        variant as u8 != 0
    }
}
impl EM3RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EM3RUN_A {
        match self.bits {
            false => EM3RUN_A::DISABLE,
            true => EM3RUN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EM3RUN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EM3RUN_A::ENABLE
    }
}
#[doc = "Field `EM3RUN` writer - EM3 Run"]
pub type EM3RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, EM3RUN_A, O>;
impl<'a, const O: u8> EM3RUN_W<'a, O> {
    #[doc = "WDOG timer is frozen in EM3."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EM3RUN_A::DISABLE)
    }
    #[doc = "WDOG timer is running in EM3."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EM3RUN_A::ENABLE)
    }
}
#[doc = "Field `EM4BLOCK` reader - EM4 Block"]
pub type EM4BLOCK_R = crate::BitReader<EM4BLOCK_A>;
#[doc = "EM4 Block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM4BLOCK_A {
    #[doc = "0: EM4 can be entered by software. See EMU for detailed description."]
    DISABLE = 0,
    #[doc = "1: EM4 cannot be entered by software."]
    ENABLE = 1,
}
impl From<EM4BLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: EM4BLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl EM4BLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EM4BLOCK_A {
        match self.bits {
            false => EM4BLOCK_A::DISABLE,
            true => EM4BLOCK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EM4BLOCK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EM4BLOCK_A::ENABLE
    }
}
#[doc = "Field `EM4BLOCK` writer - EM4 Block"]
pub type EM4BLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, EM4BLOCK_A, O>;
impl<'a, const O: u8> EM4BLOCK_W<'a, O> {
    #[doc = "EM4 can be entered by software. See EMU for detailed description."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EM4BLOCK_A::DISABLE)
    }
    #[doc = "EM4 cannot be entered by software."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EM4BLOCK_A::ENABLE)
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run"]
pub type DEBUGRUN_R = crate::BitReader<DEBUGRUN_A>;
#[doc = "Debug Mode Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBUGRUN_A {
    #[doc = "0: WDOG timer is frozen in debug mode"]
    DISABLE = 0,
    #[doc = "1: WDOG timer is running in debug mode"]
    ENABLE = 1,
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
            false => DEBUGRUN_A::DISABLE,
            true => DEBUGRUN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEBUGRUN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DEBUGRUN_A::ENABLE
    }
}
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run"]
pub type DEBUGRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, DEBUGRUN_A, O>;
impl<'a, const O: u8> DEBUGRUN_W<'a, O> {
    #[doc = "WDOG timer is frozen in debug mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DEBUGRUN_A::DISABLE)
    }
    #[doc = "WDOG timer is running in debug mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DEBUGRUN_A::ENABLE)
    }
}
#[doc = "Field `WDOGRSTDIS` reader - WDOG Reset Disable"]
pub type WDOGRSTDIS_R = crate::BitReader<WDOGRSTDIS_A>;
#[doc = "WDOG Reset Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDOGRSTDIS_A {
    #[doc = "0: A timeout will cause a WDOG reset"]
    EN = 0,
    #[doc = "1: A timeout will not cause a WDOG reset"]
    DIS = 1,
}
impl From<WDOGRSTDIS_A> for bool {
    #[inline(always)]
    fn from(variant: WDOGRSTDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl WDOGRSTDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOGRSTDIS_A {
        match self.bits {
            false => WDOGRSTDIS_A::EN,
            true => WDOGRSTDIS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WDOGRSTDIS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WDOGRSTDIS_A::DIS
    }
}
#[doc = "Field `WDOGRSTDIS` writer - WDOG Reset Disable"]
pub type WDOGRSTDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, WDOGRSTDIS_A, O>;
impl<'a, const O: u8> WDOGRSTDIS_W<'a, O> {
    #[doc = "A timeout will cause a WDOG reset"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(WDOGRSTDIS_A::EN)
    }
    #[doc = "A timeout will not cause a WDOG reset"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(WDOGRSTDIS_A::DIS)
    }
}
#[doc = "Field `PRS0MISSRSTEN` reader - PRS Src0 Missing Event WDOG Reset"]
pub type PRS0MISSRSTEN_R = crate::BitReader<bool>;
#[doc = "Field `PRS0MISSRSTEN` writer - PRS Src0 Missing Event WDOG Reset"]
pub type PRS0MISSRSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `PRS1MISSRSTEN` reader - PRS Src1 Missing Event WDOG Reset"]
pub type PRS1MISSRSTEN_R = crate::BitReader<bool>;
#[doc = "Field `PRS1MISSRSTEN` writer - PRS Src1 Missing Event WDOG Reset"]
pub type PRS1MISSRSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `PERSEL` reader - WDOG Timeout Period Select"]
pub type PERSEL_R = crate::FieldReader<u8, PERSEL_A>;
#[doc = "WDOG Timeout Period Select\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERSEL_A {
    #[doc = "0: Timeout period of 9 wdog cycles"]
    SEL0 = 0,
    #[doc = "1: Timeout period of 17 wdog cycles"]
    SEL1 = 1,
    #[doc = "2: Timeout period of 33 wdog cycles"]
    SEL2 = 2,
    #[doc = "3: Timeout period of 65 wdog cycles"]
    SEL3 = 3,
    #[doc = "4: Timeout period of 129 wdog cycles"]
    SEL4 = 4,
    #[doc = "5: Timeout period of 257 wdog cycles"]
    SEL5 = 5,
    #[doc = "6: Timeout period of 513 wdog cycles"]
    SEL6 = 6,
    #[doc = "7: Timeout period of 1k wdog cycles"]
    SEL7 = 7,
    #[doc = "8: Timeout period of 2k wdog cycles"]
    SEL8 = 8,
    #[doc = "9: Timeout period of 4k wdog cycles"]
    SEL9 = 9,
    #[doc = "10: Timeout period of 8k wdog cycles"]
    SEL10 = 10,
    #[doc = "11: Timeout period of 16k wdog cycles"]
    SEL11 = 11,
    #[doc = "12: Timeout period of 32k wdog cycles"]
    SEL12 = 12,
    #[doc = "13: Timeout period of 64k wdog cycles"]
    SEL13 = 13,
    #[doc = "14: Timeout period of 128k wdog cycles"]
    SEL14 = 14,
    #[doc = "15: Timeout period of 256k wdog cycles"]
    SEL15 = 15,
}
impl From<PERSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PERSEL_A) -> Self {
        variant as _
    }
}
impl PERSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERSEL_A {
        match self.bits {
            0 => PERSEL_A::SEL0,
            1 => PERSEL_A::SEL1,
            2 => PERSEL_A::SEL2,
            3 => PERSEL_A::SEL3,
            4 => PERSEL_A::SEL4,
            5 => PERSEL_A::SEL5,
            6 => PERSEL_A::SEL6,
            7 => PERSEL_A::SEL7,
            8 => PERSEL_A::SEL8,
            9 => PERSEL_A::SEL9,
            10 => PERSEL_A::SEL10,
            11 => PERSEL_A::SEL11,
            12 => PERSEL_A::SEL12,
            13 => PERSEL_A::SEL13,
            14 => PERSEL_A::SEL14,
            15 => PERSEL_A::SEL15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEL0`"]
    #[inline(always)]
    pub fn is_sel0(&self) -> bool {
        *self == PERSEL_A::SEL0
    }
    #[doc = "Checks if the value of the field is `SEL1`"]
    #[inline(always)]
    pub fn is_sel1(&self) -> bool {
        *self == PERSEL_A::SEL1
    }
    #[doc = "Checks if the value of the field is `SEL2`"]
    #[inline(always)]
    pub fn is_sel2(&self) -> bool {
        *self == PERSEL_A::SEL2
    }
    #[doc = "Checks if the value of the field is `SEL3`"]
    #[inline(always)]
    pub fn is_sel3(&self) -> bool {
        *self == PERSEL_A::SEL3
    }
    #[doc = "Checks if the value of the field is `SEL4`"]
    #[inline(always)]
    pub fn is_sel4(&self) -> bool {
        *self == PERSEL_A::SEL4
    }
    #[doc = "Checks if the value of the field is `SEL5`"]
    #[inline(always)]
    pub fn is_sel5(&self) -> bool {
        *self == PERSEL_A::SEL5
    }
    #[doc = "Checks if the value of the field is `SEL6`"]
    #[inline(always)]
    pub fn is_sel6(&self) -> bool {
        *self == PERSEL_A::SEL6
    }
    #[doc = "Checks if the value of the field is `SEL7`"]
    #[inline(always)]
    pub fn is_sel7(&self) -> bool {
        *self == PERSEL_A::SEL7
    }
    #[doc = "Checks if the value of the field is `SEL8`"]
    #[inline(always)]
    pub fn is_sel8(&self) -> bool {
        *self == PERSEL_A::SEL8
    }
    #[doc = "Checks if the value of the field is `SEL9`"]
    #[inline(always)]
    pub fn is_sel9(&self) -> bool {
        *self == PERSEL_A::SEL9
    }
    #[doc = "Checks if the value of the field is `SEL10`"]
    #[inline(always)]
    pub fn is_sel10(&self) -> bool {
        *self == PERSEL_A::SEL10
    }
    #[doc = "Checks if the value of the field is `SEL11`"]
    #[inline(always)]
    pub fn is_sel11(&self) -> bool {
        *self == PERSEL_A::SEL11
    }
    #[doc = "Checks if the value of the field is `SEL12`"]
    #[inline(always)]
    pub fn is_sel12(&self) -> bool {
        *self == PERSEL_A::SEL12
    }
    #[doc = "Checks if the value of the field is `SEL13`"]
    #[inline(always)]
    pub fn is_sel13(&self) -> bool {
        *self == PERSEL_A::SEL13
    }
    #[doc = "Checks if the value of the field is `SEL14`"]
    #[inline(always)]
    pub fn is_sel14(&self) -> bool {
        *self == PERSEL_A::SEL14
    }
    #[doc = "Checks if the value of the field is `SEL15`"]
    #[inline(always)]
    pub fn is_sel15(&self) -> bool {
        *self == PERSEL_A::SEL15
    }
}
#[doc = "Field `PERSEL` writer - WDOG Timeout Period Select"]
pub type PERSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, PERSEL_A, 4, O>;
impl<'a, const O: u8> PERSEL_W<'a, O> {
    #[doc = "Timeout period of 9 wdog cycles"]
    #[inline(always)]
    pub fn sel0(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL0)
    }
    #[doc = "Timeout period of 17 wdog cycles"]
    #[inline(always)]
    pub fn sel1(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL1)
    }
    #[doc = "Timeout period of 33 wdog cycles"]
    #[inline(always)]
    pub fn sel2(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL2)
    }
    #[doc = "Timeout period of 65 wdog cycles"]
    #[inline(always)]
    pub fn sel3(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL3)
    }
    #[doc = "Timeout period of 129 wdog cycles"]
    #[inline(always)]
    pub fn sel4(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL4)
    }
    #[doc = "Timeout period of 257 wdog cycles"]
    #[inline(always)]
    pub fn sel5(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL5)
    }
    #[doc = "Timeout period of 513 wdog cycles"]
    #[inline(always)]
    pub fn sel6(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL6)
    }
    #[doc = "Timeout period of 1k wdog cycles"]
    #[inline(always)]
    pub fn sel7(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL7)
    }
    #[doc = "Timeout period of 2k wdog cycles"]
    #[inline(always)]
    pub fn sel8(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL8)
    }
    #[doc = "Timeout period of 4k wdog cycles"]
    #[inline(always)]
    pub fn sel9(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL9)
    }
    #[doc = "Timeout period of 8k wdog cycles"]
    #[inline(always)]
    pub fn sel10(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL10)
    }
    #[doc = "Timeout period of 16k wdog cycles"]
    #[inline(always)]
    pub fn sel11(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL11)
    }
    #[doc = "Timeout period of 32k wdog cycles"]
    #[inline(always)]
    pub fn sel12(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL12)
    }
    #[doc = "Timeout period of 64k wdog cycles"]
    #[inline(always)]
    pub fn sel13(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL13)
    }
    #[doc = "Timeout period of 128k wdog cycles"]
    #[inline(always)]
    pub fn sel14(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL14)
    }
    #[doc = "Timeout period of 256k wdog cycles"]
    #[inline(always)]
    pub fn sel15(self) -> &'a mut W {
        self.variant(PERSEL_A::SEL15)
    }
}
#[doc = "Field `WARNSEL` reader - WDOG Warning Period Select"]
pub type WARNSEL_R = crate::FieldReader<u8, WARNSEL_A>;
#[doc = "WDOG Warning Period Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WARNSEL_A {
    #[doc = "0: Disable"]
    DIS = 0,
    #[doc = "1: Warning timeout is 25% of the Timeout."]
    SEL1 = 1,
    #[doc = "2: Warning timeout is 50% of the Timeout."]
    SEL2 = 2,
    #[doc = "3: Warning timeout is 75% of the Timeout."]
    SEL3 = 3,
}
impl From<WARNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WARNSEL_A) -> Self {
        variant as _
    }
}
impl WARNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WARNSEL_A {
        match self.bits {
            0 => WARNSEL_A::DIS,
            1 => WARNSEL_A::SEL1,
            2 => WARNSEL_A::SEL2,
            3 => WARNSEL_A::SEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WARNSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `SEL1`"]
    #[inline(always)]
    pub fn is_sel1(&self) -> bool {
        *self == WARNSEL_A::SEL1
    }
    #[doc = "Checks if the value of the field is `SEL2`"]
    #[inline(always)]
    pub fn is_sel2(&self) -> bool {
        *self == WARNSEL_A::SEL2
    }
    #[doc = "Checks if the value of the field is `SEL3`"]
    #[inline(always)]
    pub fn is_sel3(&self) -> bool {
        *self == WARNSEL_A::SEL3
    }
}
#[doc = "Field `WARNSEL` writer - WDOG Warning Period Select"]
pub type WARNSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, WARNSEL_A, 2, O>;
impl<'a, const O: u8> WARNSEL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(WARNSEL_A::DIS)
    }
    #[doc = "Warning timeout is 25% of the Timeout."]
    #[inline(always)]
    pub fn sel1(self) -> &'a mut W {
        self.variant(WARNSEL_A::SEL1)
    }
    #[doc = "Warning timeout is 50% of the Timeout."]
    #[inline(always)]
    pub fn sel2(self) -> &'a mut W {
        self.variant(WARNSEL_A::SEL2)
    }
    #[doc = "Warning timeout is 75% of the Timeout."]
    #[inline(always)]
    pub fn sel3(self) -> &'a mut W {
        self.variant(WARNSEL_A::SEL3)
    }
}
#[doc = "Field `WINSEL` reader - WDOG Illegal Window Select"]
pub type WINSEL_R = crate::FieldReader<u8, WINSEL_A>;
#[doc = "WDOG Illegal Window Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WINSEL_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Window timeout is 12.5% of the Timeout."]
    SEL1 = 1,
    #[doc = "2: Window timeout is 25% of the Timeout."]
    SEL2 = 2,
    #[doc = "3: Window timeout is 37.5% of the Timeout."]
    SEL3 = 3,
    #[doc = "4: Window timeout is 50% of the Timeout."]
    SEL4 = 4,
    #[doc = "5: Window timeout is 62.5% of the Timeout."]
    SEL5 = 5,
    #[doc = "6: Window timeout is 75.5% of the Timeout."]
    SEL6 = 6,
    #[doc = "7: Window timeout is 87.5% of the Timeout."]
    SEL7 = 7,
}
impl From<WINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WINSEL_A) -> Self {
        variant as _
    }
}
impl WINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WINSEL_A {
        match self.bits {
            0 => WINSEL_A::DIS,
            1 => WINSEL_A::SEL1,
            2 => WINSEL_A::SEL2,
            3 => WINSEL_A::SEL3,
            4 => WINSEL_A::SEL4,
            5 => WINSEL_A::SEL5,
            6 => WINSEL_A::SEL6,
            7 => WINSEL_A::SEL7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WINSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `SEL1`"]
    #[inline(always)]
    pub fn is_sel1(&self) -> bool {
        *self == WINSEL_A::SEL1
    }
    #[doc = "Checks if the value of the field is `SEL2`"]
    #[inline(always)]
    pub fn is_sel2(&self) -> bool {
        *self == WINSEL_A::SEL2
    }
    #[doc = "Checks if the value of the field is `SEL3`"]
    #[inline(always)]
    pub fn is_sel3(&self) -> bool {
        *self == WINSEL_A::SEL3
    }
    #[doc = "Checks if the value of the field is `SEL4`"]
    #[inline(always)]
    pub fn is_sel4(&self) -> bool {
        *self == WINSEL_A::SEL4
    }
    #[doc = "Checks if the value of the field is `SEL5`"]
    #[inline(always)]
    pub fn is_sel5(&self) -> bool {
        *self == WINSEL_A::SEL5
    }
    #[doc = "Checks if the value of the field is `SEL6`"]
    #[inline(always)]
    pub fn is_sel6(&self) -> bool {
        *self == WINSEL_A::SEL6
    }
    #[doc = "Checks if the value of the field is `SEL7`"]
    #[inline(always)]
    pub fn is_sel7(&self) -> bool {
        *self == WINSEL_A::SEL7
    }
}
#[doc = "Field `WINSEL` writer - WDOG Illegal Window Select"]
pub type WINSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, WINSEL_A, 3, O>;
impl<'a, const O: u8> WINSEL_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(WINSEL_A::DIS)
    }
    #[doc = "Window timeout is 12.5% of the Timeout."]
    #[inline(always)]
    pub fn sel1(self) -> &'a mut W {
        self.variant(WINSEL_A::SEL1)
    }
    #[doc = "Window timeout is 25% of the Timeout."]
    #[inline(always)]
    pub fn sel2(self) -> &'a mut W {
        self.variant(WINSEL_A::SEL2)
    }
    #[doc = "Window timeout is 37.5% of the Timeout."]
    #[inline(always)]
    pub fn sel3(self) -> &'a mut W {
        self.variant(WINSEL_A::SEL3)
    }
    #[doc = "Window timeout is 50% of the Timeout."]
    #[inline(always)]
    pub fn sel4(self) -> &'a mut W {
        self.variant(WINSEL_A::SEL4)
    }
    #[doc = "Window timeout is 62.5% of the Timeout."]
    #[inline(always)]
    pub fn sel5(self) -> &'a mut W {
        self.variant(WINSEL_A::SEL5)
    }
    #[doc = "Window timeout is 75.5% of the Timeout."]
    #[inline(always)]
    pub fn sel6(self) -> &'a mut W {
        self.variant(WINSEL_A::SEL6)
    }
    #[doc = "Window timeout is 87.5% of the Timeout."]
    #[inline(always)]
    pub fn sel7(self) -> &'a mut W {
        self.variant(WINSEL_A::SEL7)
    }
}
impl R {
    #[doc = "Bit 0 - WDOG Clear Source"]
    #[inline(always)]
    pub fn clrsrc(&self) -> CLRSRC_R {
        CLRSRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EM1 Run"]
    #[inline(always)]
    pub fn em1run(&self) -> EM1RUN_R {
        EM1RUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EM2 Run"]
    #[inline(always)]
    pub fn em2run(&self) -> EM2RUN_R {
        EM2RUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EM3 Run"]
    #[inline(always)]
    pub fn em3run(&self) -> EM3RUN_R {
        EM3RUN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EM4 Block"]
    #[inline(always)]
    pub fn em4block(&self) -> EM4BLOCK_R {
        EM4BLOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Debug Mode Run"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - WDOG Reset Disable"]
    #[inline(always)]
    pub fn wdogrstdis(&self) -> WDOGRSTDIS_R {
        WDOGRSTDIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PRS Src0 Missing Event WDOG Reset"]
    #[inline(always)]
    pub fn prs0missrsten(&self) -> PRS0MISSRSTEN_R {
        PRS0MISSRSTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PRS Src1 Missing Event WDOG Reset"]
    #[inline(always)]
    pub fn prs1missrsten(&self) -> PRS1MISSRSTEN_R {
        PRS1MISSRSTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:19 - WDOG Timeout Period Select"]
    #[inline(always)]
    pub fn persel(&self) -> PERSEL_R {
        PERSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - WDOG Warning Period Select"]
    #[inline(always)]
    pub fn warnsel(&self) -> WARNSEL_R {
        WARNSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:30 - WDOG Illegal Window Select"]
    #[inline(always)]
    pub fn winsel(&self) -> WINSEL_R {
        WINSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - WDOG Clear Source"]
    #[inline(always)]
    #[must_use]
    pub fn clrsrc(&mut self) -> CLRSRC_W<0> {
        CLRSRC_W::new(self)
    }
    #[doc = "Bit 1 - EM1 Run"]
    #[inline(always)]
    #[must_use]
    pub fn em1run(&mut self) -> EM1RUN_W<1> {
        EM1RUN_W::new(self)
    }
    #[doc = "Bit 2 - EM2 Run"]
    #[inline(always)]
    #[must_use]
    pub fn em2run(&mut self) -> EM2RUN_W<2> {
        EM2RUN_W::new(self)
    }
    #[doc = "Bit 3 - EM3 Run"]
    #[inline(always)]
    #[must_use]
    pub fn em3run(&mut self) -> EM3RUN_W<3> {
        EM3RUN_W::new(self)
    }
    #[doc = "Bit 4 - EM4 Block"]
    #[inline(always)]
    #[must_use]
    pub fn em4block(&mut self) -> EM4BLOCK_W<4> {
        EM4BLOCK_W::new(self)
    }
    #[doc = "Bit 5 - Debug Mode Run"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<5> {
        DEBUGRUN_W::new(self)
    }
    #[doc = "Bit 8 - WDOG Reset Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wdogrstdis(&mut self) -> WDOGRSTDIS_W<8> {
        WDOGRSTDIS_W::new(self)
    }
    #[doc = "Bit 9 - PRS Src0 Missing Event WDOG Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prs0missrsten(&mut self) -> PRS0MISSRSTEN_W<9> {
        PRS0MISSRSTEN_W::new(self)
    }
    #[doc = "Bit 10 - PRS Src1 Missing Event WDOG Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prs1missrsten(&mut self) -> PRS1MISSRSTEN_W<10> {
        PRS1MISSRSTEN_W::new(self)
    }
    #[doc = "Bits 16:19 - WDOG Timeout Period Select"]
    #[inline(always)]
    #[must_use]
    pub fn persel(&mut self) -> PERSEL_W<16> {
        PERSEL_W::new(self)
    }
    #[doc = "Bits 24:25 - WDOG Warning Period Select"]
    #[inline(always)]
    #[must_use]
    pub fn warnsel(&mut self) -> WARNSEL_W<24> {
        WARNSEL_W::new(self)
    }
    #[doc = "Bits 28:30 - WDOG Illegal Window Select"]
    #[inline(always)]
    #[must_use]
    pub fn winsel(&mut self) -> WINSEL_W<28> {
        WINSEL_W::new(self)
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
#[doc = "`reset()` method sets CFG to value 0x000f_0000"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_0000;
}
