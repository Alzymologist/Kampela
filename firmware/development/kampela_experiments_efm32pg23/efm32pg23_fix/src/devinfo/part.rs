#[doc = "Register `PART` reader"]
pub struct R(crate::R<PART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVICENUM` reader - Device Number"]
pub type DEVICENUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FAMILYNUM` reader - Device Family"]
pub type FAMILYNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAMILY` reader - Device Family"]
pub type FAMILY_R = crate::FieldReader<u8, FAMILY_A>;
#[doc = "Device Family\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FAMILY_A {
    #[doc = "0: Flex Gecko"]
    FG = 0,
    #[doc = "3: Z-Wave Gecko"]
    ZG = 3,
    #[doc = "5: Pearl Gecko"]
    PG = 5,
}
impl From<FAMILY_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMILY_A) -> Self {
        variant as _
    }
}
impl FAMILY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FAMILY_A> {
        match self.bits {
            0 => Some(FAMILY_A::FG),
            3 => Some(FAMILY_A::ZG),
            5 => Some(FAMILY_A::PG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FG`"]
    #[inline(always)]
    pub fn is_fg(&self) -> bool {
        *self == FAMILY_A::FG
    }
    #[doc = "Checks if the value of the field is `ZG`"]
    #[inline(always)]
    pub fn is_zg(&self) -> bool {
        *self == FAMILY_A::ZG
    }
    #[doc = "Checks if the value of the field is `PG`"]
    #[inline(always)]
    pub fn is_pg(&self) -> bool {
        *self == FAMILY_A::PG
    }
}
impl R {
    #[doc = "Bits 0:15 - Device Number"]
    #[inline(always)]
    pub fn devicenum(&self) -> DEVICENUM_R {
        DEVICENUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Device Family"]
    #[inline(always)]
    pub fn familynum(&self) -> FAMILYNUM_R {
        FAMILYNUM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Device Family"]
    #[inline(always)]
    pub fn family(&self) -> FAMILY_R {
        FAMILY_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "Part description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [part](index.html) module"]
pub struct PART_SPEC;
impl crate::RegisterSpec for PART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [part::R](R) reader structure"]
impl crate::Readable for PART_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PART to value 0"]
impl crate::Resettable for PART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
