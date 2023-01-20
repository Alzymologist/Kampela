#[doc = "Register `BIASCTRL` reader"]
pub struct R(crate::R<BIASCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIASCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIASCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIASCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIASCTRL` writer"]
pub struct W(crate::W<BIASCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIASCTRL_SPEC>;
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
impl From<crate::W<BIASCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIASCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESISTOR` reader - Resistor strength"]
pub type RESISTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESISTOR` writer - Resistor strength"]
pub type RESISTOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIASCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `BUFDRV` reader - Buffer Drive Strength"]
pub type BUFDRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUFDRV` writer - Buffer Drive Strength"]
pub type BUFDRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIASCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `BUFBIAS` reader - Buffer Bias Setting"]
pub type BUFBIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUFBIAS` writer - Buffer Bias Setting"]
pub type BUFBIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIASCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE` reader - Mode Setting"]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Mode Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: Use step down control with VLCD less than VDDX. Use VLCD\\[4:0\\]
to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    STEPDOWN = 0,
    #[doc = "1: Use the charge pump to pump VLCD above VDDX."]
    CHARGEPUMP = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::STEPDOWN,
            true => MODE_A::CHARGEPUMP,
        }
    }
    #[doc = "Checks if the value of the field is `STEPDOWN`"]
    #[inline(always)]
    pub fn is_stepdown(&self) -> bool {
        *self == MODE_A::STEPDOWN
    }
    #[doc = "Checks if the value of the field is `CHARGEPUMP`"]
    #[inline(always)]
    pub fn is_chargepump(&self) -> bool {
        *self == MODE_A::CHARGEPUMP
    }
}
#[doc = "Field `MODE` writer - Mode Setting"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIASCTRL_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Use step down control with VLCD less than VDDX. Use VLCD\\[4:0\\]
to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    #[inline(always)]
    pub fn stepdown(self) -> &'a mut W {
        self.variant(MODE_A::STEPDOWN)
    }
    #[doc = "Use the charge pump to pump VLCD above VDDX."]
    #[inline(always)]
    pub fn chargepump(self) -> &'a mut W {
        self.variant(MODE_A::CHARGEPUMP)
    }
}
#[doc = "Field `VLCD` reader - VLCD voltage level"]
pub type VLCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VLCD` writer - VLCD voltage level"]
pub type VLCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIASCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `VDDXSEL` reader - VDDX select"]
pub type VDDXSEL_R = crate::BitReader<VDDXSEL_A>;
#[doc = "VDDX select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDXSEL_A {
    #[doc = "0: Connect charge pump to digital DVDD supply"]
    DVDD = 0,
    #[doc = "1: Connect charge pump to analog AVDD supply"]
    AVDD = 1,
}
impl From<VDDXSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VDDXSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VDDXSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDXSEL_A {
        match self.bits {
            false => VDDXSEL_A::DVDD,
            true => VDDXSEL_A::AVDD,
        }
    }
    #[doc = "Checks if the value of the field is `DVDD`"]
    #[inline(always)]
    pub fn is_dvdd(&self) -> bool {
        *self == VDDXSEL_A::DVDD
    }
    #[doc = "Checks if the value of the field is `AVDD`"]
    #[inline(always)]
    pub fn is_avdd(&self) -> bool {
        *self == VDDXSEL_A::AVDD
    }
}
#[doc = "Field `VDDXSEL` writer - VDDX select"]
pub type VDDXSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIASCTRL_SPEC, VDDXSEL_A, O>;
impl<'a, const O: u8> VDDXSEL_W<'a, O> {
    #[doc = "Connect charge pump to digital DVDD supply"]
    #[inline(always)]
    pub fn dvdd(self) -> &'a mut W {
        self.variant(VDDXSEL_A::DVDD)
    }
    #[doc = "Connect charge pump to analog AVDD supply"]
    #[inline(always)]
    pub fn avdd(self) -> &'a mut W {
        self.variant(VDDXSEL_A::AVDD)
    }
}
#[doc = "Field `LCDGATE` reader - LCD Gate"]
pub type LCDGATE_R = crate::BitReader<LCDGATE_A>;
#[doc = "LCD Gate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDGATE_A {
    #[doc = "0: LCD BIAS voltages driven onto pins."]
    UNGATE = 0,
    #[doc = "1: LCD BIAS MUX tristated at the pins."]
    GATE = 1,
}
impl From<LCDGATE_A> for bool {
    #[inline(always)]
    fn from(variant: LCDGATE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDGATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDGATE_A {
        match self.bits {
            false => LCDGATE_A::UNGATE,
            true => LCDGATE_A::GATE,
        }
    }
    #[doc = "Checks if the value of the field is `UNGATE`"]
    #[inline(always)]
    pub fn is_ungate(&self) -> bool {
        *self == LCDGATE_A::UNGATE
    }
    #[doc = "Checks if the value of the field is `GATE`"]
    #[inline(always)]
    pub fn is_gate(&self) -> bool {
        *self == LCDGATE_A::GATE
    }
}
#[doc = "Field `LCDGATE` writer - LCD Gate"]
pub type LCDGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIASCTRL_SPEC, LCDGATE_A, O>;
impl<'a, const O: u8> LCDGATE_W<'a, O> {
    #[doc = "LCD BIAS voltages driven onto pins."]
    #[inline(always)]
    pub fn ungate(self) -> &'a mut W {
        self.variant(LCDGATE_A::UNGATE)
    }
    #[doc = "LCD BIAS MUX tristated at the pins."]
    #[inline(always)]
    pub fn gate(self) -> &'a mut W {
        self.variant(LCDGATE_A::GATE)
    }
}
#[doc = "Field `DMAMODE` reader - DMA Mode"]
pub type DMAMODE_R = crate::FieldReader<u8, DMAMODE_A>;
#[doc = "DMA Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAMODE_A {
    #[doc = "0: No DMA requests are generated"]
    DMADISABLE = 0,
    #[doc = "1: DMA request on frame counter event. This will also start a DMA transfer during EM23."]
    DMAFC = 1,
    #[doc = "2: DMA request on display counter event. This will also start a DMA transfer during EM23."]
    DMADISPLAY = 2,
}
impl From<DMAMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAMODE_A) -> Self {
        variant as _
    }
}
impl DMAMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAMODE_A> {
        match self.bits {
            0 => Some(DMAMODE_A::DMADISABLE),
            1 => Some(DMAMODE_A::DMAFC),
            2 => Some(DMAMODE_A::DMADISPLAY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMADISABLE`"]
    #[inline(always)]
    pub fn is_dmadisable(&self) -> bool {
        *self == DMAMODE_A::DMADISABLE
    }
    #[doc = "Checks if the value of the field is `DMAFC`"]
    #[inline(always)]
    pub fn is_dmafc(&self) -> bool {
        *self == DMAMODE_A::DMAFC
    }
    #[doc = "Checks if the value of the field is `DMADISPLAY`"]
    #[inline(always)]
    pub fn is_dmadisplay(&self) -> bool {
        *self == DMAMODE_A::DMADISPLAY
    }
}
#[doc = "Field `DMAMODE` writer - DMA Mode"]
pub type DMAMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BIASCTRL_SPEC, u8, DMAMODE_A, 2, O>;
impl<'a, const O: u8> DMAMODE_W<'a, O> {
    #[doc = "No DMA requests are generated"]
    #[inline(always)]
    pub fn dmadisable(self) -> &'a mut W {
        self.variant(DMAMODE_A::DMADISABLE)
    }
    #[doc = "DMA request on frame counter event. This will also start a DMA transfer during EM23."]
    #[inline(always)]
    pub fn dmafc(self) -> &'a mut W {
        self.variant(DMAMODE_A::DMAFC)
    }
    #[doc = "DMA request on display counter event. This will also start a DMA transfer during EM23."]
    #[inline(always)]
    pub fn dmadisplay(self) -> &'a mut W {
        self.variant(DMAMODE_A::DMADISPLAY)
    }
}
impl R {
    #[doc = "Bits 0:3 - Resistor strength"]
    #[inline(always)]
    pub fn resistor(&self) -> RESISTOR_R {
        RESISTOR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Buffer Drive Strength"]
    #[inline(always)]
    pub fn bufdrv(&self) -> BUFDRV_R {
        BUFDRV_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Buffer Bias Setting"]
    #[inline(always)]
    pub fn bufbias(&self) -> BUFBIAS_R {
        BUFBIAS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Mode Setting"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:20 - VLCD voltage level"]
    #[inline(always)]
    pub fn vlcd(&self) -> VLCD_R {
        VLCD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - VDDX select"]
    #[inline(always)]
    pub fn vddxsel(&self) -> VDDXSEL_R {
        VDDXSEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - LCD Gate"]
    #[inline(always)]
    pub fn lcdgate(&self) -> LCDGATE_R {
        LCDGATE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - DMA Mode"]
    #[inline(always)]
    pub fn dmamode(&self) -> DMAMODE_R {
        DMAMODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Resistor strength"]
    #[inline(always)]
    #[must_use]
    pub fn resistor(&mut self) -> RESISTOR_W<0> {
        RESISTOR_W::new(self)
    }
    #[doc = "Bits 4:6 - Buffer Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn bufdrv(&mut self) -> BUFDRV_W<4> {
        BUFDRV_W::new(self)
    }
    #[doc = "Bits 8:9 - Buffer Bias Setting"]
    #[inline(always)]
    #[must_use]
    pub fn bufbias(&mut self) -> BUFBIAS_W<8> {
        BUFBIAS_W::new(self)
    }
    #[doc = "Bit 12 - Mode Setting"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<12> {
        MODE_W::new(self)
    }
    #[doc = "Bits 16:20 - VLCD voltage level"]
    #[inline(always)]
    #[must_use]
    pub fn vlcd(&mut self) -> VLCD_W<16> {
        VLCD_W::new(self)
    }
    #[doc = "Bit 22 - VDDX select"]
    #[inline(always)]
    #[must_use]
    pub fn vddxsel(&mut self) -> VDDXSEL_W<22> {
        VDDXSEL_W::new(self)
    }
    #[doc = "Bit 26 - LCD Gate"]
    #[inline(always)]
    #[must_use]
    pub fn lcdgate(&mut self) -> LCDGATE_W<26> {
        LCDGATE_W::new(self)
    }
    #[doc = "Bits 30:31 - DMA Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmamode(&mut self) -> DMAMODE_W<30> {
        DMAMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biasctrl](index.html) module"]
pub struct BIASCTRL_SPEC;
impl crate::RegisterSpec for BIASCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [biasctrl::R](R) reader structure"]
impl crate::Readable for BIASCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [biasctrl::W](W) writer structure"]
impl crate::Writable for BIASCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIASCTRL to value 0x001f_0000"]
impl crate::Resettable for BIASCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x001f_0000;
}
