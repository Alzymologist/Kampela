#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACMPOUT` reader - Analog Comparator Output"]
pub type ACMPOUT_R = crate::BitReader<bool>;
#[doc = "Field `ACMPRDY` reader - Analog Comparator Ready"]
pub type ACMPRDY_R = crate::BitReader<bool>;
#[doc = "Field `INPUTCONFLICT` reader - INPUT conflict"]
pub type INPUTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `PORTALLOCERR` reader - Port allocation error"]
pub type PORTALLOCERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Output"]
    #[inline(always)]
    pub fn acmpout(&self) -> ACMPOUT_R {
        ACMPOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Comparator Ready"]
    #[inline(always)]
    pub fn acmprdy(&self) -> ACMPRDY_R {
        ACMPRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - INPUT conflict"]
    #[inline(always)]
    pub fn inputconflict(&self) -> INPUTCONFLICT_R {
        INPUTCONFLICT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port allocation error"]
    #[inline(always)]
    pub fn portallocerr(&self) -> PORTALLOCERR_R {
        PORTALLOCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
