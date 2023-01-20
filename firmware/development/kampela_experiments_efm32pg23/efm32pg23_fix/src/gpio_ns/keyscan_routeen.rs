#[doc = "Register `KEYSCAN_ROUTEEN` reader"]
pub struct R(crate::R<KEYSCAN_ROUTEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYSCAN_ROUTEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYSCAN_ROUTEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYSCAN_ROUTEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYSCAN_ROUTEEN` writer"]
pub struct W(crate::W<KEYSCAN_ROUTEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYSCAN_ROUTEEN_SPEC>;
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
impl From<crate::W<KEYSCAN_ROUTEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYSCAN_ROUTEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COLOUT0PEN` reader - COLOUT0 pin enable control bit"]
pub type COLOUT0PEN_R = crate::BitReader<bool>;
#[doc = "Field `COLOUT0PEN` writer - COLOUT0 pin enable control bit"]
pub type COLOUT0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KEYSCAN_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `COLOUT1PEN` reader - COLOUT1 pin enable control bit"]
pub type COLOUT1PEN_R = crate::BitReader<bool>;
#[doc = "Field `COLOUT1PEN` writer - COLOUT1 pin enable control bit"]
pub type COLOUT1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KEYSCAN_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `COLOUT2PEN` reader - COLOUT2 pin enable control bit"]
pub type COLOUT2PEN_R = crate::BitReader<bool>;
#[doc = "Field `COLOUT2PEN` writer - COLOUT2 pin enable control bit"]
pub type COLOUT2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KEYSCAN_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `COLOUT3PEN` reader - COLOUT3 pin enable control bit"]
pub type COLOUT3PEN_R = crate::BitReader<bool>;
#[doc = "Field `COLOUT3PEN` writer - COLOUT3 pin enable control bit"]
pub type COLOUT3PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KEYSCAN_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `COLOUT4PEN` reader - COLOUT4 pin enable control bit"]
pub type COLOUT4PEN_R = crate::BitReader<bool>;
#[doc = "Field `COLOUT4PEN` writer - COLOUT4 pin enable control bit"]
pub type COLOUT4PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KEYSCAN_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `COLOUT5PEN` reader - COLOUT5 pin enable control bit"]
pub type COLOUT5PEN_R = crate::BitReader<bool>;
#[doc = "Field `COLOUT5PEN` writer - COLOUT5 pin enable control bit"]
pub type COLOUT5PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KEYSCAN_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `COLOUT6PEN` reader - COLOUT6 pin enable control bit"]
pub type COLOUT6PEN_R = crate::BitReader<bool>;
#[doc = "Field `COLOUT6PEN` writer - COLOUT6 pin enable control bit"]
pub type COLOUT6PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KEYSCAN_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `COLOUT7PEN` reader - COLOUT7 pin enable control bit"]
pub type COLOUT7PEN_R = crate::BitReader<bool>;
#[doc = "Field `COLOUT7PEN` writer - COLOUT7 pin enable control bit"]
pub type COLOUT7PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KEYSCAN_ROUTEEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - COLOUT0 pin enable control bit"]
    #[inline(always)]
    pub fn colout0pen(&self) -> COLOUT0PEN_R {
        COLOUT0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COLOUT1 pin enable control bit"]
    #[inline(always)]
    pub fn colout1pen(&self) -> COLOUT1PEN_R {
        COLOUT1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - COLOUT2 pin enable control bit"]
    #[inline(always)]
    pub fn colout2pen(&self) -> COLOUT2PEN_R {
        COLOUT2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COLOUT3 pin enable control bit"]
    #[inline(always)]
    pub fn colout3pen(&self) -> COLOUT3PEN_R {
        COLOUT3PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - COLOUT4 pin enable control bit"]
    #[inline(always)]
    pub fn colout4pen(&self) -> COLOUT4PEN_R {
        COLOUT4PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COLOUT5 pin enable control bit"]
    #[inline(always)]
    pub fn colout5pen(&self) -> COLOUT5PEN_R {
        COLOUT5PEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - COLOUT6 pin enable control bit"]
    #[inline(always)]
    pub fn colout6pen(&self) -> COLOUT6PEN_R {
        COLOUT6PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - COLOUT7 pin enable control bit"]
    #[inline(always)]
    pub fn colout7pen(&self) -> COLOUT7PEN_R {
        COLOUT7PEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COLOUT0 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn colout0pen(&mut self) -> COLOUT0PEN_W<0> {
        COLOUT0PEN_W::new(self)
    }
    #[doc = "Bit 1 - COLOUT1 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn colout1pen(&mut self) -> COLOUT1PEN_W<1> {
        COLOUT1PEN_W::new(self)
    }
    #[doc = "Bit 2 - COLOUT2 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn colout2pen(&mut self) -> COLOUT2PEN_W<2> {
        COLOUT2PEN_W::new(self)
    }
    #[doc = "Bit 3 - COLOUT3 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn colout3pen(&mut self) -> COLOUT3PEN_W<3> {
        COLOUT3PEN_W::new(self)
    }
    #[doc = "Bit 4 - COLOUT4 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn colout4pen(&mut self) -> COLOUT4PEN_W<4> {
        COLOUT4PEN_W::new(self)
    }
    #[doc = "Bit 5 - COLOUT5 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn colout5pen(&mut self) -> COLOUT5PEN_W<5> {
        COLOUT5PEN_W::new(self)
    }
    #[doc = "Bit 6 - COLOUT6 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn colout6pen(&mut self) -> COLOUT6PEN_W<6> {
        COLOUT6PEN_W::new(self)
    }
    #[doc = "Bit 7 - COLOUT7 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn colout7pen(&mut self) -> COLOUT7PEN_W<7> {
        COLOUT7PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEYSCAN pin enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyscan_routeen](index.html) module"]
pub struct KEYSCAN_ROUTEEN_SPEC;
impl crate::RegisterSpec for KEYSCAN_ROUTEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyscan_routeen::R](R) reader structure"]
impl crate::Readable for KEYSCAN_ROUTEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyscan_routeen::W](W) writer structure"]
impl crate::Writable for KEYSCAN_ROUTEEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYSCAN_ROUTEEN to value 0"]
impl crate::Resettable for KEYSCAN_ROUTEEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
