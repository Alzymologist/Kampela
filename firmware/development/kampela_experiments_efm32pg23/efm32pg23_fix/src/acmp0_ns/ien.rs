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
#[doc = "Field `RISE` reader - Rising edge interrupt enable"]
pub type RISE_R = crate::BitReader<bool>;
#[doc = "Field `RISE` writer - Rising edge interrupt enable"]
pub type RISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `FALL` reader - Falling edge interrupt enable"]
pub type FALL_R = crate::BitReader<bool>;
#[doc = "Field `FALL` writer - Falling edge interrupt enable"]
pub type FALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ACMPRDY` reader - ACMP ready interrupt enable"]
pub type ACMPRDY_R = crate::BitReader<bool>;
#[doc = "Field `ACMPRDY` writer - ACMP ready interrupt enable"]
pub type ACMPRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `INPUTCONFLICT` reader - Input conflict interrupt enable"]
pub type INPUTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `INPUTCONFLICT` writer - Input conflict interrupt enable"]
pub type INPUTCONFLICT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `PORTALLOCERR` reader - Port allocation error interrupt enable"]
pub type PORTALLOCERR_R = crate::BitReader<bool>;
#[doc = "Field `PORTALLOCERR` writer - Port allocation error interrupt enable"]
pub type PORTALLOCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Rising edge interrupt enable"]
    #[inline(always)]
    pub fn rise(&self) -> RISE_R {
        RISE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge interrupt enable"]
    #[inline(always)]
    pub fn fall(&self) -> FALL_R {
        FALL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACMP ready interrupt enable"]
    #[inline(always)]
    pub fn acmprdy(&self) -> ACMPRDY_R {
        ACMPRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input conflict interrupt enable"]
    #[inline(always)]
    pub fn inputconflict(&self) -> INPUTCONFLICT_R {
        INPUTCONFLICT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port allocation error interrupt enable"]
    #[inline(always)]
    pub fn portallocerr(&self) -> PORTALLOCERR_R {
        PORTALLOCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rise(&mut self) -> RISE_W<0> {
        RISE_W::new(self)
    }
    #[doc = "Bit 1 - Falling edge interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fall(&mut self) -> FALL_W<1> {
        FALL_W::new(self)
    }
    #[doc = "Bit 2 - ACMP ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmprdy(&mut self) -> ACMPRDY_W<2> {
        ACMPRDY_W::new(self)
    }
    #[doc = "Bit 3 - Input conflict interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn inputconflict(&mut self) -> INPUTCONFLICT_W<3> {
        INPUTCONFLICT_W::new(self)
    }
    #[doc = "Bit 4 - Port allocation error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn portallocerr(&mut self) -> PORTALLOCERR_W<4> {
        PORTALLOCERR_W::new(self)
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
