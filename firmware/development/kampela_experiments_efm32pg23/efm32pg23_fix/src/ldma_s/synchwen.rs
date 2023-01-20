#[doc = "Register `SYNCHWEN` reader"]
pub struct R(crate::R<SYNCHWEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCHWEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCHWEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCHWEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCHWEN` writer"]
pub struct W(crate::W<SYNCHWEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCHWEN_SPEC>;
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
impl From<crate::W<SYNCHWEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCHWEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCSETEN` reader - Hardware Sync Trigger Set Enable"]
pub type SYNCSETEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCSETEN` writer - Hardware Sync Trigger Set Enable"]
pub type SYNCSETEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNCHWEN_SPEC, u8, u8, 8, O>;
#[doc = "Field `SYNCCLREN` reader - Hardware Sync Trigger Clear Enable"]
pub type SYNCCLREN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCCLREN` writer - Hardware Sync Trigger Clear Enable"]
pub type SYNCCLREN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNCHWEN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Hardware Sync Trigger Set Enable"]
    #[inline(always)]
    pub fn syncseten(&self) -> SYNCSETEN_R {
        SYNCSETEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Hardware Sync Trigger Clear Enable"]
    #[inline(always)]
    pub fn syncclren(&self) -> SYNCCLREN_R {
        SYNCCLREN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Hardware Sync Trigger Set Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncseten(&mut self) -> SYNCSETEN_W<0> {
        SYNCSETEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Hardware Sync Trigger Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncclren(&mut self) -> SYNCCLREN_W<16> {
        SYNCCLREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [synchwen](index.html) module"]
pub struct SYNCHWEN_SPEC;
impl crate::RegisterSpec for SYNCHWEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [synchwen::R](R) reader structure"]
impl crate::Readable for SYNCHWEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [synchwen::W](W) writer structure"]
impl crate::Writable for SYNCHWEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCHWEN to value 0"]
impl crate::Resettable for SYNCHWEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
