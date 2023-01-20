#[doc = "Register `PKGINFO` reader"]
pub struct R(crate::R<PKGINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKGINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKGINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKGINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEMPGRADE` reader - Temperature Grade"]
pub type TEMPGRADE_R = crate::FieldReader<u8, TEMPGRADE_A>;
#[doc = "Temperature Grade\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TEMPGRADE_A {
    #[doc = "0: -40 to 85 degC"]
    N40TO85 = 0,
    #[doc = "1: -40 to 125 degC"]
    N40TO125 = 1,
    #[doc = "2: -40 to 105 degC"]
    N40TO105 = 2,
    #[doc = "3: 0 to 70 degC"]
    N0TO70 = 3,
}
impl From<TEMPGRADE_A> for u8 {
    #[inline(always)]
    fn from(variant: TEMPGRADE_A) -> Self {
        variant as _
    }
}
impl TEMPGRADE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TEMPGRADE_A> {
        match self.bits {
            0 => Some(TEMPGRADE_A::N40TO85),
            1 => Some(TEMPGRADE_A::N40TO125),
            2 => Some(TEMPGRADE_A::N40TO105),
            3 => Some(TEMPGRADE_A::N0TO70),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `N40TO85`"]
    #[inline(always)]
    pub fn is_n40to85(&self) -> bool {
        *self == TEMPGRADE_A::N40TO85
    }
    #[doc = "Checks if the value of the field is `N40TO125`"]
    #[inline(always)]
    pub fn is_n40to125(&self) -> bool {
        *self == TEMPGRADE_A::N40TO125
    }
    #[doc = "Checks if the value of the field is `N40TO105`"]
    #[inline(always)]
    pub fn is_n40to105(&self) -> bool {
        *self == TEMPGRADE_A::N40TO105
    }
    #[doc = "Checks if the value of the field is `N0TO70`"]
    #[inline(always)]
    pub fn is_n0to70(&self) -> bool {
        *self == TEMPGRADE_A::N0TO70
    }
}
#[doc = "Field `PKGTYPE` reader - Package Type"]
pub type PKGTYPE_R = crate::FieldReader<u8, PKGTYPE_A>;
#[doc = "Package Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PKGTYPE_A {
    #[doc = "74: WLCSP package"]
    WLCSP = 74,
    #[doc = "76: BGA package"]
    BGA = 76,
    #[doc = "77: QFN package"]
    QFN = 77,
    #[doc = "81: QFP package"]
    QFP = 81,
}
impl From<PKGTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: PKGTYPE_A) -> Self {
        variant as _
    }
}
impl PKGTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PKGTYPE_A> {
        match self.bits {
            74 => Some(PKGTYPE_A::WLCSP),
            76 => Some(PKGTYPE_A::BGA),
            77 => Some(PKGTYPE_A::QFN),
            81 => Some(PKGTYPE_A::QFP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WLCSP`"]
    #[inline(always)]
    pub fn is_wlcsp(&self) -> bool {
        *self == PKGTYPE_A::WLCSP
    }
    #[doc = "Checks if the value of the field is `BGA`"]
    #[inline(always)]
    pub fn is_bga(&self) -> bool {
        *self == PKGTYPE_A::BGA
    }
    #[doc = "Checks if the value of the field is `QFN`"]
    #[inline(always)]
    pub fn is_qfn(&self) -> bool {
        *self == PKGTYPE_A::QFN
    }
    #[doc = "Checks if the value of the field is `QFP`"]
    #[inline(always)]
    pub fn is_qfp(&self) -> bool {
        *self == PKGTYPE_A::QFP
    }
}
#[doc = "Field `PINCOUNT` reader - Pin Count"]
pub type PINCOUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Temperature Grade"]
    #[inline(always)]
    pub fn tempgrade(&self) -> TEMPGRADE_R {
        TEMPGRADE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Package Type"]
    #[inline(always)]
    pub fn pkgtype(&self) -> PKGTYPE_R {
        PKGTYPE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Pin Count"]
    #[inline(always)]
    pub fn pincount(&self) -> PINCOUNT_R {
        PINCOUNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Miscellaneous device information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkginfo](index.html) module"]
pub struct PKGINFO_SPEC;
impl crate::RegisterSpec for PKGINFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkginfo::R](R) reader structure"]
impl crate::Readable for PKGINFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKGINFO to value 0"]
impl crate::Resettable for PKGINFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
