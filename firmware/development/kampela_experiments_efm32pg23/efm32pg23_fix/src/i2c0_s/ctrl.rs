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
#[doc = "Field `CORERST` reader - Soft Reset the internal state registers"]
pub type CORERST_R = crate::BitReader<CORERST_A>;
#[doc = "Soft Reset the internal state registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CORERST_A {
    #[doc = "0: No change to internal state registers"]
    DISABLE = 0,
    #[doc = "1: Reset the internal state registers"]
    ENABLE = 1,
}
impl From<CORERST_A> for bool {
    #[inline(always)]
    fn from(variant: CORERST_A) -> Self {
        variant as u8 != 0
    }
}
impl CORERST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CORERST_A {
        match self.bits {
            false => CORERST_A::DISABLE,
            true => CORERST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CORERST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CORERST_A::ENABLE
    }
}
#[doc = "Field `CORERST` writer - Soft Reset the internal state registers"]
pub type CORERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CORERST_A, O>;
impl<'a, const O: u8> CORERST_W<'a, O> {
    #[doc = "No change to internal state registers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CORERST_A::DISABLE)
    }
    #[doc = "Reset the internal state registers"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CORERST_A::ENABLE)
    }
}
#[doc = "Field `SLAVE` reader - Addressable as Follower"]
pub type SLAVE_R = crate::BitReader<SLAVE_A>;
#[doc = "Addressable as Follower\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE_A {
    #[doc = "0: All addresses will be responded to with a NACK"]
    DISABLE = 0,
    #[doc = "1: Addresses matching the programmed follower address or the general call address (if enabled) require a response from software. Other addresses are automatically responded to with a NACK."]
    ENABLE = 1,
}
impl From<SLAVE_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE_A {
        match self.bits {
            false => SLAVE_A::DISABLE,
            true => SLAVE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SLAVE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SLAVE_A::ENABLE
    }
}
#[doc = "Field `SLAVE` writer - Addressable as Follower"]
pub type SLAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, SLAVE_A, O>;
impl<'a, const O: u8> SLAVE_W<'a, O> {
    #[doc = "All addresses will be responded to with a NACK"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SLAVE_A::DISABLE)
    }
    #[doc = "Addresses matching the programmed follower address or the general call address (if enabled) require a response from software. Other addresses are automatically responded to with a NACK."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SLAVE_A::ENABLE)
    }
}
#[doc = "Field `AUTOACK` reader - Automatic Acknowledge"]
pub type AUTOACK_R = crate::BitReader<AUTOACK_A>;
#[doc = "Automatic Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOACK_A {
    #[doc = "0: Software must give one ACK command for each ACK transmitted on the I2C bus."]
    DISABLE = 0,
    #[doc = "1: Addresses that are not automatically NACK'ed, and all data is automatically acknowledged."]
    ENABLE = 1,
}
impl From<AUTOACK_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOACK_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOACK_A {
        match self.bits {
            false => AUTOACK_A::DISABLE,
            true => AUTOACK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTOACK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTOACK_A::ENABLE
    }
}
#[doc = "Field `AUTOACK` writer - Automatic Acknowledge"]
pub type AUTOACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, AUTOACK_A, O>;
impl<'a, const O: u8> AUTOACK_W<'a, O> {
    #[doc = "Software must give one ACK command for each ACK transmitted on the I2C bus."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTOACK_A::DISABLE)
    }
    #[doc = "Addresses that are not automatically NACK'ed, and all data is automatically acknowledged."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTOACK_A::ENABLE)
    }
}
#[doc = "Field `AUTOSE` reader - Automatic STOP when Empty"]
pub type AUTOSE_R = crate::BitReader<AUTOSE_A>;
#[doc = "Automatic STOP when Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOSE_A {
    #[doc = "0: A stop must be sent manually when no more data is to be transmitted."]
    DISABLE = 0,
    #[doc = "1: The leader automatically sends a STOP when no more data is available for transmission."]
    ENABLE = 1,
}
impl From<AUTOSE_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOSE_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOSE_A {
        match self.bits {
            false => AUTOSE_A::DISABLE,
            true => AUTOSE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTOSE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTOSE_A::ENABLE
    }
}
#[doc = "Field `AUTOSE` writer - Automatic STOP when Empty"]
pub type AUTOSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, AUTOSE_A, O>;
impl<'a, const O: u8> AUTOSE_W<'a, O> {
    #[doc = "A stop must be sent manually when no more data is to be transmitted."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTOSE_A::DISABLE)
    }
    #[doc = "The leader automatically sends a STOP when no more data is available for transmission."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTOSE_A::ENABLE)
    }
}
#[doc = "Field `AUTOSN` reader - Automatic STOP on NACK"]
pub type AUTOSN_R = crate::BitReader<AUTOSN_A>;
#[doc = "Automatic STOP on NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOSN_A {
    #[doc = "0: Stop is not automatically sent if a NACK is received from a follower."]
    DISABLE = 0,
    #[doc = "1: The leader automatically sends a STOP if a NACK is received from a follower."]
    ENABLE = 1,
}
impl From<AUTOSN_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOSN_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOSN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOSN_A {
        match self.bits {
            false => AUTOSN_A::DISABLE,
            true => AUTOSN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTOSN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTOSN_A::ENABLE
    }
}
#[doc = "Field `AUTOSN` writer - Automatic STOP on NACK"]
pub type AUTOSN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, AUTOSN_A, O>;
impl<'a, const O: u8> AUTOSN_W<'a, O> {
    #[doc = "Stop is not automatically sent if a NACK is received from a follower."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTOSN_A::DISABLE)
    }
    #[doc = "The leader automatically sends a STOP if a NACK is received from a follower."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTOSN_A::ENABLE)
    }
}
#[doc = "Field `ARBDIS` reader - Arbitration Disable"]
pub type ARBDIS_R = crate::BitReader<ARBDIS_A>;
#[doc = "Arbitration Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARBDIS_A {
    #[doc = "0: When a device loses arbitration, the ARBIF interrupt flag is set and the bus is released."]
    DISABLE = 0,
    #[doc = "1: When a device loses arbitration, the ARBIF interrupt flag is set, but communication proceeds."]
    ENABLE = 1,
}
impl From<ARBDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ARBDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl ARBDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBDIS_A {
        match self.bits {
            false => ARBDIS_A::DISABLE,
            true => ARBDIS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ARBDIS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ARBDIS_A::ENABLE
    }
}
#[doc = "Field `ARBDIS` writer - Arbitration Disable"]
pub type ARBDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ARBDIS_A, O>;
impl<'a, const O: u8> ARBDIS_W<'a, O> {
    #[doc = "When a device loses arbitration, the ARBIF interrupt flag is set and the bus is released."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ARBDIS_A::DISABLE)
    }
    #[doc = "When a device loses arbitration, the ARBIF interrupt flag is set, but communication proceeds."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ARBDIS_A::ENABLE)
    }
}
#[doc = "Field `GCAMEN` reader - General Call Address Match Enable"]
pub type GCAMEN_R = crate::BitReader<GCAMEN_A>;
#[doc = "General Call Address Match Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCAMEN_A {
    #[doc = "0: General call address will be NACK'ed if it is not included by the follower address and address mask."]
    DISABLE = 0,
    #[doc = "1: When a general call address is received, a software response is required"]
    ENABLE = 1,
}
impl From<GCAMEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCAMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GCAMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCAMEN_A {
        match self.bits {
            false => GCAMEN_A::DISABLE,
            true => GCAMEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GCAMEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GCAMEN_A::ENABLE
    }
}
#[doc = "Field `GCAMEN` writer - General Call Address Match Enable"]
pub type GCAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, GCAMEN_A, O>;
impl<'a, const O: u8> GCAMEN_W<'a, O> {
    #[doc = "General call address will be NACK'ed if it is not included by the follower address and address mask."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GCAMEN_A::DISABLE)
    }
    #[doc = "When a general call address is received, a software response is required"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GCAMEN_A::ENABLE)
    }
}
#[doc = "Field `TXBIL` reader - TX Buffer Interrupt Level"]
pub type TXBIL_R = crate::BitReader<TXBIL_A>;
#[doc = "TX Buffer Interrupt Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXBIL_A {
    #[doc = "0: TXBL status and the TXBL interrupt flag are set when the transmit buffer becomes empty. TXBL is cleared when the buffer becomes nonempty."]
    EMPTY = 0,
    #[doc = "1: TXBL status and the TXBL interrupt flag are set when the transmit buffer goes from full to half-full or empty. TXBL is cleared when the buffer becomes full"]
    HALF_FULL = 1,
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
            true => TXBIL_A::HALF_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXBIL_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `HALF_FULL`"]
    #[inline(always)]
    pub fn is_half_full(&self) -> bool {
        *self == TXBIL_A::HALF_FULL
    }
}
#[doc = "Field `TXBIL` writer - TX Buffer Interrupt Level"]
pub type TXBIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, TXBIL_A, O>;
impl<'a, const O: u8> TXBIL_W<'a, O> {
    #[doc = "TXBL status and the TXBL interrupt flag are set when the transmit buffer becomes empty. TXBL is cleared when the buffer becomes nonempty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXBIL_A::EMPTY)
    }
    #[doc = "TXBL status and the TXBL interrupt flag are set when the transmit buffer goes from full to half-full or empty. TXBL is cleared when the buffer becomes full"]
    #[inline(always)]
    pub fn half_full(self) -> &'a mut W {
        self.variant(TXBIL_A::HALF_FULL)
    }
}
#[doc = "Field `CLHR` reader - Clock Low High Ratio"]
pub type CLHR_R = crate::FieldReader<u8, CLHR_A>;
#[doc = "Clock Low High Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLHR_A {
    #[doc = "0: Nlow=4 and Nhigh=4, and the Nlow:Nhigh ratio is 4:4"]
    STANDARD = 0,
    #[doc = "1: Nlow=6 and Nhigh=3, and the Nlow:Nhigh ratio is 6:3"]
    ASYMMETRIC = 1,
    #[doc = "2: Nlow=11 and Nhigh=6, and the Nlow:Nhigh ratio is 11:6"]
    FAST = 2,
}
impl From<CLHR_A> for u8 {
    #[inline(always)]
    fn from(variant: CLHR_A) -> Self {
        variant as _
    }
}
impl CLHR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLHR_A> {
        match self.bits {
            0 => Some(CLHR_A::STANDARD),
            1 => Some(CLHR_A::ASYMMETRIC),
            2 => Some(CLHR_A::FAST),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == CLHR_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `ASYMMETRIC`"]
    #[inline(always)]
    pub fn is_asymmetric(&self) -> bool {
        *self == CLHR_A::ASYMMETRIC
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == CLHR_A::FAST
    }
}
#[doc = "Field `CLHR` writer - Clock Low High Ratio"]
pub type CLHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, CLHR_A, 2, O>;
impl<'a, const O: u8> CLHR_W<'a, O> {
    #[doc = "Nlow=4 and Nhigh=4, and the Nlow:Nhigh ratio is 4:4"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(CLHR_A::STANDARD)
    }
    #[doc = "Nlow=6 and Nhigh=3, and the Nlow:Nhigh ratio is 6:3"]
    #[inline(always)]
    pub fn asymmetric(self) -> &'a mut W {
        self.variant(CLHR_A::ASYMMETRIC)
    }
    #[doc = "Nlow=11 and Nhigh=6, and the Nlow:Nhigh ratio is 11:6"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(CLHR_A::FAST)
    }
}
#[doc = "Field `BITO` reader - Bus Idle Timeout"]
pub type BITO_R = crate::FieldReader<u8, BITO_A>;
#[doc = "Bus Idle Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BITO_A {
    #[doc = "0: Timeout disabled"]
    OFF = 0,
    #[doc = "1: Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    I2C40PCC = 1,
    #[doc = "2: Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    I2C80PCC = 2,
    #[doc = "3: Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    I2C160PCC = 3,
}
impl From<BITO_A> for u8 {
    #[inline(always)]
    fn from(variant: BITO_A) -> Self {
        variant as _
    }
}
impl BITO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BITO_A {
        match self.bits {
            0 => BITO_A::OFF,
            1 => BITO_A::I2C40PCC,
            2 => BITO_A::I2C80PCC,
            3 => BITO_A::I2C160PCC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BITO_A::OFF
    }
    #[doc = "Checks if the value of the field is `I2C40PCC`"]
    #[inline(always)]
    pub fn is_i2c40pcc(&self) -> bool {
        *self == BITO_A::I2C40PCC
    }
    #[doc = "Checks if the value of the field is `I2C80PCC`"]
    #[inline(always)]
    pub fn is_i2c80pcc(&self) -> bool {
        *self == BITO_A::I2C80PCC
    }
    #[doc = "Checks if the value of the field is `I2C160PCC`"]
    #[inline(always)]
    pub fn is_i2c160pcc(&self) -> bool {
        *self == BITO_A::I2C160PCC
    }
}
#[doc = "Field `BITO` writer - Bus Idle Timeout"]
pub type BITO_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, BITO_A, 2, O>;
impl<'a, const O: u8> BITO_W<'a, O> {
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(BITO_A::OFF)
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn i2c40pcc(self) -> &'a mut W {
        self.variant(BITO_A::I2C40PCC)
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn i2c80pcc(self) -> &'a mut W {
        self.variant(BITO_A::I2C80PCC)
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn i2c160pcc(self) -> &'a mut W {
        self.variant(BITO_A::I2C160PCC)
    }
}
#[doc = "Field `GIBITO` reader - Go Idle on Bus Idle Timeout"]
pub type GIBITO_R = crate::BitReader<GIBITO_A>;
#[doc = "Go Idle on Bus Idle Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIBITO_A {
    #[doc = "0: A bus idle timeout has no effect on the bus state."]
    DISABLE = 0,
    #[doc = "1: A bus idle timeout tells the I2C module that the bus is idle, allowing new transfers to be initiated."]
    ENABLE = 1,
}
impl From<GIBITO_A> for bool {
    #[inline(always)]
    fn from(variant: GIBITO_A) -> Self {
        variant as u8 != 0
    }
}
impl GIBITO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIBITO_A {
        match self.bits {
            false => GIBITO_A::DISABLE,
            true => GIBITO_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GIBITO_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GIBITO_A::ENABLE
    }
}
#[doc = "Field `GIBITO` writer - Go Idle on Bus Idle Timeout"]
pub type GIBITO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, GIBITO_A, O>;
impl<'a, const O: u8> GIBITO_W<'a, O> {
    #[doc = "A bus idle timeout has no effect on the bus state."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GIBITO_A::DISABLE)
    }
    #[doc = "A bus idle timeout tells the I2C module that the bus is idle, allowing new transfers to be initiated."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GIBITO_A::ENABLE)
    }
}
#[doc = "Field `CLTO` reader - Clock Low Timeout"]
pub type CLTO_R = crate::FieldReader<u8, CLTO_A>;
#[doc = "Clock Low Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLTO_A {
    #[doc = "0: Timeout disabled"]
    OFF = 0,
    #[doc = "1: Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    I2C40PCC = 1,
    #[doc = "2: Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    I2C80PCC = 2,
    #[doc = "3: Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    I2C160PCC = 3,
    #[doc = "4: Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    I2C320PCC = 4,
    #[doc = "5: Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    I2C1024PCC = 5,
}
impl From<CLTO_A> for u8 {
    #[inline(always)]
    fn from(variant: CLTO_A) -> Self {
        variant as _
    }
}
impl CLTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLTO_A> {
        match self.bits {
            0 => Some(CLTO_A::OFF),
            1 => Some(CLTO_A::I2C40PCC),
            2 => Some(CLTO_A::I2C80PCC),
            3 => Some(CLTO_A::I2C160PCC),
            4 => Some(CLTO_A::I2C320PCC),
            5 => Some(CLTO_A::I2C1024PCC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLTO_A::OFF
    }
    #[doc = "Checks if the value of the field is `I2C40PCC`"]
    #[inline(always)]
    pub fn is_i2c40pcc(&self) -> bool {
        *self == CLTO_A::I2C40PCC
    }
    #[doc = "Checks if the value of the field is `I2C80PCC`"]
    #[inline(always)]
    pub fn is_i2c80pcc(&self) -> bool {
        *self == CLTO_A::I2C80PCC
    }
    #[doc = "Checks if the value of the field is `I2C160PCC`"]
    #[inline(always)]
    pub fn is_i2c160pcc(&self) -> bool {
        *self == CLTO_A::I2C160PCC
    }
    #[doc = "Checks if the value of the field is `I2C320PCC`"]
    #[inline(always)]
    pub fn is_i2c320pcc(&self) -> bool {
        *self == CLTO_A::I2C320PCC
    }
    #[doc = "Checks if the value of the field is `I2C1024PCC`"]
    #[inline(always)]
    pub fn is_i2c1024pcc(&self) -> bool {
        *self == CLTO_A::I2C1024PCC
    }
}
#[doc = "Field `CLTO` writer - Clock Low Timeout"]
pub type CLTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, CLTO_A, 3, O>;
impl<'a, const O: u8> CLTO_W<'a, O> {
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLTO_A::OFF)
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn i2c40pcc(self) -> &'a mut W {
        self.variant(CLTO_A::I2C40PCC)
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn i2c80pcc(self) -> &'a mut W {
        self.variant(CLTO_A::I2C80PCC)
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn i2c160pcc(self) -> &'a mut W {
        self.variant(CLTO_A::I2C160PCC)
    }
    #[doc = "Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    #[inline(always)]
    pub fn i2c320pcc(self) -> &'a mut W {
        self.variant(CLTO_A::I2C320PCC)
    }
    #[doc = "Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    #[inline(always)]
    pub fn i2c1024pcc(self) -> &'a mut W {
        self.variant(CLTO_A::I2C1024PCC)
    }
}
#[doc = "Field `SCLMONEN` reader - SCL Monitor Enable"]
pub type SCLMONEN_R = crate::BitReader<SCLMONEN_A>;
#[doc = "SCL Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLMONEN_A {
    #[doc = "0: Disable SCL monitor"]
    DISABLE = 0,
    #[doc = "1: Enable SCL monitor"]
    ENABLE = 1,
}
impl From<SCLMONEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCLMONEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLMONEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLMONEN_A {
        match self.bits {
            false => SCLMONEN_A::DISABLE,
            true => SCLMONEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCLMONEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCLMONEN_A::ENABLE
    }
}
#[doc = "Field `SCLMONEN` writer - SCL Monitor Enable"]
pub type SCLMONEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, SCLMONEN_A, O>;
impl<'a, const O: u8> SCLMONEN_W<'a, O> {
    #[doc = "Disable SCL monitor"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCLMONEN_A::DISABLE)
    }
    #[doc = "Enable SCL monitor"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCLMONEN_A::ENABLE)
    }
}
#[doc = "Field `SDAMONEN` reader - SDA Monitor Enable"]
pub type SDAMONEN_R = crate::BitReader<SDAMONEN_A>;
#[doc = "SDA Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDAMONEN_A {
    #[doc = "0: Disable SDA Monitor"]
    DISABLE = 0,
    #[doc = "1: Enable SDA Monitor"]
    ENABLE = 1,
}
impl From<SDAMONEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDAMONEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SDAMONEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDAMONEN_A {
        match self.bits {
            false => SDAMONEN_A::DISABLE,
            true => SDAMONEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDAMONEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDAMONEN_A::ENABLE
    }
}
#[doc = "Field `SDAMONEN` writer - SDA Monitor Enable"]
pub type SDAMONEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, SDAMONEN_A, O>;
impl<'a, const O: u8> SDAMONEN_W<'a, O> {
    #[doc = "Disable SDA Monitor"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDAMONEN_A::DISABLE)
    }
    #[doc = "Enable SDA Monitor"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDAMONEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Soft Reset the internal state registers"]
    #[inline(always)]
    pub fn corerst(&self) -> CORERST_R {
        CORERST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Addressable as Follower"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic Acknowledge"]
    #[inline(always)]
    pub fn autoack(&self) -> AUTOACK_R {
        AUTOACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Automatic STOP when Empty"]
    #[inline(always)]
    pub fn autose(&self) -> AUTOSE_R {
        AUTOSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic STOP on NACK"]
    #[inline(always)]
    pub fn autosn(&self) -> AUTOSN_R {
        AUTOSN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Arbitration Disable"]
    #[inline(always)]
    pub fn arbdis(&self) -> ARBDIS_R {
        ARBDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General Call Address Match Enable"]
    #[inline(always)]
    pub fn gcamen(&self) -> GCAMEN_R {
        GCAMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&self) -> TXBIL_R {
        TXBIL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock Low High Ratio"]
    #[inline(always)]
    pub fn clhr(&self) -> CLHR_R {
        CLHR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn bito(&self) -> BITO_R {
        BITO_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Go Idle on Bus Idle Timeout"]
    #[inline(always)]
    pub fn gibito(&self) -> GIBITO_R {
        GIBITO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Clock Low Timeout"]
    #[inline(always)]
    pub fn clto(&self) -> CLTO_R {
        CLTO_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - SCL Monitor Enable"]
    #[inline(always)]
    pub fn sclmonen(&self) -> SCLMONEN_R {
        SCLMONEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SDA Monitor Enable"]
    #[inline(always)]
    pub fn sdamonen(&self) -> SDAMONEN_R {
        SDAMONEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Soft Reset the internal state registers"]
    #[inline(always)]
    #[must_use]
    pub fn corerst(&mut self) -> CORERST_W<0> {
        CORERST_W::new(self)
    }
    #[doc = "Bit 1 - Addressable as Follower"]
    #[inline(always)]
    #[must_use]
    pub fn slave(&mut self) -> SLAVE_W<1> {
        SLAVE_W::new(self)
    }
    #[doc = "Bit 2 - Automatic Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn autoack(&mut self) -> AUTOACK_W<2> {
        AUTOACK_W::new(self)
    }
    #[doc = "Bit 3 - Automatic STOP when Empty"]
    #[inline(always)]
    #[must_use]
    pub fn autose(&mut self) -> AUTOSE_W<3> {
        AUTOSE_W::new(self)
    }
    #[doc = "Bit 4 - Automatic STOP on NACK"]
    #[inline(always)]
    #[must_use]
    pub fn autosn(&mut self) -> AUTOSN_W<4> {
        AUTOSN_W::new(self)
    }
    #[doc = "Bit 5 - Arbitration Disable"]
    #[inline(always)]
    #[must_use]
    pub fn arbdis(&mut self) -> ARBDIS_W<5> {
        ARBDIS_W::new(self)
    }
    #[doc = "Bit 6 - General Call Address Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcamen(&mut self) -> GCAMEN_W<6> {
        GCAMEN_W::new(self)
    }
    #[doc = "Bit 7 - TX Buffer Interrupt Level"]
    #[inline(always)]
    #[must_use]
    pub fn txbil(&mut self) -> TXBIL_W<7> {
        TXBIL_W::new(self)
    }
    #[doc = "Bits 8:9 - Clock Low High Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn clhr(&mut self) -> CLHR_W<8> {
        CLHR_W::new(self)
    }
    #[doc = "Bits 12:13 - Bus Idle Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn bito(&mut self) -> BITO_W<12> {
        BITO_W::new(self)
    }
    #[doc = "Bit 15 - Go Idle on Bus Idle Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn gibito(&mut self) -> GIBITO_W<15> {
        GIBITO_W::new(self)
    }
    #[doc = "Bits 16:18 - Clock Low Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn clto(&mut self) -> CLTO_W<16> {
        CLTO_W::new(self)
    }
    #[doc = "Bit 20 - SCL Monitor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sclmonen(&mut self) -> SCLMONEN_W<20> {
        SCLMONEN_W::new(self)
    }
    #[doc = "Bit 21 - SDA Monitor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdamonen(&mut self) -> SDAMONEN_W<21> {
        SDAMONEN_W::new(self)
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
