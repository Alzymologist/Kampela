#[doc = "Register `IRHFCFG` reader"]
pub struct R(crate::R<IRHFCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRHFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRHFCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRHFCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRHFCFG` writer"]
pub struct W(crate::W<IRHFCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRHFCFG_SPEC>;
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
impl From<crate::W<IRHFCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRHFCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRHFEN` reader - Enable IrDA Module"]
pub type IRHFEN_R = crate::BitReader<bool>;
#[doc = "Field `IRHFEN` writer - Enable IrDA Module"]
pub type IRHFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRHFCFG_SPEC, bool, O>;
#[doc = "Field `IRHFPW` reader - IrDA TX Pulse Width"]
pub type IRHFPW_R = crate::FieldReader<u8, IRHFPW_A>;
#[doc = "IrDA TX Pulse Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRHFPW_A {
    #[doc = "0: IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    ONE = 0,
    #[doc = "1: IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    TWO = 1,
    #[doc = "2: IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    THREE = 2,
    #[doc = "3: IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    FOUR = 3,
}
impl From<IRHFPW_A> for u8 {
    #[inline(always)]
    fn from(variant: IRHFPW_A) -> Self {
        variant as _
    }
}
impl IRHFPW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRHFPW_A {
        match self.bits {
            0 => IRHFPW_A::ONE,
            1 => IRHFPW_A::TWO,
            2 => IRHFPW_A::THREE,
            3 => IRHFPW_A::FOUR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == IRHFPW_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == IRHFPW_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == IRHFPW_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == IRHFPW_A::FOUR
    }
}
#[doc = "Field `IRHFPW` writer - IrDA TX Pulse Width"]
pub type IRHFPW_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, IRHFCFG_SPEC, u8, IRHFPW_A, 2, O>;
impl<'a, const O: u8> IRHFPW_W<'a, O> {
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(IRHFPW_A::ONE)
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(IRHFPW_A::TWO)
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(IRHFPW_A::THREE)
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(IRHFPW_A::FOUR)
    }
}
#[doc = "Field `IRHFFILT` reader - IrDA RX Filter"]
pub type IRHFFILT_R = crate::BitReader<IRHFFILT_A>;
#[doc = "IrDA RX Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRHFFILT_A {
    #[doc = "0: No filter enabled"]
    DISABLE = 0,
    #[doc = "1: Filter enabled. IrDA pulse must be high for at least 5 consecutive clock cycles to be detected"]
    ENABLE = 1,
}
impl From<IRHFFILT_A> for bool {
    #[inline(always)]
    fn from(variant: IRHFFILT_A) -> Self {
        variant as u8 != 0
    }
}
impl IRHFFILT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRHFFILT_A {
        match self.bits {
            false => IRHFFILT_A::DISABLE,
            true => IRHFFILT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IRHFFILT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IRHFFILT_A::ENABLE
    }
}
#[doc = "Field `IRHFFILT` writer - IrDA RX Filter"]
pub type IRHFFILT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRHFCFG_SPEC, IRHFFILT_A, O>;
impl<'a, const O: u8> IRHFFILT_W<'a, O> {
    #[doc = "No filter enabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IRHFFILT_A::DISABLE)
    }
    #[doc = "Filter enabled. IrDA pulse must be high for at least 5 consecutive clock cycles to be detected"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IRHFFILT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn irhfen(&self) -> IRHFEN_R {
        IRHFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irhfpw(&self) -> IRHFPW_R {
        IRHFPW_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irhffilt(&self) -> IRHFFILT_R {
        IRHFFILT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    #[must_use]
    pub fn irhfen(&mut self) -> IRHFEN_W<0> {
        IRHFEN_W::new(self)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn irhfpw(&mut self) -> IRHFPW_W<1> {
        IRHFPW_W::new(self)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    #[must_use]
    pub fn irhffilt(&mut self) -> IRHFFILT_W<3> {
        IRHFFILT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irhfcfg](index.html) module"]
pub struct IRHFCFG_SPEC;
impl crate::RegisterSpec for IRHFCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irhfcfg::R](R) reader structure"]
impl crate::Readable for IRHFCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irhfcfg::W](W) writer structure"]
impl crate::Writable for IRHFCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRHFCFG to value 0"]
impl crate::Resettable for IRHFCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
