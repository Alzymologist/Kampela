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
#[doc = "Field `EXTIF0` reader - External Pin Flag"]
pub type EXTIF0_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF0` writer - External Pin Flag"]
pub type EXTIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EXTIF1` reader - External Pin Flag"]
pub type EXTIF1_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF1` writer - External Pin Flag"]
pub type EXTIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EXTIF2` reader - External Pin Flag"]
pub type EXTIF2_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF2` writer - External Pin Flag"]
pub type EXTIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EXTIF3` reader - External Pin Flag"]
pub type EXTIF3_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF3` writer - External Pin Flag"]
pub type EXTIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EXTIF4` reader - External Pin Flag"]
pub type EXTIF4_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF4` writer - External Pin Flag"]
pub type EXTIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EXTIF5` reader - External Pin Flag"]
pub type EXTIF5_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF5` writer - External Pin Flag"]
pub type EXTIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EXTIF6` reader - External Pin Flag"]
pub type EXTIF6_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF6` writer - External Pin Flag"]
pub type EXTIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EXTIF7` reader - External Pin Flag"]
pub type EXTIF7_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF7` writer - External Pin Flag"]
pub type EXTIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EXTIF8` reader - External Pin Flag"]
pub type EXTIF8_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF8` writer - External Pin Flag"]
pub type EXTIF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EXTIF9` reader - External Pin Flag"]
pub type EXTIF9_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF9` writer - External Pin Flag"]
pub type EXTIF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EXTIF10` reader - External Pin Flag"]
pub type EXTIF10_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF10` writer - External Pin Flag"]
pub type EXTIF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EXTIF11` reader - External Pin Flag"]
pub type EXTIF11_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF11` writer - External Pin Flag"]
pub type EXTIF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `EM4WU` reader - EM4 wake up"]
pub type EM4WU_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EM4WU` writer - EM4 wake up"]
pub type EM4WU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IF_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 0 - External Pin Flag"]
    #[inline(always)]
    pub fn extif0(&self) -> EXTIF0_R {
        EXTIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Pin Flag"]
    #[inline(always)]
    pub fn extif1(&self) -> EXTIF1_R {
        EXTIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Pin Flag"]
    #[inline(always)]
    pub fn extif2(&self) -> EXTIF2_R {
        EXTIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Pin Flag"]
    #[inline(always)]
    pub fn extif3(&self) -> EXTIF3_R {
        EXTIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Pin Flag"]
    #[inline(always)]
    pub fn extif4(&self) -> EXTIF4_R {
        EXTIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Pin Flag"]
    #[inline(always)]
    pub fn extif5(&self) -> EXTIF5_R {
        EXTIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Pin Flag"]
    #[inline(always)]
    pub fn extif6(&self) -> EXTIF6_R {
        EXTIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Pin Flag"]
    #[inline(always)]
    pub fn extif7(&self) -> EXTIF7_R {
        EXTIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Pin Flag"]
    #[inline(always)]
    pub fn extif8(&self) -> EXTIF8_R {
        EXTIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Pin Flag"]
    #[inline(always)]
    pub fn extif9(&self) -> EXTIF9_R {
        EXTIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Pin Flag"]
    #[inline(always)]
    pub fn extif10(&self) -> EXTIF10_R {
        EXTIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Pin Flag"]
    #[inline(always)]
    pub fn extif11(&self) -> EXTIF11_R {
        EXTIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:27 - EM4 wake up"]
    #[inline(always)]
    pub fn em4wu(&self) -> EM4WU_R {
        EM4WU_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - External Pin Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extif0(&mut self) -> EXTIF0_W<0> {
        EXTIF0_W::new(self)
    }
    #[doc = "Bit 1 - External Pin Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extif1(&mut self) -> EXTIF1_W<1> {
        EXTIF1_W::new(self)
    }
    #[doc = "Bit 2 - External Pin Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extif2(&mut self) -> EXTIF2_W<2> {
        EXTIF2_W::new(self)
    }
    #[doc = "Bit 3 - External Pin Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extif3(&mut self) -> EXTIF3_W<3> {
        EXTIF3_W::new(self)
    }
    #[doc = "Bit 4 - External Pin Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extif4(&mut self) -> EXTIF4_W<4> {
        EXTIF4_W::new(self)
    }
    #[doc = "Bit 5 - External Pin Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extif5(&mut self) -> EXTIF5_W<5> {
        EXTIF5_W::new(self)
    }
    #[doc = "Bit 6 - External Pin Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extif6(&mut self) -> EXTIF6_W<6> {
        EXTIF6_W::new(self)
    }
    #[doc = "Bit 7 - External Pin Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extif7(&mut self) -> EXTIF7_W<7> {
        EXTIF7_W::new(self)
    }
    #[doc = "Bit 8 - External Pin Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extif8(&mut self) -> EXTIF8_W<8> {
        EXTIF8_W::new(self)
    }
    #[doc = "Bit 9 - External Pin Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extif9(&mut self) -> EXTIF9_W<9> {
        EXTIF9_W::new(self)
    }
    #[doc = "Bit 10 - External Pin Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extif10(&mut self) -> EXTIF10_W<10> {
        EXTIF10_W::new(self)
    }
    #[doc = "Bit 11 - External Pin Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extif11(&mut self) -> EXTIF11_W<11> {
        EXTIF11_W::new(self)
    }
    #[doc = "Bits 16:27 - EM4 wake up"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu(&mut self) -> EM4WU_W<16> {
        EM4WU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
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
