#[doc = "Register `MISCLOCKWORD` reader"]
pub struct R(crate::R<MISCLOCKWORD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISCLOCKWORD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISCLOCKWORD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISCLOCKWORD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISCLOCKWORD` writer"]
pub struct W(crate::W<MISCLOCKWORD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISCLOCKWORD_SPEC>;
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
impl From<crate::W<MISCLOCKWORD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISCLOCKWORD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MELOCKBIT` reader - Mass Erase Lock"]
pub type MELOCKBIT_R = crate::BitReader<bool>;
#[doc = "Field `MELOCKBIT` writer - Mass Erase Lock"]
pub type MELOCKBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISCLOCKWORD_SPEC, bool, O>;
#[doc = "Field `UDLOCKBIT` reader - User Data Lock"]
pub type UDLOCKBIT_R = crate::BitReader<bool>;
#[doc = "Field `UDLOCKBIT` writer - User Data Lock"]
pub type UDLOCKBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISCLOCKWORD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Mass Erase Lock"]
    #[inline(always)]
    pub fn melockbit(&self) -> MELOCKBIT_R {
        MELOCKBIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - User Data Lock"]
    #[inline(always)]
    pub fn udlockbit(&self) -> UDLOCKBIT_R {
        UDLOCKBIT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mass Erase Lock"]
    #[inline(always)]
    #[must_use]
    pub fn melockbit(&mut self) -> MELOCKBIT_W<0> {
        MELOCKBIT_W::new(self)
    }
    #[doc = "Bit 4 - User Data Lock"]
    #[inline(always)]
    #[must_use]
    pub fn udlockbit(&mut self) -> UDLOCKBIT_W<4> {
        UDLOCKBIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misclockword](index.html) module"]
pub struct MISCLOCKWORD_SPEC;
impl crate::RegisterSpec for MISCLOCKWORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misclockword::R](R) reader structure"]
impl crate::Readable for MISCLOCKWORD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misclockword::W](W) writer structure"]
impl crate::Writable for MISCLOCKWORD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISCLOCKWORD to value 0x11"]
impl crate::Resettable for MISCLOCKWORD_SPEC {
    const RESET_VALUE: Self::Ux = 0x11;
}
