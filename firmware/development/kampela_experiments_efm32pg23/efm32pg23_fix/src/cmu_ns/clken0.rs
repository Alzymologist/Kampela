#[doc = "Register `CLKEN0` reader"]
pub struct R(crate::R<CLKEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKEN0` writer"]
pub struct W(crate::W<CLKEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKEN0_SPEC>;
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
impl From<crate::W<CLKEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDMA` reader - Enable Bus Clock"]
pub type LDMA_R = crate::BitReader<bool>;
#[doc = "Field `LDMA` writer - Enable Bus Clock"]
pub type LDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `LDMAXBAR` reader - Enable Bus Clock"]
pub type LDMAXBAR_R = crate::BitReader<bool>;
#[doc = "Field `LDMAXBAR` writer - Enable Bus Clock"]
pub type LDMAXBAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `GPCRC` reader - Enable Bus Clock"]
pub type GPCRC_R = crate::BitReader<bool>;
#[doc = "Field `GPCRC` writer - Enable Bus Clock"]
pub type GPCRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `TIMER0` reader - Enable Bus Clock"]
pub type TIMER0_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0` writer - Enable Bus Clock"]
pub type TIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `TIMER1` reader - Enable Bus Clock"]
pub type TIMER1_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1` writer - Enable Bus Clock"]
pub type TIMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `TIMER2` reader - Enable Bus Clock"]
pub type TIMER2_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2` writer - Enable Bus Clock"]
pub type TIMER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `TIMER3` reader - Enable Bus Clock"]
pub type TIMER3_R = crate::BitReader<bool>;
#[doc = "Field `TIMER3` writer - Enable Bus Clock"]
pub type TIMER3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `TIMER4` reader - Enable Bus Clock"]
pub type TIMER4_R = crate::BitReader<bool>;
#[doc = "Field `TIMER4` writer - Enable Bus Clock"]
pub type TIMER4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `USART0` reader - Enable Bus Clock"]
pub type USART0_R = crate::BitReader<bool>;
#[doc = "Field `USART0` writer - Enable Bus Clock"]
pub type USART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `IADC0` reader - Enable Bus Clock"]
pub type IADC0_R = crate::BitReader<bool>;
#[doc = "Field `IADC0` writer - Enable Bus Clock"]
pub type IADC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `AMUXCP0` reader - Enable Bus Clock"]
pub type AMUXCP0_R = crate::BitReader<bool>;
#[doc = "Field `AMUXCP0` writer - Enable Bus Clock"]
pub type AMUXCP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `LETIMER0` reader - Enable Bus Clock"]
pub type LETIMER0_R = crate::BitReader<bool>;
#[doc = "Field `LETIMER0` writer - Enable Bus Clock"]
pub type LETIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `WDOG0` reader - Enable Bus Clock"]
pub type WDOG0_R = crate::BitReader<bool>;
#[doc = "Field `WDOG0` writer - Enable Bus Clock"]
pub type WDOG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `I2C0` reader - Enable Bus Clock"]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `I2C0` writer - Enable Bus Clock"]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `I2C1` reader - Enable Bus Clock"]
pub type I2C1_R = crate::BitReader<bool>;
#[doc = "Field `I2C1` writer - Enable Bus Clock"]
pub type I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `SYSCFG` reader - Enable Bus Clock"]
pub type SYSCFG_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFG` writer - Enable Bus Clock"]
pub type SYSCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `DPLL0` reader - Enable Bus Clock"]
pub type DPLL0_R = crate::BitReader<bool>;
#[doc = "Field `DPLL0` writer - Enable Bus Clock"]
pub type DPLL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `HFRCO0` reader - Enable Bus Clock"]
pub type HFRCO0_R = crate::BitReader<bool>;
#[doc = "Field `HFRCO0` writer - Enable Bus Clock"]
pub type HFRCO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `HFRCOEM23` reader - Enable Bus Clock"]
pub type HFRCOEM23_R = crate::BitReader<bool>;
#[doc = "Field `HFRCOEM23` writer - Enable Bus Clock"]
pub type HFRCOEM23_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `HFXO0` reader - Enable Bus Clock"]
pub type HFXO0_R = crate::BitReader<bool>;
#[doc = "Field `HFXO0` writer - Enable Bus Clock"]
pub type HFXO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `FSRCO` reader - Enable Bus Clock"]
pub type FSRCO_R = crate::BitReader<bool>;
#[doc = "Field `FSRCO` writer - Enable Bus Clock"]
pub type FSRCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `LFRCO` reader - Enable Bus Clock"]
pub type LFRCO_R = crate::BitReader<bool>;
#[doc = "Field `LFRCO` writer - Enable Bus Clock"]
pub type LFRCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `LFXO` reader - Enable Bus Clock"]
pub type LFXO_R = crate::BitReader<bool>;
#[doc = "Field `LFXO` writer - Enable Bus Clock"]
pub type LFXO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `ULFRCO` reader - Enable Bus Clock"]
pub type ULFRCO_R = crate::BitReader<bool>;
#[doc = "Field `ULFRCO` writer - Enable Bus Clock"]
pub type ULFRCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `LESENSE` reader - Enable Bus Clock"]
pub type LESENSE_R = crate::BitReader<bool>;
#[doc = "Field `LESENSE` writer - Enable Bus Clock"]
pub type LESENSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `GPIO` reader - Enable Bus Clock"]
pub type GPIO_R = crate::BitReader<bool>;
#[doc = "Field `GPIO` writer - Enable Bus Clock"]
pub type GPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `PRS` reader - Enable Bus Clock"]
pub type PRS_R = crate::BitReader<bool>;
#[doc = "Field `PRS` writer - Enable Bus Clock"]
pub type PRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `BURAM` reader - Enable Bus Clock"]
pub type BURAM_R = crate::BitReader<bool>;
#[doc = "Field `BURAM` writer - Enable Bus Clock"]
pub type BURAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `BURTC` reader - Enable Bus Clock"]
pub type BURTC_R = crate::BitReader<bool>;
#[doc = "Field `BURTC` writer - Enable Bus Clock"]
pub type BURTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `SYSRTC0` reader - Enable Bus Clock"]
pub type SYSRTC0_R = crate::BitReader<bool>;
#[doc = "Field `SYSRTC0` writer - Enable Bus Clock"]
pub type SYSRTC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
#[doc = "Field `DCDC` reader - Enable Bus Clock"]
pub type DCDC_R = crate::BitReader<bool>;
#[doc = "Field `DCDC` writer - Enable Bus Clock"]
pub type DCDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Bus Clock"]
    #[inline(always)]
    pub fn ldma(&self) -> LDMA_R {
        LDMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Bus Clock"]
    #[inline(always)]
    pub fn ldmaxbar(&self) -> LDMAXBAR_R {
        LDMAXBAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Bus Clock"]
    #[inline(always)]
    pub fn gpcrc(&self) -> GPCRC_R {
        GPCRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer4(&self) -> TIMER4_R {
        TIMER4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Bus Clock"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Bus Clock"]
    #[inline(always)]
    pub fn iadc0(&self) -> IADC0_R {
        IADC0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Bus Clock"]
    #[inline(always)]
    pub fn amuxcp0(&self) -> AMUXCP0_R {
        AMUXCP0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Bus Clock"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Bus Clock"]
    #[inline(always)]
    pub fn wdog0(&self) -> WDOG0_R {
        WDOG0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Bus Clock"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Bus Clock"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Bus Clock"]
    #[inline(always)]
    pub fn syscfg(&self) -> SYSCFG_R {
        SYSCFG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Bus Clock"]
    #[inline(always)]
    pub fn dpll0(&self) -> DPLL0_R {
        DPLL0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Bus Clock"]
    #[inline(always)]
    pub fn hfrco0(&self) -> HFRCO0_R {
        HFRCO0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Bus Clock"]
    #[inline(always)]
    pub fn hfrcoem23(&self) -> HFRCOEM23_R {
        HFRCOEM23_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Bus Clock"]
    #[inline(always)]
    pub fn hfxo0(&self) -> HFXO0_R {
        HFXO0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Bus Clock"]
    #[inline(always)]
    pub fn fsrco(&self) -> FSRCO_R {
        FSRCO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Bus Clock"]
    #[inline(always)]
    pub fn lfrco(&self) -> LFRCO_R {
        LFRCO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Bus Clock"]
    #[inline(always)]
    pub fn lfxo(&self) -> LFXO_R {
        LFXO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Bus Clock"]
    #[inline(always)]
    pub fn ulfrco(&self) -> ULFRCO_R {
        ULFRCO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Bus Clock"]
    #[inline(always)]
    pub fn lesense(&self) -> LESENSE_R {
        LESENSE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Bus Clock"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Bus Clock"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Bus Clock"]
    #[inline(always)]
    pub fn buram(&self) -> BURAM_R {
        BURAM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Bus Clock"]
    #[inline(always)]
    pub fn burtc(&self) -> BURTC_R {
        BURTC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Bus Clock"]
    #[inline(always)]
    pub fn sysrtc0(&self) -> SYSRTC0_R {
        SYSRTC0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Bus Clock"]
    #[inline(always)]
    pub fn dcdc(&self) -> DCDC_R {
        DCDC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn ldma(&mut self) -> LDMA_W<0> {
        LDMA_W::new(self)
    }
    #[doc = "Bit 1 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn ldmaxbar(&mut self) -> LDMAXBAR_W<1> {
        LDMAXBAR_W::new(self)
    }
    #[doc = "Bit 3 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn gpcrc(&mut self) -> GPCRC_W<3> {
        GPCRC_W::new(self)
    }
    #[doc = "Bit 4 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn timer0(&mut self) -> TIMER0_W<4> {
        TIMER0_W::new(self)
    }
    #[doc = "Bit 5 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn timer1(&mut self) -> TIMER1_W<5> {
        TIMER1_W::new(self)
    }
    #[doc = "Bit 6 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn timer2(&mut self) -> TIMER2_W<6> {
        TIMER2_W::new(self)
    }
    #[doc = "Bit 7 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn timer3(&mut self) -> TIMER3_W<7> {
        TIMER3_W::new(self)
    }
    #[doc = "Bit 8 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn timer4(&mut self) -> TIMER4_W<8> {
        TIMER4_W::new(self)
    }
    #[doc = "Bit 9 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<9> {
        USART0_W::new(self)
    }
    #[doc = "Bit 10 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn iadc0(&mut self) -> IADC0_W<10> {
        IADC0_W::new(self)
    }
    #[doc = "Bit 11 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn amuxcp0(&mut self) -> AMUXCP0_W<11> {
        AMUXCP0_W::new(self)
    }
    #[doc = "Bit 12 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn letimer0(&mut self) -> LETIMER0_W<12> {
        LETIMER0_W::new(self)
    }
    #[doc = "Bit 13 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn wdog0(&mut self) -> WDOG0_W<13> {
        WDOG0_W::new(self)
    }
    #[doc = "Bit 14 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<14> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 15 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<15> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 16 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn syscfg(&mut self) -> SYSCFG_W<16> {
        SYSCFG_W::new(self)
    }
    #[doc = "Bit 17 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0(&mut self) -> DPLL0_W<17> {
        DPLL0_W::new(self)
    }
    #[doc = "Bit 18 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn hfrco0(&mut self) -> HFRCO0_W<18> {
        HFRCO0_W::new(self)
    }
    #[doc = "Bit 19 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcoem23(&mut self) -> HFRCOEM23_W<19> {
        HFRCOEM23_W::new(self)
    }
    #[doc = "Bit 20 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn hfxo0(&mut self) -> HFXO0_W<20> {
        HFXO0_W::new(self)
    }
    #[doc = "Bit 21 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn fsrco(&mut self) -> FSRCO_W<21> {
        FSRCO_W::new(self)
    }
    #[doc = "Bit 22 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn lfrco(&mut self) -> LFRCO_W<22> {
        LFRCO_W::new(self)
    }
    #[doc = "Bit 23 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn lfxo(&mut self) -> LFXO_W<23> {
        LFXO_W::new(self)
    }
    #[doc = "Bit 24 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn ulfrco(&mut self) -> ULFRCO_W<24> {
        ULFRCO_W::new(self)
    }
    #[doc = "Bit 25 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn lesense(&mut self) -> LESENSE_W<25> {
        LESENSE_W::new(self)
    }
    #[doc = "Bit 26 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<26> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 27 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<27> {
        PRS_W::new(self)
    }
    #[doc = "Bit 28 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn buram(&mut self) -> BURAM_W<28> {
        BURAM_W::new(self)
    }
    #[doc = "Bit 29 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn burtc(&mut self) -> BURTC_W<29> {
        BURTC_W::new(self)
    }
    #[doc = "Bit 30 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn sysrtc0(&mut self) -> SYSRTC0_W<30> {
        SYSRTC0_W::new(self)
    }
    #[doc = "Bit 31 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc(&mut self) -> DCDC_W<31> {
        DCDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clken0](index.html) module"]
pub struct CLKEN0_SPEC;
impl crate::RegisterSpec for CLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clken0::R](R) reader structure"]
impl crate::Readable for CLKEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clken0::W](W) writer structure"]
impl crate::Writable for CLKEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKEN0 to value 0"]
impl crate::Resettable for CLKEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
