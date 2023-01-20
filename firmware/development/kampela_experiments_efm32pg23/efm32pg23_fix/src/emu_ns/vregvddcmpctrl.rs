#[doc = "Register `VREGVDDCMPCTRL` reader"]
pub struct R(crate::R<VREGVDDCMPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREGVDDCMPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREGVDDCMPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREGVDDCMPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREGVDDCMPCTRL` writer"]
pub struct W(crate::W<VREGVDDCMPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREGVDDCMPCTRL_SPEC>;
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
impl From<crate::W<VREGVDDCMPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREGVDDCMPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREGINCMPEN` reader - VREGVDD comparator enable"]
pub type VREGINCMPEN_R = crate::BitReader<bool>;
#[doc = "Field `VREGINCMPEN` writer - VREGVDD comparator enable"]
pub type VREGINCMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREGVDDCMPCTRL_SPEC, bool, O>;
#[doc = "Field `THRESSEL` reader - VREGVDD comparator threshold programming"]
pub type THRESSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRESSEL` writer - VREGVDD comparator threshold programming"]
pub type THRESSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VREGVDDCMPCTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - VREGVDD comparator enable"]
    #[inline(always)]
    pub fn vregincmpen(&self) -> VREGINCMPEN_R {
        VREGINCMPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - VREGVDD comparator threshold programming"]
    #[inline(always)]
    pub fn thressel(&self) -> THRESSEL_R {
        THRESSEL_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VREGVDD comparator enable"]
    #[inline(always)]
    #[must_use]
    pub fn vregincmpen(&mut self) -> VREGINCMPEN_W<0> {
        VREGINCMPEN_W::new(self)
    }
    #[doc = "Bits 1:2 - VREGVDD comparator threshold programming"]
    #[inline(always)]
    #[must_use]
    pub fn thressel(&mut self) -> THRESSEL_W<1> {
        THRESSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vregvddcmpctrl](index.html) module"]
pub struct VREGVDDCMPCTRL_SPEC;
impl crate::RegisterSpec for VREGVDDCMPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vregvddcmpctrl::R](R) reader structure"]
impl crate::Readable for VREGVDDCMPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vregvddcmpctrl::W](W) writer structure"]
impl crate::Writable for VREGVDDCMPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREGVDDCMPCTRL to value 0x06"]
impl crate::Resettable for VREGVDDCMPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
