#[doc = "Register `ASYNC_SWLEVEL` reader"]
pub struct R(crate::R<ASYNC_SWLEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNC_SWLEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNC_SWLEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNC_SWLEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNC_SWLEVEL` writer"]
pub struct W(crate::W<ASYNC_SWLEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNC_SWLEVEL_SPEC>;
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
impl From<crate::W<ASYNC_SWLEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNC_SWLEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0LEVEL` reader - Channel Level"]
pub type CH0LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH0LEVEL` writer - Channel Level"]
pub type CH0LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNC_SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH1LEVEL` reader - Channel Level"]
pub type CH1LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH1LEVEL` writer - Channel Level"]
pub type CH1LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNC_SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH2LEVEL` reader - Channel Level"]
pub type CH2LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH2LEVEL` writer - Channel Level"]
pub type CH2LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNC_SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH3LEVEL` reader - Channel Level"]
pub type CH3LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH3LEVEL` writer - Channel Level"]
pub type CH3LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNC_SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH4LEVEL` reader - Channel Level"]
pub type CH4LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH4LEVEL` writer - Channel Level"]
pub type CH4LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNC_SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH5LEVEL` reader - Channel Level"]
pub type CH5LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH5LEVEL` writer - Channel Level"]
pub type CH5LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNC_SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH6LEVEL` reader - Channel Level"]
pub type CH6LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH6LEVEL` writer - Channel Level"]
pub type CH6LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNC_SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH7LEVEL` reader - Channel Level"]
pub type CH7LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH7LEVEL` writer - Channel Level"]
pub type CH7LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNC_SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH8LEVEL` reader - Channel Level"]
pub type CH8LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH8LEVEL` writer - Channel Level"]
pub type CH8LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNC_SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH9LEVEL` reader - Channel Level"]
pub type CH9LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH9LEVEL` writer - Channel Level"]
pub type CH9LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNC_SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH10LEVEL` reader - Channel Level"]
pub type CH10LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH10LEVEL` writer - Channel Level"]
pub type CH10LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNC_SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH11LEVEL` reader - Channel Level"]
pub type CH11LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH11LEVEL` writer - Channel Level"]
pub type CH11LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNC_SWLEVEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel Level"]
    #[inline(always)]
    pub fn ch0level(&self) -> CH0LEVEL_R {
        CH0LEVEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Level"]
    #[inline(always)]
    pub fn ch1level(&self) -> CH1LEVEL_R {
        CH1LEVEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Level"]
    #[inline(always)]
    pub fn ch2level(&self) -> CH2LEVEL_R {
        CH2LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Level"]
    #[inline(always)]
    pub fn ch3level(&self) -> CH3LEVEL_R {
        CH3LEVEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Level"]
    #[inline(always)]
    pub fn ch4level(&self) -> CH4LEVEL_R {
        CH4LEVEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Level"]
    #[inline(always)]
    pub fn ch5level(&self) -> CH5LEVEL_R {
        CH5LEVEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Level"]
    #[inline(always)]
    pub fn ch6level(&self) -> CH6LEVEL_R {
        CH6LEVEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Level"]
    #[inline(always)]
    pub fn ch7level(&self) -> CH7LEVEL_R {
        CH7LEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel Level"]
    #[inline(always)]
    pub fn ch8level(&self) -> CH8LEVEL_R {
        CH8LEVEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Level"]
    #[inline(always)]
    pub fn ch9level(&self) -> CH9LEVEL_R {
        CH9LEVEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Level"]
    #[inline(always)]
    pub fn ch10level(&self) -> CH10LEVEL_R {
        CH10LEVEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel Level"]
    #[inline(always)]
    pub fn ch11level(&self) -> CH11LEVEL_R {
        CH11LEVEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch0level(&mut self) -> CH0LEVEL_W<0> {
        CH0LEVEL_W::new(self)
    }
    #[doc = "Bit 1 - Channel Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch1level(&mut self) -> CH1LEVEL_W<1> {
        CH1LEVEL_W::new(self)
    }
    #[doc = "Bit 2 - Channel Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch2level(&mut self) -> CH2LEVEL_W<2> {
        CH2LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - Channel Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch3level(&mut self) -> CH3LEVEL_W<3> {
        CH3LEVEL_W::new(self)
    }
    #[doc = "Bit 4 - Channel Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch4level(&mut self) -> CH4LEVEL_W<4> {
        CH4LEVEL_W::new(self)
    }
    #[doc = "Bit 5 - Channel Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch5level(&mut self) -> CH5LEVEL_W<5> {
        CH5LEVEL_W::new(self)
    }
    #[doc = "Bit 6 - Channel Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch6level(&mut self) -> CH6LEVEL_W<6> {
        CH6LEVEL_W::new(self)
    }
    #[doc = "Bit 7 - Channel Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch7level(&mut self) -> CH7LEVEL_W<7> {
        CH7LEVEL_W::new(self)
    }
    #[doc = "Bit 8 - Channel Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch8level(&mut self) -> CH8LEVEL_W<8> {
        CH8LEVEL_W::new(self)
    }
    #[doc = "Bit 9 - Channel Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch9level(&mut self) -> CH9LEVEL_W<9> {
        CH9LEVEL_W::new(self)
    }
    #[doc = "Bit 10 - Channel Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch10level(&mut self) -> CH10LEVEL_W<10> {
        CH10LEVEL_W::new(self)
    }
    #[doc = "Bit 11 - Channel Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch11level(&mut self) -> CH11LEVEL_W<11> {
        CH11LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [async_swlevel](index.html) module"]
pub struct ASYNC_SWLEVEL_SPEC;
impl crate::RegisterSpec for ASYNC_SWLEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [async_swlevel::R](R) reader structure"]
impl crate::Readable for ASYNC_SWLEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [async_swlevel::W](W) writer structure"]
impl crate::Writable for ASYNC_SWLEVEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNC_SWLEVEL to value 0"]
impl crate::Resettable for ASYNC_SWLEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
