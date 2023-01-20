#[doc = "Register `DGIEN` reader"]
pub struct R(crate::R<DGIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DGIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DGIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DGIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DGIEN` writer"]
pub struct W(crate::W<DGIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DGIEN_SPEC>;
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
impl From<crate::W<DGIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DGIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM23WAKEUPDGIEN` reader - EM23 Wake up Interrupt enable"]
pub type EM23WAKEUPDGIEN_R = crate::BitReader<bool>;
#[doc = "Field `EM23WAKEUPDGIEN` writer - EM23 Wake up Interrupt enable"]
pub type EM23WAKEUPDGIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DGIEN_SPEC, bool, O>;
#[doc = "Field `TEMPDGIEN` reader - Temperature Interrupt enable"]
pub type TEMPDGIEN_R = crate::BitReader<bool>;
#[doc = "Field `TEMPDGIEN` writer - Temperature Interrupt enable"]
pub type TEMPDGIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DGIEN_SPEC, bool, O>;
#[doc = "Field `TEMPLOWDGIEN` reader - Temperature low Interrupt enable"]
pub type TEMPLOWDGIEN_R = crate::BitReader<bool>;
#[doc = "Field `TEMPLOWDGIEN` writer - Temperature low Interrupt enable"]
pub type TEMPLOWDGIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DGIEN_SPEC, bool, O>;
#[doc = "Field `TEMPHIGHDGIEN` reader - Temperature high Interrupt enable"]
pub type TEMPHIGHDGIEN_R = crate::BitReader<bool>;
#[doc = "Field `TEMPHIGHDGIEN` writer - Temperature high Interrupt enable"]
pub type TEMPHIGHDGIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DGIEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 24 - EM23 Wake up Interrupt enable"]
    #[inline(always)]
    pub fn em23wakeupdgien(&self) -> EM23WAKEUPDGIEN_R {
        EM23WAKEUPDGIEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - Temperature Interrupt enable"]
    #[inline(always)]
    pub fn tempdgien(&self) -> TEMPDGIEN_R {
        TEMPDGIEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Temperature low Interrupt enable"]
    #[inline(always)]
    pub fn templowdgien(&self) -> TEMPLOWDGIEN_R {
        TEMPLOWDGIEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Temperature high Interrupt enable"]
    #[inline(always)]
    pub fn temphighdgien(&self) -> TEMPHIGHDGIEN_R {
        TEMPHIGHDGIEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - EM23 Wake up Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeupdgien(&mut self) -> EM23WAKEUPDGIEN_W<24> {
        EM23WAKEUPDGIEN_W::new(self)
    }
    #[doc = "Bit 29 - Temperature Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tempdgien(&mut self) -> TEMPDGIEN_W<29> {
        TEMPDGIEN_W::new(self)
    }
    #[doc = "Bit 30 - Temperature low Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn templowdgien(&mut self) -> TEMPLOWDGIEN_W<30> {
        TEMPLOWDGIEN_W::new(self)
    }
    #[doc = "Bit 31 - Temperature high Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn temphighdgien(&mut self) -> TEMPHIGHDGIEN_W<31> {
        TEMPHIGHDGIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dgien](index.html) module"]
pub struct DGIEN_SPEC;
impl crate::RegisterSpec for DGIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dgien::R](R) reader structure"]
impl crate::Readable for DGIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dgien::W](W) writer structure"]
impl crate::Writable for DGIEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DGIEN to value 0"]
impl crate::Resettable for DGIEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
