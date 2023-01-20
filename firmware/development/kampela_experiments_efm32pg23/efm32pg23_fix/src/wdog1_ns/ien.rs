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
#[doc = "Field `TOUT` reader - WDOG Timeout Interrupt Enable"]
pub type TOUT_R = crate::BitReader<bool>;
#[doc = "Field `TOUT` writer - WDOG Timeout Interrupt Enable"]
pub type TOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `WARN` reader - WDOG Warning Timeout Interrupt Enable"]
pub type WARN_R = crate::BitReader<bool>;
#[doc = "Field `WARN` writer - WDOG Warning Timeout Interrupt Enable"]
pub type WARN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `WIN` reader - WDOG Window Interrupt Enable"]
pub type WIN_R = crate::BitReader<bool>;
#[doc = "Field `WIN` writer - WDOG Window Interrupt Enable"]
pub type WIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `PEM0` reader - PRS Src0 Event Missing Interrupt Enable"]
pub type PEM0_R = crate::BitReader<bool>;
#[doc = "Field `PEM0` writer - PRS Src0 Event Missing Interrupt Enable"]
pub type PEM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `PEM1` reader - PRS Src1 Event Missing Interrupt Enable"]
pub type PEM1_R = crate::BitReader<bool>;
#[doc = "Field `PEM1` writer - PRS Src1 Event Missing Interrupt Enable"]
pub type PEM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - WDOG Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDOG Warning Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn warn(&self) -> WARN_R {
        WARN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDOG Window Interrupt Enable"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PRS Src0 Event Missing Interrupt Enable"]
    #[inline(always)]
    pub fn pem0(&self) -> PEM0_R {
        PEM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PRS Src1 Event Missing Interrupt Enable"]
    #[inline(always)]
    pub fn pem1(&self) -> PEM1_R {
        PEM1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDOG Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> TOUT_W<0> {
        TOUT_W::new(self)
    }
    #[doc = "Bit 1 - WDOG Warning Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn warn(&mut self) -> WARN_W<1> {
        WARN_W::new(self)
    }
    #[doc = "Bit 2 - WDOG Window Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WIN_W<2> {
        WIN_W::new(self)
    }
    #[doc = "Bit 3 - PRS Src0 Event Missing Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pem0(&mut self) -> PEM0_W<3> {
        PEM0_W::new(self)
    }
    #[doc = "Bit 4 - PRS Src1 Event Missing Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pem1(&mut self) -> PEM1_W<4> {
        PEM1_W::new(self)
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
