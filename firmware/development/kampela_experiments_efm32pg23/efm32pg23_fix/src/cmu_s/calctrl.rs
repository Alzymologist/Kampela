#[doc = "Register `CALCTRL` reader"]
pub struct R(crate::R<CALCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALCTRL` writer"]
pub struct W(crate::W<CALCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALCTRL_SPEC>;
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
impl From<crate::W<CALCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALTOP` reader - Calibration Counter Top Value"]
pub type CALTOP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CALTOP` writer - Calibration Counter Top Value"]
pub type CALTOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALCTRL_SPEC, u32, u32, 20, O>;
#[doc = "Field `CONT` reader - Continuous Calibration"]
pub type CONT_R = crate::BitReader<bool>;
#[doc = "Field `CONT` writer - Continuous Calibration"]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALCTRL_SPEC, bool, O>;
#[doc = "Field `UPSEL` reader - Calibration Up-counter Select"]
pub type UPSEL_R = crate::FieldReader<u8, UPSEL_A>;
#[doc = "Calibration Up-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPSEL_A {
    #[doc = "0: Up-counter is not clocked"]
    DISABLED = 0,
    #[doc = "1: PRS CMU_CALUP consumer is clocking up-counter"]
    PRS = 1,
    #[doc = "2: HFXO is clocking up-counter"]
    HFXO = 2,
    #[doc = "3: LFXO is clocking up-counter"]
    LFXO = 3,
    #[doc = "4: HFRCODPLL is clocking up-counter"]
    HFRCODPLL = 4,
    #[doc = "5: HFRCOEM23 is clocking up-counter"]
    HFRCOEM23 = 5,
    #[doc = "8: FSRCO is clocking up-counter"]
    FSRCO = 8,
    #[doc = "9: LFRCO is clocking up-counter"]
    LFRCO = 9,
    #[doc = "10: ULFRCO is clocking up-counter"]
    ULFRCO = 10,
}
impl From<UPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UPSEL_A) -> Self {
        variant as _
    }
}
impl UPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UPSEL_A> {
        match self.bits {
            0 => Some(UPSEL_A::DISABLED),
            1 => Some(UPSEL_A::PRS),
            2 => Some(UPSEL_A::HFXO),
            3 => Some(UPSEL_A::LFXO),
            4 => Some(UPSEL_A::HFRCODPLL),
            5 => Some(UPSEL_A::HFRCOEM23),
            8 => Some(UPSEL_A::FSRCO),
            9 => Some(UPSEL_A::LFRCO),
            10 => Some(UPSEL_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == UPSEL_A::PRS
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == UPSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == UPSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCODPLL`"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == UPSEL_A::HFRCODPLL
    }
    #[doc = "Checks if the value of the field is `HFRCOEM23`"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == UPSEL_A::HFRCOEM23
    }
    #[doc = "Checks if the value of the field is `FSRCO`"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == UPSEL_A::FSRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == UPSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == UPSEL_A::ULFRCO
    }
}
#[doc = "Field `UPSEL` writer - Calibration Up-counter Select"]
pub type UPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALCTRL_SPEC, u8, UPSEL_A, 4, O>;
impl<'a, const O: u8> UPSEL_W<'a, O> {
    #[doc = "Up-counter is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPSEL_A::DISABLED)
    }
    #[doc = "PRS CMU_CALUP consumer is clocking up-counter"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(UPSEL_A::PRS)
    }
    #[doc = "HFXO is clocking up-counter"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(UPSEL_A::HFXO)
    }
    #[doc = "LFXO is clocking up-counter"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(UPSEL_A::LFXO)
    }
    #[doc = "HFRCODPLL is clocking up-counter"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut W {
        self.variant(UPSEL_A::HFRCODPLL)
    }
    #[doc = "HFRCOEM23 is clocking up-counter"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut W {
        self.variant(UPSEL_A::HFRCOEM23)
    }
    #[doc = "FSRCO is clocking up-counter"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut W {
        self.variant(UPSEL_A::FSRCO)
    }
    #[doc = "LFRCO is clocking up-counter"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::LFRCO)
    }
    #[doc = "ULFRCO is clocking up-counter"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::ULFRCO)
    }
}
#[doc = "Field `DOWNSEL` reader - Calibration Down-counter Select"]
pub type DOWNSEL_R = crate::FieldReader<u8, DOWNSEL_A>;
#[doc = "Calibration Down-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DOWNSEL_A {
    #[doc = "0: Down-counter is not clocked"]
    DISABLED = 0,
    #[doc = "1: HCLK is clocking down-counter"]
    HCLK = 1,
    #[doc = "2: PRS CMU_CALDN consumer is clocking down-counter"]
    PRS = 2,
    #[doc = "3: HFXO is clocking down-counter"]
    HFXO = 3,
    #[doc = "4: LFXO is clocking down-counter"]
    LFXO = 4,
    #[doc = "5: HFRCODPLL is clocking down-counter"]
    HFRCODPLL = 5,
    #[doc = "6: HFRCOEM23 is clocking down-counter"]
    HFRCOEM23 = 6,
    #[doc = "9: FSRCO is clocking down-counter"]
    FSRCO = 9,
    #[doc = "10: LFRCO is clocking down-counter"]
    LFRCO = 10,
    #[doc = "11: ULFRCO is clocking down-counter"]
    ULFRCO = 11,
}
impl From<DOWNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DOWNSEL_A) -> Self {
        variant as _
    }
}
impl DOWNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DOWNSEL_A> {
        match self.bits {
            0 => Some(DOWNSEL_A::DISABLED),
            1 => Some(DOWNSEL_A::HCLK),
            2 => Some(DOWNSEL_A::PRS),
            3 => Some(DOWNSEL_A::HFXO),
            4 => Some(DOWNSEL_A::LFXO),
            5 => Some(DOWNSEL_A::HFRCODPLL),
            6 => Some(DOWNSEL_A::HFRCOEM23),
            9 => Some(DOWNSEL_A::FSRCO),
            10 => Some(DOWNSEL_A::LFRCO),
            11 => Some(DOWNSEL_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DOWNSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == DOWNSEL_A::HCLK
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == DOWNSEL_A::PRS
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == DOWNSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == DOWNSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCODPLL`"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == DOWNSEL_A::HFRCODPLL
    }
    #[doc = "Checks if the value of the field is `HFRCOEM23`"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == DOWNSEL_A::HFRCOEM23
    }
    #[doc = "Checks if the value of the field is `FSRCO`"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == DOWNSEL_A::FSRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == DOWNSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == DOWNSEL_A::ULFRCO
    }
}
#[doc = "Field `DOWNSEL` writer - Calibration Down-counter Select"]
pub type DOWNSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CALCTRL_SPEC, u8, DOWNSEL_A, 4, O>;
impl<'a, const O: u8> DOWNSEL_W<'a, O> {
    #[doc = "Down-counter is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DOWNSEL_A::DISABLED)
    }
    #[doc = "HCLK is clocking down-counter"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HCLK)
    }
    #[doc = "PRS CMU_CALDN consumer is clocking down-counter"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(DOWNSEL_A::PRS)
    }
    #[doc = "HFXO is clocking down-counter"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFXO)
    }
    #[doc = "LFXO is clocking down-counter"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(DOWNSEL_A::LFXO)
    }
    #[doc = "HFRCODPLL is clocking down-counter"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFRCODPLL)
    }
    #[doc = "HFRCOEM23 is clocking down-counter"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFRCOEM23)
    }
    #[doc = "FSRCO is clocking down-counter"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::FSRCO)
    }
    #[doc = "LFRCO is clocking down-counter"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::LFRCO)
    }
    #[doc = "ULFRCO is clocking down-counter"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::ULFRCO)
    }
}
impl R {
    #[doc = "Bits 0:19 - Calibration Counter Top Value"]
    #[inline(always)]
    pub fn caltop(&self) -> CALTOP_R {
        CALTOP_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 23 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&self) -> UPSEL_R {
        UPSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&self) -> DOWNSEL_R {
        DOWNSEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - Calibration Counter Top Value"]
    #[inline(always)]
    #[must_use]
    pub fn caltop(&mut self) -> CALTOP_W<0> {
        CALTOP_W::new(self)
    }
    #[doc = "Bit 23 - Continuous Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<23> {
        CONT_W::new(self)
    }
    #[doc = "Bits 24:27 - Calibration Up-counter Select"]
    #[inline(always)]
    #[must_use]
    pub fn upsel(&mut self) -> UPSEL_W<24> {
        UPSEL_W::new(self)
    }
    #[doc = "Bits 28:31 - Calibration Down-counter Select"]
    #[inline(always)]
    #[must_use]
    pub fn downsel(&mut self) -> DOWNSEL_W<28> {
        DOWNSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calctrl](index.html) module"]
pub struct CALCTRL_SPEC;
impl crate::RegisterSpec for CALCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calctrl::R](R) reader structure"]
impl crate::Readable for CALCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calctrl::W](W) writer structure"]
impl crate::Writable for CALCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALCTRL to value 0"]
impl crate::Resettable for CALCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
