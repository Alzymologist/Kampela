#[doc = "Register `MODULEINFO` reader"]
pub struct R(crate::R<MODULEINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODULEINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODULEINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODULEINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HWREV` reader - No Description"]
pub type HWREV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ANTENNA` reader - No Description"]
pub type ANTENNA_R = crate::FieldReader<u8, ANTENNA_A>;
#[doc = "No Description\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANTENNA_A {
    #[doc = "0: BUILTIN"]
    BUILTIN = 0,
    #[doc = "1: CONNECTOR"]
    CONNECTOR = 1,
    #[doc = "2: RFPAD"]
    RFPAD = 2,
    #[doc = "3: INVERTEDF"]
    INVERTEDF = 3,
}
impl From<ANTENNA_A> for u8 {
    #[inline(always)]
    fn from(variant: ANTENNA_A) -> Self {
        variant as _
    }
}
impl ANTENNA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ANTENNA_A> {
        match self.bits {
            0 => Some(ANTENNA_A::BUILTIN),
            1 => Some(ANTENNA_A::CONNECTOR),
            2 => Some(ANTENNA_A::RFPAD),
            3 => Some(ANTENNA_A::INVERTEDF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BUILTIN`"]
    #[inline(always)]
    pub fn is_builtin(&self) -> bool {
        *self == ANTENNA_A::BUILTIN
    }
    #[doc = "Checks if the value of the field is `CONNECTOR`"]
    #[inline(always)]
    pub fn is_connector(&self) -> bool {
        *self == ANTENNA_A::CONNECTOR
    }
    #[doc = "Checks if the value of the field is `RFPAD`"]
    #[inline(always)]
    pub fn is_rfpad(&self) -> bool {
        *self == ANTENNA_A::RFPAD
    }
    #[doc = "Checks if the value of the field is `INVERTEDF`"]
    #[inline(always)]
    pub fn is_invertedf(&self) -> bool {
        *self == ANTENNA_A::INVERTEDF
    }
}
#[doc = "Field `MODNUMBER` reader - No Description"]
pub type MODNUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TYPE` reader - No Description"]
pub type TYPE_R = crate::BitReader<TYPE_A>;
#[doc = "No Description\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TYPE_A {
    #[doc = "0: PCB"]
    PCB = 0,
    #[doc = "1: SIP"]
    SIP = 1,
}
impl From<TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE_A {
        match self.bits {
            false => TYPE_A::PCB,
            true => TYPE_A::SIP,
        }
    }
    #[doc = "Checks if the value of the field is `PCB`"]
    #[inline(always)]
    pub fn is_pcb(&self) -> bool {
        *self == TYPE_A::PCB
    }
    #[doc = "Checks if the value of the field is `SIP`"]
    #[inline(always)]
    pub fn is_sip(&self) -> bool {
        *self == TYPE_A::SIP
    }
}
#[doc = "Field `LFXO` reader - No Description"]
pub type LFXO_R = crate::BitReader<LFXO_A>;
#[doc = "No Description\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFXO_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: PRESENT"]
    PRESENT = 1,
}
impl From<LFXO_A> for bool {
    #[inline(always)]
    fn from(variant: LFXO_A) -> Self {
        variant as u8 != 0
    }
}
impl LFXO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXO_A {
        match self.bits {
            false => LFXO_A::NONE,
            true => LFXO_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LFXO_A::NONE
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == LFXO_A::PRESENT
    }
}
#[doc = "Field `EXPRESS` reader - No Description"]
pub type EXPRESS_R = crate::BitReader<EXPRESS_A>;
#[doc = "No Description\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXPRESS_A {
    #[doc = "0: SUPPORTED"]
    SUPPORTED = 0,
    #[doc = "1: NONE"]
    NONE = 1,
}
impl From<EXPRESS_A> for bool {
    #[inline(always)]
    fn from(variant: EXPRESS_A) -> Self {
        variant as u8 != 0
    }
}
impl EXPRESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXPRESS_A {
        match self.bits {
            false => EXPRESS_A::SUPPORTED,
            true => EXPRESS_A::NONE,
        }
    }
    #[doc = "Checks if the value of the field is `SUPPORTED`"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == EXPRESS_A::SUPPORTED
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EXPRESS_A::NONE
    }
}
#[doc = "Field `LFXOCALVAL` reader - No Description"]
pub type LFXOCALVAL_R = crate::BitReader<LFXOCALVAL_A>;
#[doc = "No Description\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFXOCALVAL_A {
    #[doc = "0: VALID"]
    VALID = 0,
    #[doc = "1: NOTVALID"]
    NOTVALID = 1,
}
impl From<LFXOCALVAL_A> for bool {
    #[inline(always)]
    fn from(variant: LFXOCALVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl LFXOCALVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXOCALVAL_A {
        match self.bits {
            false => LFXOCALVAL_A::VALID,
            true => LFXOCALVAL_A::NOTVALID,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == LFXOCALVAL_A::VALID
    }
    #[doc = "Checks if the value of the field is `NOTVALID`"]
    #[inline(always)]
    pub fn is_notvalid(&self) -> bool {
        *self == LFXOCALVAL_A::NOTVALID
    }
}
#[doc = "Field `HFXOCALVAL` reader - No Description"]
pub type HFXOCALVAL_R = crate::BitReader<HFXOCALVAL_A>;
#[doc = "No Description\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFXOCALVAL_A {
    #[doc = "0: VALID"]
    VALID = 0,
    #[doc = "1: NOTVALID"]
    NOTVALID = 1,
}
impl From<HFXOCALVAL_A> for bool {
    #[inline(always)]
    fn from(variant: HFXOCALVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl HFXOCALVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXOCALVAL_A {
        match self.bits {
            false => HFXOCALVAL_A::VALID,
            true => HFXOCALVAL_A::NOTVALID,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == HFXOCALVAL_A::VALID
    }
    #[doc = "Checks if the value of the field is `NOTVALID`"]
    #[inline(always)]
    pub fn is_notvalid(&self) -> bool {
        *self == HFXOCALVAL_A::NOTVALID
    }
}
#[doc = "Field `MODNUMBERMSB` reader - No Description"]
pub type MODNUMBERMSB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PADCDC` reader - No Description"]
pub type PADCDC_R = crate::BitReader<PADCDC_A>;
#[doc = "No Description\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PADCDC_A {
    #[doc = "0: VDCDC"]
    VDCDC = 0,
    #[doc = "1: OTHER"]
    OTHER = 1,
}
impl From<PADCDC_A> for bool {
    #[inline(always)]
    fn from(variant: PADCDC_A) -> Self {
        variant as u8 != 0
    }
}
impl PADCDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PADCDC_A {
        match self.bits {
            false => PADCDC_A::VDCDC,
            true => PADCDC_A::OTHER,
        }
    }
    #[doc = "Checks if the value of the field is `VDCDC`"]
    #[inline(always)]
    pub fn is_vdcdc(&self) -> bool {
        *self == PADCDC_A::VDCDC
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == PADCDC_A::OTHER
    }
}
#[doc = "Field `PHYLIMITED` reader - No Description"]
pub type PHYLIMITED_R = crate::BitReader<PHYLIMITED_A>;
#[doc = "No Description\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHYLIMITED_A {
    #[doc = "0: LIMITED"]
    LIMITED = 0,
    #[doc = "1: UNLIMITED"]
    UNLIMITED = 1,
}
impl From<PHYLIMITED_A> for bool {
    #[inline(always)]
    fn from(variant: PHYLIMITED_A) -> Self {
        variant as u8 != 0
    }
}
impl PHYLIMITED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYLIMITED_A {
        match self.bits {
            false => PHYLIMITED_A::LIMITED,
            true => PHYLIMITED_A::UNLIMITED,
        }
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == PHYLIMITED_A::LIMITED
    }
    #[doc = "Checks if the value of the field is `UNLIMITED`"]
    #[inline(always)]
    pub fn is_unlimited(&self) -> bool {
        *self == PHYLIMITED_A::UNLIMITED
    }
}
#[doc = "Field `EXTVALID` reader - No Description"]
pub type EXTVALID_R = crate::BitReader<EXTVALID_A>;
#[doc = "No Description\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTVALID_A {
    #[doc = "0: EXTUSED"]
    EXTUSED = 0,
    #[doc = "1: EXTUNUSED"]
    EXTUNUSED = 1,
}
impl From<EXTVALID_A> for bool {
    #[inline(always)]
    fn from(variant: EXTVALID_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTVALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTVALID_A {
        match self.bits {
            false => EXTVALID_A::EXTUSED,
            true => EXTVALID_A::EXTUNUSED,
        }
    }
    #[doc = "Checks if the value of the field is `EXTUSED`"]
    #[inline(always)]
    pub fn is_extused(&self) -> bool {
        *self == EXTVALID_A::EXTUSED
    }
    #[doc = "Checks if the value of the field is `EXTUNUSED`"]
    #[inline(always)]
    pub fn is_extunused(&self) -> bool {
        *self == EXTVALID_A::EXTUNUSED
    }
}
impl R {
    #[doc = "Bits 0:4 - No Description"]
    #[inline(always)]
    pub fn hwrev(&self) -> HWREV_R {
        HWREV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - No Description"]
    #[inline(always)]
    pub fn antenna(&self) -> ANTENNA_R {
        ANTENNA_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:14 - No Description"]
    #[inline(always)]
    pub fn modnumber(&self) -> MODNUMBER_R {
        MODNUMBER_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - No Description"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - No Description"]
    #[inline(always)]
    pub fn lfxo(&self) -> LFXO_R {
        LFXO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - No Description"]
    #[inline(always)]
    pub fn express(&self) -> EXPRESS_R {
        EXPRESS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - No Description"]
    #[inline(always)]
    pub fn lfxocalval(&self) -> LFXOCALVAL_R {
        LFXOCALVAL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - No Description"]
    #[inline(always)]
    pub fn hfxocalval(&self) -> HFXOCALVAL_R {
        HFXOCALVAL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:28 - No Description"]
    #[inline(always)]
    pub fn modnumbermsb(&self) -> MODNUMBERMSB_R {
        MODNUMBERMSB_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - No Description"]
    #[inline(always)]
    pub fn padcdc(&self) -> PADCDC_R {
        PADCDC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - No Description"]
    #[inline(always)]
    pub fn phylimited(&self) -> PHYLIMITED_R {
        PHYLIMITED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - No Description"]
    #[inline(always)]
    pub fn extvalid(&self) -> EXTVALID_R {
        EXTVALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Module Information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moduleinfo](index.html) module"]
pub struct MODULEINFO_SPEC;
impl crate::RegisterSpec for MODULEINFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [moduleinfo::R](R) reader structure"]
impl crate::Readable for MODULEINFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODULEINFO to value 0xffff_ffff"]
impl crate::Resettable for MODULEINFO_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
