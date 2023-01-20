#[doc = "Register `IRCTRL` reader"]
pub struct R(crate::R<IRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRCTRL` writer"]
pub struct W(crate::W<IRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRCTRL_SPEC>;
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
impl From<crate::W<IRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IREN` reader - Enable IrDA Module"]
pub type IREN_R = crate::BitReader<bool>;
#[doc = "Field `IREN` writer - Enable IrDA Module"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRCTRL_SPEC, bool, O>;
#[doc = "Field `IRPW` reader - IrDA TX Pulse Width"]
pub type IRPW_R = crate::FieldReader<u8, IRPW_A>;
#[doc = "IrDA TX Pulse Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRPW_A {
    #[doc = "0: IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    ONE = 0,
    #[doc = "1: IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    TWO = 1,
    #[doc = "2: IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    THREE = 2,
    #[doc = "3: IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    FOUR = 3,
}
impl From<IRPW_A> for u8 {
    #[inline(always)]
    fn from(variant: IRPW_A) -> Self {
        variant as _
    }
}
impl IRPW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRPW_A {
        match self.bits {
            0 => IRPW_A::ONE,
            1 => IRPW_A::TWO,
            2 => IRPW_A::THREE,
            3 => IRPW_A::FOUR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == IRPW_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == IRPW_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == IRPW_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == IRPW_A::FOUR
    }
}
#[doc = "Field `IRPW` writer - IrDA TX Pulse Width"]
pub type IRPW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IRCTRL_SPEC, u8, IRPW_A, 2, O>;
impl<'a, const O: u8> IRPW_W<'a, O> {
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(IRPW_A::ONE)
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(IRPW_A::TWO)
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(IRPW_A::THREE)
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(IRPW_A::FOUR)
    }
}
#[doc = "Field `IRFILT` reader - IrDA RX Filter"]
pub type IRFILT_R = crate::BitReader<IRFILT_A>;
#[doc = "IrDA RX Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRFILT_A {
    #[doc = "0: No filter enabled"]
    DISABLE = 0,
    #[doc = "1: Filter enabled. IrDA pulse must be high for at least 5 consecutive clock cycles to be detected"]
    ENABLE = 1,
}
impl From<IRFILT_A> for bool {
    #[inline(always)]
    fn from(variant: IRFILT_A) -> Self {
        variant as u8 != 0
    }
}
impl IRFILT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRFILT_A {
        match self.bits {
            false => IRFILT_A::DISABLE,
            true => IRFILT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IRFILT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IRFILT_A::ENABLE
    }
}
#[doc = "Field `IRFILT` writer - IrDA RX Filter"]
pub type IRFILT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRCTRL_SPEC, IRFILT_A, O>;
impl<'a, const O: u8> IRFILT_W<'a, O> {
    #[doc = "No filter enabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IRFILT_A::DISABLE)
    }
    #[doc = "Filter enabled. IrDA pulse must be high for at least 5 consecutive clock cycles to be detected"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IRFILT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irpw(&self) -> IRPW_R {
        IRPW_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irfilt(&self) -> IRFILT_R {
        IRFILT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<0> {
        IREN_W::new(self)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn irpw(&mut self) -> IRPW_W<1> {
        IRPW_W::new(self)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    #[must_use]
    pub fn irfilt(&mut self) -> IRFILT_W<3> {
        IRFILT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irctrl](index.html) module"]
pub struct IRCTRL_SPEC;
impl crate::RegisterSpec for IRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irctrl::R](R) reader structure"]
impl crate::Readable for IRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irctrl::W](W) writer structure"]
impl crate::Writable for IRCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRCTRL to value 0"]
impl crate::Resettable for IRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
