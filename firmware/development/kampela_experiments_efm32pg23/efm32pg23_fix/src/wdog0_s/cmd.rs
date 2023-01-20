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
#[doc = "WDOG Timer Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLEAR_AW {
    #[doc = "0: WDOG timer is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: WDOG timer is cleared to 0."]
    CLEARED = 1,
}
impl From<CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLEAR` writer - WDOG Timer Clear"]
pub type CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, CLEAR_AW, O>;
impl<'a, const O: u8> CLEAR_W<'a, O> {
    #[doc = "WDOG timer is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(CLEAR_AW::UNCHANGED)
    }
    #[doc = "WDOG timer is cleared to 0."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLEAR_AW::CLEARED)
    }
}
impl W {
    #[doc = "Bit 0 - WDOG Timer Clear"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<0> {
        CLEAR_W::new(self)
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
