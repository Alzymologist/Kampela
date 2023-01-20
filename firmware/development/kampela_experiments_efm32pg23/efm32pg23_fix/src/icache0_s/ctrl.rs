#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHEDIS` reader - Cache Disable"]
pub type CACHEDIS_R = crate::BitReader<bool>;
#[doc = "Field `CACHEDIS` writer - Cache Disable"]
pub type CACHEDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `USEMPU` reader - Use MPU"]
pub type USEMPU_R = crate::BitReader<bool>;
#[doc = "Field `USEMPU` writer - Use MPU"]
pub type USEMPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `AUTOFLUSHDIS` reader - Automatic Flushing Disable"]
pub type AUTOFLUSHDIS_R = crate::BitReader<bool>;
#[doc = "Field `AUTOFLUSHDIS` writer - Automatic Flushing Disable"]
pub type AUTOFLUSHDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Cache Disable"]
    #[inline(always)]
    pub fn cachedis(&self) -> CACHEDIS_R {
        CACHEDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use MPU"]
    #[inline(always)]
    pub fn usempu(&self) -> USEMPU_R {
        USEMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic Flushing Disable"]
    #[inline(always)]
    pub fn autoflushdis(&self) -> AUTOFLUSHDIS_R {
        AUTOFLUSHDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cachedis(&mut self) -> CACHEDIS_W<0> {
        CACHEDIS_W::new(self)
    }
    #[doc = "Bit 1 - Use MPU"]
    #[inline(always)]
    #[must_use]
    pub fn usempu(&mut self) -> USEMPU_W<1> {
        USEMPU_W::new(self)
    }
    #[doc = "Bit 2 - Automatic Flushing Disable"]
    #[inline(always)]
    #[must_use]
    pub fn autoflushdis(&mut self) -> AUTOFLUSHDIS_W<2> {
        AUTOFLUSHDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
