#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP0` reader - Compare Match 0 Interrupt Enable"]
pub type COMP0_R = crate::BitReader<bool>;
#[doc = "Field `COMP0` writer - Compare Match 0 Interrupt Enable"]
pub type COMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `COMP1` reader - Compare Match 1 Interrupt Enable"]
pub type COMP1_R = crate::BitReader<bool>;
#[doc = "Field `COMP1` writer - Compare Match 1 Interrupt Enable"]
pub type COMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `UF` reader - Underflow Interrupt Enable"]
pub type UF_R = crate::BitReader<bool>;
#[doc = "Field `UF` writer - Underflow Interrupt Enable"]
pub type UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `REP0` reader - Repeat Counter 0 Interrupt Enable"]
pub type REP0_R = crate::BitReader<bool>;
#[doc = "Field `REP0` writer - Repeat Counter 0 Interrupt Enable"]
pub type REP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `REP1` reader - Repeat Counter 1 Interrupt Enable"]
pub type REP1_R = crate::BitReader<bool>;
#[doc = "Field `REP1` writer - Repeat Counter 1 Interrupt Enable"]
pub type REP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Compare Match 0 Interrupt Enable"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Match 1 Interrupt Enable"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Repeat Counter 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rep0(&self) -> REP0_R {
        REP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Repeat Counter 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rep1(&self) -> REP1_R {
        REP1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Match 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> COMP0_W<0> {
        COMP0_W::new(self)
    }
    #[doc = "Bit 1 - Compare Match 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> COMP1_W<1> {
        COMP1_W::new(self)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<2> {
        UF_W::new(self)
    }
    #[doc = "Bit 3 - Repeat Counter 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rep0(&mut self) -> REP0_W<3> {
        REP0_W::new(self)
    }
    #[doc = "Bit 4 - Repeat Counter 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rep1(&mut self) -> REP1_W<4> {
        REP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
