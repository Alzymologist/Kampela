#[doc = "Register `NSLOCK` writer"]
pub struct W(crate::W<NSLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSLOCK_SPEC>;
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
impl From<crate::W<NSLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "No Description\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SMUNSLOCKKEY_AW {
    #[doc = "11325013: Unlocks Registers"]
    UNLOCK = 11325013,
}
impl From<SMUNSLOCKKEY_AW> for u32 {
    #[inline(always)]
    fn from(variant: SMUNSLOCKKEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `SMUNSLOCKKEY` writer - No Description"]
pub type SMUNSLOCKKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NSLOCK_SPEC, u32, SMUNSLOCKKEY_AW, 24, O>;
impl<'a, const O: u8> SMUNSLOCKKEY_W<'a, O> {
    #[doc = "Unlocks Registers"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(SMUNSLOCKKEY_AW::UNLOCK)
    }
}
impl W {
    #[doc = "Bits 0:23 - No Description"]
    #[inline(always)]
    #[must_use]
    pub fn smunslockkey(&mut self) -> SMUNSLOCKKEY_W<0> {
        SMUNSLOCKKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nslock](index.html) module"]
pub struct NSLOCK_SPEC;
impl crate::RegisterSpec for NSLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [nslock::W](W) writer structure"]
impl crate::Writable for NSLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSLOCK to value 0"]
impl crate::Resettable for NSLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
