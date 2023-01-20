#[doc = "Register `CFGSYSTIC` reader"]
pub struct R(crate::R<CFGSYSTIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGSYSTIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGSYSTIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGSYSTIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGSYSTIC` writer"]
pub struct W(crate::W<CFGSYSTIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGSYSTIC_SPEC>;
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
impl From<crate::W<CFGSYSTIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGSYSTIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSTICEXTCLKEN` reader - SysTick External Clock Enable"]
pub type SYSTICEXTCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `SYSTICEXTCLKEN` writer - SysTick External Clock Enable"]
pub type SYSTICEXTCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGSYSTIC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SysTick External Clock Enable"]
    #[inline(always)]
    pub fn systicextclken(&self) -> SYSTICEXTCLKEN_R {
        SYSTICEXTCLKEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SysTick External Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn systicextclken(&mut self) -> SYSTICEXTCLKEN_W<0> {
        SYSTICEXTCLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure the source of the system tick for the M33.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgsystic](index.html) module"]
pub struct CFGSYSTIC_SPEC;
impl crate::RegisterSpec for CFGSYSTIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgsystic::R](R) reader structure"]
impl crate::Readable for CFGSYSTIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgsystic::W](W) writer structure"]
impl crate::Writable for CFGSYSTIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGSYSTIC to value 0"]
impl crate::Resettable for CFGSYSTIC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
