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
#[doc = "Field `BYPSW` reader - Bypass Switch Enabled"]
pub type BYPSW_R = crate::BitReader<bool>;
#[doc = "Field `BYPSW` writer - Bypass Switch Enabled"]
pub type BYPSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `WARM` reader - DCDC Warmup Time Done"]
pub type WARM_R = crate::BitReader<bool>;
#[doc = "Field `WARM` writer - DCDC Warmup Time Done"]
pub type WARM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RUNNING` reader - DCDC Running"]
pub type RUNNING_R = crate::BitReader<bool>;
#[doc = "Field `RUNNING` writer - DCDC Running"]
pub type RUNNING_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `VREGINLOW` reader - VREGIN below threshold"]
pub type VREGINLOW_R = crate::BitReader<bool>;
#[doc = "Field `VREGINLOW` writer - VREGIN below threshold"]
pub type VREGINLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `VREGINHIGH` reader - VREGIN above threshold"]
pub type VREGINHIGH_R = crate::BitReader<bool>;
#[doc = "Field `VREGINHIGH` writer - VREGIN above threshold"]
pub type VREGINHIGH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `REGULATION` reader - DCDC in regulation"]
pub type REGULATION_R = crate::BitReader<bool>;
#[doc = "Field `REGULATION` writer - DCDC in regulation"]
pub type REGULATION_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TMAX` reader - Ton_max Timeout Reached"]
pub type TMAX_R = crate::BitReader<bool>;
#[doc = "Field `TMAX` writer - Ton_max Timeout Reached"]
pub type TMAX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EM4ERR` reader - EM4 Entry Request Error"]
pub type EM4ERR_R = crate::BitReader<bool>;
#[doc = "Field `EM4ERR` writer - EM4 Entry Request Error"]
pub type EM4ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `PPMODE` reader - Entered Pulse Pairing mode"]
pub type PPMODE_R = crate::BitReader<bool>;
#[doc = "Field `PPMODE` writer - Entered Pulse Pairing mode"]
pub type PPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `PFMXMODE` reader - Entered PFMX mode"]
pub type PFMXMODE_R = crate::BitReader<bool>;
#[doc = "Field `PFMXMODE` writer - Entered PFMX mode"]
pub type PFMXMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Bypass Switch Enabled"]
    #[inline(always)]
    pub fn bypsw(&self) -> BYPSW_R {
        BYPSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCDC Warmup Time Done"]
    #[inline(always)]
    pub fn warm(&self) -> WARM_R {
        WARM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCDC Running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VREGIN below threshold"]
    #[inline(always)]
    pub fn vreginlow(&self) -> VREGINLOW_R {
        VREGINLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VREGIN above threshold"]
    #[inline(always)]
    pub fn vreginhigh(&self) -> VREGINHIGH_R {
        VREGINHIGH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DCDC in regulation"]
    #[inline(always)]
    pub fn regulation(&self) -> REGULATION_R {
        REGULATION_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ton_max Timeout Reached"]
    #[inline(always)]
    pub fn tmax(&self) -> TMAX_R {
        TMAX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EM4 Entry Request Error"]
    #[inline(always)]
    pub fn em4err(&self) -> EM4ERR_R {
        EM4ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Entered Pulse Pairing mode"]
    #[inline(always)]
    pub fn ppmode(&self) -> PPMODE_R {
        PPMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Entered PFMX mode"]
    #[inline(always)]
    pub fn pfmxmode(&self) -> PFMXMODE_R {
        PFMXMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass Switch Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bypsw(&mut self) -> BYPSW_W<0> {
        BYPSW_W::new(self)
    }
    #[doc = "Bit 1 - DCDC Warmup Time Done"]
    #[inline(always)]
    #[must_use]
    pub fn warm(&mut self) -> WARM_W<1> {
        WARM_W::new(self)
    }
    #[doc = "Bit 2 - DCDC Running"]
    #[inline(always)]
    #[must_use]
    pub fn running(&mut self) -> RUNNING_W<2> {
        RUNNING_W::new(self)
    }
    #[doc = "Bit 3 - VREGIN below threshold"]
    #[inline(always)]
    #[must_use]
    pub fn vreginlow(&mut self) -> VREGINLOW_W<3> {
        VREGINLOW_W::new(self)
    }
    #[doc = "Bit 4 - VREGIN above threshold"]
    #[inline(always)]
    #[must_use]
    pub fn vreginhigh(&mut self) -> VREGINHIGH_W<4> {
        VREGINHIGH_W::new(self)
    }
    #[doc = "Bit 5 - DCDC in regulation"]
    #[inline(always)]
    #[must_use]
    pub fn regulation(&mut self) -> REGULATION_W<5> {
        REGULATION_W::new(self)
    }
    #[doc = "Bit 6 - Ton_max Timeout Reached"]
    #[inline(always)]
    #[must_use]
    pub fn tmax(&mut self) -> TMAX_W<6> {
        TMAX_W::new(self)
    }
    #[doc = "Bit 7 - EM4 Entry Request Error"]
    #[inline(always)]
    #[must_use]
    pub fn em4err(&mut self) -> EM4ERR_W<7> {
        EM4ERR_W::new(self)
    }
    #[doc = "Bit 8 - Entered Pulse Pairing mode"]
    #[inline(always)]
    #[must_use]
    pub fn ppmode(&mut self) -> PPMODE_W<8> {
        PPMODE_W::new(self)
    }
    #[doc = "Bit 9 - Entered PFMX mode"]
    #[inline(always)]
    #[must_use]
    pub fn pfmxmode(&mut self) -> PFMXMODE_W<9> {
        PFMXMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
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
