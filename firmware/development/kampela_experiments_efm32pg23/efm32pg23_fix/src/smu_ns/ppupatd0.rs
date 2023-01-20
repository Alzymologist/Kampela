#[doc = "Register `PPUPATD0` reader"]
pub struct R(crate::R<PPUPATD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPUPATD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPUPATD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPUPATD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPUPATD0` writer"]
pub struct W(crate::W<PPUPATD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPUPATD0_SPEC>;
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
impl From<crate::W<PPUPATD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPUPATD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMU` reader - EMU Privileged Access"]
pub type EMU_R = crate::BitReader<bool>;
#[doc = "Field `EMU` writer - EMU Privileged Access"]
pub type EMU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `CMU` reader - CMU Privileged Access"]
pub type CMU_R = crate::BitReader<bool>;
#[doc = "Field `CMU` writer - CMU Privileged Access"]
pub type CMU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `HFRCO0` reader - HFRCO0 Privileged Access"]
pub type HFRCO0_R = crate::BitReader<bool>;
#[doc = "Field `HFRCO0` writer - HFRCO0 Privileged Access"]
pub type HFRCO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `FSRCO` reader - FSRCO Privileged Access"]
pub type FSRCO_R = crate::BitReader<bool>;
#[doc = "Field `FSRCO` writer - FSRCO Privileged Access"]
pub type FSRCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `DPLL0` reader - DPLL0 Privileged Access"]
pub type DPLL0_R = crate::BitReader<bool>;
#[doc = "Field `DPLL0` writer - DPLL0 Privileged Access"]
pub type DPLL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `LFXO` reader - LFXO Privileged Access"]
pub type LFXO_R = crate::BitReader<bool>;
#[doc = "Field `LFXO` writer - LFXO Privileged Access"]
pub type LFXO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `LFRCO` reader - LFRCO Privileged Access"]
pub type LFRCO_R = crate::BitReader<bool>;
#[doc = "Field `LFRCO` writer - LFRCO Privileged Access"]
pub type LFRCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `ULFRCO` reader - ULFRCO Privileged Access"]
pub type ULFRCO_R = crate::BitReader<bool>;
#[doc = "Field `ULFRCO` writer - ULFRCO Privileged Access"]
pub type ULFRCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `MSC` reader - MSC Privileged Access"]
pub type MSC_R = crate::BitReader<bool>;
#[doc = "Field `MSC` writer - MSC Privileged Access"]
pub type MSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `ICACHE0` reader - ICACHE0 Privileged Access"]
pub type ICACHE0_R = crate::BitReader<bool>;
#[doc = "Field `ICACHE0` writer - ICACHE0 Privileged Access"]
pub type ICACHE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `PRS` reader - PRS Privileged Access"]
pub type PRS_R = crate::BitReader<bool>;
#[doc = "Field `PRS` writer - PRS Privileged Access"]
pub type PRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `GPIO` reader - GPIO Privileged Access"]
pub type GPIO_R = crate::BitReader<bool>;
#[doc = "Field `GPIO` writer - GPIO Privileged Access"]
pub type GPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `LDMA` reader - LDMA Privileged Access"]
pub type LDMA_R = crate::BitReader<bool>;
#[doc = "Field `LDMA` writer - LDMA Privileged Access"]
pub type LDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `LDMAXBAR` reader - LDMAXBAR Privileged Access"]
pub type LDMAXBAR_R = crate::BitReader<bool>;
#[doc = "Field `LDMAXBAR` writer - LDMAXBAR Privileged Access"]
pub type LDMAXBAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `TIMER0` reader - TIMER0 Privileged Access"]
pub type TIMER0_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0` writer - TIMER0 Privileged Access"]
pub type TIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `TIMER1` reader - TIMER1 Privileged Access"]
pub type TIMER1_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1` writer - TIMER1 Privileged Access"]
pub type TIMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `TIMER2` reader - TIMER2 Privileged Access"]
pub type TIMER2_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2` writer - TIMER2 Privileged Access"]
pub type TIMER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `TIMER3` reader - TIMER3 Privileged Access"]
pub type TIMER3_R = crate::BitReader<bool>;
#[doc = "Field `TIMER3` writer - TIMER3 Privileged Access"]
pub type TIMER3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `TIMER4` reader - TIMER4 Privileged Access"]
pub type TIMER4_R = crate::BitReader<bool>;
#[doc = "Field `TIMER4` writer - TIMER4 Privileged Access"]
pub type TIMER4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `USART0` reader - USART0 Privileged Access"]
pub type USART0_R = crate::BitReader<bool>;
#[doc = "Field `USART0` writer - USART0 Privileged Access"]
pub type USART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `BURTC` reader - BURTC Privileged Access"]
pub type BURTC_R = crate::BitReader<bool>;
#[doc = "Field `BURTC` writer - BURTC Privileged Access"]
pub type BURTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `I2C1` reader - I2C1 Privileged Access"]
pub type I2C1_R = crate::BitReader<bool>;
#[doc = "Field `I2C1` writer - I2C1 Privileged Access"]
pub type I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `CHIPTESTCTRL` reader - CHIPTESTCTRL Privileged Access"]
pub type CHIPTESTCTRL_R = crate::BitReader<bool>;
#[doc = "Field `CHIPTESTCTRL` writer - CHIPTESTCTRL Privileged Access"]
pub type CHIPTESTCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `SYSCFGCFGNS` reader - SYSCFGCFGNS Privileged Access"]
pub type SYSCFGCFGNS_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGCFGNS` writer - SYSCFGCFGNS Privileged Access"]
pub type SYSCFGCFGNS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `SYSCFG` reader - SYSCFG Privileged Access"]
pub type SYSCFG_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFG` writer - SYSCFG Privileged Access"]
pub type SYSCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `BURAM` reader - BURAM Privileged Access"]
pub type BURAM_R = crate::BitReader<bool>;
#[doc = "Field `BURAM` writer - BURAM Privileged Access"]
pub type BURAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `GPCRC` reader - GPCRC Privileged Access"]
pub type GPCRC_R = crate::BitReader<bool>;
#[doc = "Field `GPCRC` writer - GPCRC Privileged Access"]
pub type GPCRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `DCDC` reader - DCDC Privileged Access"]
pub type DCDC_R = crate::BitReader<bool>;
#[doc = "Field `DCDC` writer - DCDC Privileged Access"]
pub type DCDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `HOSTMAILBOX` reader - HOSTMAILBOX Privileged Access"]
pub type HOSTMAILBOX_R = crate::BitReader<bool>;
#[doc = "Field `HOSTMAILBOX` writer - HOSTMAILBOX Privileged Access"]
pub type HOSTMAILBOX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `EUSART1` reader - EUSART1 Privileged Access"]
pub type EUSART1_R = crate::BitReader<bool>;
#[doc = "Field `EUSART1` writer - EUSART1 Privileged Access"]
pub type EUSART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
#[doc = "Field `EUSART2` reader - EUSART2 Privileged Access"]
pub type EUSART2_R = crate::BitReader<bool>;
#[doc = "Field `EUSART2` writer - EUSART2 Privileged Access"]
pub type EUSART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - EMU Privileged Access"]
    #[inline(always)]
    pub fn emu(&self) -> EMU_R {
        EMU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMU Privileged Access"]
    #[inline(always)]
    pub fn cmu(&self) -> CMU_R {
        CMU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HFRCO0 Privileged Access"]
    #[inline(always)]
    pub fn hfrco0(&self) -> HFRCO0_R {
        HFRCO0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FSRCO Privileged Access"]
    #[inline(always)]
    pub fn fsrco(&self) -> FSRCO_R {
        FSRCO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DPLL0 Privileged Access"]
    #[inline(always)]
    pub fn dpll0(&self) -> DPLL0_R {
        DPLL0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LFXO Privileged Access"]
    #[inline(always)]
    pub fn lfxo(&self) -> LFXO_R {
        LFXO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LFRCO Privileged Access"]
    #[inline(always)]
    pub fn lfrco(&self) -> LFRCO_R {
        LFRCO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ULFRCO Privileged Access"]
    #[inline(always)]
    pub fn ulfrco(&self) -> ULFRCO_R {
        ULFRCO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MSC Privileged Access"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ICACHE0 Privileged Access"]
    #[inline(always)]
    pub fn icache0(&self) -> ICACHE0_R {
        ICACHE0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PRS Privileged Access"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO Privileged Access"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LDMA Privileged Access"]
    #[inline(always)]
    pub fn ldma(&self) -> LDMA_R {
        LDMA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LDMAXBAR Privileged Access"]
    #[inline(always)]
    pub fn ldmaxbar(&self) -> LDMAXBAR_R {
        LDMAXBAR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIMER0 Privileged Access"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER1 Privileged Access"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER2 Privileged Access"]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER3 Privileged Access"]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TIMER4 Privileged Access"]
    #[inline(always)]
    pub fn timer4(&self) -> TIMER4_R {
        TIMER4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART0 Privileged Access"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BURTC Privileged Access"]
    #[inline(always)]
    pub fn burtc(&self) -> BURTC_R {
        BURTC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 Privileged Access"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CHIPTESTCTRL Privileged Access"]
    #[inline(always)]
    pub fn chiptestctrl(&self) -> CHIPTESTCTRL_R {
        CHIPTESTCTRL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SYSCFGCFGNS Privileged Access"]
    #[inline(always)]
    pub fn syscfgcfgns(&self) -> SYSCFGCFGNS_R {
        SYSCFGCFGNS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SYSCFG Privileged Access"]
    #[inline(always)]
    pub fn syscfg(&self) -> SYSCFG_R {
        SYSCFG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BURAM Privileged Access"]
    #[inline(always)]
    pub fn buram(&self) -> BURAM_R {
        BURAM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPCRC Privileged Access"]
    #[inline(always)]
    pub fn gpcrc(&self) -> GPCRC_R {
        GPCRC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DCDC Privileged Access"]
    #[inline(always)]
    pub fn dcdc(&self) -> DCDC_R {
        DCDC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HOSTMAILBOX Privileged Access"]
    #[inline(always)]
    pub fn hostmailbox(&self) -> HOSTMAILBOX_R {
        HOSTMAILBOX_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EUSART1 Privileged Access"]
    #[inline(always)]
    pub fn eusart1(&self) -> EUSART1_R {
        EUSART1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - EUSART2 Privileged Access"]
    #[inline(always)]
    pub fn eusart2(&self) -> EUSART2_R {
        EUSART2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - EMU Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn emu(&mut self) -> EMU_W<1> {
        EMU_W::new(self)
    }
    #[doc = "Bit 2 - CMU Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn cmu(&mut self) -> CMU_W<2> {
        CMU_W::new(self)
    }
    #[doc = "Bit 3 - HFRCO0 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn hfrco0(&mut self) -> HFRCO0_W<3> {
        HFRCO0_W::new(self)
    }
    #[doc = "Bit 4 - FSRCO Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn fsrco(&mut self) -> FSRCO_W<4> {
        FSRCO_W::new(self)
    }
    #[doc = "Bit 5 - DPLL0 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0(&mut self) -> DPLL0_W<5> {
        DPLL0_W::new(self)
    }
    #[doc = "Bit 6 - LFXO Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn lfxo(&mut self) -> LFXO_W<6> {
        LFXO_W::new(self)
    }
    #[doc = "Bit 7 - LFRCO Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn lfrco(&mut self) -> LFRCO_W<7> {
        LFRCO_W::new(self)
    }
    #[doc = "Bit 8 - ULFRCO Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn ulfrco(&mut self) -> ULFRCO_W<8> {
        ULFRCO_W::new(self)
    }
    #[doc = "Bit 9 - MSC Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn msc(&mut self) -> MSC_W<9> {
        MSC_W::new(self)
    }
    #[doc = "Bit 10 - ICACHE0 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn icache0(&mut self) -> ICACHE0_W<10> {
        ICACHE0_W::new(self)
    }
    #[doc = "Bit 11 - PRS Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<11> {
        PRS_W::new(self)
    }
    #[doc = "Bit 12 - GPIO Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<12> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 13 - LDMA Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn ldma(&mut self) -> LDMA_W<13> {
        LDMA_W::new(self)
    }
    #[doc = "Bit 14 - LDMAXBAR Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn ldmaxbar(&mut self) -> LDMAXBAR_W<14> {
        LDMAXBAR_W::new(self)
    }
    #[doc = "Bit 15 - TIMER0 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn timer0(&mut self) -> TIMER0_W<15> {
        TIMER0_W::new(self)
    }
    #[doc = "Bit 16 - TIMER1 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn timer1(&mut self) -> TIMER1_W<16> {
        TIMER1_W::new(self)
    }
    #[doc = "Bit 17 - TIMER2 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn timer2(&mut self) -> TIMER2_W<17> {
        TIMER2_W::new(self)
    }
    #[doc = "Bit 18 - TIMER3 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn timer3(&mut self) -> TIMER3_W<18> {
        TIMER3_W::new(self)
    }
    #[doc = "Bit 19 - TIMER4 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn timer4(&mut self) -> TIMER4_W<19> {
        TIMER4_W::new(self)
    }
    #[doc = "Bit 20 - USART0 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<20> {
        USART0_W::new(self)
    }
    #[doc = "Bit 21 - BURTC Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn burtc(&mut self) -> BURTC_W<21> {
        BURTC_W::new(self)
    }
    #[doc = "Bit 22 - I2C1 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<22> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 23 - CHIPTESTCTRL Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn chiptestctrl(&mut self) -> CHIPTESTCTRL_W<23> {
        CHIPTESTCTRL_W::new(self)
    }
    #[doc = "Bit 24 - SYSCFGCFGNS Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgcfgns(&mut self) -> SYSCFGCFGNS_W<24> {
        SYSCFGCFGNS_W::new(self)
    }
    #[doc = "Bit 25 - SYSCFG Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn syscfg(&mut self) -> SYSCFG_W<25> {
        SYSCFG_W::new(self)
    }
    #[doc = "Bit 26 - BURAM Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn buram(&mut self) -> BURAM_W<26> {
        BURAM_W::new(self)
    }
    #[doc = "Bit 27 - GPCRC Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn gpcrc(&mut self) -> GPCRC_W<27> {
        GPCRC_W::new(self)
    }
    #[doc = "Bit 28 - DCDC Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc(&mut self) -> DCDC_W<28> {
        DCDC_W::new(self)
    }
    #[doc = "Bit 29 - HOSTMAILBOX Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn hostmailbox(&mut self) -> HOSTMAILBOX_W<29> {
        HOSTMAILBOX_W::new(self)
    }
    #[doc = "Bit 30 - EUSART1 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn eusart1(&mut self) -> EUSART1_W<30> {
        EUSART1_W::new(self)
    }
    #[doc = "Bit 31 - EUSART2 Privileged Access"]
    #[inline(always)]
    #[must_use]
    pub fn eusart2(&mut self) -> EUSART2_W<31> {
        EUSART2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set peripheral bits to 1 to mark as privileged access only\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppupatd0](index.html) module"]
pub struct PPUPATD0_SPEC;
impl crate::RegisterSpec for PPUPATD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppupatd0::R](R) reader structure"]
impl crate::Readable for PPUPATD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppupatd0::W](W) writer structure"]
impl crate::Writable for PPUPATD0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPUPATD0 to value 0xffff_ffff"]
impl crate::Resettable for PPUPATD0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
