#[doc = "Register `GRP0_CAP0VALUE` reader"]
pub struct R(crate::R<GRP0_CAP0VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRP0_CAP0VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRP0_CAP0VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRP0_CAP0VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAP0VALUE` reader - Capture 0 Value"]
pub type CAP0VALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture 0 Value"]
    #[inline(always)]
    pub fn cap0value(&self) -> CAP0VALUE_R {
        CAP0VALUE_R::new(self.bits)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grp0_cap0value](index.html) module"]
pub struct GRP0_CAP0VALUE_SPEC;
impl crate::RegisterSpec for GRP0_CAP0VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grp0_cap0value::R](R) reader structure"]
impl crate::Readable for GRP0_CAP0VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GRP0_CAP0VALUE to value 0"]
impl crate::Resettable for GRP0_CAP0VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
