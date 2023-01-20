#[doc = "Register `DECBOD` reader"]
pub struct R(crate::R<DECBOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECBOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECBOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECBOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DECBOD` writer"]
pub struct W(crate::W<DECBOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DECBOD_SPEC>;
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
impl From<crate::W<DECBOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DECBOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DECBODEN` reader - DECBOD enable"]
pub type DECBODEN_R = crate::BitReader<bool>;
#[doc = "Field `DECBODEN` writer - DECBOD enable"]
pub type DECBODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECBOD_SPEC, bool, O>;
#[doc = "Field `DECBODMASK` reader - DECBOD Mask"]
pub type DECBODMASK_R = crate::BitReader<bool>;
#[doc = "Field `DECBODMASK` writer - DECBOD Mask"]
pub type DECBODMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECBOD_SPEC, bool, O>;
#[doc = "Field `DECOVMBODEN` reader - Over Voltage Monitor enable"]
pub type DECOVMBODEN_R = crate::BitReader<bool>;
#[doc = "Field `DECOVMBODEN` writer - Over Voltage Monitor enable"]
pub type DECOVMBODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECBOD_SPEC, bool, O>;
#[doc = "Field `DECOVMBODMASK` reader - Over Voltage Monitor Mask"]
pub type DECOVMBODMASK_R = crate::BitReader<bool>;
#[doc = "Field `DECOVMBODMASK` writer - Over Voltage Monitor Mask"]
pub type DECOVMBODMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECBOD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DECBOD enable"]
    #[inline(always)]
    pub fn decboden(&self) -> DECBODEN_R {
        DECBODEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DECBOD Mask"]
    #[inline(always)]
    pub fn decbodmask(&self) -> DECBODMASK_R {
        DECBODMASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Over Voltage Monitor enable"]
    #[inline(always)]
    pub fn decovmboden(&self) -> DECOVMBODEN_R {
        DECOVMBODEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Over Voltage Monitor Mask"]
    #[inline(always)]
    pub fn decovmbodmask(&self) -> DECOVMBODMASK_R {
        DECOVMBODMASK_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DECBOD enable"]
    #[inline(always)]
    #[must_use]
    pub fn decboden(&mut self) -> DECBODEN_W<0> {
        DECBODEN_W::new(self)
    }
    #[doc = "Bit 1 - DECBOD Mask"]
    #[inline(always)]
    #[must_use]
    pub fn decbodmask(&mut self) -> DECBODMASK_W<1> {
        DECBODMASK_W::new(self)
    }
    #[doc = "Bit 4 - Over Voltage Monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn decovmboden(&mut self) -> DECOVMBODEN_W<4> {
        DECOVMBODEN_W::new(self)
    }
    #[doc = "Bit 5 - Over Voltage Monitor Mask"]
    #[inline(always)]
    #[must_use]
    pub fn decovmbodmask(&mut self) -> DECOVMBODMASK_W<5> {
        DECOVMBODMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decbod](index.html) module"]
pub struct DECBOD_SPEC;
impl crate::RegisterSpec for DECBOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decbod::R](R) reader structure"]
impl crate::Readable for DECBOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [decbod::W](W) writer structure"]
impl crate::Writable for DECBOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DECBOD to value 0x22"]
impl crate::Resettable for DECBOD_SPEC {
    const RESET_VALUE: Self::Ux = 0x22;
}
