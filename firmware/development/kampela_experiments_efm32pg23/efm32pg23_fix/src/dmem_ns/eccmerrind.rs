#[doc = "Register `ECCMERRIND` reader"]
pub struct R(crate::R<ECCMERRIND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCMERRIND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCMERRIND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCMERRIND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P0` reader - Multiple ECC errors on AHB port 0"]
pub type P0_R = crate::BitReader<bool>;
#[doc = "Field `P1` reader - Multiple ECC errors on AHB port 1"]
pub type P1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Multiple ECC errors on AHB port 0"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multiple ECC errors on AHB port 1"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccmerrind](index.html) module"]
pub struct ECCMERRIND_SPEC;
impl crate::RegisterSpec for ECCMERRIND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccmerrind::R](R) reader structure"]
impl crate::Readable for ECCMERRIND_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECCMERRIND to value 0"]
impl crate::Resettable for ECCMERRIND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
