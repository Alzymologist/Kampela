#[doc = "Register `DTOGEN` reader"]
pub struct R(crate::R<DTOGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTOGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTOGEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTOGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTOGEN` writer"]
pub struct W(crate::W<DTOGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTOGEN_SPEC>;
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
impl From<crate::W<DTOGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTOGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTOGCC0EN` reader - DTI CCn Output Generation Enable"]
pub type DTOGCC0EN_R = crate::BitReader<bool>;
#[doc = "Field `DTOGCC0EN` writer - DTI CCn Output Generation Enable"]
pub type DTOGCC0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTOGEN_SPEC, bool, O>;
#[doc = "Field `DTOGCC1EN` reader - DTI CCn Output Generation Enable"]
pub type DTOGCC1EN_R = crate::BitReader<bool>;
#[doc = "Field `DTOGCC1EN` writer - DTI CCn Output Generation Enable"]
pub type DTOGCC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTOGEN_SPEC, bool, O>;
#[doc = "Field `DTOGCC2EN` reader - DTI CCn Output Generation Enable"]
pub type DTOGCC2EN_R = crate::BitReader<bool>;
#[doc = "Field `DTOGCC2EN` writer - DTI CCn Output Generation Enable"]
pub type DTOGCC2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTOGEN_SPEC, bool, O>;
#[doc = "Field `DTOGCDTI0EN` reader - DTI CDTIn Output Generation Enable"]
pub type DTOGCDTI0EN_R = crate::BitReader<bool>;
#[doc = "Field `DTOGCDTI0EN` writer - DTI CDTIn Output Generation Enable"]
pub type DTOGCDTI0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTOGEN_SPEC, bool, O>;
#[doc = "Field `DTOGCDTI1EN` reader - DTI CDTIn Output Generation Enable"]
pub type DTOGCDTI1EN_R = crate::BitReader<bool>;
#[doc = "Field `DTOGCDTI1EN` writer - DTI CDTIn Output Generation Enable"]
pub type DTOGCDTI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTOGEN_SPEC, bool, O>;
#[doc = "Field `DTOGCDTI2EN` reader - DTI CDTIn Output Generation Enable"]
pub type DTOGCDTI2EN_R = crate::BitReader<bool>;
#[doc = "Field `DTOGCDTI2EN` writer - DTI CDTIn Output Generation Enable"]
pub type DTOGCDTI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTOGEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DTI CCn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc0en(&self) -> DTOGCC0EN_R {
        DTOGCC0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI CCn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc1en(&self) -> DTOGCC1EN_R {
        DTOGCC1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTI CCn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc2en(&self) -> DTOGCC2EN_R {
        DTOGCC2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTI CDTIn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti0en(&self) -> DTOGCDTI0EN_R {
        DTOGCDTI0EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DTI CDTIn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti1en(&self) -> DTOGCDTI1EN_R {
        DTOGCDTI1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DTI CDTIn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti2en(&self) -> DTOGCDTI2EN_R {
        DTOGCDTI2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI CCn Output Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtogcc0en(&mut self) -> DTOGCC0EN_W<0> {
        DTOGCC0EN_W::new(self)
    }
    #[doc = "Bit 1 - DTI CCn Output Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtogcc1en(&mut self) -> DTOGCC1EN_W<1> {
        DTOGCC1EN_W::new(self)
    }
    #[doc = "Bit 2 - DTI CCn Output Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtogcc2en(&mut self) -> DTOGCC2EN_W<2> {
        DTOGCC2EN_W::new(self)
    }
    #[doc = "Bit 3 - DTI CDTIn Output Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtogcdti0en(&mut self) -> DTOGCDTI0EN_W<3> {
        DTOGCDTI0EN_W::new(self)
    }
    #[doc = "Bit 4 - DTI CDTIn Output Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtogcdti1en(&mut self) -> DTOGCDTI1EN_W<4> {
        DTOGCDTI1EN_W::new(self)
    }
    #[doc = "Bit 5 - DTI CDTIn Output Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtogcdti2en(&mut self) -> DTOGCDTI2EN_W<5> {
        DTOGCDTI2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtogen](index.html) module"]
pub struct DTOGEN_SPEC;
impl crate::RegisterSpec for DTOGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtogen::R](R) reader structure"]
impl crate::Readable for DTOGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtogen::W](W) writer structure"]
impl crate::Writable for DTOGEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTOGEN to value 0"]
impl crate::Resettable for DTOGEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
