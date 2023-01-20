#[doc = "Register `CH0CFG` reader"]
pub struct R(crate::R<CH0CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH0CFG` writer"]
pub struct W(crate::W<CH0CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH0CFG_SPEC>;
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
impl From<crate::W<CH0CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH0CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONVMODE` reader - Channel 0 Conversion Mode"]
pub type CONVMODE_R = crate::BitReader<CONVMODE_A>;
#[doc = "Channel 0 Conversion Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONVMODE_A {
    #[doc = "0: DAC channel 0 is set in continuous mode"]
    CONTINUOUS = 0,
    #[doc = "1: DAC channel 0 is set in sample/shut off mode"]
    SAMPLEOFF = 1,
}
impl From<CONVMODE_A> for bool {
    #[inline(always)]
    fn from(variant: CONVMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl CONVMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONVMODE_A {
        match self.bits {
            false => CONVMODE_A::CONTINUOUS,
            true => CONVMODE_A::SAMPLEOFF,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONVMODE_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `SAMPLEOFF`"]
    #[inline(always)]
    pub fn is_sampleoff(&self) -> bool {
        *self == CONVMODE_A::SAMPLEOFF
    }
}
#[doc = "Field `CONVMODE` writer - Channel 0 Conversion Mode"]
pub type CONVMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH0CFG_SPEC, CONVMODE_A, O>;
impl<'a, const O: u8> CONVMODE_W<'a, O> {
    #[doc = "DAC channel 0 is set in continuous mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONVMODE_A::CONTINUOUS)
    }
    #[doc = "DAC channel 0 is set in sample/shut off mode"]
    #[inline(always)]
    pub fn sampleoff(self) -> &'a mut W {
        self.variant(CONVMODE_A::SAMPLEOFF)
    }
}
#[doc = "Field `POWERMODE` reader - Channel 0 Power Mode"]
pub type POWERMODE_R = crate::BitReader<POWERMODE_A>;
#[doc = "Channel 0 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POWERMODE_A {
    #[doc = "0: Default is High Power Mode"]
    HIGHPOWER = 0,
    #[doc = "1: Set this bit for Low Power Mode"]
    LOWPOWER = 1,
}
impl From<POWERMODE_A> for bool {
    #[inline(always)]
    fn from(variant: POWERMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl POWERMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWERMODE_A {
        match self.bits {
            false => POWERMODE_A::HIGHPOWER,
            true => POWERMODE_A::LOWPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `HIGHPOWER`"]
    #[inline(always)]
    pub fn is_highpower(&self) -> bool {
        *self == POWERMODE_A::HIGHPOWER
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline(always)]
    pub fn is_lowpower(&self) -> bool {
        *self == POWERMODE_A::LOWPOWER
    }
}
#[doc = "Field `POWERMODE` writer - Channel 0 Power Mode"]
pub type POWERMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH0CFG_SPEC, POWERMODE_A, O>;
impl<'a, const O: u8> POWERMODE_W<'a, O> {
    #[doc = "Default is High Power Mode"]
    #[inline(always)]
    pub fn highpower(self) -> &'a mut W {
        self.variant(POWERMODE_A::HIGHPOWER)
    }
    #[doc = "Set this bit for Low Power Mode"]
    #[inline(always)]
    pub fn lowpower(self) -> &'a mut W {
        self.variant(POWERMODE_A::LOWPOWER)
    }
}
#[doc = "Field `TRIGMODE` reader - Channel 0 Trigger Mode"]
pub type TRIGMODE_R = crate::FieldReader<u8, TRIGMODE_A>;
#[doc = "Channel 0 Trigger Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGMODE_A {
    #[doc = "0: No Conversion Trigger Source Selected for Channel 0"]
    NONE = 0,
    #[doc = "1: Channel 0 is triggered by Channel 0 FIFO (CH0F) write"]
    SW = 1,
    #[doc = "2: Channel 0 is triggered by Sync PRS input. PRS Trigger should have the same clock group as VDAC."]
    SYNCPRS = 2,
    #[doc = "3: Channel 0 is triggered by LESENSE"]
    LESENSE = 3,
    #[doc = "4: Channel 0 is triggered by Internal Timer Overflow"]
    INTERNALTIMER = 4,
    #[doc = "5: Channel 0 is triggered by Async PRS input"]
    ASYNCPRS = 5,
}
impl From<TRIGMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGMODE_A) -> Self {
        variant as _
    }
}
impl TRIGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGMODE_A> {
        match self.bits {
            0 => Some(TRIGMODE_A::NONE),
            1 => Some(TRIGMODE_A::SW),
            2 => Some(TRIGMODE_A::SYNCPRS),
            3 => Some(TRIGMODE_A::LESENSE),
            4 => Some(TRIGMODE_A::INTERNALTIMER),
            5 => Some(TRIGMODE_A::ASYNCPRS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TRIGMODE_A::NONE
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == TRIGMODE_A::SW
    }
    #[doc = "Checks if the value of the field is `SYNCPRS`"]
    #[inline(always)]
    pub fn is_syncprs(&self) -> bool {
        *self == TRIGMODE_A::SYNCPRS
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == TRIGMODE_A::LESENSE
    }
    #[doc = "Checks if the value of the field is `INTERNALTIMER`"]
    #[inline(always)]
    pub fn is_internaltimer(&self) -> bool {
        *self == TRIGMODE_A::INTERNALTIMER
    }
    #[doc = "Checks if the value of the field is `ASYNCPRS`"]
    #[inline(always)]
    pub fn is_asyncprs(&self) -> bool {
        *self == TRIGMODE_A::ASYNCPRS
    }
}
#[doc = "Field `TRIGMODE` writer - Channel 0 Trigger Mode"]
pub type TRIGMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH0CFG_SPEC, u8, TRIGMODE_A, 3, O>;
impl<'a, const O: u8> TRIGMODE_W<'a, O> {
    #[doc = "No Conversion Trigger Source Selected for Channel 0"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TRIGMODE_A::NONE)
    }
    #[doc = "Channel 0 is triggered by Channel 0 FIFO (CH0F) write"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut W {
        self.variant(TRIGMODE_A::SW)
    }
    #[doc = "Channel 0 is triggered by Sync PRS input. PRS Trigger should have the same clock group as VDAC."]
    #[inline(always)]
    pub fn syncprs(self) -> &'a mut W {
        self.variant(TRIGMODE_A::SYNCPRS)
    }
    #[doc = "Channel 0 is triggered by LESENSE"]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut W {
        self.variant(TRIGMODE_A::LESENSE)
    }
    #[doc = "Channel 0 is triggered by Internal Timer Overflow"]
    #[inline(always)]
    pub fn internaltimer(self) -> &'a mut W {
        self.variant(TRIGMODE_A::INTERNALTIMER)
    }
    #[doc = "Channel 0 is triggered by Async PRS input"]
    #[inline(always)]
    pub fn asyncprs(self) -> &'a mut W {
        self.variant(TRIGMODE_A::ASYNCPRS)
    }
}
#[doc = "Field `REFRESHSOURCE` reader - Channel 0 Refresh Source"]
pub type REFRESHSOURCE_R = crate::FieldReader<u8, REFRESHSOURCE_A>;
#[doc = "Channel 0 Refresh Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFRESHSOURCE_A {
    #[doc = "0: No Refresh Source Selected for Channel 0."]
    NONE = 0,
    #[doc = "1: Channel 0 Refresh triggered by Refresh Timer Overflow"]
    REFRESHTIMER = 1,
    #[doc = "2: Channel 0 Refresh triggered by Sync PRS. PRS Trigger should have the same clock group as VDAC."]
    SYNCPRS = 2,
    #[doc = "3: Channel 0 Refresh triggered by Async PRS"]
    ASYNCPRS = 3,
}
impl From<REFRESHSOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: REFRESHSOURCE_A) -> Self {
        variant as _
    }
}
impl REFRESHSOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFRESHSOURCE_A {
        match self.bits {
            0 => REFRESHSOURCE_A::NONE,
            1 => REFRESHSOURCE_A::REFRESHTIMER,
            2 => REFRESHSOURCE_A::SYNCPRS,
            3 => REFRESHSOURCE_A::ASYNCPRS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == REFRESHSOURCE_A::NONE
    }
    #[doc = "Checks if the value of the field is `REFRESHTIMER`"]
    #[inline(always)]
    pub fn is_refreshtimer(&self) -> bool {
        *self == REFRESHSOURCE_A::REFRESHTIMER
    }
    #[doc = "Checks if the value of the field is `SYNCPRS`"]
    #[inline(always)]
    pub fn is_syncprs(&self) -> bool {
        *self == REFRESHSOURCE_A::SYNCPRS
    }
    #[doc = "Checks if the value of the field is `ASYNCPRS`"]
    #[inline(always)]
    pub fn is_asyncprs(&self) -> bool {
        *self == REFRESHSOURCE_A::ASYNCPRS
    }
}
#[doc = "Field `REFRESHSOURCE` writer - Channel 0 Refresh Source"]
pub type REFRESHSOURCE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CH0CFG_SPEC, u8, REFRESHSOURCE_A, 2, O>;
impl<'a, const O: u8> REFRESHSOURCE_W<'a, O> {
    #[doc = "No Refresh Source Selected for Channel 0."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(REFRESHSOURCE_A::NONE)
    }
    #[doc = "Channel 0 Refresh triggered by Refresh Timer Overflow"]
    #[inline(always)]
    pub fn refreshtimer(self) -> &'a mut W {
        self.variant(REFRESHSOURCE_A::REFRESHTIMER)
    }
    #[doc = "Channel 0 Refresh triggered by Sync PRS. PRS Trigger should have the same clock group as VDAC."]
    #[inline(always)]
    pub fn syncprs(self) -> &'a mut W {
        self.variant(REFRESHSOURCE_A::SYNCPRS)
    }
    #[doc = "Channel 0 Refresh triggered by Async PRS"]
    #[inline(always)]
    pub fn asyncprs(self) -> &'a mut W {
        self.variant(REFRESHSOURCE_A::ASYNCPRS)
    }
}
#[doc = "Field `FIFODVL` reader - Channel 0 FIFO Low Watermark"]
pub type FIFODVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFODVL` writer - Channel 0 FIFO Low Watermark"]
pub type FIFODVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH0CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `HIGHCAPLOADEN` reader - Channel 0 High Cap Load Mode Enable"]
pub type HIGHCAPLOADEN_R = crate::BitReader<bool>;
#[doc = "Field `HIGHCAPLOADEN` writer - Channel 0 High Cap Load Mode Enable"]
pub type HIGHCAPLOADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH0CFG_SPEC, bool, O>;
#[doc = "Field `KEEPWARM` reader - Channel 0 Keepwarm Mode Enable"]
pub type KEEPWARM_R = crate::BitReader<bool>;
#[doc = "Field `KEEPWARM` writer - Channel 0 Keepwarm Mode Enable"]
pub type KEEPWARM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH0CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel 0 Conversion Mode"]
    #[inline(always)]
    pub fn convmode(&self) -> CONVMODE_R {
        CONVMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 Power Mode"]
    #[inline(always)]
    pub fn powermode(&self) -> POWERMODE_R {
        POWERMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 0 Trigger Mode"]
    #[inline(always)]
    pub fn trigmode(&self) -> TRIGMODE_R {
        TRIGMODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Channel 0 Refresh Source"]
    #[inline(always)]
    pub fn refreshsource(&self) -> REFRESHSOURCE_R {
        REFRESHSOURCE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Channel 0 FIFO Low Watermark"]
    #[inline(always)]
    pub fn fifodvl(&self) -> FIFODVL_R {
        FIFODVL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14 - Channel 0 High Cap Load Mode Enable"]
    #[inline(always)]
    pub fn highcaploaden(&self) -> HIGHCAPLOADEN_R {
        HIGHCAPLOADEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 0 Keepwarm Mode Enable"]
    #[inline(always)]
    pub fn keepwarm(&self) -> KEEPWARM_R {
        KEEPWARM_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Conversion Mode"]
    #[inline(always)]
    #[must_use]
    pub fn convmode(&mut self) -> CONVMODE_W<0> {
        CONVMODE_W::new(self)
    }
    #[doc = "Bit 2 - Channel 0 Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn powermode(&mut self) -> POWERMODE_W<2> {
        POWERMODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel 0 Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn trigmode(&mut self) -> TRIGMODE_W<4> {
        TRIGMODE_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 0 Refresh Source"]
    #[inline(always)]
    #[must_use]
    pub fn refreshsource(&mut self) -> REFRESHSOURCE_W<8> {
        REFRESHSOURCE_W::new(self)
    }
    #[doc = "Bits 11:12 - Channel 0 FIFO Low Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn fifodvl(&mut self) -> FIFODVL_W<11> {
        FIFODVL_W::new(self)
    }
    #[doc = "Bit 14 - Channel 0 High Cap Load Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn highcaploaden(&mut self) -> HIGHCAPLOADEN_W<14> {
        HIGHCAPLOADEN_W::new(self)
    }
    #[doc = "Bit 16 - Channel 0 Keepwarm Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn keepwarm(&mut self) -> KEEPWARM_W<16> {
        KEEPWARM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0cfg](index.html) module"]
pub struct CH0CFG_SPEC;
impl crate::RegisterSpec for CH0CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch0cfg::R](R) reader structure"]
impl crate::Readable for CH0CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch0cfg::W](W) writer structure"]
impl crate::Writable for CH0CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH0CFG to value 0x10"]
impl crate::Resettable for CH0CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
