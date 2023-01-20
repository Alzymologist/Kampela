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
#[doc = "Field `AHB0ERR1B` reader - AHB0 1-bit ECC Error Interrupt Enable"]
pub type AHB0ERR1B_R = crate::BitReader<bool>;
#[doc = "Field `AHB0ERR1B` writer - AHB0 1-bit ECC Error Interrupt Enable"]
pub type AHB0ERR1B_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `AHB1ERR1B` reader - AHB1 1-bit ECC Error Interrupt Enable"]
pub type AHB1ERR1B_R = crate::BitReader<bool>;
#[doc = "Field `AHB1ERR1B` writer - AHB1 1-bit ECC Error Interrupt Enable"]
pub type AHB1ERR1B_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `AHB0ERR2B` reader - AHB0 2-bit ECC Error Interrupt Enable"]
pub type AHB0ERR2B_R = crate::BitReader<bool>;
#[doc = "Field `AHB0ERR2B` writer - AHB0 2-bit ECC Error Interrupt Enable"]
pub type AHB0ERR2B_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `AHB1ERR2B` reader - AHB1 2-bit ECC Error Interrupt Enable"]
pub type AHB1ERR2B_R = crate::BitReader<bool>;
#[doc = "Field `AHB1ERR2B` writer - AHB1 2-bit ECC Error Interrupt Enable"]
pub type AHB1ERR2B_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - AHB0 1-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn ahb0err1b(&self) -> AHB0ERR1B_R {
        AHB0ERR1B_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB1 1-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn ahb1err1b(&self) -> AHB1ERR1B_R {
        AHB1ERR1B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB0 2-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn ahb0err2b(&self) -> AHB0ERR2B_R {
        AHB0ERR2B_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AHB1 2-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn ahb1err2b(&self) -> AHB1ERR2B_R {
        AHB1ERR2B_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB0 1-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb0err1b(&mut self) -> AHB0ERR1B_W<0> {
        AHB0ERR1B_W::new(self)
    }
    #[doc = "Bit 1 - AHB1 1-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb1err1b(&mut self) -> AHB1ERR1B_W<1> {
        AHB1ERR1B_W::new(self)
    }
    #[doc = "Bit 4 - AHB0 2-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb0err2b(&mut self) -> AHB0ERR2B_W<4> {
        AHB0ERR2B_W::new(self)
    }
    #[doc = "Bit 5 - AHB1 2-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb1err2b(&mut self) -> AHB1ERR2B_W<5> {
        AHB1ERR2B_W::new(self)
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
