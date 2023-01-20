#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OF` reader - Overflow Interrupt Enable"]
pub type OF_R = crate::BitReader<bool>;
#[doc = "Field `OF` writer - Overflow Interrupt Enable"]
pub type OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `UF` reader - Underflow Interrupt Enable"]
pub type UF_R = crate::BitReader<bool>;
#[doc = "Field `UF` writer - Underflow Interrupt Enable"]
pub type UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DIRCHG` reader - Direction Change Detect Interrupt Enable"]
pub type DIRCHG_R = crate::BitReader<bool>;
#[doc = "Field `DIRCHG` writer - Direction Change Detect Interrupt Enable"]
pub type DIRCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CC0` reader - CC0 Interrupt Enable"]
pub type CC0_R = crate::BitReader<bool>;
#[doc = "Field `CC0` writer - CC0 Interrupt Enable"]
pub type CC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CC1` reader - CC1 Interrupt Enable"]
pub type CC1_R = crate::BitReader<bool>;
#[doc = "Field `CC1` writer - CC1 Interrupt Enable"]
pub type CC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CC2` reader - CC2 Interrupt Enable"]
pub type CC2_R = crate::BitReader<bool>;
#[doc = "Field `CC2` writer - CC2 Interrupt Enable"]
pub type CC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ICFWLFULL0` reader - ICFWLFULL0 Interrupt Enable"]
pub type ICFWLFULL0_R = crate::BitReader<bool>;
#[doc = "Field `ICFWLFULL0` writer - ICFWLFULL0 Interrupt Enable"]
pub type ICFWLFULL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ICFWLFULL1` reader - ICFWLFULL1 Interrupt Enable"]
pub type ICFWLFULL1_R = crate::BitReader<bool>;
#[doc = "Field `ICFWLFULL1` writer - ICFWLFULL1 Interrupt Enable"]
pub type ICFWLFULL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ICFWLFULL2` reader - ICFWLFULL2 Interrupt Enable"]
pub type ICFWLFULL2_R = crate::BitReader<bool>;
#[doc = "Field `ICFWLFULL2` writer - ICFWLFULL2 Interrupt Enable"]
pub type ICFWLFULL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ICFOF0` reader - ICFOF0 Interrupt Enable"]
pub type ICFOF0_R = crate::BitReader<bool>;
#[doc = "Field `ICFOF0` writer - ICFOF0 Interrupt Enable"]
pub type ICFOF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ICFOF1` reader - ICFOF1 Interrupt Enable"]
pub type ICFOF1_R = crate::BitReader<bool>;
#[doc = "Field `ICFOF1` writer - ICFOF1 Interrupt Enable"]
pub type ICFOF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ICFOF2` reader - ICFOF2 Interrupt Enable"]
pub type ICFOF2_R = crate::BitReader<bool>;
#[doc = "Field `ICFOF2` writer - ICFOF2 Interrupt Enable"]
pub type ICFOF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ICFUF0` reader - ICFUF0 Interrupt Enable"]
pub type ICFUF0_R = crate::BitReader<bool>;
#[doc = "Field `ICFUF0` writer - ICFUF0 Interrupt Enable"]
pub type ICFUF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ICFUF1` reader - ICFUF1 Interrupt Enable"]
pub type ICFUF1_R = crate::BitReader<bool>;
#[doc = "Field `ICFUF1` writer - ICFUF1 Interrupt Enable"]
pub type ICFUF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ICFUF2` reader - ICFUF2 Interrupt Enable"]
pub type ICFUF2_R = crate::BitReader<bool>;
#[doc = "Field `ICFUF2` writer - ICFUF2 Interrupt Enable"]
pub type ICFUF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dirchg(&self) -> DIRCHG_R {
        DIRCHG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - ICFWLFULL0 Interrupt Enable"]
    #[inline(always)]
    pub fn icfwlfull0(&self) -> ICFWLFULL0_R {
        ICFWLFULL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ICFWLFULL1 Interrupt Enable"]
    #[inline(always)]
    pub fn icfwlfull1(&self) -> ICFWLFULL1_R {
        ICFWLFULL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ICFWLFULL2 Interrupt Enable"]
    #[inline(always)]
    pub fn icfwlfull2(&self) -> ICFWLFULL2_R {
        ICFWLFULL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ICFOF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icfof0(&self) -> ICFOF0_R {
        ICFOF0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ICFOF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icfof1(&self) -> ICFOF1_R {
        ICFOF1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ICFOF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icfof2(&self) -> ICFOF2_R {
        ICFOF2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - ICFUF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icfuf0(&self) -> ICFUF0_R {
        ICFUF0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ICFUF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icfuf1(&self) -> ICFUF1_R {
        ICFUF1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ICFUF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icfuf2(&self) -> ICFUF2_R {
        ICFUF2_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<0> {
        OF_W::new(self)
    }
    #[doc = "Bit 1 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<1> {
        UF_W::new(self)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DIRCHG_W<2> {
        DIRCHG_W::new(self)
    }
    #[doc = "Bit 4 - CC0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> CC0_W<4> {
        CC0_W::new(self)
    }
    #[doc = "Bit 5 - CC1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> CC1_W<5> {
        CC1_W::new(self)
    }
    #[doc = "Bit 6 - CC2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> CC2_W<6> {
        CC2_W::new(self)
    }
    #[doc = "Bit 16 - ICFWLFULL0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icfwlfull0(&mut self) -> ICFWLFULL0_W<16> {
        ICFWLFULL0_W::new(self)
    }
    #[doc = "Bit 17 - ICFWLFULL1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icfwlfull1(&mut self) -> ICFWLFULL1_W<17> {
        ICFWLFULL1_W::new(self)
    }
    #[doc = "Bit 18 - ICFWLFULL2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icfwlfull2(&mut self) -> ICFWLFULL2_W<18> {
        ICFWLFULL2_W::new(self)
    }
    #[doc = "Bit 20 - ICFOF0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icfof0(&mut self) -> ICFOF0_W<20> {
        ICFOF0_W::new(self)
    }
    #[doc = "Bit 21 - ICFOF1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icfof1(&mut self) -> ICFOF1_W<21> {
        ICFOF1_W::new(self)
    }
    #[doc = "Bit 22 - ICFOF2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icfof2(&mut self) -> ICFOF2_W<22> {
        ICFOF2_W::new(self)
    }
    #[doc = "Bit 24 - ICFUF0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icfuf0(&mut self) -> ICFUF0_W<24> {
        ICFUF0_W::new(self)
    }
    #[doc = "Bit 25 - ICFUF1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icfuf1(&mut self) -> ICFUF1_W<25> {
        ICFUF1_W::new(self)
    }
    #[doc = "Bit 26 - ICFUF2 Interrupt Enable"]
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
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
