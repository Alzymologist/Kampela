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
#[doc = "Field `SYNC` reader - USART Synchronous Mode"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
#[doc = "USART Synchronous Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    #[doc = "0: The USART operates in asynchronous mode"]
    DISABLE = 0,
    #[doc = "1: The USART operates in synchronous mode"]
    ENABLE = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::DISABLE,
            true => SYNC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SYNC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SYNC_A::ENABLE
    }
}
#[doc = "Field `SYNC` writer - USART Synchronous Mode"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, SYNC_A, O>;
impl<'a, const O: u8> SYNC_W<'a, O> {
    #[doc = "The USART operates in asynchronous mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYNC_A::DISABLE)
    }
    #[doc = "The USART operates in synchronous mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYNC_A::ENABLE)
    }
}
#[doc = "Field `LOOPBK` reader - Loopback Enable"]
pub type LOOPBK_R = crate::BitReader<LOOPBK_A>;
#[doc = "Loopback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPBK_A {
    #[doc = "0: The receiver is connected to and receives data from U(S)n_RX"]
    DISABLE = 0,
    #[doc = "1: The receiver is connected to and receives data from U(S)n_TX"]
    ENABLE = 1,
}
impl From<LOOPBK_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPBK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPBK_A {
        match self.bits {
            false => LOOPBK_A::DISABLE,
            true => LOOPBK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOOPBK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LOOPBK_A::ENABLE
    }
}
#[doc = "Field `LOOPBK` writer - Loopback Enable"]
pub type LOOPBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, LOOPBK_A, O>;
impl<'a, const O: u8> LOOPBK_W<'a, O> {
    #[doc = "The receiver is connected to and receives data from U(S)n_RX"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOOPBK_A::DISABLE)
    }
    #[doc = "The receiver is connected to and receives data from U(S)n_TX"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOOPBK_A::ENABLE)
    }
}
#[doc = "Field `CCEN` reader - Collision Check Enable"]
pub type CCEN_R = crate::BitReader<CCEN_A>;
#[doc = "Collision Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCEN_A {
    #[doc = "0: Collision check is disabled"]
    DISABLE = 0,
    #[doc = "1: Collision check is enabled. The receiver must be enabled for the check to be performed"]
    ENABLE = 1,
}
impl From<CCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCEN_A {
        match self.bits {
            false => CCEN_A::DISABLE,
            true => CCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CCEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CCEN_A::ENABLE
    }
}
#[doc = "Field `CCEN` writer - Collision Check Enable"]
pub type CCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CCEN_A, O>;
impl<'a, const O: u8> CCEN_W<'a, O> {
    #[doc = "Collision check is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CCEN_A::DISABLE)
    }
    #[doc = "Collision check is enabled. The receiver must be enabled for the check to be performed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CCEN_A::ENABLE)
    }
}
#[doc = "Field `MPM` reader - Multi-Processor Mode"]
pub type MPM_R = crate::BitReader<MPM_A>;
#[doc = "Multi-Processor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPM_A {
    #[doc = "0: The 9th bit of incoming frames has no special function"]
    DISABLE = 0,
    #[doc = "1: An incoming frame with the 9th bit equal to MPAB will be loaded into the receive buffer regardless of RXBLOCK and will result in the MPAB interrupt flag being set"]
    ENABLE = 1,
}
impl From<MPM_A> for bool {
    #[inline(always)]
    fn from(variant: MPM_A) -> Self {
        variant as u8 != 0
    }
}
impl MPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPM_A {
        match self.bits {
            false => MPM_A::DISABLE,
            true => MPM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPM_A::ENABLE
    }
}
#[doc = "Field `MPM` writer - Multi-Processor Mode"]
pub type MPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, MPM_A, O>;
impl<'a, const O: u8> MPM_W<'a, O> {
    #[doc = "The 9th bit of incoming frames has no special function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPM_A::DISABLE)
    }
    #[doc = "An incoming frame with the 9th bit equal to MPAB will be loaded into the receive buffer regardless of RXBLOCK and will result in the MPAB interrupt flag being set"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPM_A::ENABLE)
    }
}
#[doc = "Field `MPAB` reader - Multi-Processor Address-Bit"]
pub type MPAB_R = crate::BitReader<bool>;
#[doc = "Field `MPAB` writer - Multi-Processor Address-Bit"]
pub type MPAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OVS` reader - Oversampling"]
pub type OVS_R = crate::FieldReader<u8, OVS_A>;
#[doc = "Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVS_A {
    #[doc = "0: Regular UART mode with 16X oversampling in asynchronous mode"]
    X16 = 0,
    #[doc = "1: Double speed with 8X oversampling in asynchronous mode"]
    X8 = 1,
    #[doc = "2: 6X oversampling in asynchronous mode"]
    X6 = 2,
    #[doc = "3: Quadruple speed with 4X oversampling in asynchronous mode"]
    X4 = 3,
}
impl From<OVS_A> for u8 {
    #[inline(always)]
    fn from(variant: OVS_A) -> Self {
        variant as _
    }
}
impl OVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVS_A {
        match self.bits {
            0 => OVS_A::X16,
            1 => OVS_A::X8,
            2 => OVS_A::X6,
            3 => OVS_A::X4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == OVS_A::X16
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == OVS_A::X8
    }
    #[doc = "Checks if the value of the field is `X6`"]
    #[inline(always)]
    pub fn is_x6(&self) -> bool {
        *self == OVS_A::X6
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == OVS_A::X4
    }
}
#[doc = "Field `OVS` writer - Oversampling"]
pub type OVS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, OVS_A, 2, O>;
impl<'a, const O: u8> OVS_W<'a, O> {
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVS_A::X16)
    }
    #[doc = "Double speed with 8X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVS_A::X8)
    }
    #[doc = "6X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x6(self) -> &'a mut W {
        self.variant(OVS_A::X6)
    }
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(OVS_A::X4)
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
pub type CLKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CLKPOL_A, O>;
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
#[doc = "Field `CLKPHA` reader - Clock Edge For Setup/Sample"]
pub type CLKPHA_R = crate::BitReader<CLKPHA_A>;
#[doc = "Clock Edge For Setup/Sample\n\nValue on reset: 0"]
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
#[doc = "Field `CLKPHA` writer - Clock Edge For Setup/Sample"]
pub type CLKPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CLKPHA_A, O>;
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
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MSBF_R = crate::BitReader<MSBF_A>;
#[doc = "Most Significant Bit First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBF_A {
    #[doc = "0: Data is sent with the least significant bit first"]
    DISABLE = 0,
    #[doc = "1: Data is sent with the most significant bit first"]
    ENABLE = 1,
}
impl From<MSBF_A> for bool {
    #[inline(always)]
    fn from(variant: MSBF_A) -> Self {
        variant as u8 != 0
    }
}
impl MSBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBF_A {
        match self.bits {
            false => MSBF_A::DISABLE,
            true => MSBF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MSBF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MSBF_A::ENABLE
    }
}
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, MSBF_A, O>;
impl<'a, const O: u8> MSBF_W<'a, O> {
    #[doc = "Data is sent with the least significant bit first"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MSBF_A::DISABLE)
    }
    #[doc = "Data is sent with the most significant bit first"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MSBF_A::ENABLE)
    }
}
#[doc = "Field `CSMA` reader - Action On Chip Select In Main Mode"]
pub type CSMA_R = crate::BitReader<CSMA_A>;
#[doc = "Action On Chip Select In Main Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSMA_A {
    #[doc = "0: No action taken"]
    NOACTION = 0,
    #[doc = "1: Go to secondary mode"]
    GOTOSLAVEMODE = 1,
}
impl From<CSMA_A> for bool {
    #[inline(always)]
    fn from(variant: CSMA_A) -> Self {
        variant as u8 != 0
    }
}
impl CSMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSMA_A {
        match self.bits {
            false => CSMA_A::NOACTION,
            true => CSMA_A::GOTOSLAVEMODE,
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_noaction(&self) -> bool {
        *self == CSMA_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `GOTOSLAVEMODE`"]
    #[inline(always)]
    pub fn is_gotoslavemode(&self) -> bool {
        *self == CSMA_A::GOTOSLAVEMODE
    }
}
#[doc = "Field `CSMA` writer - Action On Chip Select In Main Mode"]
pub type CSMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CSMA_A, O>;
impl<'a, const O: u8> CSMA_W<'a, O> {
    #[doc = "No action taken"]
    #[inline(always)]
    pub fn noaction(self) -> &'a mut W {
        self.variant(CSMA_A::NOACTION)
    }
    #[doc = "Go to secondary mode"]
    #[inline(always)]
    pub fn gotoslavemode(self) -> &'a mut W {
        self.variant(CSMA_A::GOTOSLAVEMODE)
    }
}
#[doc = "Field `TXBIL` reader - TX Buffer Interrupt Level"]
pub type TXBIL_R = crate::BitReader<TXBIL_A>;
#[doc = "TX Buffer Interrupt Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXBIL_A {
    #[doc = "0: TXBL and the TXBL interrupt flag are set when the transmit buffer becomes empty. TXBL is cleared when the buffer becomes nonempty."]
    EMPTY = 0,
    #[doc = "1: TXBL and TXBLIF are set when the transmit buffer goes from full to half-full or empty. TXBL is cleared when the buffer becomes full."]
    HALFFULL = 1,
}
impl From<TXBIL_A> for bool {
    #[inline(always)]
    fn from(variant: TXBIL_A) -> Self {
        variant as u8 != 0
    }
}
impl TXBIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXBIL_A {
        match self.bits {
            false => TXBIL_A::EMPTY,
            true => TXBIL_A::HALFFULL,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXBIL_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `HALFFULL`"]
    #[inline(always)]
    pub fn is_halffull(&self) -> bool {
        *self == TXBIL_A::HALFFULL
    }
}
#[doc = "Field `TXBIL` writer - TX Buffer Interrupt Level"]
pub type TXBIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, TXBIL_A, O>;
impl<'a, const O: u8> TXBIL_W<'a, O> {
    #[doc = "TXBL and the TXBL interrupt flag are set when the transmit buffer becomes empty. TXBL is cleared when the buffer becomes nonempty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXBIL_A::EMPTY)
    }
    #[doc = "TXBL and TXBLIF are set when the transmit buffer goes from full to half-full or empty. TXBL is cleared when the buffer becomes full."]
    #[inline(always)]
    pub fn halffull(self) -> &'a mut W {
        self.variant(TXBIL_A::HALFFULL)
    }
}
#[doc = "Field `RXINV` reader - Receiver Input Invert"]
pub type RXINV_R = crate::BitReader<RXINV_A>;
#[doc = "Receiver Input Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXINV_A {
    #[doc = "0: Input is passed directly to the receiver"]
    DISABLE = 0,
    #[doc = "1: Input is inverted before it is passed to the receiver"]
    ENABLE = 1,
}
impl From<RXINV_A> for bool {
    #[inline(always)]
    fn from(variant: RXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl RXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXINV_A {
        match self.bits {
            false => RXINV_A::DISABLE,
            true => RXINV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXINV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXINV_A::ENABLE
    }
}
#[doc = "Field `RXINV` writer - Receiver Input Invert"]
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RXINV_A, O>;
impl<'a, const O: u8> RXINV_W<'a, O> {
    #[doc = "Input is passed directly to the receiver"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXINV_A::DISABLE)
    }
    #[doc = "Input is inverted before it is passed to the receiver"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXINV_A::ENABLE)
    }
}
#[doc = "Field `TXINV` reader - Transmitter output Invert"]
pub type TXINV_R = crate::BitReader<TXINV_A>;
#[doc = "Transmitter output Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXINV_A {
    #[doc = "0: Output from the transmitter is passed unchanged to U(S)n_TX"]
    DISABLE = 0,
    #[doc = "1: Output from the transmitter is inverted before it is passed to U(S)n_TX"]
    ENABLE = 1,
}
impl From<TXINV_A> for bool {
    #[inline(always)]
    fn from(variant: TXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl TXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXINV_A {
        match self.bits {
            false => TXINV_A::DISABLE,
            true => TXINV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXINV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXINV_A::ENABLE
    }
}
#[doc = "Field `TXINV` writer - Transmitter output Invert"]
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, TXINV_A, O>;
impl<'a, const O: u8> TXINV_W<'a, O> {
    #[doc = "Output from the transmitter is passed unchanged to U(S)n_TX"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXINV_A::DISABLE)
    }
    #[doc = "Output from the transmitter is inverted before it is passed to U(S)n_TX"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXINV_A::ENABLE)
    }
}
#[doc = "Field `CSINV` reader - Chip Select Invert"]
pub type CSINV_R = crate::BitReader<CSINV_A>;
#[doc = "Chip Select Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSINV_A {
    #[doc = "0: Chip select is active low"]
    DISABLE = 0,
    #[doc = "1: Chip select is active high"]
    ENABLE = 1,
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
            false => CSINV_A::DISABLE,
            true => CSINV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CSINV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CSINV_A::ENABLE
    }
}
#[doc = "Field `CSINV` writer - Chip Select Invert"]
pub type CSINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CSINV_A, O>;
impl<'a, const O: u8> CSINV_W<'a, O> {
    #[doc = "Chip select is active low"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CSINV_A::DISABLE)
    }
    #[doc = "Chip select is active high"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CSINV_A::ENABLE)
    }
}
#[doc = "Field `AUTOCS` reader - Automatic Chip Select"]
pub type AUTOCS_R = crate::BitReader<bool>;
#[doc = "Field `AUTOCS` writer - Automatic Chip Select"]
pub type AUTOCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `AUTOTRI` reader - Automatic TX Tristate"]
pub type AUTOTRI_R = crate::BitReader<AUTOTRI_A>;
#[doc = "Automatic TX Tristate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOTRI_A {
    #[doc = "0: The output on U(S)n_TX when the transmitter is idle is defined by TXINV"]
    DISABLE = 0,
    #[doc = "1: U(S)n_TX is tristated whenever the transmitter is idle"]
    ENABLE = 1,
}
impl From<AUTOTRI_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOTRI_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOTRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOTRI_A {
        match self.bits {
            false => AUTOTRI_A::DISABLE,
            true => AUTOTRI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTOTRI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTOTRI_A::ENABLE
    }
}
#[doc = "Field `AUTOTRI` writer - Automatic TX Tristate"]
pub type AUTOTRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, AUTOTRI_A, O>;
impl<'a, const O: u8> AUTOTRI_W<'a, O> {
    #[doc = "The output on U(S)n_TX when the transmitter is idle is defined by TXINV"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTOTRI_A::DISABLE)
    }
    #[doc = "U(S)n_TX is tristated whenever the transmitter is idle"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTOTRI_A::ENABLE)
    }
}
#[doc = "Field `SCMODE` reader - SmartCard Mode"]
pub type SCMODE_R = crate::BitReader<bool>;
#[doc = "Field `SCMODE` writer - SmartCard Mode"]
pub type SCMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SCRETRANS` reader - SmartCard Retransmit"]
pub type SCRETRANS_R = crate::BitReader<bool>;
#[doc = "Field `SCRETRANS` writer - SmartCard Retransmit"]
pub type SCRETRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SKIPPERRF` reader - Skip Parity Error Frames"]
pub type SKIPPERRF_R = crate::BitReader<bool>;
#[doc = "Field `SKIPPERRF` writer - Skip Parity Error Frames"]
pub type SKIPPERRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BIT8DV` reader - Bit 8 Default Value"]
pub type BIT8DV_R = crate::BitReader<bool>;
#[doc = "Field `BIT8DV` writer - Bit 8 Default Value"]
pub type BIT8DV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ERRSDMA` reader - Halt DMA On Error"]
pub type ERRSDMA_R = crate::BitReader<ERRSDMA_A>;
#[doc = "Halt DMA On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRSDMA_A {
    #[doc = "0: Framing and parity errors have no effect on DMA requests from the USART"]
    DISABLE = 0,
    #[doc = "1: DMA requests from the USART are blocked while the PERR or FERR interrupt flags are set"]
    ENABLE = 1,
}
impl From<ERRSDMA_A> for bool {
    #[inline(always)]
    fn from(variant: ERRSDMA_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRSDMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRSDMA_A {
        match self.bits {
            false => ERRSDMA_A::DISABLE,
            true => ERRSDMA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERRSDMA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERRSDMA_A::ENABLE
    }
}
#[doc = "Field `ERRSDMA` writer - Halt DMA On Error"]
pub type ERRSDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ERRSDMA_A, O>;
impl<'a, const O: u8> ERRSDMA_W<'a, O> {
    #[doc = "Framing and parity errors have no effect on DMA requests from the USART"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERRSDMA_A::DISABLE)
    }
    #[doc = "DMA requests from the USART are blocked while the PERR or FERR interrupt flags are set"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERRSDMA_A::ENABLE)
    }
}
#[doc = "Field `ERRSRX` reader - Disable RX On Error"]
pub type ERRSRX_R = crate::BitReader<ERRSRX_A>;
#[doc = "Disable RX On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRSRX_A {
    #[doc = "0: Framing and parity errors have no effect on receiver"]
    DISABLE = 0,
    #[doc = "1: Framing and parity errors disable the receiver"]
    ENABLE = 1,
}
impl From<ERRSRX_A> for bool {
    #[inline(always)]
    fn from(variant: ERRSRX_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRSRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRSRX_A {
        match self.bits {
            false => ERRSRX_A::DISABLE,
            true => ERRSRX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERRSRX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERRSRX_A::ENABLE
    }
}
#[doc = "Field `ERRSRX` writer - Disable RX On Error"]
pub type ERRSRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ERRSRX_A, O>;
impl<'a, const O: u8> ERRSRX_W<'a, O> {
    #[doc = "Framing and parity errors have no effect on receiver"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERRSRX_A::DISABLE)
    }
    #[doc = "Framing and parity errors disable the receiver"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERRSRX_A::ENABLE)
    }
}
#[doc = "Field `ERRSTX` reader - Disable TX On Error"]
pub type ERRSTX_R = crate::BitReader<ERRSTX_A>;
#[doc = "Disable TX On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRSTX_A {
    #[doc = "0: Received framing and parity errors have no effect on transmitter"]
    DISABLE = 0,
    #[doc = "1: Received framing and parity errors disable the transmitter"]
    ENABLE = 1,
}
impl From<ERRSTX_A> for bool {
    #[inline(always)]
    fn from(variant: ERRSTX_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRSTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRSTX_A {
        match self.bits {
            false => ERRSTX_A::DISABLE,
            true => ERRSTX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERRSTX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERRSTX_A::ENABLE
    }
}
#[doc = "Field `ERRSTX` writer - Disable TX On Error"]
pub type ERRSTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ERRSTX_A, O>;
impl<'a, const O: u8> ERRSTX_W<'a, O> {
    #[doc = "Received framing and parity errors have no effect on transmitter"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERRSTX_A::DISABLE)
    }
    #[doc = "Received framing and parity errors disable the transmitter"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERRSTX_A::ENABLE)
    }
}
#[doc = "Field `SSSEARLY` reader - Synchronous Secondary Setup Early"]
pub type SSSEARLY_R = crate::BitReader<bool>;
#[doc = "Field `SSSEARLY` writer - Synchronous Secondary Setup Early"]
pub type SSSEARLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BYTESWAP` reader - Byteswap In Double Accesses"]
pub type BYTESWAP_R = crate::BitReader<BYTESWAP_A>;
#[doc = "Byteswap In Double Accesses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYTESWAP_A {
    #[doc = "0: Normal byte order"]
    DISABLE = 0,
    #[doc = "1: Byte order swapped"]
    ENABLE = 1,
}
impl From<BYTESWAP_A> for bool {
    #[inline(always)]
    fn from(variant: BYTESWAP_A) -> Self {
        variant as u8 != 0
    }
}
impl BYTESWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYTESWAP_A {
        match self.bits {
            false => BYTESWAP_A::DISABLE,
            true => BYTESWAP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BYTESWAP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BYTESWAP_A::ENABLE
    }
}
#[doc = "Field `BYTESWAP` writer - Byteswap In Double Accesses"]
pub type BYTESWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, BYTESWAP_A, O>;
impl<'a, const O: u8> BYTESWAP_W<'a, O> {
    #[doc = "Normal byte order"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BYTESWAP_A::DISABLE)
    }
    #[doc = "Byte order swapped"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BYTESWAP_A::ENABLE)
    }
}
#[doc = "Field `AUTOTX` reader - Always Transmit When RX Not Full"]
pub type AUTOTX_R = crate::BitReader<bool>;
#[doc = "Field `AUTOTX` writer - Always Transmit When RX Not Full"]
pub type AUTOTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `MVDIS` reader - Majority Vote Disable"]
pub type MVDIS_R = crate::BitReader<bool>;
#[doc = "Field `MVDIS` writer - Majority Vote Disable"]
pub type MVDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SMSDELAY` reader - Synchronous Main Sample Delay"]
pub type SMSDELAY_R = crate::BitReader<bool>;
#[doc = "Field `SMSDELAY` writer - Synchronous Main Sample Delay"]
pub type SMSDELAY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&self) -> LOOPBK_R {
        LOOPBK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    pub fn ccen(&self) -> CCEN_R {
        CCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&self) -> MPM_R {
        MPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&self) -> MPAB_R {
        MPAB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Edge For Setup/Sample"]
    #[inline(always)]
    pub fn clkpha(&self) -> CLKPHA_R {
        CLKPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Action On Chip Select In Main Mode"]
    #[inline(always)]
    pub fn csma(&self) -> CSMA_R {
        CSMA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&self) -> TXBIL_R {
        TXBIL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmitter output Invert"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    pub fn csinv(&self) -> CSINV_R {
        CSINV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    pub fn autocs(&self) -> AUTOCS_R {
        AUTOCS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    pub fn autotri(&self) -> AUTOTRI_R {
        AUTOTRI_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    pub fn scmode(&self) -> SCMODE_R {
        SCMODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    pub fn scretrans(&self) -> SCRETRANS_R {
        SCRETRANS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    pub fn skipperrf(&self) -> SKIPPERRF_R {
        SKIPPERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&self) -> BIT8DV_R {
        BIT8DV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Halt DMA On Error"]
    #[inline(always)]
    pub fn errsdma(&self) -> ERRSDMA_R {
        ERRSDMA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Disable RX On Error"]
    #[inline(always)]
    pub fn errsrx(&self) -> ERRSRX_R {
        ERRSRX_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable TX On Error"]
    #[inline(always)]
    pub fn errstx(&self) -> ERRSTX_R {
        ERRSTX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Synchronous Secondary Setup Early"]
    #[inline(always)]
    pub fn sssearly(&self) -> SSSEARLY_R {
        SSSEARLY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Byteswap In Double Accesses"]
    #[inline(always)]
    pub fn byteswap(&self) -> BYTESWAP_R {
        BYTESWAP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    pub fn autotx(&self) -> AUTOTX_R {
        AUTOTX_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    pub fn mvdis(&self) -> MVDIS_R {
        MVDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Synchronous Main Sample Delay"]
    #[inline(always)]
    pub fn smsdelay(&self) -> SMSDELAY_R {
        SMSDELAY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<0> {
        SYNC_W::new(self)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn loopbk(&mut self) -> LOOPBK_W<1> {
        LOOPBK_W::new(self)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccen(&mut self) -> CCEN_W<2> {
        CCEN_W::new(self)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpm(&mut self) -> MPM_W<3> {
        MPM_W::new(self)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    #[must_use]
    pub fn mpab(&mut self) -> MPAB_W<4> {
        MPAB_W::new(self)
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    #[must_use]
    pub fn ovs(&mut self) -> OVS_W<5> {
        OVS_W::new(self)
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<8> {
        CLKPOL_W::new(self)
    }
    #[doc = "Bit 9 - Clock Edge For Setup/Sample"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> CLKPHA_W<9> {
        CLKPHA_W::new(self)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<10> {
        MSBF_W::new(self)
    }
    #[doc = "Bit 11 - Action On Chip Select In Main Mode"]
    #[inline(always)]
    #[must_use]
    pub fn csma(&mut self) -> CSMA_W<11> {
        CSMA_W::new(self)
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    #[must_use]
    pub fn txbil(&mut self) -> TXBIL_W<12> {
        TXBIL_W::new(self)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<13> {
        RXINV_W::new(self)
    }
    #[doc = "Bit 14 - Transmitter output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<14> {
        TXINV_W::new(self)
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    #[must_use]
    pub fn csinv(&mut self) -> CSINV_W<15> {
        CSINV_W::new(self)
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn autocs(&mut self) -> AUTOCS_W<16> {
        AUTOCS_W::new(self)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    #[must_use]
    pub fn autotri(&mut self) -> AUTOTRI_W<17> {
        AUTOTRI_W::new(self)
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    #[must_use]
    pub fn scmode(&mut self) -> SCMODE_W<18> {
        SCMODE_W::new(self)
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    #[must_use]
    pub fn scretrans(&mut self) -> SCRETRANS_W<19> {
        SCRETRANS_W::new(self)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    #[must_use]
    pub fn skipperrf(&mut self) -> SKIPPERRF_W<20> {
        SKIPPERRF_W::new(self)
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    #[must_use]
    pub fn bit8dv(&mut self) -> BIT8DV_W<21> {
        BIT8DV_W::new(self)
    }
    #[doc = "Bit 22 - Halt DMA On Error"]
    #[inline(always)]
    #[must_use]
    pub fn errsdma(&mut self) -> ERRSDMA_W<22> {
        ERRSDMA_W::new(self)
    }
    #[doc = "Bit 23 - Disable RX On Error"]
    #[inline(always)]
    #[must_use]
    pub fn errsrx(&mut self) -> ERRSRX_W<23> {
        ERRSRX_W::new(self)
    }
    #[doc = "Bit 24 - Disable TX On Error"]
    #[inline(always)]
    #[must_use]
    pub fn errstx(&mut self) -> ERRSTX_W<24> {
        ERRSTX_W::new(self)
    }
    #[doc = "Bit 25 - Synchronous Secondary Setup Early"]
    #[inline(always)]
    #[must_use]
    pub fn sssearly(&mut self) -> SSSEARLY_W<25> {
        SSSEARLY_W::new(self)
    }
    #[doc = "Bit 28 - Byteswap In Double Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn byteswap(&mut self) -> BYTESWAP_W<28> {
        BYTESWAP_W::new(self)
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    #[must_use]
    pub fn autotx(&mut self) -> AUTOTX_W<29> {
        AUTOTX_W::new(self)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mvdis(&mut self) -> MVDIS_W<30> {
        MVDIS_W::new(self)
    }
    #[doc = "Bit 31 - Synchronous Main Sample Delay"]
    #[inline(always)]
    #[must_use]
    pub fn smsdelay(&mut self) -> SMSDELAY_W<31> {
        SMSDELAY_W::new(self)
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
