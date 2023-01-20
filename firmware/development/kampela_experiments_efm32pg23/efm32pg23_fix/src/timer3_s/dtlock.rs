#[doc = "Register `DTLOCK` writer"]
pub struct W(crate::W<DTLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTLOCK_SPEC>;
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
impl From<crate::W<DTLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DTI Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DTILOCKKEY_AW {
    #[doc = "52864: Write to unlock TIMER DTI registers"]
    UNLOCK = 52864,
}
impl From<DTILOCKKEY_AW> for u16 {
    #[inline(always)]
    fn from(variant: DTILOCKKEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `DTILOCKKEY` writer - DTI Lock Key"]
pub type DTILOCKKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DTLOCK_SPEC, u16, DTILOCKKEY_AW, 16, O>;
impl<'a, const O: u8> DTILOCKKEY_W<'a, O> {
    #[doc = "Write to unlock TIMER DTI registers"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(DTILOCKKEY_AW::UNLOCK)
    }
}
impl W {
    #[doc = "Bits 0:15 - DTI Lock Key"]
    #[inline(always)]
    #[must_use]
    pub fn dtilockkey(&mut self) -> DTILOCKKEY_W<0> {
        DTILOCKKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtlock](index.html) module"]
pub struct DTLOCK_SPEC;
impl crate::RegisterSpec for DTLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dtlock::W](W) writer structure"]
impl crate::Writable for DTLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTLOCK to value 0"]
impl crate::Resettable for DTLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
