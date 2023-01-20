#[doc = "Register `RDATACTRL` reader"]
pub struct R(crate::R<RDATACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDATACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDATACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDATACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDATACTRL` writer"]
pub struct W(crate::W<RDATACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDATACTRL_SPEC>;
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
impl From<crate::W<RDATACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDATACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFDIS` reader - Automatic Invalidate Disable"]
pub type AFDIS_R = crate::BitReader<bool>;
#[doc = "Field `AFDIS` writer - Automatic Invalidate Disable"]
pub type AFDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RDATACTRL_SPEC, bool, O>;
#[doc = "Field `DOUTBUFEN` reader - Flash dout pipeline buffer enable"]
pub type DOUTBUFEN_R = crate::BitReader<bool>;
#[doc = "Field `DOUTBUFEN` writer - Flash dout pipeline buffer enable"]
pub type DOUTBUFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RDATACTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn afdis(&self) -> AFDIS_R {
        AFDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 12 - Flash dout pipeline buffer enable"]
    #[inline(always)]
    pub fn doutbufen(&self) -> DOUTBUFEN_R {
        DOUTBUFEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Automatic Invalidate Disable"]
    #[inline(always)]
    #[must_use]
    pub fn afdis(&mut self) -> AFDIS_W<1> {
        AFDIS_W::new(self)
    }
    #[doc = "Bit 12 - Flash dout pipeline buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn doutbufen(&mut self) -> DOUTBUFEN_W<12> {
        DOUTBUFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdatactrl](index.html) module"]
pub struct RDATACTRL_SPEC;
impl crate::RegisterSpec for RDATACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdatactrl::R](R) reader structure"]
impl crate::Readable for RDATACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdatactrl::W](W) writer structure"]
impl crate::Writable for RDATACTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDATACTRL to value 0x1000"]
impl crate::Resettable for RDATACTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
