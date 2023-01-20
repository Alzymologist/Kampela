#[doc = "Register `ROOTLOCKSTATUS` reader"]
pub struct R(crate::R<ROOTLOCKSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROOTLOCKSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROOTLOCKSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROOTLOCKSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSLOCK` reader - Bus Lock"]
pub type BUSLOCK_R = crate::BitReader<bool>;
#[doc = "Field `REGLOCK` reader - Register Lock"]
pub type REGLOCK_R = crate::BitReader<bool>;
#[doc = "Field `MFRLOCK` reader - Manufacture Lock"]
pub type MFRLOCK_R = crate::BitReader<bool>;
#[doc = "Field `ROOTDBGLOCK` reader - Root Debug Lock"]
pub type ROOTDBGLOCK_R = crate::BitReader<bool>;
#[doc = "Field `USERDBGAPLOCK` reader - User Debug Access Port Lock"]
pub type USERDBGAPLOCK_R = crate::BitReader<bool>;
#[doc = "Field `USERDBGLOCK` reader - User Invasive Debug Lock"]
pub type USERDBGLOCK_R = crate::BitReader<bool>;
#[doc = "Field `USERNIDLOCK` reader - User Non-invasive Debug Lock"]
pub type USERNIDLOCK_R = crate::BitReader<bool>;
#[doc = "Field `USERSPIDLOCK` reader - User Secure Invasive Debug Lock"]
pub type USERSPIDLOCK_R = crate::BitReader<bool>;
#[doc = "Field `USERSPNIDLOCK` reader - User Secure Non-invasive Debug Lock"]
pub type USERSPNIDLOCK_R = crate::BitReader<bool>;
#[doc = "Field `EFUSEUNLOCKED` reader - E-Fuse Unlocked"]
pub type EFUSEUNLOCKED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Bus Lock"]
    #[inline(always)]
    pub fn buslock(&self) -> BUSLOCK_R {
        BUSLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register Lock"]
    #[inline(always)]
    pub fn reglock(&self) -> REGLOCK_R {
        REGLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Manufacture Lock"]
    #[inline(always)]
    pub fn mfrlock(&self) -> MFRLOCK_R {
        MFRLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Root Debug Lock"]
    #[inline(always)]
    pub fn rootdbglock(&self) -> ROOTDBGLOCK_R {
        ROOTDBGLOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - User Debug Access Port Lock"]
    #[inline(always)]
    pub fn userdbgaplock(&self) -> USERDBGAPLOCK_R {
        USERDBGAPLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - User Invasive Debug Lock"]
    #[inline(always)]
    pub fn userdbglock(&self) -> USERDBGLOCK_R {
        USERDBGLOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - User Non-invasive Debug Lock"]
    #[inline(always)]
    pub fn usernidlock(&self) -> USERNIDLOCK_R {
        USERNIDLOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - User Secure Invasive Debug Lock"]
    #[inline(always)]
    pub fn userspidlock(&self) -> USERSPIDLOCK_R {
        USERSPIDLOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - User Secure Non-invasive Debug Lock"]
    #[inline(always)]
    pub fn userspnidlock(&self) -> USERSPNIDLOCK_R {
        USERSPNIDLOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - E-Fuse Unlocked"]
    #[inline(always)]
    pub fn efuseunlocked(&self) -> EFUSEUNLOCKED_R {
        EFUSEUNLOCKED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "This register returns the status of the SE managed locks.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rootlockstatus](index.html) module"]
pub struct ROOTLOCKSTATUS_SPEC;
impl crate::RegisterSpec for ROOTLOCKSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rootlockstatus::R](R) reader structure"]
impl crate::Readable for ROOTLOCKSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ROOTLOCKSTATUS to value 0x007f_0107"]
impl crate::Resettable for ROOTLOCKSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x007f_0107;
}
