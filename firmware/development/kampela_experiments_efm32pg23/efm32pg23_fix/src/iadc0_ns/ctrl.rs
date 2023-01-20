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
#[doc = "Field `EM23WUCONVERT` reader - EM23 Wakeup on Conversion"]
pub type EM23WUCONVERT_R = crate::BitReader<EM23WUCONVERT_A>;
#[doc = "EM23 Wakeup on Conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM23WUCONVERT_A {
    #[doc = "0: When using suspend mode, conversions performed in EM2 or EM3 should not wake up the DMA until the FIFO's DVL setting is reached. This saves more power for large OSR settings or infrequent sampling."]
    WUDVL = 0,
    #[doc = "1: When using suspend mode, conversions performed in EM2 or EM3 will wake up the DMA and keep it awake until the conversions are done, regardless of the DVL setting. This mode burns more power, but it is useful when the conversion rate is faster than the time for the DMA to cycle through wake up and going back to sleep as it converts more than 4 scan table entries. Without using the wake up on conversion mode, the FIFO may overflow while the DMA is going in and out of sleep."]
    WUCONVERT = 1,
}
impl From<EM23WUCONVERT_A> for bool {
    #[inline(always)]
    fn from(variant: EM23WUCONVERT_A) -> Self {
        variant as u8 != 0
    }
}
impl EM23WUCONVERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EM23WUCONVERT_A {
        match self.bits {
            false => EM23WUCONVERT_A::WUDVL,
            true => EM23WUCONVERT_A::WUCONVERT,
        }
    }
    #[doc = "Checks if the value of the field is `WUDVL`"]
    #[inline(always)]
    pub fn is_wudvl(&self) -> bool {
        *self == EM23WUCONVERT_A::WUDVL
    }
    #[doc = "Checks if the value of the field is `WUCONVERT`"]
    #[inline(always)]
    pub fn is_wuconvert(&self) -> bool {
        *self == EM23WUCONVERT_A::WUCONVERT
    }
}
#[doc = "Field `EM23WUCONVERT` writer - EM23 Wakeup on Conversion"]
pub type EM23WUCONVERT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SPEC, EM23WUCONVERT_A, O>;
impl<'a, const O: u8> EM23WUCONVERT_W<'a, O> {
    #[doc = "When using suspend mode, conversions performed in EM2 or EM3 should not wake up the DMA until the FIFO's DVL setting is reached. This saves more power for large OSR settings or infrequent sampling."]
    #[inline(always)]
    pub fn wudvl(self) -> &'a mut W {
        self.variant(EM23WUCONVERT_A::WUDVL)
    }
    #[doc = "When using suspend mode, conversions performed in EM2 or EM3 will wake up the DMA and keep it awake until the conversions are done, regardless of the DVL setting. This mode burns more power, but it is useful when the conversion rate is faster than the time for the DMA to cycle through wake up and going back to sleep as it converts more than 4 scan table entries. Without using the wake up on conversion mode, the FIFO may overflow while the DMA is going in and out of sleep."]
    #[inline(always)]
    pub fn wuconvert(self) -> &'a mut W {
        self.variant(EM23WUCONVERT_A::WUCONVERT)
    }
}
#[doc = "Field `ADCCLKSUSPEND0` reader - ADC_CLK Suspend - PRS0"]
pub type ADCCLKSUSPEND0_R = crate::BitReader<ADCCLKSUSPEND0_A>;
#[doc = "ADC_CLK Suspend - PRS0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCCLKSUSPEND0_A {
    #[doc = "0: Normal mode which does not disable the ADC_CLK."]
    PRSWUDIS = 0,
    #[doc = "1: ADCCLKWUEN will gate off ADC_CLK until the trigger is detected provided the internal timer is not selected as the trigger. Once the trigger is detected the ADC_CLK will be started, the band gap will be started, the ADC will be warmed up, and the SCAN Table and the Single entry will be converted. Once the conversions are done, the ADC_CLK will be gated off."]
    PRSWUEN = 1,
}
impl From<ADCCLKSUSPEND0_A> for bool {
    #[inline(always)]
    fn from(variant: ADCCLKSUSPEND0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCCLKSUSPEND0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCCLKSUSPEND0_A {
        match self.bits {
            false => ADCCLKSUSPEND0_A::PRSWUDIS,
            true => ADCCLKSUSPEND0_A::PRSWUEN,
        }
    }
    #[doc = "Checks if the value of the field is `PRSWUDIS`"]
    #[inline(always)]
    pub fn is_prswudis(&self) -> bool {
        *self == ADCCLKSUSPEND0_A::PRSWUDIS
    }
    #[doc = "Checks if the value of the field is `PRSWUEN`"]
    #[inline(always)]
    pub fn is_prswuen(&self) -> bool {
        *self == ADCCLKSUSPEND0_A::PRSWUEN
    }
}
#[doc = "Field `ADCCLKSUSPEND0` writer - ADC_CLK Suspend - PRS0"]
pub type ADCCLKSUSPEND0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SPEC, ADCCLKSUSPEND0_A, O>;
impl<'a, const O: u8> ADCCLKSUSPEND0_W<'a, O> {
    #[doc = "Normal mode which does not disable the ADC_CLK."]
    #[inline(always)]
    pub fn prswudis(self) -> &'a mut W {
        self.variant(ADCCLKSUSPEND0_A::PRSWUDIS)
    }
    #[doc = "ADCCLKWUEN will gate off ADC_CLK until the trigger is detected provided the internal timer is not selected as the trigger. Once the trigger is detected the ADC_CLK will be started, the band gap will be started, the ADC will be warmed up, and the SCAN Table and the Single entry will be converted. Once the conversions are done, the ADC_CLK will be gated off."]
    #[inline(always)]
    pub fn prswuen(self) -> &'a mut W {
        self.variant(ADCCLKSUSPEND0_A::PRSWUEN)
    }
}
#[doc = "Field `ADCCLKSUSPEND1` reader - ADC_CLK Suspend - PRS1"]
pub type ADCCLKSUSPEND1_R = crate::BitReader<ADCCLKSUSPEND1_A>;
#[doc = "ADC_CLK Suspend - PRS1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCCLKSUSPEND1_A {
    #[doc = "0: Normal mode which does not disable the ADC_CLK."]
    PRSWUDIS = 0,
    #[doc = "1: ADCCLKWUEN will gate off ADC_CLK until the trigger is detected provided the internal timer is not selected as the trigger. Once the trigger is detected the ADC_CLK will be started, the band gap will be started, the ADC will be warmed up, and the SCAN Table and the Single entry will be converted. Once the conversions are done, the ADC_CLK will be gated off."]
    PRSWUEN = 1,
}
impl From<ADCCLKSUSPEND1_A> for bool {
    #[inline(always)]
    fn from(variant: ADCCLKSUSPEND1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCCLKSUSPEND1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCCLKSUSPEND1_A {
        match self.bits {
            false => ADCCLKSUSPEND1_A::PRSWUDIS,
            true => ADCCLKSUSPEND1_A::PRSWUEN,
        }
    }
    #[doc = "Checks if the value of the field is `PRSWUDIS`"]
    #[inline(always)]
    pub fn is_prswudis(&self) -> bool {
        *self == ADCCLKSUSPEND1_A::PRSWUDIS
    }
    #[doc = "Checks if the value of the field is `PRSWUEN`"]
    #[inline(always)]
    pub fn is_prswuen(&self) -> bool {
        *self == ADCCLKSUSPEND1_A::PRSWUEN
    }
}
#[doc = "Field `ADCCLKSUSPEND1` writer - ADC_CLK Suspend - PRS1"]
pub type ADCCLKSUSPEND1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SPEC, ADCCLKSUSPEND1_A, O>;
impl<'a, const O: u8> ADCCLKSUSPEND1_W<'a, O> {
    #[doc = "Normal mode which does not disable the ADC_CLK."]
    #[inline(always)]
    pub fn prswudis(self) -> &'a mut W {
        self.variant(ADCCLKSUSPEND1_A::PRSWUDIS)
    }
    #[doc = "ADCCLKWUEN will gate off ADC_CLK until the trigger is detected provided the internal timer is not selected as the trigger. Once the trigger is detected the ADC_CLK will be started, the band gap will be started, the ADC will be warmed up, and the SCAN Table and the Single entry will be converted. Once the conversions are done, the ADC_CLK will be gated off."]
    #[inline(always)]
    pub fn prswuen(self) -> &'a mut W {
        self.variant(ADCCLKSUSPEND1_A::PRSWUEN)
    }
}
#[doc = "Field `DBGHALT` reader - Debug Halt"]
pub type DBGHALT_R = crate::BitReader<DBGHALT_A>;
#[doc = "Debug Halt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGHALT_A {
    #[doc = "0: Continue operation as normal during debug mode"]
    NORMAL = 0,
    #[doc = "1: Complete the current conversion and then halt during debug mode"]
    HALT = 1,
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
            false => DBGHALT_A::NORMAL,
            true => DBGHALT_A::HALT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DBGHALT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == DBGHALT_A::HALT
    }
}
#[doc = "Field `DBGHALT` writer - Debug Halt"]
pub type DBGHALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DBGHALT_A, O>;
impl<'a, const O: u8> DBGHALT_W<'a, O> {
    #[doc = "Continue operation as normal during debug mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DBGHALT_A::NORMAL)
    }
    #[doc = "Complete the current conversion and then halt during debug mode"]
    #[inline(always)]
    pub fn halt(self) -> &'a mut W {
        self.variant(DBGHALT_A::HALT)
    }
}
#[doc = "Field `WARMUPMODE` reader - Warmup Mode"]
pub type WARMUPMODE_R = crate::FieldReader<u8, WARMUPMODE_A>;
#[doc = "Warmup Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WARMUPMODE_A {
    #[doc = "0: Shut down the IADC after conversions have completed."]
    NORMAL = 0,
    #[doc = "1: Switch to standby mode after conversions have completed. The next warmup time will require 1us."]
    KEEPINSTANDBY = 1,
    #[doc = "2: Keep IADC fully powered after conversions have completed."]
    KEEPWARM = 2,
}
impl From<WARMUPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMUPMODE_A) -> Self {
        variant as _
    }
}
impl WARMUPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WARMUPMODE_A> {
        match self.bits {
            0 => Some(WARMUPMODE_A::NORMAL),
            1 => Some(WARMUPMODE_A::KEEPINSTANDBY),
            2 => Some(WARMUPMODE_A::KEEPWARM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `KEEPINSTANDBY`"]
    #[inline(always)]
    pub fn is_keepinstandby(&self) -> bool {
        *self == WARMUPMODE_A::KEEPINSTANDBY
    }
    #[doc = "Checks if the value of the field is `KEEPWARM`"]
    #[inline(always)]
    pub fn is_keepwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPWARM
    }
}
#[doc = "Field `WARMUPMODE` writer - Warmup Mode"]
pub type WARMUPMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, WARMUPMODE_A, 2, O>;
impl<'a, const O: u8> WARMUPMODE_W<'a, O> {
    #[doc = "Shut down the IADC after conversions have completed."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::NORMAL)
    }
    #[doc = "Switch to standby mode after conversions have completed. The next warmup time will require 1us."]
    #[inline(always)]
    pub fn keepinstandby(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPINSTANDBY)
    }
    #[doc = "Keep IADC fully powered after conversions have completed."]
    #[inline(always)]
    pub fn keepwarm(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPWARM)
    }
}
#[doc = "Field `TIMEBASE` reader - Time Base"]
pub type TIMEBASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMEBASE` writer - Time Base"]
pub type TIMEBASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `HSCLKRATE` reader - High Speed Clock Rate"]
pub type HSCLKRATE_R = crate::FieldReader<u8, HSCLKRATE_A>;
#[doc = "High Speed Clock Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSCLKRATE_A {
    #[doc = "0: Use CMU_CLK_ADC directly. The source clock must be 40 MHz or less."]
    DIV1 = 0,
    #[doc = "1: Divide CMU_CLK_ADC by 2 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    DIV2 = 1,
    #[doc = "2: Divide CMU_CLK_ADC by 3 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    DIV3 = 2,
    #[doc = "3: Divide CMU_CLK_ADC by 4 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    DIV4 = 3,
}
impl From<HSCLKRATE_A> for u8 {
    #[inline(always)]
    fn from(variant: HSCLKRATE_A) -> Self {
        variant as _
    }
}
impl HSCLKRATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HSCLKRATE_A> {
        match self.bits {
            0 => Some(HSCLKRATE_A::DIV1),
            1 => Some(HSCLKRATE_A::DIV2),
            2 => Some(HSCLKRATE_A::DIV3),
            3 => Some(HSCLKRATE_A::DIV4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HSCLKRATE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HSCLKRATE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == HSCLKRATE_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSCLKRATE_A::DIV4
    }
}
#[doc = "Field `HSCLKRATE` writer - High Speed Clock Rate"]
pub type HSCLKRATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, HSCLKRATE_A, 3, O>;
impl<'a, const O: u8> HSCLKRATE_W<'a, O> {
    #[doc = "Use CMU_CLK_ADC directly. The source clock must be 40 MHz or less."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HSCLKRATE_A::DIV1)
    }
    #[doc = "Divide CMU_CLK_ADC by 2 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HSCLKRATE_A::DIV2)
    }
    #[doc = "Divide CMU_CLK_ADC by 3 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(HSCLKRATE_A::DIV3)
    }
    #[doc = "Divide CMU_CLK_ADC by 4 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HSCLKRATE_A::DIV4)
    }
}
impl R {
    #[doc = "Bit 0 - EM23 Wakeup on Conversion"]
    #[inline(always)]
    pub fn em23wuconvert(&self) -> EM23WUCONVERT_R {
        EM23WUCONVERT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC_CLK Suspend - PRS0"]
    #[inline(always)]
    pub fn adcclksuspend0(&self) -> ADCCLKSUSPEND0_R {
        ADCCLKSUSPEND0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC_CLK Suspend - PRS1"]
    #[inline(always)]
    pub fn adcclksuspend1(&self) -> ADCCLKSUSPEND1_R {
        ADCCLKSUSPEND1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Warmup Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Time Base"]
    #[inline(always)]
    pub fn timebase(&self) -> TIMEBASE_R {
        TIMEBASE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 28:30 - High Speed Clock Rate"]
    #[inline(always)]
    pub fn hsclkrate(&self) -> HSCLKRATE_R {
        HSCLKRATE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EM23 Wakeup on Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn em23wuconvert(&mut self) -> EM23WUCONVERT_W<0> {
        EM23WUCONVERT_W::new(self)
    }
    #[doc = "Bit 1 - ADC_CLK Suspend - PRS0"]
    #[inline(always)]
    #[must_use]
    pub fn adcclksuspend0(&mut self) -> ADCCLKSUSPEND0_W<1> {
        ADCCLKSUSPEND0_W::new(self)
    }
    #[doc = "Bit 2 - ADC_CLK Suspend - PRS1"]
    #[inline(always)]
    #[must_use]
    pub fn adcclksuspend1(&mut self) -> ADCCLKSUSPEND1_W<2> {
        ADCCLKSUSPEND1_W::new(self)
    }
    #[doc = "Bit 3 - Debug Halt"]
    #[inline(always)]
    #[must_use]
    pub fn dbghalt(&mut self) -> DBGHALT_W<3> {
        DBGHALT_W::new(self)
    }
    #[doc = "Bits 4:5 - Warmup Mode"]
    #[inline(always)]
    #[must_use]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W<4> {
        WARMUPMODE_W::new(self)
    }
    #[doc = "Bits 16:22 - Time Base"]
    #[inline(always)]
    #[must_use]
    pub fn timebase(&mut self) -> TIMEBASE_W<16> {
        TIMEBASE_W::new(self)
    }
    #[doc = "Bits 28:30 - High Speed Clock Rate"]
    #[inline(always)]
    #[must_use]
    pub fn hsclkrate(&mut self) -> HSCLKRATE_W<28> {
        HSCLKRATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
