#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OF` reader - Overflow Interrupt Flag"]
pub type OF_R = crate::BitReader<bool>;
#[doc = "Field `OF` writer - Overflow Interrupt Flag"]
pub type OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `UF` reader - Underflow Interrupt Flag"]
pub type UF_R = crate::BitReader<bool>;
#[doc = "Field `UF` writer - Underflow Interrupt Flag"]
pub type UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `DIRCHG` reader - Direction Change Detect Interrupt Flag"]
pub type DIRCHG_R = crate::BitReader<bool>;
#[doc = "Field `DIRCHG` writer - Direction Change Detect Interrupt Flag"]
pub type DIRCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `CC0` reader - Capture Compare Channel 0 Interrupt Flag"]
pub type CC0_R = crate::BitReader<bool>;
#[doc = "Field `CC0` writer - Capture Compare Channel 0 Interrupt Flag"]
pub type CC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `CC1` reader - Capture Compare Channel 1 Interrupt Flag"]
pub type CC1_R = crate::BitReader<bool>;
#[doc = "Field `CC1` writer - Capture Compare Channel 1 Interrupt Flag"]
pub type CC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `CC2` reader - Capture Compare Channel 2 Interrupt Flag"]
pub type CC2_R = crate::BitReader<bool>;
#[doc = "Field `CC2` writer - Capture Compare Channel 2 Interrupt Flag"]
pub type CC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ICFWLFULL0` reader - Input Capture Watermark Level Full"]
pub type ICFWLFULL0_R = crate::BitReader<bool>;
#[doc = "Field `ICFWLFULL0` writer - Input Capture Watermark Level Full"]
pub type ICFWLFULL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ICFWLFULL1` reader - Input Capture Watermark Level Full"]
pub type ICFWLFULL1_R = crate::BitReader<bool>;
#[doc = "Field `ICFWLFULL1` writer - Input Capture Watermark Level Full"]
pub type ICFWLFULL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ICFWLFULL2` reader - Input Capture Watermark Level Full"]
pub type ICFWLFULL2_R = crate::BitReader<bool>;
#[doc = "Field `ICFWLFULL2` writer - Input Capture Watermark Level Full"]
pub type ICFWLFULL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ICFOF0` reader - Input Capture FIFO overflow"]
pub type ICFOF0_R = crate::BitReader<bool>;
#[doc = "Field `ICFOF0` writer - Input Capture FIFO overflow"]
pub type ICFOF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ICFOF1` reader - Input Capture FIFO overflow"]
pub type ICFOF1_R = crate::BitReader<bool>;
#[doc = "Field `ICFOF1` writer - Input Capture FIFO overflow"]
pub type ICFOF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ICFOF2` reader - Input Capture FIFO overflow"]
pub type ICFOF2_R = crate::BitReader<bool>;
#[doc = "Field `ICFOF2` writer - Input Capture FIFO overflow"]
pub type ICFOF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ICFUF0` reader - Input capture FIFO underflow"]
pub type ICFUF0_R = crate::BitReader<bool>;
#[doc = "Field `ICFUF0` writer - Input capture FIFO underflow"]
pub type ICFUF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ICFUF1` reader - Input capture FIFO underflow"]
pub type ICFUF1_R = crate::BitReader<bool>;
#[doc = "Field `ICFUF1` writer - Input capture FIFO underflow"]
pub type ICFUF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ICFUF2` reader - Input capture FIFO underflow"]
pub type ICFUF2_R = crate::BitReader<bool>;
#[doc = "Field `ICFUF2` writer - Input capture FIFO underflow"]
pub type ICFUF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dirchg(&self) -> DIRCHG_R {
        DIRCHG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture Compare Channel 0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture Compare Channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture Compare Channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Input Capture Watermark Level Full"]
    #[inline(always)]
    pub fn icfwlfull0(&self) -> ICFWLFULL0_R {
        ICFWLFULL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input Capture Watermark Level Full"]
    #[inline(always)]
    pub fn icfwlfull1(&self) -> ICFWLFULL1_R {
        ICFWLFULL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input Capture Watermark Level Full"]
    #[inline(always)]
    pub fn icfwlfull2(&self) -> ICFWLFULL2_R {
        ICFWLFULL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Input Capture FIFO overflow"]
    #[inline(always)]
    pub fn icfof0(&self) -> ICFOF0_R {
        ICFOF0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input Capture FIFO overflow"]
    #[inline(always)]
    pub fn icfof1(&self) -> ICFOF1_R {
        ICFOF1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input Capture FIFO overflow"]
    #[inline(always)]
    pub fn icfof2(&self) -> ICFOF2_R {
        ICFOF2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Input capture FIFO underflow"]
    #[inline(always)]
    pub fn icfuf0(&self) -> ICFUF0_R {
        ICFUF0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Input capture FIFO underflow"]
    #[inline(always)]
    pub fn icfuf1(&self) -> ICFUF1_R {
        ICFUF1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Input capture FIFO underflow"]
    #[inline(always)]
    pub fn icfuf2(&self) -> ICFUF2_R {
        ICFUF2_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<0> {
        OF_W::new(self)
    }
    #[doc = "Bit 1 - Underflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<1> {
        UF_W::new(self)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DIRCHG_W<2> {
        DIRCHG_W::new(self)
    }
    #[doc = "Bit 4 - Capture Compare Channel 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> CC0_W<4> {
        CC0_W::new(self)
    }
    #[doc = "Bit 5 - Capture Compare Channel 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> CC1_W<5> {
        CC1_W::new(self)
    }
    #[doc = "Bit 6 - Capture Compare Channel 2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> CC2_W<6> {
        CC2_W::new(self)
    }
    #[doc = "Bit 16 - Input Capture Watermark Level Full"]
    #[inline(always)]
    #[must_use]
    pub fn icfwlfull0(&mut self) -> ICFWLFULL0_W<16> {
        ICFWLFULL0_W::new(self)
    }
    #[doc = "Bit 17 - Input Capture Watermark Level Full"]
    #[inline(always)]
    #[must_use]
    pub fn icfwlfull1(&mut self) -> ICFWLFULL1_W<17> {
        ICFWLFULL1_W::new(self)
    }
    #[doc = "Bit 18 - Input Capture Watermark Level Full"]
    #[inline(always)]
    #[must_use]
    pub fn icfwlfull2(&mut self) -> ICFWLFULL2_W<18> {
        ICFWLFULL2_W::new(self)
    }
    #[doc = "Bit 20 - Input Capture FIFO overflow"]
    #[inline(always)]
    #[must_use]
    pub fn icfof0(&mut self) -> ICFOF0_W<20> {
        ICFOF0_W::new(self)
    }
    #[doc = "Bit 21 - Input Capture FIFO overflow"]
    #[inline(always)]
    #[must_use]
    pub fn icfof1(&mut self) -> ICFOF1_W<21> {
        ICFOF1_W::new(self)
    }
    #[doc = "Bit 22 - Input Capture FIFO overflow"]
    #[inline(always)]
    #[must_use]
    pub fn icfof2(&mut self) -> ICFOF2_W<22> {
        ICFOF2_W::new(self)
    }
    #[doc = "Bit 24 - Input capture FIFO underflow"]
    #[inline(always)]
    #[must_use]
    pub fn icfuf0(&mut self) -> ICFUF0_W<24> {
        ICFUF0_W::new(self)
    }
    #[doc = "Bit 25 - Input capture FIFO underflow"]
    #[inline(always)]
    #[must_use]
    pub fn icfuf1(&mut self) -> ICFUF1_W<25> {
        ICFUF1_W::new(self)
    }
    #[doc = "Bit 26 - Input capture FIFO underflow"]
    #[inline(always)]
    #[must_use]
    pub fn icfuf2(&mut self) -> ICFUF2_W<26> {
        ICFUF2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
