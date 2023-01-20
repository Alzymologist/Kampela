#[doc = "Register `BUFOUTTRIM` reader"]
pub struct R(crate::R<BUFOUTTRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFOUTTRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFOUTTRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFOUTTRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUFOUTTRIM` writer"]
pub struct W(crate::W<BUFOUTTRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFOUTTRIM_SPEC>;
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
impl From<crate::W<BUFOUTTRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFOUTTRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTRTRIMANA` reader - BUFOUT Reference Trim"]
pub type VTRTRIMANA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VTRTRIMANA` writer - BUFOUT Reference Trim"]
pub type VTRTRIMANA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUFOUTTRIM_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - BUFOUT Reference Trim"]
    #[inline(always)]
    pub fn vtrtrimana(&self) -> VTRTRIMANA_R {
        VTRTRIMANA_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - BUFOUT Reference Trim"]
    #[inline(always)]
    #[must_use]
    pub fn vtrtrimana(&mut self) -> VTRTRIMANA_W<0> {
        VTRTRIMANA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bufouttrim](index.html) module"]
pub struct BUFOUTTRIM_SPEC;
impl crate::RegisterSpec for BUFOUTTRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bufouttrim::R](R) reader structure"]
impl crate::Readable for BUFOUTTRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bufouttrim::W](W) writer structure"]
impl crate::Writable for BUFOUTTRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUFOUTTRIM to value 0x08"]
impl crate::Resettable for BUFOUTTRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
