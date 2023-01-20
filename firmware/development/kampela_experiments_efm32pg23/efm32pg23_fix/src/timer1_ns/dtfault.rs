#[doc = "Register `DTFAULT` reader"]
pub struct R(crate::R<DTFAULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTFAULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTFAULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTFAULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DTPRS0F` reader - DTI PRS 0 Fault"]
pub type DTPRS0F_R = crate::BitReader<bool>;
#[doc = "Field `DTPRS1F` reader - DTI PRS 1 Fault"]
pub type DTPRS1F_R = crate::BitReader<bool>;
#[doc = "Field `DTDBGF` reader - DTI Debugger Fault"]
pub type DTDBGF_R = crate::BitReader<bool>;
#[doc = "Field `DTLOCKUPF` reader - DTI Lockup Fault"]
pub type DTLOCKUPF_R = crate::BitReader<bool>;
#[doc = "Field `DTEM23F` reader - DTI EM23 Entry Fault"]
pub type DTEM23F_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DTI PRS 0 Fault"]
    #[inline(always)]
    pub fn dtprs0f(&self) -> DTPRS0F_R {
        DTPRS0F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI PRS 1 Fault"]
    #[inline(always)]
    pub fn dtprs1f(&self) -> DTPRS1F_R {
        DTPRS1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTI Debugger Fault"]
    #[inline(always)]
    pub fn dtdbgf(&self) -> DTDBGF_R {
        DTDBGF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTI Lockup Fault"]
    #[inline(always)]
    pub fn dtlockupf(&self) -> DTLOCKUPF_R {
        DTLOCKUPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DTI EM23 Entry Fault"]
    #[inline(always)]
    pub fn dtem23f(&self) -> DTEM23F_R {
        DTEM23F_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtfault](index.html) module"]
pub struct DTFAULT_SPEC;
impl crate::RegisterSpec for DTFAULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtfault::R](R) reader structure"]
impl crate::Readable for DTFAULT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTFAULT to value 0"]
impl crate::Resettable for DTFAULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
