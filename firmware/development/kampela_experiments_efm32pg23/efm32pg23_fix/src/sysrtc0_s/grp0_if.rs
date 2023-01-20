#[doc = "Register `GRP0_IF` reader"]
pub struct R(crate::R<GRP0_IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRP0_IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRP0_IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRP0_IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GRP0_IF` writer"]
pub struct W(crate::W<GRP0_IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRP0_IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GRP0_IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRP0_IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF` reader - Overflow Interrupt Flag"]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `OVF` writer - Overflow Interrupt Flag"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRP0_IF_SPEC, bool, O>;
#[doc = "Field `CMP0` reader - Compare 0 Interrupt Flag"]
pub type CMP0_R = crate::BitReader<bool>;
#[doc = "Field `CMP0` writer - Compare 0 Interrupt Flag"]
pub type CMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRP0_IF_SPEC, bool, O>;
#[doc = "Field `CMP1` reader - Compare 1 Interrupt Flag"]
pub type CMP1_R = crate::BitReader<bool>;
#[doc = "Field `CMP1` writer - Compare 1 Interrupt Flag"]
pub type CMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRP0_IF_SPEC, bool, O>;
#[doc = "Field `CAP0` reader - Capture 0 Interrupt Flag"]
pub type CAP0_R = crate::BitReader<bool>;
#[doc = "Field `CAP0` writer - Capture 0 Interrupt Flag"]
pub type CAP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRP0_IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 0 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp0(&self) -> CMP0_R {
        CMP0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture 0 Interrupt Flag"]
    #[inline(always)]
    pub fn cap0(&self) -> CAP0_R {
        CAP0_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<0> {
        OVF_W::new(self)
    }
    #[doc = "Bit 1 - Compare 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0(&mut self) -> CMP0_W<1> {
        CMP0_W::new(self)
    }
    #[doc = "Bit 2 - Compare 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> CMP1_W<2> {
        CMP1_W::new(self)
    }
    #[doc = "Bit 3 - Capture 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cap0(&mut self) -> CAP0_W<3> {
        CAP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grp0_if](index.html) module"]
pub struct GRP0_IF_SPEC;
impl crate::RegisterSpec for GRP0_IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grp0_if::R](R) reader structure"]
impl crate::Readable for GRP0_IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [grp0_if::W](W) writer structure"]
impl crate::Writable for GRP0_IF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GRP0_IF to value 0"]
impl crate::Resettable for GRP0_IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
