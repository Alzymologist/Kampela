#[doc = "Register `GRP0_CTRL` reader"]
pub struct R(crate::R<GRP0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRP0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRP0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRP0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GRP0_CTRL` writer"]
pub struct W(crate::W<GRP0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRP0_CTRL_SPEC>;
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
impl From<crate::W<GRP0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRP0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP0EN` reader - Compare 0 Enable"]
pub type CMP0EN_R = crate::BitReader<bool>;
#[doc = "Field `CMP0EN` writer - Compare 0 Enable"]
pub type CMP0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRP0_CTRL_SPEC, bool, O>;
#[doc = "Field `CMP1EN` reader - Compare 1 Enable"]
pub type CMP1EN_R = crate::BitReader<bool>;
#[doc = "Field `CMP1EN` writer - Compare 1 Enable"]
pub type CMP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRP0_CTRL_SPEC, bool, O>;
#[doc = "Field `CAP0EN` reader - Capture 0 Enable"]
pub type CAP0EN_R = crate::BitReader<bool>;
#[doc = "Field `CAP0EN` writer - Capture 0 Enable"]
pub type CAP0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRP0_CTRL_SPEC, bool, O>;
#[doc = "Field `CMP0CMOA` reader - Compare 0 Compare Match Output Action"]
pub type CMP0CMOA_R = crate::FieldReader<u8, CMP0CMOA_A>;
#[doc = "Compare 0 Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP0CMOA_A {
    #[doc = "0: Cleared on the next cycle"]
    CLEAR = 0,
    #[doc = "1: Set on the next cycle"]
    SET = 1,
    #[doc = "2: Set on the next cycle, cleared on the cycle after"]
    PULSE = 2,
    #[doc = "3: Inverted on the next cycle"]
    TOGGLE = 3,
    #[doc = "4: Export this channel's CMP IF"]
    CMPIF = 4,
}
impl From<CMP0CMOA_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0CMOA_A) -> Self {
        variant as _
    }
}
impl CMP0CMOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMP0CMOA_A> {
        match self.bits {
            0 => Some(CMP0CMOA_A::CLEAR),
            1 => Some(CMP0CMOA_A::SET),
            2 => Some(CMP0CMOA_A::PULSE),
            3 => Some(CMP0CMOA_A::TOGGLE),
            4 => Some(CMP0CMOA_A::CMPIF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMP0CMOA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMP0CMOA_A::SET
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == CMP0CMOA_A::PULSE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CMP0CMOA_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CMPIF`"]
    #[inline(always)]
    pub fn is_cmpif(&self) -> bool {
        *self == CMP0CMOA_A::CMPIF
    }
}
#[doc = "Field `CMP0CMOA` writer - Compare 0 Compare Match Output Action"]
pub type CMP0CMOA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GRP0_CTRL_SPEC, u8, CMP0CMOA_A, 3, O>;
impl<'a, const O: u8> CMP0CMOA_W<'a, O> {
    #[doc = "Cleared on the next cycle"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMP0CMOA_A::CLEAR)
    }
    #[doc = "Set on the next cycle"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CMP0CMOA_A::SET)
    }
    #[doc = "Set on the next cycle, cleared on the cycle after"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(CMP0CMOA_A::PULSE)
    }
    #[doc = "Inverted on the next cycle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CMP0CMOA_A::TOGGLE)
    }
    #[doc = "Export this channel's CMP IF"]
    #[inline(always)]
    pub fn cmpif(self) -> &'a mut W {
        self.variant(CMP0CMOA_A::CMPIF)
    }
}
#[doc = "Field `CMP1CMOA` reader - Compare 1 Compare Match Output Action"]
pub type CMP1CMOA_R = crate::FieldReader<u8, CMP1CMOA_A>;
#[doc = "Compare 1 Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP1CMOA_A {
    #[doc = "0: Cleared on the next cycle"]
    CLEAR = 0,
    #[doc = "1: Set on the next cycle"]
    SET = 1,
    #[doc = "2: Set on the next cycle, cleared on the cycle after"]
    PULSE = 2,
    #[doc = "3: Inverted on the next cycle"]
    TOGGLE = 3,
    #[doc = "4: Export this channel's CMP IF"]
    CMPIF = 4,
}
impl From<CMP1CMOA_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP1CMOA_A) -> Self {
        variant as _
    }
}
impl CMP1CMOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMP1CMOA_A> {
        match self.bits {
            0 => Some(CMP1CMOA_A::CLEAR),
            1 => Some(CMP1CMOA_A::SET),
            2 => Some(CMP1CMOA_A::PULSE),
            3 => Some(CMP1CMOA_A::TOGGLE),
            4 => Some(CMP1CMOA_A::CMPIF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMP1CMOA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMP1CMOA_A::SET
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == CMP1CMOA_A::PULSE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CMP1CMOA_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CMPIF`"]
    #[inline(always)]
    pub fn is_cmpif(&self) -> bool {
        *self == CMP1CMOA_A::CMPIF
    }
}
#[doc = "Field `CMP1CMOA` writer - Compare 1 Compare Match Output Action"]
pub type CMP1CMOA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GRP0_CTRL_SPEC, u8, CMP1CMOA_A, 3, O>;
impl<'a, const O: u8> CMP1CMOA_W<'a, O> {
    #[doc = "Cleared on the next cycle"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMP1CMOA_A::CLEAR)
    }
    #[doc = "Set on the next cycle"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CMP1CMOA_A::SET)
    }
    #[doc = "Set on the next cycle, cleared on the cycle after"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(CMP1CMOA_A::PULSE)
    }
    #[doc = "Inverted on the next cycle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CMP1CMOA_A::TOGGLE)
    }
    #[doc = "Export this channel's CMP IF"]
    #[inline(always)]
    pub fn cmpif(self) -> &'a mut W {
        self.variant(CMP1CMOA_A::CMPIF)
    }
}
#[doc = "Field `CAP0EDGE` reader - Capture 0 Edge Select"]
pub type CAP0EDGE_R = crate::FieldReader<u8, CAP0EDGE_A>;
#[doc = "Capture 0 Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAP0EDGE_A {
    #[doc = "0: Rising edges detected"]
    RISING = 0,
    #[doc = "1: Falling edges detected"]
    FALLING = 1,
    #[doc = "2: Both edges detected"]
    BOTH = 2,
}
impl From<CAP0EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAP0EDGE_A) -> Self {
        variant as _
    }
}
impl CAP0EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAP0EDGE_A> {
        match self.bits {
            0 => Some(CAP0EDGE_A::RISING),
            1 => Some(CAP0EDGE_A::FALLING),
            2 => Some(CAP0EDGE_A::BOTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == CAP0EDGE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == CAP0EDGE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CAP0EDGE_A::BOTH
    }
}
#[doc = "Field `CAP0EDGE` writer - Capture 0 Edge Select"]
pub type CAP0EDGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GRP0_CTRL_SPEC, u8, CAP0EDGE_A, 2, O>;
impl<'a, const O: u8> CAP0EDGE_W<'a, O> {
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CAP0EDGE_A::RISING)
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(CAP0EDGE_A::FALLING)
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(CAP0EDGE_A::BOTH)
    }
}
impl R {
    #[doc = "Bit 0 - Compare 0 Enable"]
    #[inline(always)]
    pub fn cmp0en(&self) -> CMP0EN_R {
        CMP0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1 Enable"]
    #[inline(always)]
    pub fn cmp1en(&self) -> CMP1EN_R {
        CMP1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture 0 Enable"]
    #[inline(always)]
    pub fn cap0en(&self) -> CAP0EN_R {
        CAP0EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Compare 0 Compare Match Output Action"]
    #[inline(always)]
    pub fn cmp0cmoa(&self) -> CMP0CMOA_R {
        CMP0CMOA_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Compare 1 Compare Match Output Action"]
    #[inline(always)]
    pub fn cmp1cmoa(&self) -> CMP1CMOA_R {
        CMP1CMOA_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10 - Capture 0 Edge Select"]
    #[inline(always)]
    pub fn cap0edge(&self) -> CAP0EDGE_R {
        CAP0EDGE_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Compare 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0en(&mut self) -> CMP0EN_W<0> {
        CMP0EN_W::new(self)
    }
    #[doc = "Bit 1 - Compare 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1en(&mut self) -> CMP1EN_W<1> {
        CMP1EN_W::new(self)
    }
    #[doc = "Bit 2 - Capture 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cap0en(&mut self) -> CAP0EN_W<2> {
        CAP0EN_W::new(self)
    }
    #[doc = "Bits 3:5 - Compare 0 Compare Match Output Action"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0cmoa(&mut self) -> CMP0CMOA_W<3> {
        CMP0CMOA_W::new(self)
    }
    #[doc = "Bits 6:8 - Compare 1 Compare Match Output Action"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1cmoa(&mut self) -> CMP1CMOA_W<6> {
        CMP1CMOA_W::new(self)
    }
    #[doc = "Bits 9:10 - Capture 0 Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn cap0edge(&mut self) -> CAP0EDGE_W<9> {
        CAP0EDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grp0_ctrl](index.html) module"]
pub struct GRP0_CTRL_SPEC;
impl crate::RegisterSpec for GRP0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grp0_ctrl::R](R) reader structure"]
impl crate::Readable for GRP0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [grp0_ctrl::W](W) writer structure"]
impl crate::Writable for GRP0_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GRP0_CTRL to value 0"]
impl crate::Resettable for GRP0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
