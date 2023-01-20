#[doc = "Register `M33CTRL` reader"]
pub struct R(crate::R<M33CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M33CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M33CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M33CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M33CTRL` writer"]
pub struct W(crate::W<M33CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M33CTRL_SPEC>;
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
impl From<crate::W<M33CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M33CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKSVTAIRCR` reader - New BitField"]
pub type LOCKSVTAIRCR_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSVTAIRCR` writer - New BitField"]
pub type LOCKSVTAIRCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, M33CTRL_SPEC, bool, O>;
#[doc = "Field `LOCKNSVTOR` reader - New BitField"]
pub type LOCKNSVTOR_R = crate::BitReader<bool>;
#[doc = "Field `LOCKNSVTOR` writer - New BitField"]
pub type LOCKNSVTOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, M33CTRL_SPEC, bool, O>;
#[doc = "Field `LOCKSMPU` reader - New BitField"]
pub type LOCKSMPU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSMPU` writer - New BitField"]
pub type LOCKSMPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, M33CTRL_SPEC, bool, O>;
#[doc = "Field `LOCKNSMPU` reader - New BitField"]
pub type LOCKNSMPU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKNSMPU` writer - New BitField"]
pub type LOCKNSMPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, M33CTRL_SPEC, bool, O>;
#[doc = "Field `LOCKSAU` reader - New BitField"]
pub type LOCKSAU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSAU` writer - New BitField"]
pub type LOCKSAU_W<'a, const O: u8> = crate::BitWriter<'a, u32, M33CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - New BitField"]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New BitField"]
    #[inline(always)]
    pub fn locknsvtor(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - New BitField"]
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - New BitField"]
    #[inline(always)]
    pub fn locknsmpu(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - New BitField"]
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - New BitField"]
    #[inline(always)]
    #[must_use]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W<0> {
        LOCKSVTAIRCR_W::new(self)
    }
    #[doc = "Bit 1 - New BitField"]
    #[inline(always)]
    #[must_use]
    pub fn locknsvtor(&mut self) -> LOCKNSVTOR_W<1> {
        LOCKNSVTOR_W::new(self)
    }
    #[doc = "Bit 2 - New BitField"]
    #[inline(always)]
    #[must_use]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W<2> {
        LOCKSMPU_W::new(self)
    }
    #[doc = "Bit 3 - New BitField"]
    #[inline(always)]
    #[must_use]
    pub fn locknsmpu(&mut self) -> LOCKNSMPU_W<3> {
        LOCKNSMPU_W::new(self)
    }
    #[doc = "Bit 4 - New BitField"]
    #[inline(always)]
    #[must_use]
    pub fn locksau(&mut self) -> LOCKSAU_W<4> {
        LOCKSAU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Holds the M33 control settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m33ctrl](index.html) module"]
pub struct M33CTRL_SPEC;
impl crate::RegisterSpec for M33CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m33ctrl::R](R) reader structure"]
impl crate::Readable for M33CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m33ctrl::W](W) writer structure"]
impl crate::Writable for M33CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M33CTRL to value 0"]
impl crate::Resettable for M33CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
