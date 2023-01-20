#[doc = "Register `RX_PROT` reader"]
pub struct R(crate::R<RX_PROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_PROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_PROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_PROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNPROTECTED` reader - UNPROTECTED"]
pub type UNPROTECTED_R = crate::BitReader<bool>;
#[doc = "Field `PRIVILEGED` reader - PRIVILEGED"]
pub type PRIVILEGED_R = crate::BitReader<bool>;
#[doc = "Field `NONSECURE` reader - NONSECURE"]
pub type NONSECURE_R = crate::BitReader<bool>;
#[doc = "Field `USER` reader - USER"]
pub type USER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 21 - UNPROTECTED"]
    #[inline(always)]
    pub fn unprotected(&self) -> UNPROTECTED_R {
        UNPROTECTED_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PRIVILEGED"]
    #[inline(always)]
    pub fn privileged(&self) -> PRIVILEGED_R {
        PRIVILEGED_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - NONSECURE"]
    #[inline(always)]
    pub fn nonsecure(&self) -> NONSECURE_R {
        NONSECURE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - USER"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "RX Protection register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_prot](index.html) module"]
pub struct RX_PROT_SPEC;
impl crate::RegisterSpec for RX_PROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_prot::R](R) reader structure"]
impl crate::Readable for RX_PROT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_PROT to value 0"]
impl crate::Resettable for RX_PROT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
