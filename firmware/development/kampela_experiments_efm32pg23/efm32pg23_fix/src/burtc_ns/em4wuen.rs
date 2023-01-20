#[doc = "Register `EM4WUEN` reader"]
pub struct R(crate::R<EM4WUEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM4WUEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM4WUEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM4WUEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM4WUEN` writer"]
pub struct W(crate::W<EM4WUEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM4WUEN_SPEC>;
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
impl From<crate::W<EM4WUEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM4WUEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFEM4WUEN` reader - Overflow EM4 Wakeup Enable"]
pub type OFEM4WUEN_R = crate::BitReader<bool>;
#[doc = "Field `OFEM4WUEN` writer - Overflow EM4 Wakeup Enable"]
pub type OFEM4WUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EM4WUEN_SPEC, bool, O>;
#[doc = "Field `COMPEM4WUEN` reader - Compare Match EM4 Wakeup Enable"]
pub type COMPEM4WUEN_R = crate::BitReader<bool>;
#[doc = "Field `COMPEM4WUEN` writer - Compare Match EM4 Wakeup Enable"]
pub type COMPEM4WUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EM4WUEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Overflow EM4 Wakeup Enable"]
    #[inline(always)]
    pub fn ofem4wuen(&self) -> OFEM4WUEN_R {
        OFEM4WUEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Match EM4 Wakeup Enable"]
    #[inline(always)]
    pub fn compem4wuen(&self) -> COMPEM4WUEN_R {
        COMPEM4WUEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow EM4 Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ofem4wuen(&mut self) -> OFEM4WUEN_W<0> {
        OFEM4WUEN_W::new(self)
    }
    #[doc = "Bit 1 - Compare Match EM4 Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compem4wuen(&mut self) -> COMPEM4WUEN_W<1> {
        COMPEM4WUEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em4wuen](index.html) module"]
pub struct EM4WUEN_SPEC;
impl crate::RegisterSpec for EM4WUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em4wuen::R](R) reader structure"]
impl crate::Readable for EM4WUEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em4wuen::W](W) writer structure"]
impl crate::Writable for EM4WUEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EM4WUEN to value 0"]
impl crate::Resettable for EM4WUEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
