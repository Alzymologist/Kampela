#[doc = "Register `DMEM0PORTMAPSEL` reader"]
pub struct R(crate::R<DMEM0PORTMAPSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMEM0PORTMAPSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMEM0PORTMAPSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMEM0PORTMAPSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMEM0PORTMAPSEL` writer"]
pub struct W(crate::W<DMEM0PORTMAPSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMEM0PORTMAPSEL_SPEC>;
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
impl From<crate::W<DMEM0PORTMAPSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMEM0PORTMAPSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDMAPORTSEL` reader - LDMA portmap selection"]
pub type LDMAPORTSEL_R = crate::BitReader<bool>;
#[doc = "Field `LDMAPORTSEL` writer - LDMA portmap selection"]
pub type LDMAPORTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMEM0PORTMAPSEL_SPEC, bool, O>;
#[doc = "Field `SRWAESPORTSEL` reader - SRWAES portmap selection"]
pub type SRWAESPORTSEL_R = crate::BitReader<bool>;
#[doc = "Field `SRWAESPORTSEL` writer - SRWAES portmap selection"]
pub type SRWAESPORTSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMEM0PORTMAPSEL_SPEC, bool, O>;
#[doc = "Field `AHBSRWPORTSEL` reader - AHBSRW portmap selection"]
pub type AHBSRWPORTSEL_R = crate::BitReader<bool>;
#[doc = "Field `AHBSRWPORTSEL` writer - AHBSRW portmap selection"]
pub type AHBSRWPORTSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMEM0PORTMAPSEL_SPEC, bool, O>;
#[doc = "Field `SRWECA0PORTSEL` reader - SRWECA0 portmap selection"]
pub type SRWECA0PORTSEL_R = crate::BitReader<bool>;
#[doc = "Field `SRWECA0PORTSEL` writer - SRWECA0 portmap selection"]
pub type SRWECA0PORTSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMEM0PORTMAPSEL_SPEC, bool, O>;
#[doc = "Field `SRWECA1PORTSEL` reader - SRWECA1 portmap selection"]
pub type SRWECA1PORTSEL_R = crate::BitReader<bool>;
#[doc = "Field `SRWECA1PORTSEL` writer - SRWECA1 portmap selection"]
pub type SRWECA1PORTSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMEM0PORTMAPSEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LDMA portmap selection"]
    #[inline(always)]
    pub fn ldmaportsel(&self) -> LDMAPORTSEL_R {
        LDMAPORTSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRWAES portmap selection"]
    #[inline(always)]
    pub fn srwaesportsel(&self) -> SRWAESPORTSEL_R {
        SRWAESPORTSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHBSRW portmap selection"]
    #[inline(always)]
    pub fn ahbsrwportsel(&self) -> AHBSRWPORTSEL_R {
        AHBSRWPORTSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRWECA0 portmap selection"]
    #[inline(always)]
    pub fn srweca0portsel(&self) -> SRWECA0PORTSEL_R {
        SRWECA0PORTSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRWECA1 portmap selection"]
    #[inline(always)]
    pub fn srweca1portsel(&self) -> SRWECA1PORTSEL_R {
        SRWECA1PORTSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LDMA portmap selection"]
    #[inline(always)]
    #[must_use]
    pub fn ldmaportsel(&mut self) -> LDMAPORTSEL_W<0> {
        LDMAPORTSEL_W::new(self)
    }
    #[doc = "Bit 1 - SRWAES portmap selection"]
    #[inline(always)]
    #[must_use]
    pub fn srwaesportsel(&mut self) -> SRWAESPORTSEL_W<1> {
        SRWAESPORTSEL_W::new(self)
    }
    #[doc = "Bit 2 - AHBSRW portmap selection"]
    #[inline(always)]
    #[must_use]
    pub fn ahbsrwportsel(&mut self) -> AHBSRWPORTSEL_W<2> {
        AHBSRWPORTSEL_W::new(self)
    }
    #[doc = "Bit 3 - SRWECA0 portmap selection"]
    #[inline(always)]
    #[must_use]
    pub fn srweca0portsel(&mut self) -> SRWECA0PORTSEL_W<3> {
        SRWECA0PORTSEL_W::new(self)
    }
    #[doc = "Bit 4 - SRWECA1 portmap selection"]
    #[inline(always)]
    #[must_use]
    pub fn srweca1portsel(&mut self) -> SRWECA1PORTSEL_W<4> {
        SRWECA1PORTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure DMEM0 port remap selection.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmem0portmapsel](index.html) module"]
pub struct DMEM0PORTMAPSEL_SPEC;
impl crate::RegisterSpec for DMEM0PORTMAPSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmem0portmapsel::R](R) reader structure"]
impl crate::Readable for DMEM0PORTMAPSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmem0portmapsel::W](W) writer structure"]
impl crate::Writable for DMEM0PORTMAPSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMEM0PORTMAPSEL to value 0x13"]
impl crate::Resettable for DMEM0PORTMAPSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x13;
}
