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
#[doc = "Field `CH0CD` reader - CH0 Conversion Done Interrupt Flag"]
pub type CH0CD_R = crate::BitReader<bool>;
#[doc = "Field `CH0CD` writer - CH0 Conversion Done Interrupt Flag"]
pub type CH0CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH1CD` reader - CH1 Conversion Done Interrupt Flag"]
pub type CH1CD_R = crate::BitReader<bool>;
#[doc = "Field `CH1CD` writer - CH1 Conversion Done Interrupt Flag"]
pub type CH1CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH0OF` reader - CH0 Data Overflow Interrupt Flag"]
pub type CH0OF_R = crate::BitReader<bool>;
#[doc = "Field `CH0OF` writer - CH0 Data Overflow Interrupt Flag"]
pub type CH0OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH1OF` reader - CH1 Data Overflow Interrupt Flag"]
pub type CH1OF_R = crate::BitReader<bool>;
#[doc = "Field `CH1OF` writer - CH1 Data Overflow Interrupt Flag"]
pub type CH1OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH0UF` reader - CH0 Data Underflow Interrupt Flag"]
pub type CH0UF_R = crate::BitReader<bool>;
#[doc = "Field `CH0UF` writer - CH0 Data Underflow Interrupt Flag"]
pub type CH0UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH1UF` reader - CH1 Data Underflow Interrupt Flag"]
pub type CH1UF_R = crate::BitReader<bool>;
#[doc = "Field `CH1UF` writer - CH1 Data Underflow Interrupt Flag"]
pub type CH1UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ABUSALLOCERR` reader - ABUS Allocation Error Interrupt Flag"]
pub type ABUSALLOCERR_R = crate::BitReader<bool>;
#[doc = "Field `ABUSALLOCERR` writer - ABUS Allocation Error Interrupt Flag"]
pub type ABUSALLOCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH0DVL` reader - CH0 Data Valid Level Interrupt Flag"]
pub type CH0DVL_R = crate::BitReader<bool>;
#[doc = "Field `CH0DVL` writer - CH0 Data Valid Level Interrupt Flag"]
pub type CH0DVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH1DVL` reader - CH1 Data Valid Level Interrupt Flag"]
pub type CH1DVL_R = crate::BitReader<bool>;
#[doc = "Field `CH1DVL` writer - CH1 Data Valid Level Interrupt Flag"]
pub type CH1DVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ABUSINPUTCONFLICT` reader - ABUS Input Conflict Interrupt Flag"]
pub type ABUSINPUTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `ABUSINPUTCONFLICT` writer - ABUS Input Conflict Interrupt Flag"]
pub type ABUSINPUTCONFLICT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CH0 Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn ch0cd(&self) -> CH0CD_R {
        CH0CD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn ch1cd(&self) -> CH1CD_R {
        CH1CD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CH0 Data Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH1 Data Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH0 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0uf(&self) -> CH0UF_R {
        CH0UF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH1 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1uf(&self) -> CH1UF_R {
        CH1UF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 18 - ABUS Allocation Error Interrupt Flag"]
    #[inline(always)]
    pub fn abusallocerr(&self) -> ABUSALLOCERR_R {
        ABUSALLOCERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CH0 Data Valid Level Interrupt Flag"]
    #[inline(always)]
    pub fn ch0dvl(&self) -> CH0DVL_R {
        CH0DVL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CH1 Data Valid Level Interrupt Flag"]
    #[inline(always)]
    pub fn ch1dvl(&self) -> CH1DVL_R {
        CH1DVL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - ABUS Input Conflict Interrupt Flag"]
    #[inline(always)]
    pub fn abusinputconflict(&self) -> ABUSINPUTCONFLICT_R {
        ABUSINPUTCONFLICT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Conversion Done Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cd(&mut self) -> CH0CD_W<0> {
        CH0CD_W::new(self)
    }
    #[doc = "Bit 1 - CH1 Conversion Done Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cd(&mut self) -> CH1CD_W<1> {
        CH1CD_W::new(self)
    }
    #[doc = "Bit 4 - CH0 Data Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0of(&mut self) -> CH0OF_W<4> {
        CH0OF_W::new(self)
    }
    #[doc = "Bit 5 - CH1 Data Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1of(&mut self) -> CH1OF_W<5> {
        CH1OF_W::new(self)
    }
    #[doc = "Bit 8 - CH0 Data Underflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0uf(&mut self) -> CH0UF_W<8> {
        CH0UF_W::new(self)
    }
    #[doc = "Bit 9 - CH1 Data Underflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1uf(&mut self) -> CH1UF_W<9> {
        CH1UF_W::new(self)
    }
    #[doc = "Bit 18 - ABUS Allocation Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn abusallocerr(&mut self) -> ABUSALLOCERR_W<18> {
        ABUSALLOCERR_W::new(self)
    }
    #[doc = "Bit 20 - CH0 Data Valid Level Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0dvl(&mut self) -> CH0DVL_W<20> {
        CH0DVL_W::new(self)
    }
    #[doc = "Bit 21 - CH1 Data Valid Level Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1dvl(&mut self) -> CH1DVL_W<21> {
        CH1DVL_W::new(self)
    }
    #[doc = "Bit 26 - ABUS Input Conflict Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn abusinputconflict(&mut self) -> ABUSINPUTCONFLICT_W<26> {
        ABUSINPUTCONFLICT_W::new(self)
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
