#[doc = "Register `BMPUSATD0` reader"]
pub struct R(crate::R<BMPUSATD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMPUSATD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMPUSATD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMPUSATD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMPUSATD0` writer"]
pub struct W(crate::W<BMPUSATD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMPUSATD0_SPEC>;
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
impl From<crate::W<BMPUSATD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMPUSATD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDMA` reader - MCU LDMA secure mode"]
pub type LDMA_R = crate::BitReader<bool>;
#[doc = "Field `LDMA` writer - MCU LDMA secure mode"]
pub type LDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMPUSATD0_SPEC, bool, O>;
#[doc = "Field `SEEXTDMA` reader - SEEXTDMA secure mode"]
pub type SEEXTDMA_R = crate::BitReader<bool>;
#[doc = "Field `SEEXTDMA` writer - SEEXTDMA secure mode"]
pub type SEEXTDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMPUSATD0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - MCU LDMA secure mode"]
    #[inline(always)]
    pub fn ldma(&self) -> LDMA_R {
        LDMA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - SEEXTDMA secure mode"]
    #[inline(always)]
    pub fn seextdma(&self) -> SEEXTDMA_R {
        SEEXTDMA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - MCU LDMA secure mode"]
    #[inline(always)]
    #[must_use]
    pub fn ldma(&mut self) -> LDMA_W<2> {
        LDMA_W::new(self)
    }
    #[doc = "Bit 5 - SEEXTDMA secure mode"]
    #[inline(always)]
    #[must_use]
    pub fn seextdma(&mut self) -> SEEXTDMA_W<5> {
        SEEXTDMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set master bits to 1 to mark as a secure master\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmpusatd0](index.html) module"]
pub struct BMPUSATD0_SPEC;
impl crate::RegisterSpec for BMPUSATD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmpusatd0::R](R) reader structure"]
impl crate::Readable for BMPUSATD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmpusatd0::W](W) writer structure"]
impl crate::Writable for BMPUSATD0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMPUSATD0 to value 0x3f"]
impl crate::Resettable for BMPUSATD0_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
