#[doc = "Register `LOCK` writer"]
pub struct W(crate::W<LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCK_SPEC>;
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
impl From<crate::W<LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configuration Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum LETIMERLOCKKEY_AW {
    #[doc = "52476: Write to unock LETIMER lockable registers"]
    UNLOCK = 52476,
}
impl From<LETIMERLOCKKEY_AW> for u16 {
    #[inline(always)]
    fn from(variant: LETIMERLOCKKEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `LETIMERLOCKKEY` writer - Configuration Lock Key"]
pub type LETIMERLOCKKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LOCK_SPEC, u16, LETIMERLOCKKEY_AW, 16, O>;
impl<'a, const O: u8> LETIMERLOCKKEY_W<'a, O> {
    #[doc = "Write to unock LETIMER lockable registers"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LETIMERLOCKKEY_AW::UNLOCK)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configuration Lock Key"]
    #[inline(always)]
    #[must_use]
    pub fn letimerlockkey(&mut self) -> LETIMERLOCKKEY_W<0> {
        LETIMERLOCKKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](index.html) module"]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lock::W](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
