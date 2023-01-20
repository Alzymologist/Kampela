#[doc = "Register `LESENSE_ROUTEEN` reader"]
pub struct R(crate::R<LESENSE_ROUTEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LESENSE_ROUTEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LESENSE_ROUTEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LESENSE_ROUTEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LESENSE_ROUTEEN` writer"]
pub struct W(crate::W<LESENSE_ROUTEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LESENSE_ROUTEEN_SPEC>;
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
impl From<crate::W<LESENSE_ROUTEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LESENSE_ROUTEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0OUTPEN` reader - CH0OUT pin enable control bit"]
pub type CH0OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH0OUTPEN` writer - CH0OUT pin enable control bit"]
pub type CH0OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH1OUTPEN` reader - CH1OUT pin enable control bit"]
pub type CH1OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH1OUTPEN` writer - CH1OUT pin enable control bit"]
pub type CH1OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH2OUTPEN` reader - CH2OUT pin enable control bit"]
pub type CH2OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2OUTPEN` writer - CH2OUT pin enable control bit"]
pub type CH2OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH3OUTPEN` reader - CH3OUT pin enable control bit"]
pub type CH3OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH3OUTPEN` writer - CH3OUT pin enable control bit"]
pub type CH3OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH4OUTPEN` reader - CH4OUT pin enable control bit"]
pub type CH4OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH4OUTPEN` writer - CH4OUT pin enable control bit"]
pub type CH4OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH5OUTPEN` reader - CH5OUT pin enable control bit"]
pub type CH5OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH5OUTPEN` writer - CH5OUT pin enable control bit"]
pub type CH5OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH6OUTPEN` reader - CH6OUT pin enable control bit"]
pub type CH6OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH6OUTPEN` writer - CH6OUT pin enable control bit"]
pub type CH6OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH7OUTPEN` reader - CH7OUT pin enable control bit"]
pub type CH7OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH7OUTPEN` writer - CH7OUT pin enable control bit"]
pub type CH7OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH8OUTPEN` reader - CH8OUT pin enable control bit"]
pub type CH8OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH8OUTPEN` writer - CH8OUT pin enable control bit"]
pub type CH8OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH9OUTPEN` reader - CH9OUT pin enable control bit"]
pub type CH9OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH9OUTPEN` writer - CH9OUT pin enable control bit"]
pub type CH9OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH10OUTPEN` reader - CH10OUT pin enable control bit"]
pub type CH10OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH10OUTPEN` writer - CH10OUT pin enable control bit"]
pub type CH10OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH11OUTPEN` reader - CH11OUT pin enable control bit"]
pub type CH11OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH11OUTPEN` writer - CH11OUT pin enable control bit"]
pub type CH11OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH12OUTPEN` reader - CH12OUT pin enable control bit"]
pub type CH12OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH12OUTPEN` writer - CH12OUT pin enable control bit"]
pub type CH12OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH13OUTPEN` reader - CH13OUT pin enable control bit"]
pub type CH13OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH13OUTPEN` writer - CH13OUT pin enable control bit"]
pub type CH13OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH14OUTPEN` reader - CH14OUT pin enable control bit"]
pub type CH14OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH14OUTPEN` writer - CH14OUT pin enable control bit"]
pub type CH14OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `CH15OUTPEN` reader - CH15OUT pin enable control bit"]
pub type CH15OUTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CH15OUTPEN` writer - CH15OUT pin enable control bit"]
pub type CH15OUTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LESENSE_ROUTEEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CH0OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch0outpen(&self) -> CH0OUTPEN_R {
        CH0OUTPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch1outpen(&self) -> CH1OUTPEN_R {
        CH1OUTPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch2outpen(&self) -> CH2OUTPEN_R {
        CH2OUTPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch3outpen(&self) -> CH3OUTPEN_R {
        CH3OUTPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH4OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch4outpen(&self) -> CH4OUTPEN_R {
        CH4OUTPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH5OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch5outpen(&self) -> CH5OUTPEN_R {
        CH5OUTPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH6OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch6outpen(&self) -> CH6OUTPEN_R {
        CH6OUTPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH7OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch7outpen(&self) -> CH7OUTPEN_R {
        CH7OUTPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CH8OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch8outpen(&self) -> CH8OUTPEN_R {
        CH8OUTPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH9OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch9outpen(&self) -> CH9OUTPEN_R {
        CH9OUTPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CH10OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch10outpen(&self) -> CH10OUTPEN_R {
        CH10OUTPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CH11OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch11outpen(&self) -> CH11OUTPEN_R {
        CH11OUTPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CH12OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch12outpen(&self) -> CH12OUTPEN_R {
        CH12OUTPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CH13OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch13outpen(&self) -> CH13OUTPEN_R {
        CH13OUTPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CH14OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch14outpen(&self) -> CH14OUTPEN_R {
        CH14OUTPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CH15OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch15outpen(&self) -> CH15OUTPEN_R {
        CH15OUTPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch0outpen(&mut self) -> CH0OUTPEN_W<0> {
        CH0OUTPEN_W::new(self)
    }
    #[doc = "Bit 1 - CH1OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch1outpen(&mut self) -> CH1OUTPEN_W<1> {
        CH1OUTPEN_W::new(self)
    }
    #[doc = "Bit 2 - CH2OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch2outpen(&mut self) -> CH2OUTPEN_W<2> {
        CH2OUTPEN_W::new(self)
    }
    #[doc = "Bit 3 - CH3OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch3outpen(&mut self) -> CH3OUTPEN_W<3> {
        CH3OUTPEN_W::new(self)
    }
    #[doc = "Bit 4 - CH4OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch4outpen(&mut self) -> CH4OUTPEN_W<4> {
        CH4OUTPEN_W::new(self)
    }
    #[doc = "Bit 5 - CH5OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch5outpen(&mut self) -> CH5OUTPEN_W<5> {
        CH5OUTPEN_W::new(self)
    }
    #[doc = "Bit 6 - CH6OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch6outpen(&mut self) -> CH6OUTPEN_W<6> {
        CH6OUTPEN_W::new(self)
    }
    #[doc = "Bit 7 - CH7OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch7outpen(&mut self) -> CH7OUTPEN_W<7> {
        CH7OUTPEN_W::new(self)
    }
    #[doc = "Bit 8 - CH8OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch8outpen(&mut self) -> CH8OUTPEN_W<8> {
        CH8OUTPEN_W::new(self)
    }
    #[doc = "Bit 9 - CH9OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch9outpen(&mut self) -> CH9OUTPEN_W<9> {
        CH9OUTPEN_W::new(self)
    }
    #[doc = "Bit 10 - CH10OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch10outpen(&mut self) -> CH10OUTPEN_W<10> {
        CH10OUTPEN_W::new(self)
    }
    #[doc = "Bit 11 - CH11OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch11outpen(&mut self) -> CH11OUTPEN_W<11> {
        CH11OUTPEN_W::new(self)
    }
    #[doc = "Bit 12 - CH12OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch12outpen(&mut self) -> CH12OUTPEN_W<12> {
        CH12OUTPEN_W::new(self)
    }
    #[doc = "Bit 13 - CH13OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch13outpen(&mut self) -> CH13OUTPEN_W<13> {
        CH13OUTPEN_W::new(self)
    }
    #[doc = "Bit 14 - CH14OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch14outpen(&mut self) -> CH14OUTPEN_W<14> {
        CH14OUTPEN_W::new(self)
    }
    #[doc = "Bit 15 - CH15OUT pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch15outpen(&mut self) -> CH15OUTPEN_W<15> {
        CH15OUTPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LESENSE pin enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lesense_routeen](index.html) module"]
pub struct LESENSE_ROUTEEN_SPEC;
impl crate::RegisterSpec for LESENSE_ROUTEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lesense_routeen::R](R) reader structure"]
impl crate::Readable for LESENSE_ROUTEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lesense_routeen::W](W) writer structure"]
impl crate::Writable for LESENSE_ROUTEEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LESENSE_ROUTEEN to value 0"]
impl crate::Resettable for LESENSE_ROUTEEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
