#[doc = "Register `TRIGGER` reader"]
pub struct R(crate::R<TRIGGER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGGER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGGER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGGER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIGGER` writer"]
pub struct W(crate::W<TRIGGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIGGER_SPEC>;
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
impl From<crate::W<TRIGGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIGGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCANTRIGSEL` reader - Scan Trigger Select"]
pub type SCANTRIGSEL_R = crate::FieldReader<u8, SCANTRIGSEL_A>;
#[doc = "Scan Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCANTRIGSEL_A {
    #[doc = "0: Immediate triggering. The scan queue will be disabled once all conversions in the scan table are complete, unless TRIGGERACTION is set to continuous."]
    IMMEDIATE = 0,
    #[doc = "1: Triggers when the local timer count reaches zero."]
    TIMER = 1,
    #[doc = "2: Triggers on PRS0 from a timer module that is using the same clock group as the ADC and has been programmed to use the same clock source as the ADC. The prescale may be different between the ADC and the timer module."]
    PRSCLKGRP = 2,
    #[doc = "3: Triggers on asynchronous PRS0 positive edge. Requires PRS0 to go low for 3 ADC_CLKs before another positive edge can be detected. Generates an additional delay of 1 to 2 ADC_SRC_CLK cycles for synchronization."]
    PRSPOS = 3,
    #[doc = "4: Triggers on asynchronous PRS0 negative edge. Requires PRS0 to go high for 3 ADC_CLKs before another negative edge can be detected. Generates an additional delay of 1 to 2 ADC_SRC_CLK cycles for synchronization. PRSNEG should only be used when the trigger source is from a module that remains powered during EM23. For modules (ie: TIMER) that power down during EM23, PRSPOS should be used for an asynchronous trigger, and PRSCLKGRP should be used for a synchronous trigger."]
    PRSNEG = 4,
    #[doc = "5: Triggers on LESENSE convert request. When using the LESENSE for the SCAN Table, only one entry is converted per LESENSE convert request."]
    LESENSE = 5,
}
impl From<SCANTRIGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCANTRIGSEL_A) -> Self {
        variant as _
    }
}
impl SCANTRIGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCANTRIGSEL_A> {
        match self.bits {
            0 => Some(SCANTRIGSEL_A::IMMEDIATE),
            1 => Some(SCANTRIGSEL_A::TIMER),
            2 => Some(SCANTRIGSEL_A::PRSCLKGRP),
            3 => Some(SCANTRIGSEL_A::PRSPOS),
            4 => Some(SCANTRIGSEL_A::PRSNEG),
            5 => Some(SCANTRIGSEL_A::LESENSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == SCANTRIGSEL_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == SCANTRIGSEL_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PRSCLKGRP`"]
    #[inline(always)]
    pub fn is_prsclkgrp(&self) -> bool {
        *self == SCANTRIGSEL_A::PRSCLKGRP
    }
    #[doc = "Checks if the value of the field is `PRSPOS`"]
    #[inline(always)]
    pub fn is_prspos(&self) -> bool {
        *self == SCANTRIGSEL_A::PRSPOS
    }
    #[doc = "Checks if the value of the field is `PRSNEG`"]
    #[inline(always)]
    pub fn is_prsneg(&self) -> bool {
        *self == SCANTRIGSEL_A::PRSNEG
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == SCANTRIGSEL_A::LESENSE
    }
}
#[doc = "Field `SCANTRIGSEL` writer - Scan Trigger Select"]
pub type SCANTRIGSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIGGER_SPEC, u8, SCANTRIGSEL_A, 3, O>;
impl<'a, const O: u8> SCANTRIGSEL_W<'a, O> {
    #[doc = "Immediate triggering. The scan queue will be disabled once all conversions in the scan table are complete, unless TRIGGERACTION is set to continuous."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(SCANTRIGSEL_A::IMMEDIATE)
    }
    #[doc = "Triggers when the local timer count reaches zero."]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(SCANTRIGSEL_A::TIMER)
    }
    #[doc = "Triggers on PRS0 from a timer module that is using the same clock group as the ADC and has been programmed to use the same clock source as the ADC. The prescale may be different between the ADC and the timer module."]
    #[inline(always)]
    pub fn prsclkgrp(self) -> &'a mut W {
        self.variant(SCANTRIGSEL_A::PRSCLKGRP)
    }
    #[doc = "Triggers on asynchronous PRS0 positive edge. Requires PRS0 to go low for 3 ADC_CLKs before another positive edge can be detected. Generates an additional delay of 1 to 2 ADC_SRC_CLK cycles for synchronization."]
    #[inline(always)]
    pub fn prspos(self) -> &'a mut W {
        self.variant(SCANTRIGSEL_A::PRSPOS)
    }
    #[doc = "Triggers on asynchronous PRS0 negative edge. Requires PRS0 to go high for 3 ADC_CLKs before another negative edge can be detected. Generates an additional delay of 1 to 2 ADC_SRC_CLK cycles for synchronization. PRSNEG should only be used when the trigger source is from a module that remains powered during EM23. For modules (ie: TIMER) that power down during EM23, PRSPOS should be used for an asynchronous trigger, and PRSCLKGRP should be used for a synchronous trigger."]
    #[inline(always)]
    pub fn prsneg(self) -> &'a mut W {
        self.variant(SCANTRIGSEL_A::PRSNEG)
    }
    #[doc = "Triggers on LESENSE convert request. When using the LESENSE for the SCAN Table, only one entry is converted per LESENSE convert request."]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut W {
        self.variant(SCANTRIGSEL_A::LESENSE)
    }
}
#[doc = "Field `SCANTRIGACTION` reader - Scan Trigger Action"]
pub type SCANTRIGACTION_R = crate::BitReader<SCANTRIGACTION_A>;
#[doc = "Scan Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCANTRIGACTION_A {
    #[doc = "0: For TRIGSEL=IMMEDIATE, goes through the scan table once and disables queue. For TRIGSEL = TIMER, PRSCLKGRP, PRSPOS, PRSNEG, goes through the scan table once per trigger."]
    ONCE = 0,
    #[doc = "1: Goes through the scan table, converts each entry with a mask bit set, and puts it back into the scan queue to repeat again continuously. The queues are first come first serve. If both queues are triggered, the single queue will get to convert after each scan table completes. The scan queue will get to convert after each single conversion completes."]
    CONTINUOUS = 1,
}
impl From<SCANTRIGACTION_A> for bool {
    #[inline(always)]
    fn from(variant: SCANTRIGACTION_A) -> Self {
        variant as u8 != 0
    }
}
impl SCANTRIGACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCANTRIGACTION_A {
        match self.bits {
            false => SCANTRIGACTION_A::ONCE,
            true => SCANTRIGACTION_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `ONCE`"]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == SCANTRIGACTION_A::ONCE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == SCANTRIGACTION_A::CONTINUOUS
    }
}
#[doc = "Field `SCANTRIGACTION` writer - Scan Trigger Action"]
pub type SCANTRIGACTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRIGGER_SPEC, SCANTRIGACTION_A, O>;
impl<'a, const O: u8> SCANTRIGACTION_W<'a, O> {
    #[doc = "For TRIGSEL=IMMEDIATE, goes through the scan table once and disables queue. For TRIGSEL = TIMER, PRSCLKGRP, PRSPOS, PRSNEG, goes through the scan table once per trigger."]
    #[inline(always)]
    pub fn once(self) -> &'a mut W {
        self.variant(SCANTRIGACTION_A::ONCE)
    }
    #[doc = "Goes through the scan table, converts each entry with a mask bit set, and puts it back into the scan queue to repeat again continuously. The queues are first come first serve. If both queues are triggered, the single queue will get to convert after each scan table completes. The scan queue will get to convert after each single conversion completes."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SCANTRIGACTION_A::CONTINUOUS)
    }
}
#[doc = "Field `SINGLETRIGSEL` reader - Single Trigger Select"]
pub type SINGLETRIGSEL_R = crate::FieldReader<u8, SINGLETRIGSEL_A>;
#[doc = "Single Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SINGLETRIGSEL_A {
    #[doc = "0: Immediate triggering. The single queue will be disabled once the conversion is complete, unless TRIGGERACTION is set to continuous."]
    IMMEDIATE = 0,
    #[doc = "1: Triggers when the local timer count reaches zero."]
    TIMER = 1,
    #[doc = "2: Triggers on PRS1 from a timer module that is using the same clock group as the ADC and has been programmed to use the same clock source as the ADC. The prescale may be different between the ADC and the timer module."]
    PRSCLKGRP = 2,
    #[doc = "3: Triggers on asynchronous PRS1 positive edge. Requires PRS1 to go low for 3 ADC_CLKs before another positive edge can be detected. Generates an additional delay of 1 to 2 ADC_SRC_CLK cycles for synchronization."]
    PRSPOS = 3,
    #[doc = "4: Triggers on asynchronous PRS1 negative edge. Requires PRS1 to go high for 3 ADC_CLKs before another negative edge can be detected. Generates an additional delay of 1 to 2 ADC_SRC_CLK cycles for synchronization. PRSNEG should only be used when the trigger source is from a module that remains powered during EM23. For modules (ie: TIMER) that power down during EM23, PRSPOS should be used for an asynchronous trigger, and PRSCLKGRP should be used for a synchronous trigger."]
    PRSNEG = 4,
}
impl From<SINGLETRIGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SINGLETRIGSEL_A) -> Self {
        variant as _
    }
}
impl SINGLETRIGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SINGLETRIGSEL_A> {
        match self.bits {
            0 => Some(SINGLETRIGSEL_A::IMMEDIATE),
            1 => Some(SINGLETRIGSEL_A::TIMER),
            2 => Some(SINGLETRIGSEL_A::PRSCLKGRP),
            3 => Some(SINGLETRIGSEL_A::PRSPOS),
            4 => Some(SINGLETRIGSEL_A::PRSNEG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == SINGLETRIGSEL_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == SINGLETRIGSEL_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PRSCLKGRP`"]
    #[inline(always)]
    pub fn is_prsclkgrp(&self) -> bool {
        *self == SINGLETRIGSEL_A::PRSCLKGRP
    }
    #[doc = "Checks if the value of the field is `PRSPOS`"]
    #[inline(always)]
    pub fn is_prspos(&self) -> bool {
        *self == SINGLETRIGSEL_A::PRSPOS
    }
    #[doc = "Checks if the value of the field is `PRSNEG`"]
    #[inline(always)]
    pub fn is_prsneg(&self) -> bool {
        *self == SINGLETRIGSEL_A::PRSNEG
    }
}
#[doc = "Field `SINGLETRIGSEL` writer - Single Trigger Select"]
pub type SINGLETRIGSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIGGER_SPEC, u8, SINGLETRIGSEL_A, 3, O>;
impl<'a, const O: u8> SINGLETRIGSEL_W<'a, O> {
    #[doc = "Immediate triggering. The single queue will be disabled once the conversion is complete, unless TRIGGERACTION is set to continuous."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(SINGLETRIGSEL_A::IMMEDIATE)
    }
    #[doc = "Triggers when the local timer count reaches zero."]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(SINGLETRIGSEL_A::TIMER)
    }
    #[doc = "Triggers on PRS1 from a timer module that is using the same clock group as the ADC and has been programmed to use the same clock source as the ADC. The prescale may be different between the ADC and the timer module."]
    #[inline(always)]
    pub fn prsclkgrp(self) -> &'a mut W {
        self.variant(SINGLETRIGSEL_A::PRSCLKGRP)
    }
    #[doc = "Triggers on asynchronous PRS1 positive edge. Requires PRS1 to go low for 3 ADC_CLKs before another positive edge can be detected. Generates an additional delay of 1 to 2 ADC_SRC_CLK cycles for synchronization."]
    #[inline(always)]
    pub fn prspos(self) -> &'a mut W {
        self.variant(SINGLETRIGSEL_A::PRSPOS)
    }
    #[doc = "Triggers on asynchronous PRS1 negative edge. Requires PRS1 to go high for 3 ADC_CLKs before another negative edge can be detected. Generates an additional delay of 1 to 2 ADC_SRC_CLK cycles for synchronization. PRSNEG should only be used when the trigger source is from a module that remains powered during EM23. For modules (ie: TIMER) that power down during EM23, PRSPOS should be used for an asynchronous trigger, and PRSCLKGRP should be used for a synchronous trigger."]
    #[inline(always)]
    pub fn prsneg(self) -> &'a mut W {
        self.variant(SINGLETRIGSEL_A::PRSNEG)
    }
}
#[doc = "Field `SINGLETRIGACTION` reader - Single Trigger Action"]
pub type SINGLETRIGACTION_R = crate::BitReader<SINGLETRIGACTION_A>;
#[doc = "Single Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINGLETRIGACTION_A {
    #[doc = "0: For TRIGSEL=IMMEDIATE, converts the single queue once and disables queue. For TRIGSEL = TIMER, PRSCLKGRP, PRSPOS, PRSNEG, converts the single queue once per trigger."]
    ONCE = 0,
    #[doc = "1: Converts the single queue, then checks for a pending scan queue before converting the single queue again continuously. The queues are first come first serve. If both queues are continuous, the IADC alternates between them."]
    CONTINUOUS = 1,
}
impl From<SINGLETRIGACTION_A> for bool {
    #[inline(always)]
    fn from(variant: SINGLETRIGACTION_A) -> Self {
        variant as u8 != 0
    }
}
impl SINGLETRIGACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINGLETRIGACTION_A {
        match self.bits {
            false => SINGLETRIGACTION_A::ONCE,
            true => SINGLETRIGACTION_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `ONCE`"]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == SINGLETRIGACTION_A::ONCE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == SINGLETRIGACTION_A::CONTINUOUS
    }
}
#[doc = "Field `SINGLETRIGACTION` writer - Single Trigger Action"]
pub type SINGLETRIGACTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRIGGER_SPEC, SINGLETRIGACTION_A, O>;
impl<'a, const O: u8> SINGLETRIGACTION_W<'a, O> {
    #[doc = "For TRIGSEL=IMMEDIATE, converts the single queue once and disables queue. For TRIGSEL = TIMER, PRSCLKGRP, PRSPOS, PRSNEG, converts the single queue once per trigger."]
    #[inline(always)]
    pub fn once(self) -> &'a mut W {
        self.variant(SINGLETRIGACTION_A::ONCE)
    }
    #[doc = "Converts the single queue, then checks for a pending scan queue before converting the single queue again continuously. The queues are first come first serve. If both queues are continuous, the IADC alternates between them."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SINGLETRIGACTION_A::CONTINUOUS)
    }
}
#[doc = "Field `SINGLETAILGATE` reader - Single Tailgate Enable"]
pub type SINGLETAILGATE_R = crate::BitReader<SINGLETAILGATE_A>;
#[doc = "Single Tailgate Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINGLETAILGATE_A {
    #[doc = "0: The single queue is ready to start warming up and converting once the trigger had been detected."]
    TAILGATEOFF = 0,
    #[doc = "1: After the single queue's trigger is detected, it must wait until the end of a scan operation before the Single queue can be converted."]
    TAILGATEON = 1,
}
impl From<SINGLETAILGATE_A> for bool {
    #[inline(always)]
    fn from(variant: SINGLETAILGATE_A) -> Self {
        variant as u8 != 0
    }
}
impl SINGLETAILGATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINGLETAILGATE_A {
        match self.bits {
            false => SINGLETAILGATE_A::TAILGATEOFF,
            true => SINGLETAILGATE_A::TAILGATEON,
        }
    }
    #[doc = "Checks if the value of the field is `TAILGATEOFF`"]
    #[inline(always)]
    pub fn is_tailgateoff(&self) -> bool {
        *self == SINGLETAILGATE_A::TAILGATEOFF
    }
    #[doc = "Checks if the value of the field is `TAILGATEON`"]
    #[inline(always)]
    pub fn is_tailgateon(&self) -> bool {
        *self == SINGLETAILGATE_A::TAILGATEON
    }
}
#[doc = "Field `SINGLETAILGATE` writer - Single Tailgate Enable"]
pub type SINGLETAILGATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRIGGER_SPEC, SINGLETAILGATE_A, O>;
impl<'a, const O: u8> SINGLETAILGATE_W<'a, O> {
    #[doc = "The single queue is ready to start warming up and converting once the trigger had been detected."]
    #[inline(always)]
    pub fn tailgateoff(self) -> &'a mut W {
        self.variant(SINGLETAILGATE_A::TAILGATEOFF)
    }
    #[doc = "After the single queue's trigger is detected, it must wait until the end of a scan operation before the Single queue can be converted."]
    #[inline(always)]
    pub fn tailgateon(self) -> &'a mut W {
        self.variant(SINGLETAILGATE_A::TAILGATEON)
    }
}
impl R {
    #[doc = "Bits 0:2 - Scan Trigger Select"]
    #[inline(always)]
    pub fn scantrigsel(&self) -> SCANTRIGSEL_R {
        SCANTRIGSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Scan Trigger Action"]
    #[inline(always)]
    pub fn scantrigaction(&self) -> SCANTRIGACTION_R {
        SCANTRIGACTION_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Single Trigger Select"]
    #[inline(always)]
    pub fn singletrigsel(&self) -> SINGLETRIGSEL_R {
        SINGLETRIGSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Single Trigger Action"]
    #[inline(always)]
    pub fn singletrigaction(&self) -> SINGLETRIGACTION_R {
        SINGLETRIGACTION_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Tailgate Enable"]
    #[inline(always)]
    pub fn singletailgate(&self) -> SINGLETAILGATE_R {
        SINGLETAILGATE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Scan Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn scantrigsel(&mut self) -> SCANTRIGSEL_W<0> {
        SCANTRIGSEL_W::new(self)
    }
    #[doc = "Bit 4 - Scan Trigger Action"]
    #[inline(always)]
    #[must_use]
    pub fn scantrigaction(&mut self) -> SCANTRIGACTION_W<4> {
        SCANTRIGACTION_W::new(self)
    }
    #[doc = "Bits 8:10 - Single Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn singletrigsel(&mut self) -> SINGLETRIGSEL_W<8> {
        SINGLETRIGSEL_W::new(self)
    }
    #[doc = "Bit 12 - Single Trigger Action"]
    #[inline(always)]
    #[must_use]
    pub fn singletrigaction(&mut self) -> SINGLETRIGACTION_W<12> {
        SINGLETRIGACTION_W::new(self)
    }
    #[doc = "Bit 16 - Single Tailgate Enable"]
    #[inline(always)]
    #[must_use]
    pub fn singletailgate(&mut self) -> SINGLETAILGATE_W<16> {
        SINGLETAILGATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigger](index.html) module"]
pub struct TRIGGER_SPEC;
impl crate::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigger::R](R) reader structure"]
impl crate::Readable for TRIGGER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trigger::W](W) writer structure"]
impl crate::Writable for TRIGGER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
