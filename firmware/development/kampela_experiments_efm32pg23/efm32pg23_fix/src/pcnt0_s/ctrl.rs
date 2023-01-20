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
#[doc = "Field `S1CDIR` reader - Count Direction Determined By S1"]
pub type S1CDIR_R = crate::BitReader<bool>;
#[doc = "Field `S1CDIR` writer - Count Direction Determined By S1"]
pub type S1CDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CNTDIR` reader - Non-Quadrature Mode Counter Direction Co"]
pub type CNTDIR_R = crate::BitReader<CNTDIR_A>;
#[doc = "Non-Quadrature Mode Counter Direction Co\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTDIR_A {
    #[doc = "0: Up counter mode."]
    UP = 0,
    #[doc = "1: Down counter mode."]
    DOWN = 1,
}
impl From<CNTDIR_A> for bool {
    #[inline(always)]
    fn from(variant: CNTDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTDIR_A {
        match self.bits {
            false => CNTDIR_A::UP,
            true => CNTDIR_A::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == CNTDIR_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == CNTDIR_A::DOWN
    }
}
#[doc = "Field `CNTDIR` writer - Non-Quadrature Mode Counter Direction Co"]
pub type CNTDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CNTDIR_A, O>;
impl<'a, const O: u8> CNTDIR_W<'a, O> {
    #[doc = "Up counter mode."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(CNTDIR_A::UP)
    }
    #[doc = "Down counter mode."]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(CNTDIR_A::DOWN)
    }
}
#[doc = "Field `EDGE` reader - Edge Select"]
pub type EDGE_R = crate::BitReader<EDGE_A>;
#[doc = "Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE_A {
    #[doc = "0: Positive edges on the PCNTn_S0IN inputs are counted in OVSSINGLE mode. Does not invert PCNTn_S1IN input in OVSSINGLE and EXTCLKSINGLE modes"]
    POS = 0,
    #[doc = "1: Negative edges on the PCNTn_S0IN inputs are counted in OVSSINGLE mode. Inverts the PCNTn_S1IN input in OVSSINGLE and EXTCLKSINGLE modes"]
    NEG = 1,
}
impl From<EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE_A {
        match self.bits {
            false => EDGE_A::POS,
            true => EDGE_A::NEG,
        }
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == EDGE_A::POS
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == EDGE_A::NEG
    }
}
#[doc = "Field `EDGE` writer - Edge Select"]
pub type EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, EDGE_A, O>;
impl<'a, const O: u8> EDGE_W<'a, O> {
    #[doc = "Positive edges on the PCNTn_S0IN inputs are counted in OVSSINGLE mode. Does not invert PCNTn_S1IN input in OVSSINGLE and EXTCLKSINGLE modes"]
    #[inline(always)]
    pub fn pos(self) -> &'a mut W {
        self.variant(EDGE_A::POS)
    }
    #[doc = "Negative edges on the PCNTn_S0IN inputs are counted in OVSSINGLE mode. Inverts the PCNTn_S1IN input in OVSSINGLE and EXTCLKSINGLE modes"]
    #[inline(always)]
    pub fn neg(self) -> &'a mut W {
        self.variant(EDGE_A::NEG)
    }
}
#[doc = "Field `CNTEV` reader - Controls When the Counter Counts"]
pub type CNTEV_R = crate::FieldReader<u8, CNTEV_A>;
#[doc = "Controls When the Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTEV_A {
    #[doc = "0: Counts up on up-count and down on down-count events."]
    BOTH = 0,
    #[doc = "1: Only counts up on up-count events."]
    UP = 1,
    #[doc = "2: Only counts down on down-count events."]
    DOWN = 2,
}
impl From<CNTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTEV_A) -> Self {
        variant as _
    }
}
impl CNTEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNTEV_A> {
        match self.bits {
            0 => Some(CNTEV_A::BOTH),
            1 => Some(CNTEV_A::UP),
            2 => Some(CNTEV_A::DOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CNTEV_A::BOTH
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == CNTEV_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == CNTEV_A::DOWN
    }
}
#[doc = "Field `CNTEV` writer - Controls When the Counter Counts"]
pub type CNTEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, CNTEV_A, 2, O>;
impl<'a, const O: u8> CNTEV_W<'a, O> {
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(CNTEV_A::BOTH)
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(CNTEV_A::UP)
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(CNTEV_A::DOWN)
    }
}
#[doc = "Field `AUXCNTEV` reader - Controls When the Aux Counter Counts"]
pub type AUXCNTEV_R = crate::FieldReader<u8, AUXCNTEV_A>;
#[doc = "Controls When the Aux Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AUXCNTEV_A {
    #[doc = "0: Counts up on both up-count and down-count events."]
    BOTH = 0,
    #[doc = "1: Counts up on up-count events."]
    UP = 1,
    #[doc = "2: Counts up on down-count events."]
    DOWN = 2,
}
impl From<AUXCNTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXCNTEV_A) -> Self {
        variant as _
    }
}
impl AUXCNTEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AUXCNTEV_A> {
        match self.bits {
            0 => Some(AUXCNTEV_A::BOTH),
            1 => Some(AUXCNTEV_A::UP),
            2 => Some(AUXCNTEV_A::DOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == AUXCNTEV_A::BOTH
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == AUXCNTEV_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == AUXCNTEV_A::DOWN
    }
}
#[doc = "Field `AUXCNTEV` writer - Controls When the Aux Counter Counts"]
pub type AUXCNTEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, AUXCNTEV_A, 2, O>;
impl<'a, const O: u8> AUXCNTEV_W<'a, O> {
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(AUXCNTEV_A::BOTH)
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(AUXCNTEV_A::UP)
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(AUXCNTEV_A::DOWN)
    }
}
impl R {
    #[doc = "Bit 0 - Count Direction Determined By S1"]
    #[inline(always)]
    pub fn s1cdir(&self) -> S1CDIR_R {
        S1CDIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-Quadrature Mode Counter Direction Co"]
    #[inline(always)]
    pub fn cntdir(&self) -> CNTDIR_R {
        CNTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Edge Select"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Controls When the Counter Counts"]
    #[inline(always)]
    pub fn cntev(&self) -> CNTEV_R {
        CNTEV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Controls When the Aux Counter Counts"]
    #[inline(always)]
    pub fn auxcntev(&self) -> AUXCNTEV_R {
        AUXCNTEV_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Count Direction Determined By S1"]
    #[inline(always)]
    #[must_use]
    pub fn s1cdir(&mut self) -> S1CDIR_W<0> {
        S1CDIR_W::new(self)
    }
    #[doc = "Bit 1 - Non-Quadrature Mode Counter Direction Co"]
    #[inline(always)]
    #[must_use]
    pub fn cntdir(&mut self) -> CNTDIR_W<1> {
        CNTDIR_W::new(self)
    }
    #[doc = "Bit 2 - Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<2> {
        EDGE_W::new(self)
    }
    #[doc = "Bits 4:5 - Controls When the Counter Counts"]
    #[inline(always)]
    #[must_use]
    pub fn cntev(&mut self) -> CNTEV_W<4> {
        CNTEV_W::new(self)
    }
    #[doc = "Bits 6:7 - Controls When the Aux Counter Counts"]
    #[inline(always)]
    #[must_use]
    pub fn auxcntev(&mut self) -> AUXCNTEV_W<6> {
        AUXCNTEV_W::new(self)
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
