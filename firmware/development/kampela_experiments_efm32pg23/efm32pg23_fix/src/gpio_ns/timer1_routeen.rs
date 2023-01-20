#[doc = "Register `TIMER1_ROUTEEN` reader"]
pub struct R(crate::R<TIMER1_ROUTEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_ROUTEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_ROUTEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_ROUTEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_ROUTEEN` writer"]
pub struct W(crate::W<TIMER1_ROUTEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_ROUTEEN_SPEC>;
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
impl From<crate::W<TIMER1_ROUTEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_ROUTEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC0PEN` reader - CC0 pin enable control bit"]
pub type CC0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CC0PEN` writer - CC0 pin enable control bit"]
pub type CC0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER1_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CC1PEN` reader - CC1 pin enable control bit"]
pub type CC1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CC1PEN` writer - CC1 pin enable control bit"]
pub type CC1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER1_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CC2PEN` reader - CC2 pin enable control bit"]
pub type CC2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CC2PEN` writer - CC2 pin enable control bit"]
pub type CC2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER1_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CCC0PEN` reader - CDTI0 pin enable control bit"]
pub type CCC0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CCC0PEN` writer - CDTI0 pin enable control bit"]
pub type CCC0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER1_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CCC1PEN` reader - CDTI1 pin enable control bit"]
pub type CCC1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CCC1PEN` writer - CDTI1 pin enable control bit"]
pub type CCC1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER1_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CCC2PEN` reader - CDTI2 pin enable control bit"]
pub type CCC2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CCC2PEN` writer - CDTI2 pin enable control bit"]
pub type CCC2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER1_ROUTEEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CC0 pin enable control bit"]
    #[inline(always)]
    pub fn cc0pen(&self) -> CC0PEN_R {
        CC0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1 pin enable control bit"]
    #[inline(always)]
    pub fn cc1pen(&self) -> CC1PEN_R {
        CC1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC2 pin enable control bit"]
    #[inline(always)]
    pub fn cc2pen(&self) -> CC2PEN_R {
        CC2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CDTI0 pin enable control bit"]
    #[inline(always)]
    pub fn ccc0pen(&self) -> CCC0PEN_R {
        CCC0PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CDTI1 pin enable control bit"]
    #[inline(always)]
    pub fn ccc1pen(&self) -> CCC1PEN_R {
        CCC1PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CDTI2 pin enable control bit"]
    #[inline(always)]
    pub fn ccc2pen(&self) -> CCC2PEN_R {
        CCC2PEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC0 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn cc0pen(&mut self) -> CC0PEN_W<0> {
        CC0PEN_W::new(self)
    }
    #[doc = "Bit 1 - CC1 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn cc1pen(&mut self) -> CC1PEN_W<1> {
        CC1PEN_W::new(self)
    }
    #[doc = "Bit 2 - CC2 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn cc2pen(&mut self) -> CC2PEN_W<2> {
        CC2PEN_W::new(self)
    }
    #[doc = "Bit 3 - CDTI0 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ccc0pen(&mut self) -> CCC0PEN_W<3> {
        CCC0PEN_W::new(self)
    }
    #[doc = "Bit 4 - CDTI1 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ccc1pen(&mut self) -> CCC1PEN_W<4> {
        CCC1PEN_W::new(self)
    }
    #[doc = "Bit 5 - CDTI2 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ccc2pen(&mut self) -> CCC2PEN_W<5> {
        CCC2PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMER1 pin enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_routeen](index.html) module"]
pub struct TIMER1_ROUTEEN_SPEC;
impl crate::RegisterSpec for TIMER1_ROUTEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_routeen::R](R) reader structure"]
impl crate::Readable for TIMER1_ROUTEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_routeen::W](W) writer structure"]
impl crate::Writable for TIMER1_ROUTEEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER1_ROUTEEN to value 0"]
impl crate::Resettable for TIMER1_ROUTEEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
