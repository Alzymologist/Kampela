#[doc = "Register `WRITECTRL` reader"]
pub struct R(crate::R<WRITECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRITECTRL` writer"]
pub struct W(crate::W<WRITECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITECTRL_SPEC>;
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
impl From<crate::W<WRITECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WREN` reader - Enable Write/Erase Controller"]
pub type WREN_R = crate::BitReader<bool>;
#[doc = "Field `WREN` writer - Enable Write/Erase Controller"]
pub type WREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECTRL_SPEC, bool, O>;
#[doc = "Field `IRQERASEABORT` reader - Abort Page Erase on Interrupt"]
pub type IRQERASEABORT_R = crate::BitReader<bool>;
#[doc = "Field `IRQERASEABORT` writer - Abort Page Erase on Interrupt"]
pub type IRQERASEABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECTRL_SPEC, bool, O>;
#[doc = "Field `LPWRITE` reader - Low-Power Write"]
pub type LPWRITE_R = crate::BitReader<bool>;
#[doc = "Field `LPWRITE` writer - Low-Power Write"]
pub type LPWRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECTRL_SPEC, bool, O>;
#[doc = "Field `RANGECOUNT` reader - EraseRange Count"]
pub type RANGECOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RANGECOUNT` writer - EraseRange Count"]
pub type RANGECOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRITECTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    pub fn irqeraseabort(&self) -> IRQERASEABORT_R {
        IRQERASEABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Low-Power Write"]
    #[inline(always)]
    pub fn lpwrite(&self) -> LPWRITE_R {
        LPWRITE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:23 - EraseRange Count"]
    #[inline(always)]
    pub fn rangecount(&self) -> RANGECOUNT_R {
        RANGECOUNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<0> {
        WREN_W::new(self)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn irqeraseabort(&mut self) -> IRQERASEABORT_W<1> {
        IRQERASEABORT_W::new(self)
    }
    #[doc = "Bit 3 - Low-Power Write"]
    #[inline(always)]
    #[must_use]
    pub fn lpwrite(&mut self) -> LPWRITE_W<3> {
        LPWRITE_W::new(self)
    }
    #[doc = "Bits 16:23 - EraseRange Count"]
    #[inline(always)]
    #[must_use]
    pub fn rangecount(&mut self) -> RANGECOUNT_W<16> {
        RANGECOUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writectrl](index.html) module"]
pub struct WRITECTRL_SPEC;
impl crate::RegisterSpec for WRITECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [writectrl::R](R) reader structure"]
impl crate::Readable for WRITECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [writectrl::W](W) writer structure"]
impl crate::Writable for WRITECTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRITECTRL to value 0"]
impl crate::Resettable for WRITECTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
