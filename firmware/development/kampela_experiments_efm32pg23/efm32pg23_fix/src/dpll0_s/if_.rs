#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - Lock Interrupt Flag"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock Interrupt Flag"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `LOCKFAILLOW` reader - Lock Failure Low Interrupt Flag"]
pub type LOCKFAILLOW_R = crate::BitReader<bool>;
#[doc = "Field `LOCKFAILLOW` writer - Lock Failure Low Interrupt Flag"]
pub type LOCKFAILLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `LOCKFAILHIGH` reader - Lock Failure High Interrupt Flag"]
pub type LOCKFAILHIGH_R = crate::BitReader<bool>;
#[doc = "Field `LOCKFAILHIGH` writer - Lock Failure High Interrupt Flag"]
pub type LOCKFAILHIGH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Lock Interrupt Flag"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Failure Low Interrupt Flag"]
    #[inline(always)]
    pub fn lockfaillow(&self) -> LOCKFAILLOW_R {
        LOCKFAILLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Failure High Interrupt Flag"]
    #[inline(always)]
    pub fn lockfailhigh(&self) -> LOCKFAILHIGH_R {
        LOCKFAILHIGH_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 1 - Lock Failure Low Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lockfaillow(&mut self) -> LOCKFAILLOW_W<1> {
        LOCKFAILLOW_W::new(self)
    }
    #[doc = "Bit 2 - Lock Failure High Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lockfailhigh(&mut self) -> LOCKFAILHIGH_W<2> {
        LOCKFAILHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
