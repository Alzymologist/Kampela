#[doc = "Register `DGIF` reader"]
pub struct R(crate::R<DGIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DGIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DGIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DGIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DGIF` writer"]
pub struct W(crate::W<DGIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DGIF_SPEC>;
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
impl From<crate::W<DGIF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DGIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM23WAKEUPDGIF` reader - EM23 Wake up Interrupt flag"]
pub type EM23WAKEUPDGIF_R = crate::BitReader<bool>;
#[doc = "Field `EM23WAKEUPDGIF` writer - EM23 Wake up Interrupt flag"]
pub type EM23WAKEUPDGIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DGIF_SPEC, bool, O>;
#[doc = "Field `TEMPDGIF` reader - Temperature Interrupt flag"]
pub type TEMPDGIF_R = crate::BitReader<bool>;
#[doc = "Field `TEMPDGIF` writer - Temperature Interrupt flag"]
pub type TEMPDGIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DGIF_SPEC, bool, O>;
#[doc = "Field `TEMPLOWDGIF` reader - Temperature low Interrupt flag"]
pub type TEMPLOWDGIF_R = crate::BitReader<bool>;
#[doc = "Field `TEMPLOWDGIF` writer - Temperature low Interrupt flag"]
pub type TEMPLOWDGIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DGIF_SPEC, bool, O>;
#[doc = "Field `TEMPHIGHDGIF` reader - Temperature high Interrupt flag"]
pub type TEMPHIGHDGIF_R = crate::BitReader<bool>;
#[doc = "Field `TEMPHIGHDGIF` writer - Temperature high Interrupt flag"]
pub type TEMPHIGHDGIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DGIF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 24 - EM23 Wake up Interrupt flag"]
    #[inline(always)]
    pub fn em23wakeupdgif(&self) -> EM23WAKEUPDGIF_R {
        EM23WAKEUPDGIF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - Temperature Interrupt flag"]
    #[inline(always)]
    pub fn tempdgif(&self) -> TEMPDGIF_R {
        TEMPDGIF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Temperature low Interrupt flag"]
    #[inline(always)]
    pub fn templowdgif(&self) -> TEMPLOWDGIF_R {
        TEMPLOWDGIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Temperature high Interrupt flag"]
    #[inline(always)]
    pub fn temphighdgif(&self) -> TEMPHIGHDGIF_R {
        TEMPHIGHDGIF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - EM23 Wake up Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeupdgif(&mut self) -> EM23WAKEUPDGIF_W<24> {
        EM23WAKEUPDGIF_W::new(self)
    }
    #[doc = "Bit 29 - Temperature Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tempdgif(&mut self) -> TEMPDGIF_W<29> {
        TEMPDGIF_W::new(self)
    }
    #[doc = "Bit 30 - Temperature low Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn templowdgif(&mut self) -> TEMPLOWDGIF_W<30> {
        TEMPLOWDGIF_W::new(self)
    }
    #[doc = "Bit 31 - Temperature high Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn temphighdgif(&mut self) -> TEMPHIGHDGIF_W<31> {
        TEMPHIGHDGIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dgif](index.html) module"]
pub struct DGIF_SPEC;
impl crate::RegisterSpec for DGIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dgif::R](R) reader structure"]
impl crate::Readable for DGIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dgif::W](W) writer structure"]
impl crate::Writable for DGIF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DGIF to value 0"]
impl crate::Resettable for DGIF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
