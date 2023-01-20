#[doc = "Register `PWRCTRL` reader"]
pub struct R(crate::R<PWRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCTRL` writer"]
pub struct W(crate::W<PWRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCTRL_SPEC>;
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
impl From<crate::W<PWRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWROFFONEM1ENTRY` reader - Power down Flash macro when enter EM1"]
pub type PWROFFONEM1ENTRY_R = crate::BitReader<bool>;
#[doc = "Field `PWROFFONEM1ENTRY` writer - Power down Flash macro when enter EM1"]
pub type PWROFFONEM1ENTRY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCTRL_SPEC, bool, O>;
#[doc = "Field `PWROFFONEM1PENTRY` reader - Power down Flash macro when enter EM1P"]
pub type PWROFFONEM1PENTRY_R = crate::BitReader<bool>;
#[doc = "Field `PWROFFONEM1PENTRY` writer - Power down Flash macro when enter EM1P"]
pub type PWROFFONEM1PENTRY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCTRL_SPEC, bool, O>;
#[doc = "Field `PWROFFENTRYAGAIN` reader - POWER down flash again in EM1/EM1p"]
pub type PWROFFENTRYAGAIN_R = crate::BitReader<bool>;
#[doc = "Field `PWROFFENTRYAGAIN` writer - POWER down flash again in EM1/EM1p"]
pub type PWROFFENTRYAGAIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCTRL_SPEC, bool, O>;
#[doc = "Field `PWROFFDLY` reader - Power down delay"]
pub type PWROFFDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWROFFDLY` writer - Power down delay"]
pub type PWROFFDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWRCTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Power down Flash macro when enter EM1"]
    #[inline(always)]
    pub fn pwroffonem1entry(&self) -> PWROFFONEM1ENTRY_R {
        PWROFFONEM1ENTRY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down Flash macro when enter EM1P"]
    #[inline(always)]
    pub fn pwroffonem1pentry(&self) -> PWROFFONEM1PENTRY_R {
        PWROFFONEM1PENTRY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - POWER down flash again in EM1/EM1p"]
    #[inline(always)]
    pub fn pwroffentryagain(&self) -> PWROFFENTRYAGAIN_R {
        PWROFFENTRYAGAIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Power down delay"]
    #[inline(always)]
    pub fn pwroffdly(&self) -> PWROFFDLY_R {
        PWROFFDLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Power down Flash macro when enter EM1"]
    #[inline(always)]
    #[must_use]
    pub fn pwroffonem1entry(&mut self) -> PWROFFONEM1ENTRY_W<0> {
        PWROFFONEM1ENTRY_W::new(self)
    }
    #[doc = "Bit 1 - Power down Flash macro when enter EM1P"]
    #[inline(always)]
    #[must_use]
    pub fn pwroffonem1pentry(&mut self) -> PWROFFONEM1PENTRY_W<1> {
        PWROFFONEM1PENTRY_W::new(self)
    }
    #[doc = "Bit 4 - POWER down flash again in EM1/EM1p"]
    #[inline(always)]
    #[must_use]
    pub fn pwroffentryagain(&mut self) -> PWROFFENTRYAGAIN_W<4> {
        PWROFFENTRYAGAIN_W::new(self)
    }
    #[doc = "Bits 16:23 - Power down delay"]
    #[inline(always)]
    #[must_use]
    pub fn pwroffdly(&mut self) -> PWROFFDLY_W<16> {
        PWROFFDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctrl](index.html) module"]
pub struct PWRCTRL_SPEC;
impl crate::RegisterSpec for PWRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrctrl::R](R) reader structure"]
impl crate::Readable for PWRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrctrl::W](W) writer structure"]
impl crate::Writable for PWRCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCTRL to value 0x0010_0002"]
impl crate::Resettable for PWRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0002;
}
