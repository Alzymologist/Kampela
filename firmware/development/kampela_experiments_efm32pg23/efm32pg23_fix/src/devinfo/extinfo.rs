#[doc = "Register `EXTINFO` reader"]
pub struct R(crate::R<EXTINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TYPE` reader - Type"]
pub type TYPE_R = crate::FieldReader<u8, TYPE_A>;
#[doc = "Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "255: NONE"]
    NONE = 255,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
impl TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TYPE_A> {
        match self.bits {
            255 => Some(TYPE_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TYPE_A::NONE
    }
}
#[doc = "Field `CONNECTION` reader - Connection"]
pub type CONNECTION_R = crate::FieldReader<u8, CONNECTION_A>;
#[doc = "Connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONNECTION_A {
    #[doc = "0: SPI control interface"]
    SPI = 0,
    #[doc = "255: No interface"]
    NONE = 255,
}
impl From<CONNECTION_A> for u8 {
    #[inline(always)]
    fn from(variant: CONNECTION_A) -> Self {
        variant as _
    }
}
impl CONNECTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CONNECTION_A> {
        match self.bits {
            0 => Some(CONNECTION_A::SPI),
            255 => Some(CONNECTION_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == CONNECTION_A::SPI
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CONNECTION_A::NONE
    }
}
#[doc = "Field `REV` reader - Revision"]
pub type REV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Type"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Connection"]
    #[inline(always)]
    pub fn connection(&self) -> CONNECTION_R {
        CONNECTION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Revision"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "External component description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extinfo](index.html) module"]
pub struct EXTINFO_SPEC;
impl crate::RegisterSpec for EXTINFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extinfo::R](R) reader structure"]
impl crate::Readable for EXTINFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTINFO to value 0"]
impl crate::Resettable for EXTINFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
