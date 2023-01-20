#[doc = "Register `CFG0` reader"]
pub struct R(crate::R<CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG0` writer"]
pub struct W(crate::W<CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG0_SPEC>;
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
impl From<crate::W<CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNC` reader - Synchronous Mode"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
#[doc = "Synchronous Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    #[doc = "0: The EUSART operates in asynchronous mode"]
    ASYNC = 0,
    #[doc = "1: The EUSART operates in synchronous mode"]
    SYNC = 1,
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
            false => SYNC_A::ASYNC,
            true => SYNC_A::SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == SYNC_A::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == SYNC_A::SYNC
    }
}
#[doc = "Field `SYNC` writer - Synchronous Mode"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, SYNC_A, O>;
impl<'a, const O: u8> SYNC_W<'a, O> {
    #[doc = "The EUSART operates in asynchronous mode"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(SYNC_A::ASYNC)
    }
    #[doc = "The EUSART operates in synchronous mode"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(SYNC_A::SYNC)
    }
}
#[doc = "Field `LOOPBK` reader - Loopback Enable"]
pub type LOOPBK_R = crate::BitReader<LOOPBK_A>;
#[doc = "Loopback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPBK_A {
    #[doc = "0: The receiver is connected to and receives data from UARTn_RX"]
    DISABLE = 0,
    #[doc = "1: The receiver is connected to and receives data from UARTn_TX"]
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
pub type LOOPBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, LOOPBK_A, O>;
impl<'a, const O: u8> LOOPBK_W<'a, O> {
    #[doc = "The receiver is connected to and receives data from UARTn_RX"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOOPBK_A::DISABLE)
    }
    #[doc = "The receiver is connected to and receives data from UARTn_TX"]
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
pub type CCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, CCEN_A, O>;
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
    #[doc = "1: An incoming frame with the 9th bit equal to MPAB will be loaded into the RX FIFO regardless of RXBLOCK and will result in the MPAB interrupt flag being set"]
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
pub type MPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, MPM_A, O>;
impl<'a, const O: u8> MPM_W<'a, O> {
    #[doc = "The 9th bit of incoming frames has no special function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPM_A::DISABLE)
    }
    #[doc = "An incoming frame with the 9th bit equal to MPAB will be loaded into the RX FIFO regardless of RXBLOCK and will result in the MPAB interrupt flag being set"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPM_A::ENABLE)
    }
}
#[doc = "Field `MPAB` reader - Multi-Processor Address-Bit"]
pub type MPAB_R = crate::BitReader<bool>;
#[doc = "Field `MPAB` writer - Multi-Processor Address-Bit"]
pub type MPAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, O>;
#[doc = "Field `OVS` reader - Oversampling"]
pub type OVS_R = crate::FieldReader<u8, OVS_A>;
#[doc = "Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVS_A {
    #[doc = "0: 16X oversampling"]
    X16 = 0,
    #[doc = "1: 8X oversampling"]
    X8 = 1,
    #[doc = "2: 6X oversampling"]
    X6 = 2,
    #[doc = "3: 4X oversampling"]
    X4 = 3,
    #[doc = "4: Disable oversampling (for LF operation)"]
    DISABLE = 4,
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
    pub fn variant(&self) -> Option<OVS_A> {
        match self.bits {
            0 => Some(OVS_A::X16),
            1 => Some(OVS_A::X8),
            2 => Some(OVS_A::X6),
            3 => Some(OVS_A::X4),
            4 => Some(OVS_A::DISABLE),
            _ => None,
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
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OVS_A::DISABLE
    }
}
#[doc = "Field `OVS` writer - Oversampling"]
pub type OVS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, OVS_A, 3, O>;
impl<'a, const O: u8> OVS_W<'a, O> {
    #[doc = "16X oversampling"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVS_A::X16)
    }
    #[doc = "8X oversampling"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVS_A::X8)
    }
    #[doc = "6X oversampling"]
    #[inline(always)]
    pub fn x6(self) -> &'a mut W {
        self.variant(OVS_A::X6)
    }
    #[doc = "4X oversampling"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(OVS_A::X4)
    }
    #[doc = "Disable oversampling (for LF operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OVS_A::DISABLE)
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
pub type MSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, MSBF_A, O>;
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
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, RXINV_A, O>;
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
    #[doc = "0: Output from the transmitter is passed unchanged to UARTn_TX"]
    DISABLE = 0,
    #[doc = "1: Output from the transmitter is inverted before it is passed to UARTn_TX"]
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
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, TXINV_A, O>;
impl<'a, const O: u8> TXINV_W<'a, O> {
    #[doc = "Output from the transmitter is passed unchanged to UARTn_TX"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXINV_A::DISABLE)
    }
    #[doc = "Output from the transmitter is inverted before it is passed to UARTn_TX"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXINV_A::ENABLE)
    }
}
#[doc = "Field `AUTOTRI` reader - Automatic TX Tristate"]
pub type AUTOTRI_R = crate::BitReader<AUTOTRI_A>;
#[doc = "Automatic TX Tristate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOTRI_A {
    #[doc = "0: The output on UARTn_TX when the transmitter is idle is defined by TXINV"]
    DISABLE = 0,
    #[doc = "1: UARTn_TX is tristated whenever the transmitter is idle"]
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
pub type AUTOTRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, AUTOTRI_A, O>;
impl<'a, const O: u8> AUTOTRI_W<'a, O> {
    #[doc = "The output on UARTn_TX when the transmitter is idle is defined by TXINV"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTOTRI_A::DISABLE)
    }
    #[doc = "UARTn_TX is tristated whenever the transmitter is idle"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTOTRI_A::ENABLE)
    }
}
#[doc = "Field `SKIPPERRF` reader - Skip Parity Error Frames"]
pub type SKIPPERRF_R = crate::BitReader<bool>;
#[doc = "Field `SKIPPERRF` writer - Skip Parity Error Frames"]
pub type SKIPPERRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, O>;
#[doc = "Field `ERRSDMA` reader - Halt DMA Read On Error"]
pub type ERRSDMA_R = crate::BitReader<ERRSDMA_A>;
#[doc = "Halt DMA Read On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRSDMA_A {
    #[doc = "0: Framing and parity errors have no effect on DMA requests from the EUSART"]
    DISABLE = 0,
    #[doc = "1: DMA requests from the EUSART are blocked while the PERR or FERR interrupt flags are set"]
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
#[doc = "Field `ERRSDMA` writer - Halt DMA Read On Error"]
pub type ERRSDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, ERRSDMA_A, O>;
impl<'a, const O: u8> ERRSDMA_W<'a, O> {
    #[doc = "Framing and parity errors have no effect on DMA requests from the EUSART"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERRSDMA_A::DISABLE)
    }
    #[doc = "DMA requests from the EUSART are blocked while the PERR or FERR interrupt flags are set"]
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
pub type ERRSRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, ERRSRX_A, O>;
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
pub type ERRSTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, ERRSTX_A, O>;
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
#[doc = "Field `MVDIS` reader - Majority Vote Disable"]
pub type MVDIS_R = crate::BitReader<bool>;
#[doc = "Field `MVDIS` writer - Majority Vote Disable"]
pub type MVDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, O>;
#[doc = "Field `AUTOBAUDEN` reader - AUTOBAUD detection enable"]
pub type AUTOBAUDEN_R = crate::BitReader<bool>;
#[doc = "Field `AUTOBAUDEN` writer - AUTOBAUD detection enable"]
pub type AUTOBAUDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Synchronous Mode"]
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
    #[doc = "Bits 5:7 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    pub fn autotri(&self) -> AUTOTRI_R {
        AUTOTRI_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    pub fn skipperrf(&self) -> SKIPPERRF_R {
        SKIPPERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Halt DMA Read On Error"]
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
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    pub fn mvdis(&self) -> MVDIS_R {
        MVDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AUTOBAUD detection enable"]
    #[inline(always)]
    pub fn autobauden(&self) -> AUTOBAUDEN_R {
        AUTOBAUDEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Mode"]
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
    #[doc = "Bits 5:7 - Oversampling"]
    #[inline(always)]
    #[must_use]
    pub fn ovs(&mut self) -> OVS_W<5> {
        OVS_W::new(self)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<10> {
        MSBF_W::new(self)
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
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    #[must_use]
    pub fn autotri(&mut self) -> AUTOTRI_W<17> {
        AUTOTRI_W::new(self)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    #[must_use]
    pub fn skipperrf(&mut self) -> SKIPPERRF_W<20> {
        SKIPPERRF_W::new(self)
    }
    #[doc = "Bit 22 - Halt DMA Read On Error"]
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
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mvdis(&mut self) -> MVDIS_W<30> {
        MVDIS_W::new(self)
    }
    #[doc = "Bit 31 - AUTOBAUD detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn autobauden(&mut self) -> AUTOBAUDEN_W<31> {
        AUTOBAUDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](index.html) module"]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg0::R](R) reader structure"]
impl crate::Readable for CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg0::W](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
