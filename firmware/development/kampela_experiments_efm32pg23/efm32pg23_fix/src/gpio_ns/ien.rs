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
#[doc = "Field `EXTIEN0` reader - External Pin Enable"]
pub type EXTIEN0_R = crate::BitReader<bool>;
#[doc = "Field `EXTIEN0` writer - External Pin Enable"]
pub type EXTIEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EXTIEN1` reader - External Pin Enable"]
pub type EXTIEN1_R = crate::BitReader<bool>;
#[doc = "Field `EXTIEN1` writer - External Pin Enable"]
pub type EXTIEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EXTIEN2` reader - External Pin Enable"]
pub type EXTIEN2_R = crate::BitReader<bool>;
#[doc = "Field `EXTIEN2` writer - External Pin Enable"]
pub type EXTIEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EXTIEN3` reader - External Pin Enable"]
pub type EXTIEN3_R = crate::BitReader<bool>;
#[doc = "Field `EXTIEN3` writer - External Pin Enable"]
pub type EXTIEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EXTIEN4` reader - External Pin Enable"]
pub type EXTIEN4_R = crate::BitReader<bool>;
#[doc = "Field `EXTIEN4` writer - External Pin Enable"]
pub type EXTIEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EXTIEN5` reader - External Pin Enable"]
pub type EXTIEN5_R = crate::BitReader<bool>;
#[doc = "Field `EXTIEN5` writer - External Pin Enable"]
pub type EXTIEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EXTIEN6` reader - External Pin Enable"]
pub type EXTIEN6_R = crate::BitReader<bool>;
#[doc = "Field `EXTIEN6` writer - External Pin Enable"]
pub type EXTIEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EXTIEN7` reader - External Pin Enable"]
pub type EXTIEN7_R = crate::BitReader<bool>;
#[doc = "Field `EXTIEN7` writer - External Pin Enable"]
pub type EXTIEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EXTIEN8` reader - External Pin Enable"]
pub type EXTIEN8_R = crate::BitReader<bool>;
#[doc = "Field `EXTIEN8` writer - External Pin Enable"]
pub type EXTIEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EXTIEN9` reader - External Pin Enable"]
pub type EXTIEN9_R = crate::BitReader<bool>;
#[doc = "Field `EXTIEN9` writer - External Pin Enable"]
pub type EXTIEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EXTIEN10` reader - External Pin Enable"]
pub type EXTIEN10_R = crate::BitReader<bool>;
#[doc = "Field `EXTIEN10` writer - External Pin Enable"]
pub type EXTIEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EXTIEN11` reader - External Pin Enable"]
pub type EXTIEN11_R = crate::BitReader<bool>;
#[doc = "Field `EXTIEN11` writer - External Pin Enable"]
pub type EXTIEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM4WUIEN0` reader - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN0_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUIEN0` writer - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM4WUIEN1` reader - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN1_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUIEN1` writer - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM4WUIEN2` reader - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN2_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUIEN2` writer - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM4WUIEN3` reader - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN3_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUIEN3` writer - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM4WUIEN4` reader - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN4_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUIEN4` writer - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM4WUIEN5` reader - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN5_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUIEN5` writer - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM4WUIEN6` reader - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN6_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUIEN6` writer - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM4WUIEN7` reader - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN7_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUIEN7` writer - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM4WUIEN8` reader - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN8_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUIEN8` writer - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM4WUIEN9` reader - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN9_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUIEN9` writer - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM4WUIEN10` reader - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN10_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUIEN10` writer - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM4WUIEN11` reader - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN11_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUIEN11` writer - EM4 Wake Up Interrupt En"]
pub type EM4WUIEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - External Pin Enable"]
    #[inline(always)]
    pub fn extien0(&self) -> EXTIEN0_R {
        EXTIEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Pin Enable"]
    #[inline(always)]
    pub fn extien1(&self) -> EXTIEN1_R {
        EXTIEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Pin Enable"]
    #[inline(always)]
    pub fn extien2(&self) -> EXTIEN2_R {
        EXTIEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Pin Enable"]
    #[inline(always)]
    pub fn extien3(&self) -> EXTIEN3_R {
        EXTIEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Pin Enable"]
    #[inline(always)]
    pub fn extien4(&self) -> EXTIEN4_R {
        EXTIEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Pin Enable"]
    #[inline(always)]
    pub fn extien5(&self) -> EXTIEN5_R {
        EXTIEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Pin Enable"]
    #[inline(always)]
    pub fn extien6(&self) -> EXTIEN6_R {
        EXTIEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Pin Enable"]
    #[inline(always)]
    pub fn extien7(&self) -> EXTIEN7_R {
        EXTIEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Pin Enable"]
    #[inline(always)]
    pub fn extien8(&self) -> EXTIEN8_R {
        EXTIEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Pin Enable"]
    #[inline(always)]
    pub fn extien9(&self) -> EXTIEN9_R {
        EXTIEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Pin Enable"]
    #[inline(always)]
    pub fn extien10(&self) -> EXTIEN10_R {
        EXTIEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Pin Enable"]
    #[inline(always)]
    pub fn extien11(&self) -> EXTIEN11_R {
        EXTIEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    pub fn em4wuien0(&self) -> EM4WUIEN0_R {
        EM4WUIEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    pub fn em4wuien1(&self) -> EM4WUIEN1_R {
        EM4WUIEN1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    pub fn em4wuien2(&self) -> EM4WUIEN2_R {
        EM4WUIEN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    pub fn em4wuien3(&self) -> EM4WUIEN3_R {
        EM4WUIEN3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    pub fn em4wuien4(&self) -> EM4WUIEN4_R {
        EM4WUIEN4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    pub fn em4wuien5(&self) -> EM4WUIEN5_R {
        EM4WUIEN5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    pub fn em4wuien6(&self) -> EM4WUIEN6_R {
        EM4WUIEN6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    pub fn em4wuien7(&self) -> EM4WUIEN7_R {
        EM4WUIEN7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    pub fn em4wuien8(&self) -> EM4WUIEN8_R {
        EM4WUIEN8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    pub fn em4wuien9(&self) -> EM4WUIEN9_R {
        EM4WUIEN9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    pub fn em4wuien10(&self) -> EM4WUIEN10_R {
        EM4WUIEN10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    pub fn em4wuien11(&self) -> EM4WUIEN11_R {
        EM4WUIEN11_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extien0(&mut self) -> EXTIEN0_W<0> {
        EXTIEN0_W::new(self)
    }
    #[doc = "Bit 1 - External Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extien1(&mut self) -> EXTIEN1_W<1> {
        EXTIEN1_W::new(self)
    }
    #[doc = "Bit 2 - External Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extien2(&mut self) -> EXTIEN2_W<2> {
        EXTIEN2_W::new(self)
    }
    #[doc = "Bit 3 - External Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extien3(&mut self) -> EXTIEN3_W<3> {
        EXTIEN3_W::new(self)
    }
    #[doc = "Bit 4 - External Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extien4(&mut self) -> EXTIEN4_W<4> {
        EXTIEN4_W::new(self)
    }
    #[doc = "Bit 5 - External Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extien5(&mut self) -> EXTIEN5_W<5> {
        EXTIEN5_W::new(self)
    }
    #[doc = "Bit 6 - External Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extien6(&mut self) -> EXTIEN6_W<6> {
        EXTIEN6_W::new(self)
    }
    #[doc = "Bit 7 - External Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extien7(&mut self) -> EXTIEN7_W<7> {
        EXTIEN7_W::new(self)
    }
    #[doc = "Bit 8 - External Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extien8(&mut self) -> EXTIEN8_W<8> {
        EXTIEN8_W::new(self)
    }
    #[doc = "Bit 9 - External Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extien9(&mut self) -> EXTIEN9_W<9> {
        EXTIEN9_W::new(self)
    }
    #[doc = "Bit 10 - External Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extien10(&mut self) -> EXTIEN10_W<10> {
        EXTIEN10_W::new(self)
    }
    #[doc = "Bit 11 - External Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extien11(&mut self) -> EXTIEN11_W<11> {
        EXTIEN11_W::new(self)
    }
    #[doc = "Bit 16 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuien0(&mut self) -> EM4WUIEN0_W<16> {
        EM4WUIEN0_W::new(self)
    }
    #[doc = "Bit 17 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuien1(&mut self) -> EM4WUIEN1_W<17> {
        EM4WUIEN1_W::new(self)
    }
    #[doc = "Bit 18 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuien2(&mut self) -> EM4WUIEN2_W<18> {
        EM4WUIEN2_W::new(self)
    }
    #[doc = "Bit 19 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuien3(&mut self) -> EM4WUIEN3_W<19> {
        EM4WUIEN3_W::new(self)
    }
    #[doc = "Bit 20 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuien4(&mut self) -> EM4WUIEN4_W<20> {
        EM4WUIEN4_W::new(self)
    }
    #[doc = "Bit 21 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuien5(&mut self) -> EM4WUIEN5_W<21> {
        EM4WUIEN5_W::new(self)
    }
    #[doc = "Bit 22 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuien6(&mut self) -> EM4WUIEN6_W<22> {
        EM4WUIEN6_W::new(self)
    }
    #[doc = "Bit 23 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuien7(&mut self) -> EM4WUIEN7_W<23> {
        EM4WUIEN7_W::new(self)
    }
    #[doc = "Bit 24 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuien8(&mut self) -> EM4WUIEN8_W<24> {
        EM4WUIEN8_W::new(self)
    }
    #[doc = "Bit 25 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuien9(&mut self) -> EM4WUIEN9_W<25> {
        EM4WUIEN9_W::new(self)
    }
    #[doc = "Bit 26 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuien10(&mut self) -> EM4WUIEN10_W<26> {
        EM4WUIEN10_W::new(self)
    }
    #[doc = "Bit 27 - EM4 Wake Up Interrupt En"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuien11(&mut self) -> EM4WUIEN11_W<27> {
        EM4WUIEN11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
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
