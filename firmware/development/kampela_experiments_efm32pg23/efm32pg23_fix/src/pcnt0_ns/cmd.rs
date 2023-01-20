#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORERST` writer - PCNT Clock Domain Reset"]
pub type CORERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CNTRST` writer - CNT Reset"]
pub type CNTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `AUXCNTRST` writer - AUXCNT Reset"]
pub type AUXCNTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `LCNTIM` writer - Load CNT Immediately"]
pub type LCNTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `STARTCNT` writer - Start Main Counter"]
pub type STARTCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `STARTAUXCNT` writer - Start Aux Counter"]
pub type STARTAUXCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `STOPCNT` writer - Stop Main Counter"]
pub type STOPCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `STOPAUXCNT` writer - Stop Aux Counter"]
pub type STOPAUXCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - PCNT Clock Domain Reset"]
    #[inline(always)]
    #[must_use]
    pub fn corerst(&mut self) -> CORERST_W<0> {
        CORERST_W::new(self)
    }
    #[doc = "Bit 1 - CNT Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cntrst(&mut self) -> CNTRST_W<1> {
        CNTRST_W::new(self)
    }
    #[doc = "Bit 2 - AUXCNT Reset"]
    #[inline(always)]
    #[must_use]
    pub fn auxcntrst(&mut self) -> AUXCNTRST_W<2> {
        AUXCNTRST_W::new(self)
    }
    #[doc = "Bit 4 - Load CNT Immediately"]
    #[inline(always)]
    #[must_use]
    pub fn lcntim(&mut self) -> LCNTIM_W<4> {
        LCNTIM_W::new(self)
    }
    #[doc = "Bit 8 - Start Main Counter"]
    #[inline(always)]
    #[must_use]
    pub fn startcnt(&mut self) -> STARTCNT_W<8> {
        STARTCNT_W::new(self)
    }
    #[doc = "Bit 9 - Start Aux Counter"]
    #[inline(always)]
    #[must_use]
    pub fn startauxcnt(&mut self) -> STARTAUXCNT_W<9> {
        STARTAUXCNT_W::new(self)
    }
    #[doc = "Bit 10 - Stop Main Counter"]
    #[inline(always)]
    #[must_use]
    pub fn stopcnt(&mut self) -> STOPCNT_W<10> {
        STOPCNT_W::new(self)
    }
    #[doc = "Bit 11 - Stop Aux Counter"]
    #[inline(always)]
    #[must_use]
    pub fn stopauxcnt(&mut self) -> STOPAUXCNT_W<11> {
        STOPAUXCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
