#[doc = "Register `CFG2` reader"]
pub struct R(crate::R<CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG2` writer"]
pub struct W(crate::W<CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG2_SPEC>;
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
impl From<crate::W<CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTER` reader - Main mode"]
pub type MASTER_R = crate::BitReader<MASTER_A>;
#[doc = "Main mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER_A {
    #[doc = "0: Secondary mode"]
    SLAVE = 0,
    #[doc = "1: Main mode"]
    MASTER = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
impl MASTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::SLAVE,
            true => MASTER_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MASTER_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MASTER_A::MASTER
    }
}
#[doc = "Field `MASTER` writer - Main mode"]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, MASTER_A, O>;
impl<'a, const O: u8> MASTER_W<'a, O> {
    #[doc = "Secondary mode"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MASTER_A::SLAVE)
    }
    #[doc = "Main mode"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MASTER_A::MASTER)
    }
}
#[doc = "Field `CLKPOL` reader - Clock Polarity"]
pub type CLKPOL_R = crate::BitReader<CLKPOL_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKPOL_A {
    #[doc = "0: The bus clock used in synchronous mode has a low base value"]
    IDLELOW = 0,
    #[doc = "1: The bus clock used in synchronous mode has a high base value"]
    IDLEHIGH = 1,
}
impl From<CLKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKPOL_A {
        match self.bits {
            false => CLKPOL_A::IDLELOW,
            true => CLKPOL_A::IDLEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLELOW`"]
    #[inline(always)]
    pub fn is_idlelow(&self) -> bool {
        *self == CLKPOL_A::IDLELOW
    }
    #[doc = "Checks if the value of the field is `IDLEHIGH`"]
    #[inline(always)]
    pub fn is_idlehigh(&self) -> bool {
        *self == CLKPOL_A::IDLEHIGH
    }
}
#[doc = "Field `CLKPOL` writer - Clock Polarity"]
pub type CLKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, CLKPOL_A, O>;
impl<'a, const O: u8> CLKPOL_W<'a, O> {
    #[doc = "The bus clock used in synchronous mode has a low base value"]
    #[inline(always)]
    pub fn idlelow(self) -> &'a mut W {
        self.variant(CLKPOL_A::IDLELOW)
    }
    #[doc = "The bus clock used in synchronous mode has a high base value"]
    #[inline(always)]
    pub fn idlehigh(self) -> &'a mut W {
        self.variant(CLKPOL_A::IDLEHIGH)
    }
}
#[doc = "Field `CLKPHA` reader - Clock Edge for Setup/Sample"]
pub type CLKPHA_R = crate::BitReader<CLKPHA_A>;
#[doc = "Clock Edge for Setup/Sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKPHA_A {
    #[doc = "0: Data is sampled on the leading edge and set-up on the trailing edge of the bus clock in synchronous mode"]
    SAMPLELEADING = 0,
    #[doc = "1: Data is set-up on the leading edge and sampled on the trailing edge of the bus clock in synchronous mode"]
    SAMPLETRAILING = 1,
}
impl From<CLKPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKPHA_A {
        match self.bits {
            false => CLKPHA_A::SAMPLELEADING,
            true => CLKPHA_A::SAMPLETRAILING,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLELEADING`"]
    #[inline(always)]
    pub fn is_sampleleading(&self) -> bool {
        *self == CLKPHA_A::SAMPLELEADING
    }
    #[doc = "Checks if the value of the field is `SAMPLETRAILING`"]
    #[inline(always)]
    pub fn is_sampletrailing(&self) -> bool {
        *self == CLKPHA_A::SAMPLETRAILING
    }
}
#[doc = "Field `CLKPHA` writer - Clock Edge for Setup/Sample"]
pub type CLKPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, CLKPHA_A, O>;
impl<'a, const O: u8> CLKPHA_W<'a, O> {
    #[doc = "Data is sampled on the leading edge and set-up on the trailing edge of the bus clock in synchronous mode"]
    #[inline(always)]
    pub fn sampleleading(self) -> &'a mut W {
        self.variant(CLKPHA_A::SAMPLELEADING)
    }
    #[doc = "Data is set-up on the leading edge and sampled on the trailing edge of the bus clock in synchronous mode"]
    #[inline(always)]
    pub fn sampletrailing(self) -> &'a mut W {
        self.variant(CLKPHA_A::SAMPLETRAILING)
    }
}
#[doc = "Field `CSINV` reader - Chip Select Invert"]
pub type CSINV_R = crate::BitReader<CSINV_A>;
#[doc = "Chip Select Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSINV_A {
    #[doc = "0: Chip select is active low"]
    AL = 0,
    #[doc = "1: Chip select is active high"]
    AH = 1,
}
impl From<CSINV_A> for bool {
    #[inline(always)]
    fn from(variant: CSINV_A) -> Self {
        variant as u8 != 0
    }
}
impl CSINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSINV_A {
        match self.bits {
            false => CSINV_A::AL,
            true => CSINV_A::AH,
        }
    }
    #[doc = "Checks if the value of the field is `AL`"]
    #[inline(always)]
    pub fn is_al(&self) -> bool {
        *self == CSINV_A::AL
    }
    #[doc = "Checks if the value of the field is `AH`"]
    #[inline(always)]
    pub fn is_ah(&self) -> bool {
        *self == CSINV_A::AH
    }
}
#[doc = "Field `CSINV` writer - Chip Select Invert"]
pub type CSINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, CSINV_A, O>;
impl<'a, const O: u8> CSINV_W<'a, O> {
    #[doc = "Chip select is active low"]
    #[inline(always)]
    pub fn al(self) -> &'a mut W {
        self.variant(CSINV_A::AL)
    }
    #[doc = "Chip select is active high"]
    #[inline(always)]
    pub fn ah(self) -> &'a mut W {
        self.variant(CSINV_A::AH)
    }
}
#[doc = "Field `AUTOTX` reader - Always Transmit When RXFIFO Not Full"]
pub type AUTOTX_R = crate::BitReader<bool>;
#[doc = "Field `AUTOTX` writer - Always Transmit When RXFIFO Not Full"]
pub type AUTOTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
#[doc = "Field `AUTOCS` reader - Automatic Chip Select"]
pub type AUTOCS_R = crate::BitReader<bool>;
#[doc = "Field `AUTOCS` writer - Automatic Chip Select"]
pub type AUTOCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
#[doc = "Field `CLKPRSEN` reader - PRS CLK Enable"]
pub type CLKPRSEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKPRSEN` writer - PRS CLK Enable"]
pub type CLKPRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
#[doc = "Field `FORCELOAD` reader - Force Load to Shift Register"]
pub type FORCELOAD_R = crate::BitReader<bool>;
#[doc = "Field `FORCELOAD` writer - Force Load to Shift Register"]
pub type FORCELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
#[doc = "Field `SDIV` reader - Sync Clock Div"]
pub type SDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDIV` writer - Sync Clock Div"]
pub type SDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Main mode"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Edge for Setup/Sample"]
    #[inline(always)]
    pub fn clkpha(&self) -> CLKPHA_R {
        CLKPHA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Chip Select Invert"]
    #[inline(always)]
    pub fn csinv(&self) -> CSINV_R {
        CSINV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Always Transmit When RXFIFO Not Full"]
    #[inline(always)]
    pub fn autotx(&self) -> AUTOTX_R {
        AUTOTX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Chip Select"]
    #[inline(always)]
    pub fn autocs(&self) -> AUTOCS_R {
        AUTOCS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PRS CLK Enable"]
    #[inline(always)]
    pub fn clkprsen(&self) -> CLKPRSEN_R {
        CLKPRSEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Load to Shift Register"]
    #[inline(always)]
    pub fn forceload(&self) -> FORCELOAD_R {
        FORCELOAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Sync Clock Div"]
    #[inline(always)]
    pub fn sdiv(&self) -> SDIV_R {
        SDIV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Main mode"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<0> {
        MASTER_W::new(self)
    }
    #[doc = "Bit 1 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<1> {
        CLKPOL_W::new(self)
    }
    #[doc = "Bit 2 - Clock Edge for Setup/Sample"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> CLKPHA_W<2> {
        CLKPHA_W::new(self)
    }
    #[doc = "Bit 3 - Chip Select Invert"]
    #[inline(always)]
    #[must_use]
    pub fn csinv(&mut self) -> CSINV_W<3> {
        CSINV_W::new(self)
    }
    #[doc = "Bit 4 - Always Transmit When RXFIFO Not Full"]
    #[inline(always)]
    #[must_use]
    pub fn autotx(&mut self) -> AUTOTX_W<4> {
        AUTOTX_W::new(self)
    }
    #[doc = "Bit 5 - Automatic Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn autocs(&mut self) -> AUTOCS_W<5> {
        AUTOCS_W::new(self)
    }
    #[doc = "Bit 6 - PRS CLK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkprsen(&mut self) -> CLKPRSEN_W<6> {
        CLKPRSEN_W::new(self)
    }
    #[doc = "Bit 7 - Force Load to Shift Register"]
    #[inline(always)]
    #[must_use]
    pub fn forceload(&mut self) -> FORCELOAD_W<7> {
        FORCELOAD_W::new(self)
    }
    #[doc = "Bits 24:31 - Sync Clock Div"]
    #[inline(always)]
    #[must_use]
    pub fn sdiv(&mut self) -> SDIV_W<24> {
        SDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](index.html) module"]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg2::R](R) reader structure"]
impl crate::Readable for CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg2::W](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0x20"]
impl crate::Resettable for CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
