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
#[doc = "Field `RDY` reader - LFXO Ready Interrupt Flag"]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `RDY` writer - LFXO Ready Interrupt Flag"]
pub type RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `POSEDGE` reader - Rising Edge Interrupt Flag"]
pub type POSEDGE_R = crate::BitReader<bool>;
#[doc = "Field `POSEDGE` writer - Rising Edge Interrupt Flag"]
pub type POSEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `NEGEDGE` reader - Falling Edge Interrupt Flag"]
pub type NEGEDGE_R = crate::BitReader<bool>;
#[doc = "Field `NEGEDGE` writer - Falling Edge Interrupt Flag"]
pub type NEGEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `FAIL` reader - LFXO Failure Interrupt Flag"]
pub type FAIL_R = crate::BitReader<bool>;
#[doc = "Field `FAIL` writer - LFXO Failure Interrupt Flag"]
pub type FAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising Edge Interrupt Flag"]
    #[inline(always)]
    pub fn posedge(&self) -> POSEDGE_R {
        POSEDGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling Edge Interrupt Flag"]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXO Failure Interrupt Flag"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFXO Ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<0> {
        RDY_W::new(self)
    }
    #[doc = "Bit 1 - Rising Edge Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn posedge(&mut self) -> POSEDGE_W<1> {
        POSEDGE_W::new(self)
    }
    #[doc = "Bit 2 - Falling Edge Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn negedge(&mut self) -> NEGEDGE_W<2> {
        NEGEDGE_W::new(self)
    }
    #[doc = "Bit 3 - LFXO Failure Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fail(&mut self) -> FAIL_W<3> {
        FAIL_W::new(self)
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
