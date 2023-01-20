#[doc = "Register `CHDONE` reader"]
pub struct R(crate::R<CHDONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHDONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHDONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHDONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHDONE` writer"]
pub struct W(crate::W<CHDONE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHDONE_SPEC>;
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
impl From<crate::W<CHDONE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHDONE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHDONE0` reader - DMA Channel Link done intr flag"]
pub type CHDONE0_R = crate::BitReader<bool>;
#[doc = "Field `CHDONE0` writer - DMA Channel Link done intr flag"]
pub type CHDONE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDONE_SPEC, bool, O>;
#[doc = "Field `CHDONE1` reader - DMA Channel Link done intr flag"]
pub type CHDONE1_R = crate::BitReader<bool>;
#[doc = "Field `CHDONE1` writer - DMA Channel Link done intr flag"]
pub type CHDONE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDONE_SPEC, bool, O>;
#[doc = "Field `CHDONE2` reader - DMA Channel Link done intr flag"]
pub type CHDONE2_R = crate::BitReader<bool>;
#[doc = "Field `CHDONE2` writer - DMA Channel Link done intr flag"]
pub type CHDONE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDONE_SPEC, bool, O>;
#[doc = "Field `CHDONE3` reader - DMA Channel Link done intr flag"]
pub type CHDONE3_R = crate::BitReader<bool>;
#[doc = "Field `CHDONE3` writer - DMA Channel Link done intr flag"]
pub type CHDONE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDONE_SPEC, bool, O>;
#[doc = "Field `CHDONE4` reader - DMA Channel Link done intr flag"]
pub type CHDONE4_R = crate::BitReader<bool>;
#[doc = "Field `CHDONE4` writer - DMA Channel Link done intr flag"]
pub type CHDONE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDONE_SPEC, bool, O>;
#[doc = "Field `CHDONE5` reader - DMA Channel Link done intr flag"]
pub type CHDONE5_R = crate::BitReader<bool>;
#[doc = "Field `CHDONE5` writer - DMA Channel Link done intr flag"]
pub type CHDONE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDONE_SPEC, bool, O>;
#[doc = "Field `CHDONE6` reader - DMA Channel Link done intr flag"]
pub type CHDONE6_R = crate::BitReader<bool>;
#[doc = "Field `CHDONE6` writer - DMA Channel Link done intr flag"]
pub type CHDONE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDONE_SPEC, bool, O>;
#[doc = "Field `CHDONE7` reader - DMA Channel Link done intr flag"]
pub type CHDONE7_R = crate::BitReader<bool>;
#[doc = "Field `CHDONE7` writer - DMA Channel Link done intr flag"]
pub type CHDONE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDONE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone0(&self) -> CHDONE0_R {
        CHDONE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone1(&self) -> CHDONE1_R {
        CHDONE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone2(&self) -> CHDONE2_R {
        CHDONE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone3(&self) -> CHDONE3_R {
        CHDONE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone4(&self) -> CHDONE4_R {
        CHDONE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone5(&self) -> CHDONE5_R {
        CHDONE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone6(&self) -> CHDONE6_R {
        CHDONE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone7(&self) -> CHDONE7_R {
        CHDONE7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel Link done intr flag"]
    #[inline(always)]
    #[must_use]
    pub fn chdone0(&mut self) -> CHDONE0_W<0> {
        CHDONE0_W::new(self)
    }
    #[doc = "Bit 1 - DMA Channel Link done intr flag"]
    #[inline(always)]
    #[must_use]
    pub fn chdone1(&mut self) -> CHDONE1_W<1> {
        CHDONE1_W::new(self)
    }
    #[doc = "Bit 2 - DMA Channel Link done intr flag"]
    #[inline(always)]
    #[must_use]
    pub fn chdone2(&mut self) -> CHDONE2_W<2> {
        CHDONE2_W::new(self)
    }
    #[doc = "Bit 3 - DMA Channel Link done intr flag"]
    #[inline(always)]
    #[must_use]
    pub fn chdone3(&mut self) -> CHDONE3_W<3> {
        CHDONE3_W::new(self)
    }
    #[doc = "Bit 4 - DMA Channel Link done intr flag"]
    #[inline(always)]
    #[must_use]
    pub fn chdone4(&mut self) -> CHDONE4_W<4> {
        CHDONE4_W::new(self)
    }
    #[doc = "Bit 5 - DMA Channel Link done intr flag"]
    #[inline(always)]
    #[must_use]
    pub fn chdone5(&mut self) -> CHDONE5_W<5> {
        CHDONE5_W::new(self)
    }
    #[doc = "Bit 6 - DMA Channel Link done intr flag"]
    #[inline(always)]
    #[must_use]
    pub fn chdone6(&mut self) -> CHDONE6_W<6> {
        CHDONE6_W::new(self)
    }
    #[doc = "Bit 7 - DMA Channel Link done intr flag"]
    #[inline(always)]
    #[must_use]
    pub fn chdone7(&mut self) -> CHDONE7_W<7> {
        CHDONE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdone](index.html) module"]
pub struct CHDONE_SPEC;
impl crate::RegisterSpec for CHDONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chdone::R](R) reader structure"]
impl crate::Readable for CHDONE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chdone::W](W) writer structure"]
impl crate::Writable for CHDONE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHDONE to value 0"]
impl crate::Resettable for CHDONE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
