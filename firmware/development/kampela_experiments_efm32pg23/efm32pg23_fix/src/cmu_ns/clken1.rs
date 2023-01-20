#[doc = "Register `CLKEN1` reader"]
pub struct R(crate::R<CLKEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKEN1` writer"]
pub struct W(crate::W<CLKEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKEN1_SPEC>;
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
impl From<crate::W<CLKEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOSTMAILBOX` reader - Enable Bus Clock"]
pub type HOSTMAILBOX_R = crate::BitReader<bool>;
#[doc = "Field `HOSTMAILBOX` writer - Enable Bus Clock"]
pub type HOSTMAILBOX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `SEMAILBOXHOST` reader - Enable Bus Clock"]
pub type SEMAILBOXHOST_R = crate::BitReader<bool>;
#[doc = "Field `SEMAILBOXHOST` writer - Enable Bus Clock"]
pub type SEMAILBOXHOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `LCD` reader - Enable Bus Clock"]
pub type LCD_R = crate::BitReader<bool>;
#[doc = "Field `LCD` writer - Enable Bus Clock"]
pub type LCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `KEYSCAN` reader - Enable Bus Clock"]
pub type KEYSCAN_R = crate::BitReader<bool>;
#[doc = "Field `KEYSCAN` writer - Enable Bus Clock"]
pub type KEYSCAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `SMU` reader - Enable Bus Clock"]
pub type SMU_R = crate::BitReader<bool>;
#[doc = "Field `SMU` writer - Enable Bus Clock"]
pub type SMU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `ICACHE0` reader - Enable Bus Clock"]
pub type ICACHE0_R = crate::BitReader<bool>;
#[doc = "Field `ICACHE0` writer - Enable Bus Clock"]
pub type ICACHE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `MSC` reader - Enable Bus Clock"]
pub type MSC_R = crate::BitReader<bool>;
#[doc = "Field `MSC` writer - Enable Bus Clock"]
pub type MSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `WDOG1` reader - Enable Bus Clock"]
pub type WDOG1_R = crate::BitReader<bool>;
#[doc = "Field `WDOG1` writer - Enable Bus Clock"]
pub type WDOG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `ACMP0` reader - Enable Bus Clock"]
pub type ACMP0_R = crate::BitReader<bool>;
#[doc = "Field `ACMP0` writer - Enable Bus Clock"]
pub type ACMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `ACMP1` reader - Enable Bus Clock"]
pub type ACMP1_R = crate::BitReader<bool>;
#[doc = "Field `ACMP1` writer - Enable Bus Clock"]
pub type ACMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `VDAC0` reader - Enable Bus Clock"]
pub type VDAC0_R = crate::BitReader<bool>;
#[doc = "Field `VDAC0` writer - Enable Bus Clock"]
pub type VDAC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `PCNT0` reader - Enable Bus Clock"]
pub type PCNT0_R = crate::BitReader<bool>;
#[doc = "Field `PCNT0` writer - Enable Bus Clock"]
pub type PCNT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `EUSART0` reader - Enable Bus Clock"]
pub type EUSART0_R = crate::BitReader<bool>;
#[doc = "Field `EUSART0` writer - Enable Bus Clock"]
pub type EUSART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `EUSART1` reader - Enable Bus Clock"]
pub type EUSART1_R = crate::BitReader<bool>;
#[doc = "Field `EUSART1` writer - Enable Bus Clock"]
pub type EUSART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `EUSART2` reader - Enable Bus Clock"]
pub type EUSART2_R = crate::BitReader<bool>;
#[doc = "Field `EUSART2` writer - Enable Bus Clock"]
pub type EUSART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `DMEM` reader - Enable Bus Clock"]
pub type DMEM_R = crate::BitReader<bool>;
#[doc = "Field `DMEM` writer - Enable Bus Clock"]
pub type DMEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - Enable Bus Clock"]
    #[inline(always)]
    pub fn hostmailbox(&self) -> HOSTMAILBOX_R {
        HOSTMAILBOX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Bus Clock"]
    #[inline(always)]
    pub fn semailboxhost(&self) -> SEMAILBOXHOST_R {
        SEMAILBOXHOST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Bus Clock"]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Bus Clock"]
    #[inline(always)]
    pub fn keyscan(&self) -> KEYSCAN_R {
        KEYSCAN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Bus Clock"]
    #[inline(always)]
    pub fn smu(&self) -> SMU_R {
        SMU_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Bus Clock"]
    #[inline(always)]
    pub fn icache0(&self) -> ICACHE0_R {
        ICACHE0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Bus Clock"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Bus Clock"]
    #[inline(always)]
    pub fn wdog1(&self) -> WDOG1_R {
        WDOG1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Bus Clock"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Bus Clock"]
    #[inline(always)]
    pub fn acmp1(&self) -> ACMP1_R {
        ACMP1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Bus Clock"]
    #[inline(always)]
    pub fn vdac0(&self) -> VDAC0_R {
        VDAC0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Bus Clock"]
    #[inline(always)]
    pub fn pcnt0(&self) -> PCNT0_R {
        PCNT0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart0(&self) -> EUSART0_R {
        EUSART0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart1(&self) -> EUSART1_R {
        EUSART1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart2(&self) -> EUSART2_R {
        EUSART2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Bus Clock"]
    #[inline(always)]
    pub fn dmem(&self) -> DMEM_R {
        DMEM_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn hostmailbox(&mut self) -> HOSTMAILBOX_W<8> {
        HOSTMAILBOX_W::new(self)
    }
    #[doc = "Bit 10 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn semailboxhost(&mut self) -> SEMAILBOXHOST_W<10> {
        SEMAILBOXHOST_W::new(self)
    }
    #[doc = "Bit 12 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn lcd(&mut self) -> LCD_W<12> {
        LCD_W::new(self)
    }
    #[doc = "Bit 13 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn keyscan(&mut self) -> KEYSCAN_W<13> {
        KEYSCAN_W::new(self)
    }
    #[doc = "Bit 14 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn smu(&mut self) -> SMU_W<14> {
        SMU_W::new(self)
    }
    #[doc = "Bit 15 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn icache0(&mut self) -> ICACHE0_W<15> {
        ICACHE0_W::new(self)
    }
    #[doc = "Bit 16 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn msc(&mut self) -> MSC_W<16> {
        MSC_W::new(self)
    }
    #[doc = "Bit 17 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn wdog1(&mut self) -> WDOG1_W<17> {
        WDOG1_W::new(self)
    }
    #[doc = "Bit 18 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0(&mut self) -> ACMP0_W<18> {
        ACMP0_W::new(self)
    }
    #[doc = "Bit 19 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1(&mut self) -> ACMP1_W<19> {
        ACMP1_W::new(self)
    }
    #[doc = "Bit 20 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn vdac0(&mut self) -> VDAC0_W<20> {
        VDAC0_W::new(self)
    }
    #[doc = "Bit 21 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0(&mut self) -> PCNT0_W<21> {
        PCNT0_W::new(self)
    }
    #[doc = "Bit 22 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn eusart0(&mut self) -> EUSART0_W<22> {
        EUSART0_W::new(self)
    }
    #[doc = "Bit 23 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn eusart1(&mut self) -> EUSART1_W<23> {
        EUSART1_W::new(self)
    }
    #[doc = "Bit 24 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn eusart2(&mut self) -> EUSART2_W<24> {
        EUSART2_W::new(self)
    }
    #[doc = "Bit 27 - Enable Bus Clock"]
    #[inline(always)]
    #[must_use]
    pub fn dmem(&mut self) -> DMEM_W<27> {
        DMEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clken1](index.html) module"]
pub struct CLKEN1_SPEC;
impl crate::RegisterSpec for CLKEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clken1::R](R) reader structure"]
impl crate::Readable for CLKEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clken1::W](W) writer structure"]
impl crate::Writable for CLKEN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKEN1 to value 0"]
impl crate::Resettable for CLKEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
