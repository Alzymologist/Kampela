#[doc = "Register `BOD3SENSE` reader"]
pub struct R(crate::R<BOD3SENSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD3SENSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD3SENSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD3SENSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD3SENSE` writer"]
pub struct W(crate::W<BOD3SENSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD3SENSE_SPEC>;
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
impl From<crate::W<BOD3SENSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD3SENSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVDDBODEN` reader - AVDD BOD enable"]
pub type AVDDBODEN_R = crate::BitReader<bool>;
#[doc = "Field `AVDDBODEN` writer - AVDD BOD enable"]
pub type AVDDBODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD3SENSE_SPEC, bool, O>;
#[doc = "Field `VDDIO0BODEN` reader - VDDIO0 BOD enable"]
pub type VDDIO0BODEN_R = crate::BitReader<bool>;
#[doc = "Field `VDDIO0BODEN` writer - VDDIO0 BOD enable"]
pub type VDDIO0BODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD3SENSE_SPEC, bool, O>;
#[doc = "Field `VDDIO1BODEN` reader - VDDIO1 BOD enable"]
pub type VDDIO1BODEN_R = crate::BitReader<bool>;
#[doc = "Field `VDDIO1BODEN` writer - VDDIO1 BOD enable"]
pub type VDDIO1BODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD3SENSE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - AVDD BOD enable"]
    #[inline(always)]
    pub fn avddboden(&self) -> AVDDBODEN_R {
        AVDDBODEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VDDIO0 BOD enable"]
    #[inline(always)]
    pub fn vddio0boden(&self) -> VDDIO0BODEN_R {
        VDDIO0BODEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VDDIO1 BOD enable"]
    #[inline(always)]
    pub fn vddio1boden(&self) -> VDDIO1BODEN_R {
        VDDIO1BODEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AVDD BOD enable"]
    #[inline(always)]
    #[must_use]
    pub fn avddboden(&mut self) -> AVDDBODEN_W<0> {
        AVDDBODEN_W::new(self)
    }
    #[doc = "Bit 1 - VDDIO0 BOD enable"]
    #[inline(always)]
    #[must_use]
    pub fn vddio0boden(&mut self) -> VDDIO0BODEN_W<1> {
        VDDIO0BODEN_W::new(self)
    }
    #[doc = "Bit 2 - VDDIO1 BOD enable"]
    #[inline(always)]
    #[must_use]
    pub fn vddio1boden(&mut self) -> VDDIO1BODEN_W<2> {
        VDDIO1BODEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod3sense](index.html) module"]
pub struct BOD3SENSE_SPEC;
impl crate::RegisterSpec for BOD3SENSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod3sense::R](R) reader structure"]
impl crate::Readable for BOD3SENSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod3sense::W](W) writer structure"]
impl crate::Writable for BOD3SENSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD3SENSE to value 0"]
impl crate::Resettable for BOD3SENSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
