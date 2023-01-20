#[doc = "Register `SWCAPA0` reader"]
pub struct R(crate::R<SWCAPA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWCAPA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWCAPA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWCAPA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ZIGBEE` reader - Zigbee Capability"]
pub type ZIGBEE_R = crate::FieldReader<u8, ZIGBEE_A>;
#[doc = "Zigbee Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ZIGBEE_A {
    #[doc = "0: ZigBee stack capability not available"]
    LEVEL0 = 0,
    #[doc = "1: GreenPower only"]
    LEVEL1 = 1,
    #[doc = "2: ZigBee and GreenPower"]
    LEVEL2 = 2,
    #[doc = "3: ZigBee Only"]
    LEVEL3 = 3,
}
impl From<ZIGBEE_A> for u8 {
    #[inline(always)]
    fn from(variant: ZIGBEE_A) -> Self {
        variant as _
    }
}
impl ZIGBEE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZIGBEE_A {
        match self.bits {
            0 => ZIGBEE_A::LEVEL0,
            1 => ZIGBEE_A::LEVEL1,
            2 => ZIGBEE_A::LEVEL2,
            3 => ZIGBEE_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == ZIGBEE_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == ZIGBEE_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == ZIGBEE_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == ZIGBEE_A::LEVEL3
    }
}
#[doc = "Field `THREAD` reader - Thread Capability"]
pub type THREAD_R = crate::FieldReader<u8, THREAD_A>;
#[doc = "Thread Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THREAD_A {
    #[doc = "0: RF4CE stack capability not available"]
    LEVEL0 = 0,
    #[doc = "1: RF4CE stack enabled"]
    LEVEL1 = 1,
    #[doc = "2: N/A"]
    LEVEL2 = 2,
    #[doc = "3: N/A"]
    LEVEL3 = 3,
}
impl From<THREAD_A> for u8 {
    #[inline(always)]
    fn from(variant: THREAD_A) -> Self {
        variant as _
    }
}
impl THREAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREAD_A {
        match self.bits {
            0 => THREAD_A::LEVEL0,
            1 => THREAD_A::LEVEL1,
            2 => THREAD_A::LEVEL2,
            3 => THREAD_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == THREAD_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == THREAD_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == THREAD_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == THREAD_A::LEVEL3
    }
}
#[doc = "Field `RF4CE` reader - RF4CE Capability"]
pub type RF4CE_R = crate::FieldReader<u8, RF4CE_A>;
#[doc = "RF4CE Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RF4CE_A {
    #[doc = "0: Thread stack capability not available"]
    LEVEL0 = 0,
    #[doc = "1: Thread stack enabled"]
    LEVEL1 = 1,
    #[doc = "2: N/A"]
    LEVEL2 = 2,
    #[doc = "3: N/A"]
    LEVEL3 = 3,
}
impl From<RF4CE_A> for u8 {
    #[inline(always)]
    fn from(variant: RF4CE_A) -> Self {
        variant as _
    }
}
impl RF4CE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF4CE_A {
        match self.bits {
            0 => RF4CE_A::LEVEL0,
            1 => RF4CE_A::LEVEL1,
            2 => RF4CE_A::LEVEL2,
            3 => RF4CE_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RF4CE_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RF4CE_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RF4CE_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == RF4CE_A::LEVEL3
    }
}
#[doc = "Field `BTSMART` reader - Bluetooth Smart Capability"]
pub type BTSMART_R = crate::FieldReader<u8, BTSMART_A>;
#[doc = "Bluetooth Smart Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BTSMART_A {
    #[doc = "0: Bluetooth SMART stack capability not available"]
    LEVEL0 = 0,
    #[doc = "1: Bluetooth SMART enabled"]
    LEVEL1 = 1,
    #[doc = "2: N/A"]
    LEVEL2 = 2,
    #[doc = "3: N/A"]
    LEVEL3 = 3,
}
impl From<BTSMART_A> for u8 {
    #[inline(always)]
    fn from(variant: BTSMART_A) -> Self {
        variant as _
    }
}
impl BTSMART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTSMART_A {
        match self.bits {
            0 => BTSMART_A::LEVEL0,
            1 => BTSMART_A::LEVEL1,
            2 => BTSMART_A::LEVEL2,
            3 => BTSMART_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == BTSMART_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == BTSMART_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == BTSMART_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == BTSMART_A::LEVEL3
    }
}
#[doc = "Field `CONNECT` reader - Connect Capability"]
pub type CONNECT_R = crate::FieldReader<u8, CONNECT_A>;
#[doc = "Connect Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONNECT_A {
    #[doc = "0: Connect stack capability not available"]
    LEVEL0 = 0,
    #[doc = "1: Connect enabled"]
    LEVEL1 = 1,
    #[doc = "2: N/A"]
    LEVEL2 = 2,
    #[doc = "3: N/A"]
    LEVEL3 = 3,
}
impl From<CONNECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CONNECT_A) -> Self {
        variant as _
    }
}
impl CONNECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONNECT_A {
        match self.bits {
            0 => CONNECT_A::LEVEL0,
            1 => CONNECT_A::LEVEL1,
            2 => CONNECT_A::LEVEL2,
            3 => CONNECT_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == CONNECT_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == CONNECT_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == CONNECT_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == CONNECT_A::LEVEL3
    }
}
#[doc = "Field `SRI` reader - RAIL Capability"]
pub type SRI_R = crate::FieldReader<u8, SRI_A>;
#[doc = "RAIL Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRI_A {
    #[doc = "0: RAIL capability not available"]
    LEVEL0 = 0,
    #[doc = "1: RAIL enabled"]
    LEVEL1 = 1,
    #[doc = "2: N/A"]
    LEVEL2 = 2,
    #[doc = "3: N/A"]
    LEVEL3 = 3,
}
impl From<SRI_A> for u8 {
    #[inline(always)]
    fn from(variant: SRI_A) -> Self {
        variant as _
    }
}
impl SRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRI_A {
        match self.bits {
            0 => SRI_A::LEVEL0,
            1 => SRI_A::LEVEL1,
            2 => SRI_A::LEVEL2,
            3 => SRI_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == SRI_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == SRI_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == SRI_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == SRI_A::LEVEL3
    }
}
#[doc = "Field `ZWAVE` reader - Z-Wave Capability"]
pub type ZWAVE_R = crate::FieldReader<u8, ZWAVE_A>;
#[doc = "Z-Wave Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ZWAVE_A {
    #[doc = "0: Z-Wave stack capability not available"]
    LEVEL0 = 0,
    #[doc = "1: Z-Wave Gateway"]
    LEVEL1 = 1,
    #[doc = "2: Z-Wave End Device"]
    LEVEL2 = 2,
    #[doc = "3: Z-Wave Sensor"]
    LEVEL3 = 3,
    #[doc = "4: Z-Wave Lighting"]
    LEVEL4 = 4,
}
impl From<ZWAVE_A> for u8 {
    #[inline(always)]
    fn from(variant: ZWAVE_A) -> Self {
        variant as _
    }
}
impl ZWAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ZWAVE_A> {
        match self.bits {
            0 => Some(ZWAVE_A::LEVEL0),
            1 => Some(ZWAVE_A::LEVEL1),
            2 => Some(ZWAVE_A::LEVEL2),
            3 => Some(ZWAVE_A::LEVEL3),
            4 => Some(ZWAVE_A::LEVEL4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == ZWAVE_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == ZWAVE_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == ZWAVE_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == ZWAVE_A::LEVEL3
    }
    #[doc = "Checks if the value of the field is `LEVEL4`"]
    #[inline(always)]
    pub fn is_level4(&self) -> bool {
        *self == ZWAVE_A::LEVEL4
    }
}
impl R {
    #[doc = "Bits 0:1 - Zigbee Capability"]
    #[inline(always)]
    pub fn zigbee(&self) -> ZIGBEE_R {
        ZIGBEE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Thread Capability"]
    #[inline(always)]
    pub fn thread(&self) -> THREAD_R {
        THREAD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - RF4CE Capability"]
    #[inline(always)]
    pub fn rf4ce(&self) -> RF4CE_R {
        RF4CE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Bluetooth Smart Capability"]
    #[inline(always)]
    pub fn btsmart(&self) -> BTSMART_R {
        BTSMART_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Connect Capability"]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - RAIL Capability"]
    #[inline(always)]
    pub fn sri(&self) -> SRI_R {
        SRI_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Z-Wave Capability"]
    #[inline(always)]
    pub fn zwave(&self) -> ZWAVE_R {
        ZWAVE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[doc = "Software Capability Vector 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swcapa0](index.html) module"]
pub struct SWCAPA0_SPEC;
impl crate::RegisterSpec for SWCAPA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swcapa0::R](R) reader structure"]
impl crate::Readable for SWCAPA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SWCAPA0 to value 0"]
impl crate::Resettable for SWCAPA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
