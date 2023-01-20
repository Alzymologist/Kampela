#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGHALT` reader - Debug halt"]
pub type DBGHALT_R = crate::BitReader<DBGHALT_A>;
#[doc = "Debug halt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGHALT_A {
    #[doc = "0: Continue normal EUSART operation even if core is halted"]
    DISABLE = 0,
    #[doc = "1: If core is halted, receive one frame and then halt reception by deactivating RTS. Next frame reception happens when the core is unhalted during single stepping."]
    ENABLE = 1,
}
impl From<DBGHALT_A> for bool {
    #[inline(always)]
    fn from(variant: DBGHALT_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGHALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGHALT_A {
        match self.bits {
            false => DBGHALT_A::DISABLE,
            true => DBGHALT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DBGHALT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DBGHALT_A::ENABLE
    }
}
#[doc = "Field `DBGHALT` writer - Debug halt"]
pub type DBGHALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, DBGHALT_A, O>;
impl<'a, const O: u8> DBGHALT_W<'a, O> {
    #[doc = "Continue normal EUSART operation even if core is halted"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DBGHALT_A::DISABLE)
    }
    #[doc = "If core is halted, receive one frame and then halt reception by deactivating RTS. Next frame reception happens when the core is unhalted during single stepping."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DBGHALT_A::ENABLE)
    }
}
#[doc = "Field `CTSINV` reader - Clear-to-send Invert Enable"]
pub type CTSINV_R = crate::BitReader<CTSINV_A>;
#[doc = "Clear-to-send Invert Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSINV_A {
    #[doc = "0: The CTS pin is active low"]
    DISABLE = 0,
    #[doc = "1: The CTS pin is active high"]
    ENABLE = 1,
}
impl From<CTSINV_A> for bool {
    #[inline(always)]
    fn from(variant: CTSINV_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSINV_A {
        match self.bits {
            false => CTSINV_A::DISABLE,
            true => CTSINV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTSINV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTSINV_A::ENABLE
    }
}
#[doc = "Field `CTSINV` writer - Clear-to-send Invert Enable"]
pub type CTSINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, CTSINV_A, O>;
impl<'a, const O: u8> CTSINV_W<'a, O> {
    #[doc = "The CTS pin is active low"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTSINV_A::DISABLE)
    }
    #[doc = "The CTS pin is active high"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTSINV_A::ENABLE)
    }
}
#[doc = "Field `CTSEN` reader - Clear-to-send Enable"]
pub type CTSEN_R = crate::BitReader<CTSEN_A>;
#[doc = "Clear-to-send Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSEN_A {
    #[doc = "0: Ignore CTS"]
    DISABLE = 0,
    #[doc = "1: Stop transmitting when CTS is inactive"]
    ENABLE = 1,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            false => CTSEN_A::DISABLE,
            true => CTSEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTSEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTSEN_A::ENABLE
    }
}
#[doc = "Field `CTSEN` writer - Clear-to-send Enable"]
pub type CTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, CTSEN_A, O>;
impl<'a, const O: u8> CTSEN_W<'a, O> {
    #[doc = "Ignore CTS"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTSEN_A::DISABLE)
    }
    #[doc = "Stop transmitting when CTS is inactive"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTSEN_A::ENABLE)
    }
}
#[doc = "Field `RTSINV` reader - Request-to-send Invert Enable"]
pub type RTSINV_R = crate::BitReader<RTSINV_A>;
#[doc = "Request-to-send Invert Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSINV_A {
    #[doc = "0: The RTS pin is active low"]
    DISABLE = 0,
    #[doc = "1: The RTS pin is active high"]
    ENABLE = 1,
}
impl From<RTSINV_A> for bool {
    #[inline(always)]
    fn from(variant: RTSINV_A) -> Self {
        variant as u8 != 0
    }
}
impl RTSINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSINV_A {
        match self.bits {
            false => RTSINV_A::DISABLE,
            true => RTSINV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTSINV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTSINV_A::ENABLE
    }
}
#[doc = "Field `RTSINV` writer - Request-to-send Invert Enable"]
pub type RTSINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, RTSINV_A, O>;
impl<'a, const O: u8> RTSINV_W<'a, O> {
    #[doc = "The RTS pin is active low"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTSINV_A::DISABLE)
    }
    #[doc = "The RTS pin is active high"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTSINV_A::ENABLE)
    }
}
#[doc = "Field `RXTIMEOUT` reader - RX Timeout"]
pub type RXTIMEOUT_R = crate::FieldReader<u8, RXTIMEOUT_A>;
#[doc = "RX Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXTIMEOUT_A {
    #[doc = "0: DISABLED"]
    DISABLED = 0,
    #[doc = "1: ONEFRAME"]
    ONEFRAME = 1,
    #[doc = "2: TWOFRAMES"]
    TWOFRAMES = 2,
    #[doc = "3: THREEFRAMES"]
    THREEFRAMES = 3,
    #[doc = "4: FOURFRAMES"]
    FOURFRAMES = 4,
    #[doc = "5: FIVEFRAMES"]
    FIVEFRAMES = 5,
    #[doc = "6: SIXFRAMES"]
    SIXFRAMES = 6,
    #[doc = "7: SEVENFRAMES"]
    SEVENFRAMES = 7,
}
impl From<RXTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: RXTIMEOUT_A) -> Self {
        variant as _
    }
}
impl RXTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTIMEOUT_A {
        match self.bits {
            0 => RXTIMEOUT_A::DISABLED,
            1 => RXTIMEOUT_A::ONEFRAME,
            2 => RXTIMEOUT_A::TWOFRAMES,
            3 => RXTIMEOUT_A::THREEFRAMES,
            4 => RXTIMEOUT_A::FOURFRAMES,
            5 => RXTIMEOUT_A::FIVEFRAMES,
            6 => RXTIMEOUT_A::SIXFRAMES,
            7 => RXTIMEOUT_A::SEVENFRAMES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXTIMEOUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ONEFRAME`"]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == RXTIMEOUT_A::ONEFRAME
    }
    #[doc = "Checks if the value of the field is `TWOFRAMES`"]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == RXTIMEOUT_A::TWOFRAMES
    }
    #[doc = "Checks if the value of the field is `THREEFRAMES`"]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == RXTIMEOUT_A::THREEFRAMES
    }
    #[doc = "Checks if the value of the field is `FOURFRAMES`"]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == RXTIMEOUT_A::FOURFRAMES
    }
    #[doc = "Checks if the value of the field is `FIVEFRAMES`"]
    #[inline(always)]
    pub fn is_fiveframes(&self) -> bool {
        *self == RXTIMEOUT_A::FIVEFRAMES
    }
    #[doc = "Checks if the value of the field is `SIXFRAMES`"]
    #[inline(always)]
    pub fn is_sixframes(&self) -> bool {
        *self == RXTIMEOUT_A::SIXFRAMES
    }
    #[doc = "Checks if the value of the field is `SEVENFRAMES`"]
    #[inline(always)]
    pub fn is_sevenframes(&self) -> bool {
        *self == RXTIMEOUT_A::SEVENFRAMES
    }
}
#[doc = "Field `RXTIMEOUT` writer - RX Timeout"]
pub type RXTIMEOUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, RXTIMEOUT_A, 3, O>;
impl<'a, const O: u8> RXTIMEOUT_W<'a, O> {
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXTIMEOUT_A::DISABLED)
    }
    #[doc = "ONEFRAME"]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut W {
        self.variant(RXTIMEOUT_A::ONEFRAME)
    }
    #[doc = "TWOFRAMES"]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut W {
        self.variant(RXTIMEOUT_A::TWOFRAMES)
    }
    #[doc = "THREEFRAMES"]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut W {
        self.variant(RXTIMEOUT_A::THREEFRAMES)
    }
    #[doc = "FOURFRAMES"]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut W {
        self.variant(RXTIMEOUT_A::FOURFRAMES)
    }
    #[doc = "FIVEFRAMES"]
    #[inline(always)]
    pub fn fiveframes(self) -> &'a mut W {
        self.variant(RXTIMEOUT_A::FIVEFRAMES)
    }
    #[doc = "SIXFRAMES"]
    #[inline(always)]
    pub fn sixframes(self) -> &'a mut W {
        self.variant(RXTIMEOUT_A::SIXFRAMES)
    }
    #[doc = "SEVENFRAMES"]
    #[inline(always)]
    pub fn sevenframes(self) -> &'a mut W {
        self.variant(RXTIMEOUT_A::SEVENFRAMES)
    }
}
#[doc = "Field `TXDMAWU` reader - Transmitter DMA Wakeup"]
pub type TXDMAWU_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAWU` writer - Transmitter DMA Wakeup"]
pub type TXDMAWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `RXDMAWU` reader - Receiver DMA Wakeup"]
pub type RXDMAWU_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAWU` writer - Receiver DMA Wakeup"]
pub type RXDMAWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `SFUBRX` reader - Start Frame Unblock Receiver"]
pub type SFUBRX_R = crate::BitReader<bool>;
#[doc = "Field `SFUBRX` writer - Start Frame Unblock Receiver"]
pub type SFUBRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `RXPRSEN` reader - PRS RX Enable"]
pub type RXPRSEN_R = crate::BitReader<bool>;
#[doc = "Field `RXPRSEN` writer - PRS RX Enable"]
pub type RXPRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `TXFIW` reader - TX FIFO Interrupt Watermark"]
pub type TXFIW_R = crate::FieldReader<u8, TXFIW_A>;
#[doc = "TX FIFO Interrupt Watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXFIW_A {
    #[doc = "0: TXFL status flag and IF are set when the TX FIFO has space for at least one more frame."]
    ONEFRAME = 0,
    #[doc = "1: TXFL status flag and IF are set when the TX FIFO has space for at least two more frames."]
    TWOFRAMES = 1,
    #[doc = "2: TXFL status flag and IF are set when the TX FIFO has space for at least three more frames."]
    THREEFRAMES = 2,
    #[doc = "3: TXFL status flag and IF are set when the TX FIFO has space for at least four more frames."]
    FOURFRAMES = 3,
    #[doc = "4: TXFL status flag and IF are set when the TX FIFO has space for at least five more frames."]
    FIVEFRAMES = 4,
    #[doc = "5: TXFL status flag and IF are set when the TX FIFO has space for at least six more frames."]
    SIXFRAMES = 5,
    #[doc = "6: TXFL status flag and IF are set when the TX FIFO has space for at least seven more frames."]
    SEVENFRAMES = 6,
    #[doc = "7: TXFL status flag and IF are set when the TX FIFO has space for at least eight more frames."]
    EIGHTFRAMES = 7,
    #[doc = "8: TXFL status flag and IF are set when the TX FIFO has space for at least nine more frames."]
    NINEFRAMES = 8,
    #[doc = "9: TXFL status flag and IF are set when the TX FIFO has space for at least ten more frames."]
    TENFRAMES = 9,
    #[doc = "10: TXFL status flag and IF are set when the TX FIFO has space for at least eleven more frames."]
    ELEVENFRAMES = 10,
    #[doc = "11: TXFL status flag and IF are set when the TX FIFO has space for at least twelve more frames."]
    TWELVEFRAMES = 11,
    #[doc = "12: TXFL status flag and IF are set when the TX FIFO has space for at least thriteen more frames."]
    THIRTEENFRAMES = 12,
    #[doc = "13: TXFL status flag and IF are set when the TX FIFO has space for at least fourteen more frames."]
    FOURTEENFRAMES = 13,
    #[doc = "14: TXFL status flag and IF are set when the TX FIFO has space for at least fifteen more frames."]
    FIFTEENFRAMES = 14,
    #[doc = "15: TXFL status flag and IF are set when the TX FIFO has space for at least sixteen more frames."]
    SIXTEENFRAMES = 15,
}
impl From<TXFIW_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFIW_A) -> Self {
        variant as _
    }
}
impl TXFIW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIW_A {
        match self.bits {
            0 => TXFIW_A::ONEFRAME,
            1 => TXFIW_A::TWOFRAMES,
            2 => TXFIW_A::THREEFRAMES,
            3 => TXFIW_A::FOURFRAMES,
            4 => TXFIW_A::FIVEFRAMES,
            5 => TXFIW_A::SIXFRAMES,
            6 => TXFIW_A::SEVENFRAMES,
            7 => TXFIW_A::EIGHTFRAMES,
            8 => TXFIW_A::NINEFRAMES,
            9 => TXFIW_A::TENFRAMES,
            10 => TXFIW_A::ELEVENFRAMES,
            11 => TXFIW_A::TWELVEFRAMES,
            12 => TXFIW_A::THIRTEENFRAMES,
            13 => TXFIW_A::FOURTEENFRAMES,
            14 => TXFIW_A::FIFTEENFRAMES,
            15 => TXFIW_A::SIXTEENFRAMES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONEFRAME`"]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == TXFIW_A::ONEFRAME
    }
    #[doc = "Checks if the value of the field is `TWOFRAMES`"]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == TXFIW_A::TWOFRAMES
    }
    #[doc = "Checks if the value of the field is `THREEFRAMES`"]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == TXFIW_A::THREEFRAMES
    }
    #[doc = "Checks if the value of the field is `FOURFRAMES`"]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == TXFIW_A::FOURFRAMES
    }
    #[doc = "Checks if the value of the field is `FIVEFRAMES`"]
    #[inline(always)]
    pub fn is_fiveframes(&self) -> bool {
        *self == TXFIW_A::FIVEFRAMES
    }
    #[doc = "Checks if the value of the field is `SIXFRAMES`"]
    #[inline(always)]
    pub fn is_sixframes(&self) -> bool {
        *self == TXFIW_A::SIXFRAMES
    }
    #[doc = "Checks if the value of the field is `SEVENFRAMES`"]
    #[inline(always)]
    pub fn is_sevenframes(&self) -> bool {
        *self == TXFIW_A::SEVENFRAMES
    }
    #[doc = "Checks if the value of the field is `EIGHTFRAMES`"]
    #[inline(always)]
    pub fn is_eightframes(&self) -> bool {
        *self == TXFIW_A::EIGHTFRAMES
    }
    #[doc = "Checks if the value of the field is `NINEFRAMES`"]
    #[inline(always)]
    pub fn is_nineframes(&self) -> bool {
        *self == TXFIW_A::NINEFRAMES
    }
    #[doc = "Checks if the value of the field is `TENFRAMES`"]
    #[inline(always)]
    pub fn is_tenframes(&self) -> bool {
        *self == TXFIW_A::TENFRAMES
    }
    #[doc = "Checks if the value of the field is `ELEVENFRAMES`"]
    #[inline(always)]
    pub fn is_elevenframes(&self) -> bool {
        *self == TXFIW_A::ELEVENFRAMES
    }
    #[doc = "Checks if the value of the field is `TWELVEFRAMES`"]
    #[inline(always)]
    pub fn is_twelveframes(&self) -> bool {
        *self == TXFIW_A::TWELVEFRAMES
    }
    #[doc = "Checks if the value of the field is `THIRTEENFRAMES`"]
    #[inline(always)]
    pub fn is_thirteenframes(&self) -> bool {
        *self == TXFIW_A::THIRTEENFRAMES
    }
    #[doc = "Checks if the value of the field is `FOURTEENFRAMES`"]
    #[inline(always)]
    pub fn is_fourteenframes(&self) -> bool {
        *self == TXFIW_A::FOURTEENFRAMES
    }
    #[doc = "Checks if the value of the field is `FIFTEENFRAMES`"]
    #[inline(always)]
    pub fn is_fifteenframes(&self) -> bool {
        *self == TXFIW_A::FIFTEENFRAMES
    }
    #[doc = "Checks if the value of the field is `SIXTEENFRAMES`"]
    #[inline(always)]
    pub fn is_sixteenframes(&self) -> bool {
        *self == TXFIW_A::SIXTEENFRAMES
    }
}
#[doc = "Field `TXFIW` writer - TX FIFO Interrupt Watermark"]
pub type TXFIW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, TXFIW_A, 4, O>;
impl<'a, const O: u8> TXFIW_W<'a, O> {
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least one more frame."]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut W {
        self.variant(TXFIW_A::ONEFRAME)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least two more frames."]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut W {
        self.variant(TXFIW_A::TWOFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least three more frames."]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut W {
        self.variant(TXFIW_A::THREEFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least four more frames."]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut W {
        self.variant(TXFIW_A::FOURFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least five more frames."]
    #[inline(always)]
    pub fn fiveframes(self) -> &'a mut W {
        self.variant(TXFIW_A::FIVEFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least six more frames."]
    #[inline(always)]
    pub fn sixframes(self) -> &'a mut W {
        self.variant(TXFIW_A::SIXFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least seven more frames."]
    #[inline(always)]
    pub fn sevenframes(self) -> &'a mut W {
        self.variant(TXFIW_A::SEVENFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least eight more frames."]
    #[inline(always)]
    pub fn eightframes(self) -> &'a mut W {
        self.variant(TXFIW_A::EIGHTFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least nine more frames."]
    #[inline(always)]
    pub fn nineframes(self) -> &'a mut W {
        self.variant(TXFIW_A::NINEFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least ten more frames."]
    #[inline(always)]
    pub fn tenframes(self) -> &'a mut W {
        self.variant(TXFIW_A::TENFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least eleven more frames."]
    #[inline(always)]
    pub fn elevenframes(self) -> &'a mut W {
        self.variant(TXFIW_A::ELEVENFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least twelve more frames."]
    #[inline(always)]
    pub fn twelveframes(self) -> &'a mut W {
        self.variant(TXFIW_A::TWELVEFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least thriteen more frames."]
    #[inline(always)]
    pub fn thirteenframes(self) -> &'a mut W {
        self.variant(TXFIW_A::THIRTEENFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least fourteen more frames."]
    #[inline(always)]
    pub fn fourteenframes(self) -> &'a mut W {
        self.variant(TXFIW_A::FOURTEENFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least fifteen more frames."]
    #[inline(always)]
    pub fn fifteenframes(self) -> &'a mut W {
        self.variant(TXFIW_A::FIFTEENFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least sixteen more frames."]
    #[inline(always)]
    pub fn sixteenframes(self) -> &'a mut W {
        self.variant(TXFIW_A::SIXTEENFRAMES)
    }
}
#[doc = "Field `RTSRXFW` reader - Request-to-send RX FIFO Watermark"]
pub type RTSRXFW_R = crate::FieldReader<u8, RTSRXFW_A>;
#[doc = "Request-to-send RX FIFO Watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTSRXFW_A {
    #[doc = "0: RTS is set if there is space for at least one more frame in the RX FIFO."]
    ONEFRAME = 0,
    #[doc = "1: RTS is set if there is space for at least two more frames in the RX FIFO."]
    TWOFRAMES = 1,
    #[doc = "2: RTS is set if there is space for at least three more frames in the RX FIFO."]
    THREEFRAMES = 2,
    #[doc = "3: RTS is set if there is space for four more frames in the RX FIFO."]
    FOURFRAMES = 3,
    #[doc = "4: RTS is set if there is space for five more frames in the RX FIFO."]
    FIVEFRAMES = 4,
    #[doc = "5: RTS is set if there is space for six more frames in the RX FIFO."]
    SIXFRAMES = 5,
    #[doc = "6: RTS is set if there is space for seven more frames in the RX FIFO."]
    SEVENFRAMES = 6,
    #[doc = "7: RTS is set if there is space for eight more frames in the RX FIFO."]
    EIGHTFRAMES = 7,
    #[doc = "8: RTS is set if there is space for nine more frames in the RX FIFO."]
    NINEFRAMES = 8,
    #[doc = "9: RTS is set if there is space for ten more frames in the RX FIFO."]
    TENFRAMES = 9,
    #[doc = "10: RTS is set if there is space for eleven more frames in the RX FIFO."]
    ELEVENFRAMES = 10,
    #[doc = "11: RTS is set if there is space for twelve more frames in the RX FIFO."]
    TWELVEFRAMES = 11,
    #[doc = "12: RTS is set if there is space for thirteen more frames in the RX FIFO."]
    THIRTEENFRAMES = 12,
    #[doc = "13: RTS is set if there is space for fourteen more frames in the RX FIFO."]
    FOURTEENFRAMES = 13,
    #[doc = "14: RTS is set if there is space for fifteen more frames in the RX FIFO."]
    FIFTEENFRAMES = 14,
    #[doc = "15: RTS is set if there is space for sixteen more frames in the RX FIFO."]
    SIXTEENFRAMES = 15,
}
impl From<RTSRXFW_A> for u8 {
    #[inline(always)]
    fn from(variant: RTSRXFW_A) -> Self {
        variant as _
    }
}
impl RTSRXFW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSRXFW_A {
        match self.bits {
            0 => RTSRXFW_A::ONEFRAME,
            1 => RTSRXFW_A::TWOFRAMES,
            2 => RTSRXFW_A::THREEFRAMES,
            3 => RTSRXFW_A::FOURFRAMES,
            4 => RTSRXFW_A::FIVEFRAMES,
            5 => RTSRXFW_A::SIXFRAMES,
            6 => RTSRXFW_A::SEVENFRAMES,
            7 => RTSRXFW_A::EIGHTFRAMES,
            8 => RTSRXFW_A::NINEFRAMES,
            9 => RTSRXFW_A::TENFRAMES,
            10 => RTSRXFW_A::ELEVENFRAMES,
            11 => RTSRXFW_A::TWELVEFRAMES,
            12 => RTSRXFW_A::THIRTEENFRAMES,
            13 => RTSRXFW_A::FOURTEENFRAMES,
            14 => RTSRXFW_A::FIFTEENFRAMES,
            15 => RTSRXFW_A::SIXTEENFRAMES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONEFRAME`"]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == RTSRXFW_A::ONEFRAME
    }
    #[doc = "Checks if the value of the field is `TWOFRAMES`"]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == RTSRXFW_A::TWOFRAMES
    }
    #[doc = "Checks if the value of the field is `THREEFRAMES`"]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == RTSRXFW_A::THREEFRAMES
    }
    #[doc = "Checks if the value of the field is `FOURFRAMES`"]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == RTSRXFW_A::FOURFRAMES
    }
    #[doc = "Checks if the value of the field is `FIVEFRAMES`"]
    #[inline(always)]
    pub fn is_fiveframes(&self) -> bool {
        *self == RTSRXFW_A::FIVEFRAMES
    }
    #[doc = "Checks if the value of the field is `SIXFRAMES`"]
    #[inline(always)]
    pub fn is_sixframes(&self) -> bool {
        *self == RTSRXFW_A::SIXFRAMES
    }
    #[doc = "Checks if the value of the field is `SEVENFRAMES`"]
    #[inline(always)]
    pub fn is_sevenframes(&self) -> bool {
        *self == RTSRXFW_A::SEVENFRAMES
    }
    #[doc = "Checks if the value of the field is `EIGHTFRAMES`"]
    #[inline(always)]
    pub fn is_eightframes(&self) -> bool {
        *self == RTSRXFW_A::EIGHTFRAMES
    }
    #[doc = "Checks if the value of the field is `NINEFRAMES`"]
    #[inline(always)]
    pub fn is_nineframes(&self) -> bool {
        *self == RTSRXFW_A::NINEFRAMES
    }
    #[doc = "Checks if the value of the field is `TENFRAMES`"]
    #[inline(always)]
    pub fn is_tenframes(&self) -> bool {
        *self == RTSRXFW_A::TENFRAMES
    }
    #[doc = "Checks if the value of the field is `ELEVENFRAMES`"]
    #[inline(always)]
    pub fn is_elevenframes(&self) -> bool {
        *self == RTSRXFW_A::ELEVENFRAMES
    }
    #[doc = "Checks if the value of the field is `TWELVEFRAMES`"]
    #[inline(always)]
    pub fn is_twelveframes(&self) -> bool {
        *self == RTSRXFW_A::TWELVEFRAMES
    }
    #[doc = "Checks if the value of the field is `THIRTEENFRAMES`"]
    #[inline(always)]
    pub fn is_thirteenframes(&self) -> bool {
        *self == RTSRXFW_A::THIRTEENFRAMES
    }
    #[doc = "Checks if the value of the field is `FOURTEENFRAMES`"]
    #[inline(always)]
    pub fn is_fourteenframes(&self) -> bool {
        *self == RTSRXFW_A::FOURTEENFRAMES
    }
    #[doc = "Checks if the value of the field is `FIFTEENFRAMES`"]
    #[inline(always)]
    pub fn is_fifteenframes(&self) -> bool {
        *self == RTSRXFW_A::FIFTEENFRAMES
    }
    #[doc = "Checks if the value of the field is `SIXTEENFRAMES`"]
    #[inline(always)]
    pub fn is_sixteenframes(&self) -> bool {
        *self == RTSRXFW_A::SIXTEENFRAMES
    }
}
#[doc = "Field `RTSRXFW` writer - Request-to-send RX FIFO Watermark"]
pub type RTSRXFW_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, RTSRXFW_A, 4, O>;
impl<'a, const O: u8> RTSRXFW_W<'a, O> {
    #[doc = "RTS is set if there is space for at least one more frame in the RX FIFO."]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut W {
        self.variant(RTSRXFW_A::ONEFRAME)
    }
    #[doc = "RTS is set if there is space for at least two more frames in the RX FIFO."]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::TWOFRAMES)
    }
    #[doc = "RTS is set if there is space for at least three more frames in the RX FIFO."]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::THREEFRAMES)
    }
    #[doc = "RTS is set if there is space for four more frames in the RX FIFO."]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::FOURFRAMES)
    }
    #[doc = "RTS is set if there is space for five more frames in the RX FIFO."]
    #[inline(always)]
    pub fn fiveframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::FIVEFRAMES)
    }
    #[doc = "RTS is set if there is space for six more frames in the RX FIFO."]
    #[inline(always)]
    pub fn sixframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::SIXFRAMES)
    }
    #[doc = "RTS is set if there is space for seven more frames in the RX FIFO."]
    #[inline(always)]
    pub fn sevenframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::SEVENFRAMES)
    }
    #[doc = "RTS is set if there is space for eight more frames in the RX FIFO."]
    #[inline(always)]
    pub fn eightframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::EIGHTFRAMES)
    }
    #[doc = "RTS is set if there is space for nine more frames in the RX FIFO."]
    #[inline(always)]
    pub fn nineframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::NINEFRAMES)
    }
    #[doc = "RTS is set if there is space for ten more frames in the RX FIFO."]
    #[inline(always)]
    pub fn tenframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::TENFRAMES)
    }
    #[doc = "RTS is set if there is space for eleven more frames in the RX FIFO."]
    #[inline(always)]
    pub fn elevenframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::ELEVENFRAMES)
    }
    #[doc = "RTS is set if there is space for twelve more frames in the RX FIFO."]
    #[inline(always)]
    pub fn twelveframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::TWELVEFRAMES)
    }
    #[doc = "RTS is set if there is space for thirteen more frames in the RX FIFO."]
    #[inline(always)]
    pub fn thirteenframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::THIRTEENFRAMES)
    }
    #[doc = "RTS is set if there is space for fourteen more frames in the RX FIFO."]
    #[inline(always)]
    pub fn fourteenframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::FOURTEENFRAMES)
    }
    #[doc = "RTS is set if there is space for fifteen more frames in the RX FIFO."]
    #[inline(always)]
    pub fn fifteenframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::FIFTEENFRAMES)
    }
    #[doc = "RTS is set if there is space for sixteen more frames in the RX FIFO."]
    #[inline(always)]
    pub fn sixteenframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::SIXTEENFRAMES)
    }
}
#[doc = "Field `RXFIW` reader - RX FIFO Interrupt Watermark"]
pub type RXFIW_R = crate::FieldReader<u8, RXFIW_A>;
#[doc = "RX FIFO Interrupt Watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXFIW_A {
    #[doc = "0: RXFL status flag and IF are set when the RX FIFO has at least one frame in it."]
    ONEFRAME = 0,
    #[doc = "1: RXFL status flag and IF are set when the RX FIFO has at least two frames in it."]
    TWOFRAMES = 1,
    #[doc = "2: RXFL status flag and IF are set when the RX FIFO has at least three frames in it."]
    THREEFRAMES = 2,
    #[doc = "3: RXFL status flag and IF are set when the RX FIFO has at least four frames in it."]
    FOURFRAMES = 3,
    #[doc = "4: RXFL status flag and IF are set when the RX FIFO has at least five frames in it."]
    FIVEFRAMES = 4,
    #[doc = "5: RXFL status flag and IF are set when the RX FIFO has at least six frames in it."]
    SIXFRAMES = 5,
    #[doc = "6: RXFL status flag and IF are set when the RX FIFO has at least seven frames in it."]
    SEVENFRAMES = 6,
    #[doc = "7: RXFL status flag and IF are set when the RX FIFO has at least eight frames in it."]
    EIGHTFRAMES = 7,
    #[doc = "8: RXFL status flag and IF are set when the RX FIFO has at least nine frames in it."]
    NINEFRAMES = 8,
    #[doc = "9: RXFL status flag and IF are set when the RX FIFO has at least ten frames in it."]
    TENFRAMES = 9,
    #[doc = "10: RXFL status flag and IF are set when the RX FIFO has at least eleven frames in it."]
    ELEVENFRAMES = 10,
    #[doc = "11: RXFL status flag and IF are set when the RX FIFO has at least twelve frames in it."]
    TWELVEFRAMES = 11,
    #[doc = "12: RXFL status flag and IF are set when the RX FIFO has at least thriteen frames in it."]
    THIRTEENFRAMES = 12,
    #[doc = "13: RXFL status flag and IF are set when the RX FIFO has at least fourteen frames in it."]
    FOURTEENFRAMES = 13,
    #[doc = "14: RXFL status flag and IF are set when the RX FIFO has at least fifteen frames in it."]
    FIFTEENFRAMES = 14,
    #[doc = "15: RXFL status flag and IF are set when the RX FIFO has at least sixteen frames in it."]
    SIXTEENFRAMES = 15,
}
impl From<RXFIW_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFIW_A) -> Self {
        variant as _
    }
}
impl RXFIW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIW_A {
        match self.bits {
            0 => RXFIW_A::ONEFRAME,
            1 => RXFIW_A::TWOFRAMES,
            2 => RXFIW_A::THREEFRAMES,
            3 => RXFIW_A::FOURFRAMES,
            4 => RXFIW_A::FIVEFRAMES,
            5 => RXFIW_A::SIXFRAMES,
            6 => RXFIW_A::SEVENFRAMES,
            7 => RXFIW_A::EIGHTFRAMES,
            8 => RXFIW_A::NINEFRAMES,
            9 => RXFIW_A::TENFRAMES,
            10 => RXFIW_A::ELEVENFRAMES,
            11 => RXFIW_A::TWELVEFRAMES,
            12 => RXFIW_A::THIRTEENFRAMES,
            13 => RXFIW_A::FOURTEENFRAMES,
            14 => RXFIW_A::FIFTEENFRAMES,
            15 => RXFIW_A::SIXTEENFRAMES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONEFRAME`"]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == RXFIW_A::ONEFRAME
    }
    #[doc = "Checks if the value of the field is `TWOFRAMES`"]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == RXFIW_A::TWOFRAMES
    }
    #[doc = "Checks if the value of the field is `THREEFRAMES`"]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == RXFIW_A::THREEFRAMES
    }
    #[doc = "Checks if the value of the field is `FOURFRAMES`"]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == RXFIW_A::FOURFRAMES
    }
    #[doc = "Checks if the value of the field is `FIVEFRAMES`"]
    #[inline(always)]
    pub fn is_fiveframes(&self) -> bool {
        *self == RXFIW_A::FIVEFRAMES
    }
    #[doc = "Checks if the value of the field is `SIXFRAMES`"]
    #[inline(always)]
    pub fn is_sixframes(&self) -> bool {
        *self == RXFIW_A::SIXFRAMES
    }
    #[doc = "Checks if the value of the field is `SEVENFRAMES`"]
    #[inline(always)]
    pub fn is_sevenframes(&self) -> bool {
        *self == RXFIW_A::SEVENFRAMES
    }
    #[doc = "Checks if the value of the field is `EIGHTFRAMES`"]
    #[inline(always)]
    pub fn is_eightframes(&self) -> bool {
        *self == RXFIW_A::EIGHTFRAMES
    }
    #[doc = "Checks if the value of the field is `NINEFRAMES`"]
    #[inline(always)]
    pub fn is_nineframes(&self) -> bool {
        *self == RXFIW_A::NINEFRAMES
    }
    #[doc = "Checks if the value of the field is `TENFRAMES`"]
    #[inline(always)]
    pub fn is_tenframes(&self) -> bool {
        *self == RXFIW_A::TENFRAMES
    }
    #[doc = "Checks if the value of the field is `ELEVENFRAMES`"]
    #[inline(always)]
    pub fn is_elevenframes(&self) -> bool {
        *self == RXFIW_A::ELEVENFRAMES
    }
    #[doc = "Checks if the value of the field is `TWELVEFRAMES`"]
    #[inline(always)]
    pub fn is_twelveframes(&self) -> bool {
        *self == RXFIW_A::TWELVEFRAMES
    }
    #[doc = "Checks if the value of the field is `THIRTEENFRAMES`"]
    #[inline(always)]
    pub fn is_thirteenframes(&self) -> bool {
        *self == RXFIW_A::THIRTEENFRAMES
    }
    #[doc = "Checks if the value of the field is `FOURTEENFRAMES`"]
    #[inline(always)]
    pub fn is_fourteenframes(&self) -> bool {
        *self == RXFIW_A::FOURTEENFRAMES
    }
    #[doc = "Checks if the value of the field is `FIFTEENFRAMES`"]
    #[inline(always)]
    pub fn is_fifteenframes(&self) -> bool {
        *self == RXFIW_A::FIFTEENFRAMES
    }
    #[doc = "Checks if the value of the field is `SIXTEENFRAMES`"]
    #[inline(always)]
    pub fn is_sixteenframes(&self) -> bool {
        *self == RXFIW_A::SIXTEENFRAMES
    }
}
#[doc = "Field `RXFIW` writer - RX FIFO Interrupt Watermark"]
pub type RXFIW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, RXFIW_A, 4, O>;
impl<'a, const O: u8> RXFIW_W<'a, O> {
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least one frame in it."]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut W {
        self.variant(RXFIW_A::ONEFRAME)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least two frames in it."]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut W {
        self.variant(RXFIW_A::TWOFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least three frames in it."]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut W {
        self.variant(RXFIW_A::THREEFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least four frames in it."]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut W {
        self.variant(RXFIW_A::FOURFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least five frames in it."]
    #[inline(always)]
    pub fn fiveframes(self) -> &'a mut W {
        self.variant(RXFIW_A::FIVEFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least six frames in it."]
    #[inline(always)]
    pub fn sixframes(self) -> &'a mut W {
        self.variant(RXFIW_A::SIXFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least seven frames in it."]
    #[inline(always)]
    pub fn sevenframes(self) -> &'a mut W {
        self.variant(RXFIW_A::SEVENFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least eight frames in it."]
    #[inline(always)]
    pub fn eightframes(self) -> &'a mut W {
        self.variant(RXFIW_A::EIGHTFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least nine frames in it."]
    #[inline(always)]
    pub fn nineframes(self) -> &'a mut W {
        self.variant(RXFIW_A::NINEFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least ten frames in it."]
    #[inline(always)]
    pub fn tenframes(self) -> &'a mut W {
        self.variant(RXFIW_A::TENFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least eleven frames in it."]
    #[inline(always)]
    pub fn elevenframes(self) -> &'a mut W {
        self.variant(RXFIW_A::ELEVENFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least twelve frames in it."]
    #[inline(always)]
    pub fn twelveframes(self) -> &'a mut W {
        self.variant(RXFIW_A::TWELVEFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least thriteen frames in it."]
    #[inline(always)]
    pub fn thirteenframes(self) -> &'a mut W {
        self.variant(RXFIW_A::THIRTEENFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least fourteen frames in it."]
    #[inline(always)]
    pub fn fourteenframes(self) -> &'a mut W {
        self.variant(RXFIW_A::FOURTEENFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least fifteen frames in it."]
    #[inline(always)]
    pub fn fifteenframes(self) -> &'a mut W {
        self.variant(RXFIW_A::FIFTEENFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least sixteen frames in it."]
    #[inline(always)]
    pub fn sixteenframes(self) -> &'a mut W {
        self.variant(RXFIW_A::SIXTEENFRAMES)
    }
}
impl R {
    #[doc = "Bit 0 - Debug halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear-to-send Invert Enable"]
    #[inline(always)]
    pub fn ctsinv(&self) -> CTSINV_R {
        CTSINV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear-to-send Enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Request-to-send Invert Enable"]
    #[inline(always)]
    pub fn rtsinv(&self) -> RTSINV_R {
        RTSINV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - RX Timeout"]
    #[inline(always)]
    pub fn rxtimeout(&self) -> RXTIMEOUT_R {
        RXTIMEOUT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 9 - Transmitter DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&self) -> TXDMAWU_R {
        TXDMAWU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receiver DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&self) -> RXDMAWU_R {
        RXDMAWU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Start Frame Unblock Receiver"]
    #[inline(always)]
    pub fn sfubrx(&self) -> SFUBRX_R {
        SFUBRX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprsen(&self) -> RXPRSEN_R {
        RXPRSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - TX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn txfiw(&self) -> TXFIW_R {
        TXFIW_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - Request-to-send RX FIFO Watermark"]
    #[inline(always)]
    pub fn rtsrxfw(&self) -> RTSRXFW_R {
        RTSRXFW_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 27:30 - RX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn rxfiw(&self) -> RXFIW_R {
        RXFIW_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Debug halt"]
    #[inline(always)]
    #[must_use]
    pub fn dbghalt(&mut self) -> DBGHALT_W<0> {
        DBGHALT_W::new(self)
    }
    #[doc = "Bit 1 - Clear-to-send Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsinv(&mut self) -> CTSINV_W<1> {
        CTSINV_W::new(self)
    }
    #[doc = "Bit 2 - Clear-to-send Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CTSEN_W<2> {
        CTSEN_W::new(self)
    }
    #[doc = "Bit 3 - Request-to-send Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsinv(&mut self) -> RTSINV_W<3> {
        RTSINV_W::new(self)
    }
    #[doc = "Bits 4:6 - RX Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn rxtimeout(&mut self) -> RXTIMEOUT_W<4> {
        RXTIMEOUT_W::new(self)
    }
    #[doc = "Bit 9 - Transmitter DMA Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn txdmawu(&mut self) -> TXDMAWU_W<9> {
        TXDMAWU_W::new(self)
    }
    #[doc = "Bit 10 - Receiver DMA Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmawu(&mut self) -> RXDMAWU_W<10> {
        RXDMAWU_W::new(self)
    }
    #[doc = "Bit 11 - Start Frame Unblock Receiver"]
    #[inline(always)]
    #[must_use]
    pub fn sfubrx(&mut self) -> SFUBRX_W<11> {
        SFUBRX_W::new(self)
    }
    #[doc = "Bit 15 - PRS RX Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxprsen(&mut self) -> RXPRSEN_W<15> {
        RXPRSEN_W::new(self)
    }
    #[doc = "Bits 16:19 - TX FIFO Interrupt Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn txfiw(&mut self) -> TXFIW_W<16> {
        TXFIW_W::new(self)
    }
    #[doc = "Bits 22:25 - Request-to-send RX FIFO Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rtsrxfw(&mut self) -> RTSRXFW_W<22> {
        RTSRXFW_W::new(self)
    }
    #[doc = "Bits 27:30 - RX FIFO Interrupt Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rxfiw(&mut self) -> RXFIW_W<27> {
        RXFIW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
