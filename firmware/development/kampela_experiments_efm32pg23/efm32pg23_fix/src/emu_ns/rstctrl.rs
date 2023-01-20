#[doc = "Register `RSTCTRL` reader"]
pub struct R(crate::R<RSTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCTRL` writer"]
pub struct W(crate::W<RSTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCTRL_SPEC>;
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
impl From<crate::W<RSTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOG0RMODE` reader - Enable WDOG0 reset"]
pub type WDOG0RMODE_R = crate::BitReader<WDOG0RMODE_A>;
#[doc = "Enable WDOG0 reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDOG0RMODE_A {
    #[doc = "0: Reset request is blocked"]
    DISABLED = 0,
    #[doc = "1: The entire device is reset except some EMU registers"]
    ENABLED = 1,
}
impl From<WDOG0RMODE_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG0RMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl WDOG0RMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG0RMODE_A {
        match self.bits {
            false => WDOG0RMODE_A::DISABLED,
            true => WDOG0RMODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDOG0RMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDOG0RMODE_A::ENABLED
    }
}
#[doc = "Field `WDOG0RMODE` writer - Enable WDOG0 reset"]
pub type WDOG0RMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTRL_SPEC, WDOG0RMODE_A, O>;
impl<'a, const O: u8> WDOG0RMODE_W<'a, O> {
    #[doc = "Reset request is blocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDOG0RMODE_A::DISABLED)
    }
    #[doc = "The entire device is reset except some EMU registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDOG0RMODE_A::ENABLED)
    }
}
#[doc = "Field `SYSRMODE` reader - Enable M33 System reset"]
pub type SYSRMODE_R = crate::BitReader<SYSRMODE_A>;
#[doc = "Enable M33 System reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSRMODE_A {
    #[doc = "0: Reset request is blocked"]
    DISABLED = 0,
    #[doc = "1: Device is reset except some EMU registers"]
    ENABLED = 1,
}
impl From<SYSRMODE_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRMODE_A {
        match self.bits {
            false => SYSRMODE_A::DISABLED,
            true => SYSRMODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSRMODE_A::ENABLED
    }
}
#[doc = "Field `SYSRMODE` writer - Enable M33 System reset"]
pub type SYSRMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTRL_SPEC, SYSRMODE_A, O>;
impl<'a, const O: u8> SYSRMODE_W<'a, O> {
    #[doc = "Reset request is blocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSRMODE_A::DISABLED)
    }
    #[doc = "Device is reset except some EMU registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSRMODE_A::ENABLED)
    }
}
#[doc = "Field `LOCKUPRMODE` reader - Enable M33 Lockup reset"]
pub type LOCKUPRMODE_R = crate::BitReader<LOCKUPRMODE_A>;
#[doc = "Enable M33 Lockup reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKUPRMODE_A {
    #[doc = "0: Reset Request is Block"]
    DISABLED = 0,
    #[doc = "1: The entire device is reset except some EMU registers"]
    ENABLED = 1,
}
impl From<LOCKUPRMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUPRMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCKUPRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUPRMODE_A {
        match self.bits {
            false => LOCKUPRMODE_A::DISABLED,
            true => LOCKUPRMODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOCKUPRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOCKUPRMODE_A::ENABLED
    }
}
#[doc = "Field `LOCKUPRMODE` writer - Enable M33 Lockup reset"]
pub type LOCKUPRMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTRL_SPEC, LOCKUPRMODE_A, O>;
impl<'a, const O: u8> LOCKUPRMODE_W<'a, O> {
    #[doc = "Reset Request is Block"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCKUPRMODE_A::DISABLED)
    }
    #[doc = "The entire device is reset except some EMU registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOCKUPRMODE_A::ENABLED)
    }
}
#[doc = "Field `AVDDBODRMODE` reader - Enable AVDD BOD reset"]
pub type AVDDBODRMODE_R = crate::BitReader<AVDDBODRMODE_A>;
#[doc = "Enable AVDD BOD reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVDDBODRMODE_A {
    #[doc = "0: Reset Request is block"]
    DISABLED = 0,
    #[doc = "1: The entire device is reset except some EMU registers"]
    ENABLED = 1,
}
impl From<AVDDBODRMODE_A> for bool {
    #[inline(always)]
    fn from(variant: AVDDBODRMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl AVDDBODRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVDDBODRMODE_A {
        match self.bits {
            false => AVDDBODRMODE_A::DISABLED,
            true => AVDDBODRMODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AVDDBODRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AVDDBODRMODE_A::ENABLED
    }
}
#[doc = "Field `AVDDBODRMODE` writer - Enable AVDD BOD reset"]
pub type AVDDBODRMODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RSTCTRL_SPEC, AVDDBODRMODE_A, O>;
impl<'a, const O: u8> AVDDBODRMODE_W<'a, O> {
    #[doc = "Reset Request is block"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AVDDBODRMODE_A::DISABLED)
    }
    #[doc = "The entire device is reset except some EMU registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AVDDBODRMODE_A::ENABLED)
    }
}
#[doc = "Field `IOVDD0BODRMODE` reader - Enable VDDIO0 BOD reset"]
pub type IOVDD0BODRMODE_R = crate::BitReader<IOVDD0BODRMODE_A>;
#[doc = "Enable VDDIO0 BOD reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOVDD0BODRMODE_A {
    #[doc = "0: Reset request is blocked"]
    DISABLED = 0,
    #[doc = "1: The entire device is reset except some EMU registers"]
    ENABLED = 1,
}
impl From<IOVDD0BODRMODE_A> for bool {
    #[inline(always)]
    fn from(variant: IOVDD0BODRMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl IOVDD0BODRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOVDD0BODRMODE_A {
        match self.bits {
            false => IOVDD0BODRMODE_A::DISABLED,
            true => IOVDD0BODRMODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOVDD0BODRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOVDD0BODRMODE_A::ENABLED
    }
}
#[doc = "Field `IOVDD0BODRMODE` writer - Enable VDDIO0 BOD reset"]
pub type IOVDD0BODRMODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RSTCTRL_SPEC, IOVDD0BODRMODE_A, O>;
impl<'a, const O: u8> IOVDD0BODRMODE_W<'a, O> {
    #[doc = "Reset request is blocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOVDD0BODRMODE_A::DISABLED)
    }
    #[doc = "The entire device is reset except some EMU registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOVDD0BODRMODE_A::ENABLED)
    }
}
#[doc = "Field `DECBODRMODE` reader - Enable DECBOD reset"]
pub type DECBODRMODE_R = crate::BitReader<DECBODRMODE_A>;
#[doc = "Enable DECBOD reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DECBODRMODE_A {
    #[doc = "0: Reset request is blocked"]
    DISABLED = 0,
    #[doc = "1: The entire device is reset"]
    ENABLED = 1,
}
impl From<DECBODRMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DECBODRMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DECBODRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECBODRMODE_A {
        match self.bits {
            false => DECBODRMODE_A::DISABLED,
            true => DECBODRMODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DECBODRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DECBODRMODE_A::ENABLED
    }
}
#[doc = "Field `DECBODRMODE` writer - Enable DECBOD reset"]
pub type DECBODRMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTRL_SPEC, DECBODRMODE_A, O>;
impl<'a, const O: u8> DECBODRMODE_W<'a, O> {
    #[doc = "Reset request is blocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DECBODRMODE_A::DISABLED)
    }
    #[doc = "The entire device is reset"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DECBODRMODE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable WDOG0 reset"]
    #[inline(always)]
    pub fn wdog0rmode(&self) -> WDOG0RMODE_R {
        WDOG0RMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enable M33 System reset"]
    #[inline(always)]
    pub fn sysrmode(&self) -> SYSRMODE_R {
        SYSRMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable M33 Lockup reset"]
    #[inline(always)]
    pub fn lockuprmode(&self) -> LOCKUPRMODE_R {
        LOCKUPRMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable AVDD BOD reset"]
    #[inline(always)]
    pub fn avddbodrmode(&self) -> AVDDBODRMODE_R {
        AVDDBODRMODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable VDDIO0 BOD reset"]
    #[inline(always)]
    pub fn iovdd0bodrmode(&self) -> IOVDD0BODRMODE_R {
        IOVDD0BODRMODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable DECBOD reset"]
    #[inline(always)]
    pub fn decbodrmode(&self) -> DECBODRMODE_R {
        DECBODRMODE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable WDOG0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn wdog0rmode(&mut self) -> WDOG0RMODE_W<0> {
        WDOG0RMODE_W::new(self)
    }
    #[doc = "Bit 2 - Enable M33 System reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysrmode(&mut self) -> SYSRMODE_W<2> {
        SYSRMODE_W::new(self)
    }
    #[doc = "Bit 3 - Enable M33 Lockup reset"]
    #[inline(always)]
    #[must_use]
    pub fn lockuprmode(&mut self) -> LOCKUPRMODE_W<3> {
        LOCKUPRMODE_W::new(self)
    }
    #[doc = "Bit 6 - Enable AVDD BOD reset"]
    #[inline(always)]
    #[must_use]
    pub fn avddbodrmode(&mut self) -> AVDDBODRMODE_W<6> {
        AVDDBODRMODE_W::new(self)
    }
    #[doc = "Bit 7 - Enable VDDIO0 BOD reset"]
    #[inline(always)]
    #[must_use]
    pub fn iovdd0bodrmode(&mut self) -> IOVDD0BODRMODE_W<7> {
        IOVDD0BODRMODE_W::new(self)
    }
    #[doc = "Bit 10 - Enable DECBOD reset"]
    #[inline(always)]
    #[must_use]
    pub fn decbodrmode(&mut self) -> DECBODRMODE_W<10> {
        DECBODRMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctrl](index.html) module"]
pub struct RSTCTRL_SPEC;
impl crate::RegisterSpec for RSTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctrl::R](R) reader structure"]
impl crate::Readable for RSTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstctrl::W](W) writer structure"]
impl crate::Writable for RSTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTCTRL to value 0x0006_0407"]
impl crate::Resettable for RSTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0006_0407;
}
