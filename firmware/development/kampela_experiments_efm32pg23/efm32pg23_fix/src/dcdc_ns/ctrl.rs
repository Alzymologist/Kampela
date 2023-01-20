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
#[doc = "Field `MODE` reader - DCDC/Bypass Mode Control"]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "DCDC/Bypass Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: DCDC is OFF, bypass switch is enabled"]
    BYPASS = 0,
    #[doc = "1: Request DCDC regulation, bypass switch disabled"]
    DCDCREGULATION = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::BYPASS,
            true => MODE_A::DCDCREGULATION,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == MODE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `DCDCREGULATION`"]
    #[inline(always)]
    pub fn is_dcdcregulation(&self) -> bool {
        *self == MODE_A::DCDCREGULATION
    }
}
#[doc = "Field `MODE` writer - DCDC/Bypass Mode Control"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "DCDC is OFF, bypass switch is enabled"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(MODE_A::BYPASS)
    }
    #[doc = "Request DCDC regulation, bypass switch disabled"]
    #[inline(always)]
    pub fn dcdcregulation(self) -> &'a mut W {
        self.variant(MODE_A::DCDCREGULATION)
    }
}
#[doc = "Field `IPKTMAXCTRL` reader - Ton_max timeout control"]
pub type IPKTMAXCTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPKTMAXCTRL` writer - Ton_max timeout control"]
pub type IPKTMAXCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - DCDC/Bypass Mode Control"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:8 - Ton_max timeout control"]
    #[inline(always)]
    pub fn ipktmaxctrl(&self) -> IPKTMAXCTRL_R {
        IPKTMAXCTRL_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DCDC/Bypass Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 4:8 - Ton_max timeout control"]
    #[inline(always)]
    #[must_use]
    pub fn ipktmaxctrl(&mut self) -> IPKTMAXCTRL_W<4> {
        IPKTMAXCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0x0100"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
