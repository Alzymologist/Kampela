#[doc = "Register `ESAURTYPES0` reader"]
pub struct R(crate::R<ESAURTYPES0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESAURTYPES0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESAURTYPES0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESAURTYPES0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESAURTYPES0` writer"]
pub struct W(crate::W<ESAURTYPES0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESAURTYPES0_SPEC>;
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
impl From<crate::W<ESAURTYPES0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESAURTYPES0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESAUR3NS` reader - Region 3 Non-Secure"]
pub type ESAUR3NS_R = crate::BitReader<bool>;
#[doc = "Field `ESAUR3NS` writer - Region 3 Non-Secure"]
pub type ESAUR3NS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESAURTYPES0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - Region 3 Non-Secure"]
    #[inline(always)]
    pub fn esaur3ns(&self) -> ESAUR3NS_R {
        ESAUR3NS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Region 3 Non-Secure"]
    #[inline(always)]
    #[must_use]
    pub fn esaur3ns(&mut self) -> ESAUR3NS_W<12> {
        ESAUR3NS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esaurtypes0](index.html) module"]
pub struct ESAURTYPES0_SPEC;
impl crate::RegisterSpec for ESAURTYPES0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esaurtypes0::R](R) reader structure"]
impl crate::Readable for ESAURTYPES0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esaurtypes0::W](W) writer structure"]
impl crate::Writable for ESAURTYPES0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESAURTYPES0 to value 0"]
impl crate::Resettable for ESAURTYPES0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
