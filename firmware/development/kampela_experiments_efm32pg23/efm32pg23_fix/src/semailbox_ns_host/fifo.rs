#[doc = "Register `FIFO` reader"]
pub struct R(crate::R<FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO` writer"]
pub struct W(crate::W<FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_SPEC>;
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
impl From<crate::W<FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO` reader - FIFO"]
pub type FIFO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIFO` writer - FIFO"]
pub type FIFO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - FIFO"]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn fifo(&mut self) -> FIFO_W<0> {
        FIFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A write access to any address in this area will be mapped to the TX FIFO (only for the payload). A read access to any address in this area will be mapped to the RX FIFO (only for the payload). Using an address range (16 x 32-bit) rather than one single address mapped to the FIFO allows using incremental bursts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](index.html) module"]
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo::R](R) reader structure"]
impl crate::Readable for FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo::W](W) writer structure"]
impl crate::Writable for FIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO to value 0"]
impl crate::Resettable for FIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
