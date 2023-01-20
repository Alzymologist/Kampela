#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOTRDYVAL` reader - Not Ready Value"]
pub type NOTRDYVAL_R = crate::BitReader<NOTRDYVAL_A>;
#[doc = "Not Ready Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTRDYVAL_A {
    #[doc = "0: ACMP output is 0 when the ACMP is not ready."]
    LOW = 0,
    #[doc = "1: ACMP output is 1 when the ACMP is not ready."]
    HIGH = 1,
}
impl From<NOTRDYVAL_A> for bool {
    #[inline(always)]
    fn from(variant: NOTRDYVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl NOTRDYVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOTRDYVAL_A {
        match self.bits {
            false => NOTRDYVAL_A::LOW,
            true => NOTRDYVAL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == NOTRDYVAL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == NOTRDYVAL_A::HIGH
    }
}
#[doc = "Field `NOTRDYVAL` writer - Not Ready Value"]
pub type NOTRDYVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, NOTRDYVAL_A, O>;
impl<'a, const O: u8> NOTRDYVAL_W<'a, O> {
    #[doc = "ACMP output is 0 when the ACMP is not ready."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(NOTRDYVAL_A::LOW)
    }
    #[doc = "ACMP output is 1 when the ACMP is not ready."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(NOTRDYVAL_A::HIGH)
    }
}
#[doc = "Field `GPIOINV` reader - Comparator GPIO Output Invert"]
pub type GPIOINV_R = crate::BitReader<GPIOINV_A>;
#[doc = "Comparator GPIO Output Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOINV_A {
    #[doc = "0: The comparator output to GPIO is not inverted"]
    NOTINV = 0,
    #[doc = "1: The comparator output to GPIO is inverted"]
    INV = 1,
}
impl From<GPIOINV_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOINV_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOINV_A {
        match self.bits {
            false => GPIOINV_A::NOTINV,
            true => GPIOINV_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINV`"]
    #[inline(always)]
    pub fn is_notinv(&self) -> bool {
        *self == GPIOINV_A::NOTINV
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == GPIOINV_A::INV
    }
}
#[doc = "Field `GPIOINV` writer - Comparator GPIO Output Invert"]
pub type GPIOINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, GPIOINV_A, O>;
impl<'a, const O: u8> GPIOINV_W<'a, O> {
    #[doc = "The comparator output to GPIO is not inverted"]
    #[inline(always)]
    pub fn notinv(self) -> &'a mut W {
        self.variant(GPIOINV_A::NOTINV)
    }
    #[doc = "The comparator output to GPIO is inverted"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(GPIOINV_A::INV)
    }
}
impl R {
    #[doc = "Bit 0 - Not Ready Value"]
    #[inline(always)]
    pub fn notrdyval(&self) -> NOTRDYVAL_R {
        NOTRDYVAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator GPIO Output Invert"]
    #[inline(always)]
    pub fn gpioinv(&self) -> GPIOINV_R {
        GPIOINV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Not Ready Value"]
    #[inline(always)]
    #[must_use]
    pub fn notrdyval(&mut self) -> NOTRDYVAL_W<0> {
        NOTRDYVAL_W::new(self)
    }
    #[doc = "Bit 1 - Comparator GPIO Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn gpioinv(&mut self) -> GPIOINV_W<1> {
        GPIOINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
