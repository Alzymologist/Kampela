#[doc = "Register `STATE` reader"]
pub struct R(crate::R<STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - Bus Busy"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `MASTER` reader - Leader"]
pub type MASTER_R = crate::BitReader<bool>;
#[doc = "Field `TRANSMITTER` reader - Transmitter"]
pub type TRANSMITTER_R = crate::BitReader<bool>;
#[doc = "Field `NACKED` reader - Nack Received"]
pub type NACKED_R = crate::BitReader<bool>;
#[doc = "Field `BUSHOLD` reader - Bus Held"]
pub type BUSHOLD_R = crate::BitReader<bool>;
#[doc = "Field `STATE` reader - Transmission State"]
pub type STATE_R = crate::FieldReader<u8, STATE_A>;
#[doc = "Transmission State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: No transmission is being performed."]
    IDLE = 0,
    #[doc = "1: Waiting for idle. Will send a start condition as soon as the bus is idle."]
    WAIT = 1,
    #[doc = "2: Start transmit phase"]
    START = 2,
    #[doc = "3: Address transmit or receive phase"]
    ADDR = 3,
    #[doc = "4: Address ack/nack transmit or receive phase"]
    ADDRACK = 4,
    #[doc = "5: Data transmit or receive phase"]
    DATA = 5,
    #[doc = "6: Data ack/nack transmit or receive phase"]
    DATAACK = 6,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            0 => Some(STATE_A::IDLE),
            1 => Some(STATE_A::WAIT),
            2 => Some(STATE_A::START),
            3 => Some(STATE_A::ADDR),
            4 => Some(STATE_A::ADDRACK),
            5 => Some(STATE_A::DATA),
            6 => Some(STATE_A::DATAACK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == STATE_A::WAIT
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STATE_A::START
    }
    #[doc = "Checks if the value of the field is `ADDR`"]
    #[inline(always)]
    pub fn is_addr(&self) -> bool {
        *self == STATE_A::ADDR
    }
    #[doc = "Checks if the value of the field is `ADDRACK`"]
    #[inline(always)]
    pub fn is_addrack(&self) -> bool {
        *self == STATE_A::ADDRACK
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == STATE_A::DATA
    }
    #[doc = "Checks if the value of the field is `DATAACK`"]
    #[inline(always)]
    pub fn is_dataack(&self) -> bool {
        *self == STATE_A::DATAACK
    }
}
impl R {
    #[doc = "Bit 0 - Bus Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Leader"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter"]
    #[inline(always)]
    pub fn transmitter(&self) -> TRANSMITTER_R {
        TRANSMITTER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Nack Received"]
    #[inline(always)]
    pub fn nacked(&self) -> NACKED_R {
        NACKED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Held"]
    #[inline(always)]
    pub fn bushold(&self) -> BUSHOLD_R {
        BUSHOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Transmission State"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 5) & 7) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](index.html) module"]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state::R](R) reader structure"]
impl crate::Readable for STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE to value 0x01"]
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
