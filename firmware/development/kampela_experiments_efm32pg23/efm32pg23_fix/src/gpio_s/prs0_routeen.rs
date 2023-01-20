#[doc = "Register `PRS0_ROUTEEN` reader"]
pub struct R(crate::R<PRS0_ROUTEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRS0_ROUTEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRS0_ROUTEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRS0_ROUTEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRS0_ROUTEEN` writer"]
pub struct W(crate::W<PRS0_ROUTEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRS0_ROUTEEN_SPEC>;
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
impl From<crate::W<PRS0_ROUTEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRS0_ROUTEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASYNCH0PEN` reader - ASYNCH0 pin enable control bit"]
pub type ASYNCH0PEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCH0PEN` writer - ASYNCH0 pin enable control bit"]
pub type ASYNCH0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `ASYNCH1PEN` reader - ASYNCH1 pin enable control bit"]
pub type ASYNCH1PEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCH1PEN` writer - ASYNCH1 pin enable control bit"]
pub type ASYNCH1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `ASYNCH2PEN` reader - ASYNCH2 pin enable control bit"]
pub type ASYNCH2PEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCH2PEN` writer - ASYNCH2 pin enable control bit"]
pub type ASYNCH2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `ASYNCH3PEN` reader - ASYNCH3 pin enable control bit"]
pub type ASYNCH3PEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCH3PEN` writer - ASYNCH3 pin enable control bit"]
pub type ASYNCH3PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `ASYNCH4PEN` reader - ASYNCH4 pin enable control bit"]
pub type ASYNCH4PEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCH4PEN` writer - ASYNCH4 pin enable control bit"]
pub type ASYNCH4PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `ASYNCH5PEN` reader - ASYNCH5 pin enable control bit"]
pub type ASYNCH5PEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCH5PEN` writer - ASYNCH5 pin enable control bit"]
pub type ASYNCH5PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `ASYNCH6PEN` reader - ASYNCH6 pin enable control bit"]
pub type ASYNCH6PEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCH6PEN` writer - ASYNCH6 pin enable control bit"]
pub type ASYNCH6PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `ASYNCH7PEN` reader - ASYNCH7 pin enable control bit"]
pub type ASYNCH7PEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCH7PEN` writer - ASYNCH7 pin enable control bit"]
pub type ASYNCH7PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `ASYNCH8PEN` reader - ASYNCH8 pin enable control bit"]
pub type ASYNCH8PEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCH8PEN` writer - ASYNCH8 pin enable control bit"]
pub type ASYNCH8PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `ASYNCH9PEN` reader - ASYNCH9 pin enable control bit"]
pub type ASYNCH9PEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCH9PEN` writer - ASYNCH9 pin enable control bit"]
pub type ASYNCH9PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `ASYNCH10PEN` reader - ASYNCH10 pin enable control bit"]
pub type ASYNCH10PEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCH10PEN` writer - ASYNCH10 pin enable control bit"]
pub type ASYNCH10PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `ASYNCH11PEN` reader - ASYNCH11 pin enable control bit"]
pub type ASYNCH11PEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCH11PEN` writer - ASYNCH11 pin enable control bit"]
pub type ASYNCH11PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `SYNCH0PEN` reader - SYNCH0 pin enable control bit"]
pub type SYNCH0PEN_R = crate::BitReader<bool>;
#[doc = "Field `SYNCH0PEN` writer - SYNCH0 pin enable control bit"]
pub type SYNCH0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `SYNCH1PEN` reader - SYNCH1 pin enable control bit"]
pub type SYNCH1PEN_R = crate::BitReader<bool>;
#[doc = "Field `SYNCH1PEN` writer - SYNCH1 pin enable control bit"]
pub type SYNCH1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `SYNCH2PEN` reader - SYNCH2 pin enable control bit"]
pub type SYNCH2PEN_R = crate::BitReader<bool>;
#[doc = "Field `SYNCH2PEN` writer - SYNCH2 pin enable control bit"]
pub type SYNCH2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
#[doc = "Field `SYNCH3PEN` reader - SYNCH3 pin enable control bit"]
pub type SYNCH3PEN_R = crate::BitReader<bool>;
#[doc = "Field `SYNCH3PEN` writer - SYNCH3 pin enable control bit"]
pub type SYNCH3PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS0_ROUTEEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ASYNCH0 pin enable control bit"]
    #[inline(always)]
    pub fn asynch0pen(&self) -> ASYNCH0PEN_R {
        ASYNCH0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ASYNCH1 pin enable control bit"]
    #[inline(always)]
    pub fn asynch1pen(&self) -> ASYNCH1PEN_R {
        ASYNCH1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASYNCH2 pin enable control bit"]
    #[inline(always)]
    pub fn asynch2pen(&self) -> ASYNCH2PEN_R {
        ASYNCH2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ASYNCH3 pin enable control bit"]
    #[inline(always)]
    pub fn asynch3pen(&self) -> ASYNCH3PEN_R {
        ASYNCH3PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ASYNCH4 pin enable control bit"]
    #[inline(always)]
    pub fn asynch4pen(&self) -> ASYNCH4PEN_R {
        ASYNCH4PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ASYNCH5 pin enable control bit"]
    #[inline(always)]
    pub fn asynch5pen(&self) -> ASYNCH5PEN_R {
        ASYNCH5PEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ASYNCH6 pin enable control bit"]
    #[inline(always)]
    pub fn asynch6pen(&self) -> ASYNCH6PEN_R {
        ASYNCH6PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ASYNCH7 pin enable control bit"]
    #[inline(always)]
    pub fn asynch7pen(&self) -> ASYNCH7PEN_R {
        ASYNCH7PEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ASYNCH8 pin enable control bit"]
    #[inline(always)]
    pub fn asynch8pen(&self) -> ASYNCH8PEN_R {
        ASYNCH8PEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ASYNCH9 pin enable control bit"]
    #[inline(always)]
    pub fn asynch9pen(&self) -> ASYNCH9PEN_R {
        ASYNCH9PEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ASYNCH10 pin enable control bit"]
    #[inline(always)]
    pub fn asynch10pen(&self) -> ASYNCH10PEN_R {
        ASYNCH10PEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ASYNCH11 pin enable control bit"]
    #[inline(always)]
    pub fn asynch11pen(&self) -> ASYNCH11PEN_R {
        ASYNCH11PEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SYNCH0 pin enable control bit"]
    #[inline(always)]
    pub fn synch0pen(&self) -> SYNCH0PEN_R {
        SYNCH0PEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SYNCH1 pin enable control bit"]
    #[inline(always)]
    pub fn synch1pen(&self) -> SYNCH1PEN_R {
        SYNCH1PEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SYNCH2 pin enable control bit"]
    #[inline(always)]
    pub fn synch2pen(&self) -> SYNCH2PEN_R {
        SYNCH2PEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SYNCH3 pin enable control bit"]
    #[inline(always)]
    pub fn synch3pen(&self) -> SYNCH3PEN_R {
        SYNCH3PEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ASYNCH0 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn asynch0pen(&mut self) -> ASYNCH0PEN_W<0> {
        ASYNCH0PEN_W::new(self)
    }
    #[doc = "Bit 1 - ASYNCH1 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn asynch1pen(&mut self) -> ASYNCH1PEN_W<1> {
        ASYNCH1PEN_W::new(self)
    }
    #[doc = "Bit 2 - ASYNCH2 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn asynch2pen(&mut self) -> ASYNCH2PEN_W<2> {
        ASYNCH2PEN_W::new(self)
    }
    #[doc = "Bit 3 - ASYNCH3 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn asynch3pen(&mut self) -> ASYNCH3PEN_W<3> {
        ASYNCH3PEN_W::new(self)
    }
    #[doc = "Bit 4 - ASYNCH4 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn asynch4pen(&mut self) -> ASYNCH4PEN_W<4> {
        ASYNCH4PEN_W::new(self)
    }
    #[doc = "Bit 5 - ASYNCH5 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn asynch5pen(&mut self) -> ASYNCH5PEN_W<5> {
        ASYNCH5PEN_W::new(self)
    }
    #[doc = "Bit 6 - ASYNCH6 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn asynch6pen(&mut self) -> ASYNCH6PEN_W<6> {
        ASYNCH6PEN_W::new(self)
    }
    #[doc = "Bit 7 - ASYNCH7 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn asynch7pen(&mut self) -> ASYNCH7PEN_W<7> {
        ASYNCH7PEN_W::new(self)
    }
    #[doc = "Bit 8 - ASYNCH8 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn asynch8pen(&mut self) -> ASYNCH8PEN_W<8> {
        ASYNCH8PEN_W::new(self)
    }
    #[doc = "Bit 9 - ASYNCH9 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn asynch9pen(&mut self) -> ASYNCH9PEN_W<9> {
        ASYNCH9PEN_W::new(self)
    }
    #[doc = "Bit 10 - ASYNCH10 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn asynch10pen(&mut self) -> ASYNCH10PEN_W<10> {
        ASYNCH10PEN_W::new(self)
    }
    #[doc = "Bit 11 - ASYNCH11 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn asynch11pen(&mut self) -> ASYNCH11PEN_W<11> {
        ASYNCH11PEN_W::new(self)
    }
    #[doc = "Bit 12 - SYNCH0 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn synch0pen(&mut self) -> SYNCH0PEN_W<12> {
        SYNCH0PEN_W::new(self)
    }
    #[doc = "Bit 13 - SYNCH1 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn synch1pen(&mut self) -> SYNCH1PEN_W<13> {
        SYNCH1PEN_W::new(self)
    }
    #[doc = "Bit 14 - SYNCH2 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn synch2pen(&mut self) -> SYNCH2PEN_W<14> {
        SYNCH2PEN_W::new(self)
    }
    #[doc = "Bit 15 - SYNCH3 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn synch3pen(&mut self) -> SYNCH3PEN_W<15> {
        SYNCH3PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PRS0 pin enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs0_routeen](index.html) module"]
pub struct PRS0_ROUTEEN_SPEC;
impl crate::RegisterSpec for PRS0_ROUTEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prs0_routeen::R](R) reader structure"]
impl crate::Readable for PRS0_ROUTEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prs0_routeen::W](W) writer structure"]
impl crate::Writable for PRS0_ROUTEEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRS0_ROUTEEN to value 0"]
impl crate::Resettable for PRS0_ROUTEEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
