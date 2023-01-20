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
#[doc = "Field `ADCMODE` reader - ADC Mode"]
pub type ADCMODE_R = crate::FieldReader<u8, ADCMODE_A>;
#[doc = "ADC Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCMODE_A {
    #[doc = "0: High speed mode with a maximum CLK_ADC of 10 MHz."]
    NORMAL = 0,
    #[doc = "1: Double high speed mode with a maximum CLK_ADC of 20 MHz. Power consumption is boosted to allow faster conversions."]
    HIGHSPEED = 1,
    #[doc = "2: High accuracy mode with maximum CLK_ADC of 5 MHz."]
    HIGHACCURACY = 2,
}
impl From<ADCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCMODE_A) -> Self {
        variant as _
    }
}
impl ADCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCMODE_A> {
        match self.bits {
            0 => Some(ADCMODE_A::NORMAL),
            1 => Some(ADCMODE_A::HIGHSPEED),
            2 => Some(ADCMODE_A::HIGHACCURACY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ADCMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_highspeed(&self) -> bool {
        *self == ADCMODE_A::HIGHSPEED
    }
    #[doc = "Checks if the value of the field is `HIGHACCURACY`"]
    #[inline(always)]
    pub fn is_highaccuracy(&self) -> bool {
        *self == ADCMODE_A::HIGHACCURACY
    }
}
#[doc = "Field `ADCMODE` writer - ADC Mode"]
pub type ADCMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, ADCMODE_A, 2, O>;
impl<'a, const O: u8> ADCMODE_W<'a, O> {
    #[doc = "High speed mode with a maximum CLK_ADC of 10 MHz."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ADCMODE_A::NORMAL)
    }
    #[doc = "Double high speed mode with a maximum CLK_ADC of 20 MHz. Power consumption is boosted to allow faster conversions."]
    #[inline(always)]
    pub fn highspeed(self) -> &'a mut W {
        self.variant(ADCMODE_A::HIGHSPEED)
    }
    #[doc = "High accuracy mode with maximum CLK_ADC of 5 MHz."]
    #[inline(always)]
    pub fn highaccuracy(self) -> &'a mut W {
        self.variant(ADCMODE_A::HIGHACCURACY)
    }
}
#[doc = "Field `OSRHS` reader - High Speed OSR"]
pub type OSRHS_R = crate::FieldReader<u8, OSRHS_A>;
#[doc = "High Speed OSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSRHS_A {
    #[doc = "0: High speed over sampling of 2x."]
    HISPD2 = 0,
    #[doc = "1: High speed over sampling of 4x."]
    HISPD4 = 1,
    #[doc = "2: High speed over sampling of 8x."]
    HISPD8 = 2,
    #[doc = "3: High speed over sampling of 16x."]
    HISPD16 = 3,
    #[doc = "4: HIgh speed over sampling of 32x."]
    HISPD32 = 4,
    #[doc = "5: High speed over sampling of 64x."]
    HISPD64 = 5,
}
impl From<OSRHS_A> for u8 {
    #[inline(always)]
    fn from(variant: OSRHS_A) -> Self {
        variant as _
    }
}
impl OSRHS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSRHS_A> {
        match self.bits {
            0 => Some(OSRHS_A::HISPD2),
            1 => Some(OSRHS_A::HISPD4),
            2 => Some(OSRHS_A::HISPD8),
            3 => Some(OSRHS_A::HISPD16),
            4 => Some(OSRHS_A::HISPD32),
            5 => Some(OSRHS_A::HISPD64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HISPD2`"]
    #[inline(always)]
    pub fn is_hispd2(&self) -> bool {
        *self == OSRHS_A::HISPD2
    }
    #[doc = "Checks if the value of the field is `HISPD4`"]
    #[inline(always)]
    pub fn is_hispd4(&self) -> bool {
        *self == OSRHS_A::HISPD4
    }
    #[doc = "Checks if the value of the field is `HISPD8`"]
    #[inline(always)]
    pub fn is_hispd8(&self) -> bool {
        *self == OSRHS_A::HISPD8
    }
    #[doc = "Checks if the value of the field is `HISPD16`"]
    #[inline(always)]
    pub fn is_hispd16(&self) -> bool {
        *self == OSRHS_A::HISPD16
    }
    #[doc = "Checks if the value of the field is `HISPD32`"]
    #[inline(always)]
    pub fn is_hispd32(&self) -> bool {
        *self == OSRHS_A::HISPD32
    }
    #[doc = "Checks if the value of the field is `HISPD64`"]
    #[inline(always)]
    pub fn is_hispd64(&self) -> bool {
        *self == OSRHS_A::HISPD64
    }
}
#[doc = "Field `OSRHS` writer - High Speed OSR"]
pub type OSRHS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, OSRHS_A, 3, O>;
impl<'a, const O: u8> OSRHS_W<'a, O> {
    #[doc = "High speed over sampling of 2x."]
    #[inline(always)]
    pub fn hispd2(self) -> &'a mut W {
        self.variant(OSRHS_A::HISPD2)
    }
    #[doc = "High speed over sampling of 4x."]
    #[inline(always)]
    pub fn hispd4(self) -> &'a mut W {
        self.variant(OSRHS_A::HISPD4)
    }
    #[doc = "High speed over sampling of 8x."]
    #[inline(always)]
    pub fn hispd8(self) -> &'a mut W {
        self.variant(OSRHS_A::HISPD8)
    }
    #[doc = "High speed over sampling of 16x."]
    #[inline(always)]
    pub fn hispd16(self) -> &'a mut W {
        self.variant(OSRHS_A::HISPD16)
    }
    #[doc = "HIgh speed over sampling of 32x."]
    #[inline(always)]
    pub fn hispd32(self) -> &'a mut W {
        self.variant(OSRHS_A::HISPD32)
    }
    #[doc = "High speed over sampling of 64x."]
    #[inline(always)]
    pub fn hispd64(self) -> &'a mut W {
        self.variant(OSRHS_A::HISPD64)
    }
}
#[doc = "Field `OSRHA` reader - High Accuracy OSR"]
pub type OSRHA_R = crate::FieldReader<u8, OSRHA_A>;
#[doc = "High Accuracy OSR\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSRHA_A {
    #[doc = "0: High accuracy over sampling of 16x."]
    HIACC16 = 0,
    #[doc = "1: High accuracy over sampling of 32x."]
    HIACC32 = 1,
    #[doc = "2: High accuracy over sampling of 64x."]
    HIACC64 = 2,
    #[doc = "3: High accuracy over sampling of 92x."]
    HIACC92 = 3,
    #[doc = "4: High accuracy over sampling of 128x."]
    HIACC128 = 4,
    #[doc = "5: High accuracy over sampling of 256x."]
    HIACC256 = 5,
}
impl From<OSRHA_A> for u8 {
    #[inline(always)]
    fn from(variant: OSRHA_A) -> Self {
        variant as _
    }
}
impl OSRHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSRHA_A> {
        match self.bits {
            0 => Some(OSRHA_A::HIACC16),
            1 => Some(OSRHA_A::HIACC32),
            2 => Some(OSRHA_A::HIACC64),
            3 => Some(OSRHA_A::HIACC92),
            4 => Some(OSRHA_A::HIACC128),
            5 => Some(OSRHA_A::HIACC256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HIACC16`"]
    #[inline(always)]
    pub fn is_hiacc16(&self) -> bool {
        *self == OSRHA_A::HIACC16
    }
    #[doc = "Checks if the value of the field is `HIACC32`"]
    #[inline(always)]
    pub fn is_hiacc32(&self) -> bool {
        *self == OSRHA_A::HIACC32
    }
    #[doc = "Checks if the value of the field is `HIACC64`"]
    #[inline(always)]
    pub fn is_hiacc64(&self) -> bool {
        *self == OSRHA_A::HIACC64
    }
    #[doc = "Checks if the value of the field is `HIACC92`"]
    #[inline(always)]
    pub fn is_hiacc92(&self) -> bool {
        *self == OSRHA_A::HIACC92
    }
    #[doc = "Checks if the value of the field is `HIACC128`"]
    #[inline(always)]
    pub fn is_hiacc128(&self) -> bool {
        *self == OSRHA_A::HIACC128
    }
    #[doc = "Checks if the value of the field is `HIACC256`"]
    #[inline(always)]
    pub fn is_hiacc256(&self) -> bool {
        *self == OSRHA_A::HIACC256
    }
}
#[doc = "Field `OSRHA` writer - High Accuracy OSR"]
pub type OSRHA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, OSRHA_A, 3, O>;
impl<'a, const O: u8> OSRHA_W<'a, O> {
    #[doc = "High accuracy over sampling of 16x."]
    #[inline(always)]
    pub fn hiacc16(self) -> &'a mut W {
        self.variant(OSRHA_A::HIACC16)
    }
    #[doc = "High accuracy over sampling of 32x."]
    #[inline(always)]
    pub fn hiacc32(self) -> &'a mut W {
        self.variant(OSRHA_A::HIACC32)
    }
    #[doc = "High accuracy over sampling of 64x."]
    #[inline(always)]
    pub fn hiacc64(self) -> &'a mut W {
        self.variant(OSRHA_A::HIACC64)
    }
    #[doc = "High accuracy over sampling of 92x."]
    #[inline(always)]
    pub fn hiacc92(self) -> &'a mut W {
        self.variant(OSRHA_A::HIACC92)
    }
    #[doc = "High accuracy over sampling of 128x."]
    #[inline(always)]
    pub fn hiacc128(self) -> &'a mut W {
        self.variant(OSRHA_A::HIACC128)
    }
    #[doc = "High accuracy over sampling of 256x."]
    #[inline(always)]
    pub fn hiacc256(self) -> &'a mut W {
        self.variant(OSRHA_A::HIACC256)
    }
}
#[doc = "Field `ANALOGGAIN` reader - Analog Gain"]
pub type ANALOGGAIN_R = crate::FieldReader<u8, ANALOGGAIN_A>;
#[doc = "Analog Gain\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANALOGGAIN_A {
    #[doc = "1: Analog gain of 0.5x."]
    ANAGAIN0P5 = 1,
    #[doc = "2: Analog gain of 1x."]
    ANAGAIN1 = 2,
    #[doc = "3: Analog gain of 2x."]
    ANAGAIN2 = 3,
    #[doc = "4: Analog gain of 3x."]
    ANAGAIN3 = 4,
    #[doc = "5: Analog gain of 4x."]
    ANAGAIN4 = 5,
}
impl From<ANALOGGAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: ANALOGGAIN_A) -> Self {
        variant as _
    }
}
impl ANALOGGAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ANALOGGAIN_A> {
        match self.bits {
            1 => Some(ANALOGGAIN_A::ANAGAIN0P5),
            2 => Some(ANALOGGAIN_A::ANAGAIN1),
            3 => Some(ANALOGGAIN_A::ANAGAIN2),
            4 => Some(ANALOGGAIN_A::ANAGAIN3),
            5 => Some(ANALOGGAIN_A::ANAGAIN4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ANAGAIN0P5`"]
    #[inline(always)]
    pub fn is_anagain0p5(&self) -> bool {
        *self == ANALOGGAIN_A::ANAGAIN0P5
    }
    #[doc = "Checks if the value of the field is `ANAGAIN1`"]
    #[inline(always)]
    pub fn is_anagain1(&self) -> bool {
        *self == ANALOGGAIN_A::ANAGAIN1
    }
    #[doc = "Checks if the value of the field is `ANAGAIN2`"]
    #[inline(always)]
    pub fn is_anagain2(&self) -> bool {
        *self == ANALOGGAIN_A::ANAGAIN2
    }
    #[doc = "Checks if the value of the field is `ANAGAIN3`"]
    #[inline(always)]
    pub fn is_anagain3(&self) -> bool {
        *self == ANALOGGAIN_A::ANAGAIN3
    }
    #[doc = "Checks if the value of the field is `ANAGAIN4`"]
    #[inline(always)]
    pub fn is_anagain4(&self) -> bool {
        *self == ANALOGGAIN_A::ANAGAIN4
    }
}
#[doc = "Field `ANALOGGAIN` writer - Analog Gain"]
pub type ANALOGGAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG0_SPEC, u8, ANALOGGAIN_A, 3, O>;
impl<'a, const O: u8> ANALOGGAIN_W<'a, O> {
    #[doc = "Analog gain of 0.5x."]
    #[inline(always)]
    pub fn anagain0p5(self) -> &'a mut W {
        self.variant(ANALOGGAIN_A::ANAGAIN0P5)
    }
    #[doc = "Analog gain of 1x."]
    #[inline(always)]
    pub fn anagain1(self) -> &'a mut W {
        self.variant(ANALOGGAIN_A::ANAGAIN1)
    }
    #[doc = "Analog gain of 2x."]
    #[inline(always)]
    pub fn anagain2(self) -> &'a mut W {
        self.variant(ANALOGGAIN_A::ANAGAIN2)
    }
    #[doc = "Analog gain of 3x."]
    #[inline(always)]
    pub fn anagain3(self) -> &'a mut W {
        self.variant(ANALOGGAIN_A::ANAGAIN3)
    }
    #[doc = "Analog gain of 4x."]
    #[inline(always)]
    pub fn anagain4(self) -> &'a mut W {
        self.variant(ANALOGGAIN_A::ANAGAIN4)
    }
}
#[doc = "Field `REFSEL` reader - Reference Select"]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
#[doc = "Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal 1.21 V reference."]
    VBGR = 0,
    #[doc = "1: External Reference. (Calibrated for 1.25V nominal.)"]
    VREF = 1,
    #[doc = "2: External Reference. Supports 2.5V in high accuracy mode."]
    VREF2P5 = 2,
    #[doc = "3: AVDD (unbuffered)"]
    VDDX = 3,
    #[doc = "4: AVDD (buffered) * 0.8"]
    VDDX0P8BUF = 4,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::VBGR),
            1 => Some(REFSEL_A::VREF),
            2 => Some(REFSEL_A::VREF2P5),
            3 => Some(REFSEL_A::VDDX),
            4 => Some(REFSEL_A::VDDX0P8BUF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VBGR`"]
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == REFSEL_A::VBGR
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == REFSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `VREF2P5`"]
    #[inline(always)]
    pub fn is_vref2p5(&self) -> bool {
        *self == REFSEL_A::VREF2P5
    }
    #[doc = "Checks if the value of the field is `VDDX`"]
    #[inline(always)]
    pub fn is_vddx(&self) -> bool {
        *self == REFSEL_A::VDDX
    }
    #[doc = "Checks if the value of the field is `VDDX0P8BUF`"]
    #[inline(always)]
    pub fn is_vddx0p8buf(&self) -> bool {
        *self == REFSEL_A::VDDX0P8BUF
    }
}
#[doc = "Field `REFSEL` writer - Reference Select"]
pub type REFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, REFSEL_A, 3, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "Internal 1.21 V reference."]
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut W {
        self.variant(REFSEL_A::VBGR)
    }
    #[doc = "External Reference. (Calibrated for 1.25V nominal.)"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(REFSEL_A::VREF)
    }
    #[doc = "External Reference. Supports 2.5V in high accuracy mode."]
    #[inline(always)]
    pub fn vref2p5(self) -> &'a mut W {
        self.variant(REFSEL_A::VREF2P5)
    }
    #[doc = "AVDD (unbuffered)"]
    #[inline(always)]
    pub fn vddx(self) -> &'a mut W {
        self.variant(REFSEL_A::VDDX)
    }
    #[doc = "AVDD (buffered) * 0.8"]
    #[inline(always)]
    pub fn vddx0p8buf(self) -> &'a mut W {
        self.variant(REFSEL_A::VDDX0P8BUF)
    }
}
#[doc = "Field `DIGAVG` reader - Digital Averaging"]
pub type DIGAVG_R = crate::FieldReader<u8, DIGAVG_A>;
#[doc = "Digital Averaging\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIGAVG_A {
    #[doc = "0: Collect one output word (no digital averaging)."]
    AVG1 = 0,
    #[doc = "1: Collect and average 2 digital output words."]
    AVG2 = 1,
    #[doc = "2: Collect and average 4 digital output words."]
    AVG4 = 2,
    #[doc = "3: Collect and average 8 digital output words."]
    AVG8 = 3,
    #[doc = "4: Collect and average 16 digital output words."]
    AVG16 = 4,
}
impl From<DIGAVG_A> for u8 {
    #[inline(always)]
    fn from(variant: DIGAVG_A) -> Self {
        variant as _
    }
}
impl DIGAVG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIGAVG_A> {
        match self.bits {
            0 => Some(DIGAVG_A::AVG1),
            1 => Some(DIGAVG_A::AVG2),
            2 => Some(DIGAVG_A::AVG4),
            3 => Some(DIGAVG_A::AVG8),
            4 => Some(DIGAVG_A::AVG16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AVG1`"]
    #[inline(always)]
    pub fn is_avg1(&self) -> bool {
        *self == DIGAVG_A::AVG1
    }
    #[doc = "Checks if the value of the field is `AVG2`"]
    #[inline(always)]
    pub fn is_avg2(&self) -> bool {
        *self == DIGAVG_A::AVG2
    }
    #[doc = "Checks if the value of the field is `AVG4`"]
    #[inline(always)]
    pub fn is_avg4(&self) -> bool {
        *self == DIGAVG_A::AVG4
    }
    #[doc = "Checks if the value of the field is `AVG8`"]
    #[inline(always)]
    pub fn is_avg8(&self) -> bool {
        *self == DIGAVG_A::AVG8
    }
    #[doc = "Checks if the value of the field is `AVG16`"]
    #[inline(always)]
    pub fn is_avg16(&self) -> bool {
        *self == DIGAVG_A::AVG16
    }
}
#[doc = "Field `DIGAVG` writer - Digital Averaging"]
pub type DIGAVG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, DIGAVG_A, 3, O>;
impl<'a, const O: u8> DIGAVG_W<'a, O> {
    #[doc = "Collect one output word (no digital averaging)."]
    #[inline(always)]
    pub fn avg1(self) -> &'a mut W {
        self.variant(DIGAVG_A::AVG1)
    }
    #[doc = "Collect and average 2 digital output words."]
    #[inline(always)]
    pub fn avg2(self) -> &'a mut W {
        self.variant(DIGAVG_A::AVG2)
    }
    #[doc = "Collect and average 4 digital output words."]
    #[inline(always)]
    pub fn avg4(self) -> &'a mut W {
        self.variant(DIGAVG_A::AVG4)
    }
    #[doc = "Collect and average 8 digital output words."]
    #[inline(always)]
    pub fn avg8(self) -> &'a mut W {
        self.variant(DIGAVG_A::AVG8)
    }
    #[doc = "Collect and average 16 digital output words."]
    #[inline(always)]
    pub fn avg16(self) -> &'a mut W {
        self.variant(DIGAVG_A::AVG16)
    }
}
#[doc = "Field `TWOSCOMPL` reader - Two's Complement"]
pub type TWOSCOMPL_R = crate::FieldReader<u8, TWOSCOMPL_A>;
#[doc = "Two's Complement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TWOSCOMPL_A {
    #[doc = "0: Automatic: Single ended measurements are reported as unipolar and differential measurements are reported as bipolar."]
    AUTO = 0,
    #[doc = "1: Force all measurements to result in unipolar output. Negative differential numbers will saturate to 0."]
    FORCEUNIPOLAR = 1,
    #[doc = "2: Force all measurements to result in bipolar output. Single ended measurements are half the range, but allow for small negative measurements."]
    FORCEBIPOLAR = 2,
}
impl From<TWOSCOMPL_A> for u8 {
    #[inline(always)]
    fn from(variant: TWOSCOMPL_A) -> Self {
        variant as _
    }
}
impl TWOSCOMPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TWOSCOMPL_A> {
        match self.bits {
            0 => Some(TWOSCOMPL_A::AUTO),
            1 => Some(TWOSCOMPL_A::FORCEUNIPOLAR),
            2 => Some(TWOSCOMPL_A::FORCEBIPOLAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == TWOSCOMPL_A::AUTO
    }
    #[doc = "Checks if the value of the field is `FORCEUNIPOLAR`"]
    #[inline(always)]
    pub fn is_forceunipolar(&self) -> bool {
        *self == TWOSCOMPL_A::FORCEUNIPOLAR
    }
    #[doc = "Checks if the value of the field is `FORCEBIPOLAR`"]
    #[inline(always)]
    pub fn is_forcebipolar(&self) -> bool {
        *self == TWOSCOMPL_A::FORCEBIPOLAR
    }
}
#[doc = "Field `TWOSCOMPL` writer - Two's Complement"]
pub type TWOSCOMPL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG0_SPEC, u8, TWOSCOMPL_A, 2, O>;
impl<'a, const O: u8> TWOSCOMPL_W<'a, O> {
    #[doc = "Automatic: Single ended measurements are reported as unipolar and differential measurements are reported as bipolar."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(TWOSCOMPL_A::AUTO)
    }
    #[doc = "Force all measurements to result in unipolar output. Negative differential numbers will saturate to 0."]
    #[inline(always)]
    pub fn forceunipolar(self) -> &'a mut W {
        self.variant(TWOSCOMPL_A::FORCEUNIPOLAR)
    }
    #[doc = "Force all measurements to result in bipolar output. Single ended measurements are half the range, but allow for small negative measurements."]
    #[inline(always)]
    pub fn forcebipolar(self) -> &'a mut W {
        self.variant(TWOSCOMPL_A::FORCEBIPOLAR)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC Mode"]
    #[inline(always)]
    pub fn adcmode(&self) -> ADCMODE_R {
        ADCMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - High Speed OSR"]
    #[inline(always)]
    pub fn osrhs(&self) -> OSRHS_R {
        OSRHS_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - High Accuracy OSR"]
    #[inline(always)]
    pub fn osrha(&self) -> OSRHA_R {
        OSRHA_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Analog Gain"]
    #[inline(always)]
    pub fn analoggain(&self) -> ANALOGGAIN_R {
        ANALOGGAIN_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Reference Select"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Digital Averaging"]
    #[inline(always)]
    pub fn digavg(&self) -> DIGAVG_R {
        DIGAVG_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Two's Complement"]
    #[inline(always)]
    pub fn twoscompl(&self) -> TWOSCOMPL_R {
        TWOSCOMPL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adcmode(&mut self) -> ADCMODE_W<0> {
        ADCMODE_W::new(self)
    }
    #[doc = "Bits 2:4 - High Speed OSR"]
    #[inline(always)]
    #[must_use]
    pub fn osrhs(&mut self) -> OSRHS_W<2> {
        OSRHS_W::new(self)
    }
    #[doc = "Bits 5:7 - High Accuracy OSR"]
    #[inline(always)]
    #[must_use]
    pub fn osrha(&mut self) -> OSRHA_W<5> {
        OSRHA_W::new(self)
    }
    #[doc = "Bits 12:14 - Analog Gain"]
    #[inline(always)]
    #[must_use]
    pub fn analoggain(&mut self) -> ANALOGGAIN_W<12> {
        ANALOGGAIN_W::new(self)
    }
    #[doc = "Bits 16:18 - Reference Select"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<16> {
        REFSEL_W::new(self)
    }
    #[doc = "Bits 21:23 - Digital Averaging"]
    #[inline(always)]
    #[must_use]
    pub fn digavg(&mut self) -> DIGAVG_W<21> {
        DIGAVG_W::new(self)
    }
    #[doc = "Bits 28:29 - Two's Complement"]
    #[inline(always)]
    #[must_use]
    pub fn twoscompl(&mut self) -> TWOSCOMPL_W<28> {
        TWOSCOMPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](index.html) module"]
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
#[doc = "`reset()` method sets CFG0 to value 0x2060"]
impl crate::Resettable for CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x2060;
}
