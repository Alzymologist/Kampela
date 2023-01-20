#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HITOF` reader - Hit Overflow Interrupt Enable"]
pub type HITOF_R = crate::BitReader<bool>;
#[doc = "Field `HITOF` writer - Hit Overflow Interrupt Enable"]
pub type HITOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `MISSOF` reader - Miss Overflow Interrupt Enable"]
pub type MISSOF_R = crate::BitReader<bool>;
#[doc = "Field `MISSOF` writer - Miss Overflow Interrupt Enable"]
pub type MISSOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `AHITOF` reader - Advanced Hit Overflow Interrupt Enable"]
pub type AHITOF_R = crate::BitReader<bool>;
#[doc = "Field `AHITOF` writer - Advanced Hit Overflow Interrupt Enable"]
pub type AHITOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `RAMERROR` reader - RAM error Interrupt Enable"]
pub type RAMERROR_R = crate::BitReader<bool>;
#[doc = "Field `RAMERROR` writer - RAM error Interrupt Enable"]
pub type RAMERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Hit Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn hitof(&self) -> HITOF_R {
        HITOF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Miss Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn missof(&self) -> MISSOF_R {
        MISSOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Advanced Hit Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ahitof(&self) -> AHITOF_R {
        AHITOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - RAM error Interrupt Enable"]
    #[inline(always)]
    pub fn ramerror(&self) -> RAMERROR_R {
        RAMERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hit Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hitof(&mut self) -> HITOF_W<0> {
        HITOF_W::new(self)
    }
    #[doc = "Bit 1 - Miss Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn missof(&mut self) -> MISSOF_W<1> {
        MISSOF_W::new(self)
    }
    #[doc = "Bit 2 - Advanced Hit Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahitof(&mut self) -> AHITOF_W<2> {
        AHITOF_W::new(self)
    }
    #[doc = "Bit 8 - RAM error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramerror(&mut self) -> RAMERROR_W<8> {
        RAMERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
