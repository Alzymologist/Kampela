#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGC` reader - LFXO AGC Enable"]
pub type AGC_R = crate::BitReader<bool>;
#[doc = "Field `AGC` writer - LFXO AGC Enable"]
pub type AGC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `HIGHAMPL` reader - LFXO High Amplitude Enable"]
pub type HIGHAMPL_R = crate::BitReader<bool>;
#[doc = "Field `HIGHAMPL` writer - LFXO High Amplitude Enable"]
pub type HIGHAMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `MODE` reader - LFXO Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "LFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: A 32768Hz crystal should be connected to the LF crystal pads. Voltage must not exceed VDDIO."]
    XTAL = 0,
    #[doc = "1: An external sine source with minimum amplitude 100mv (zero-to-peak) and maximum amplitude 500mV (zero-to-peak) should be connected in series with LFXTAL_I pin. Minimum voltage should be larger than ground and maximum voltage smaller than VDDIO. The sine source does not need to be ac coupled externally as it is ac couples inside LFXO. LFXTAL_O is free to be used as a general purpose GPIO."]
    BUFEXTCLK = 1,
    #[doc = "2: An external 32KHz CMOS clock should be provided on LFXTAL_I. LFXTAL_O is free to be used as a general purpose GPIO."]
    DIGEXTCLK = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::XTAL),
            1 => Some(MODE_A::BUFEXTCLK),
            2 => Some(MODE_A::DIGEXTCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == MODE_A::XTAL
    }
    #[doc = "Checks if the value of the field is `BUFEXTCLK`"]
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == MODE_A::BUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DIGEXTCLK`"]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == MODE_A::DIGEXTCLK
    }
}
#[doc = "Field `MODE` writer - LFXO Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "A 32768Hz crystal should be connected to the LF crystal pads. Voltage must not exceed VDDIO."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(MODE_A::XTAL)
    }
    #[doc = "An external sine source with minimum amplitude 100mv (zero-to-peak) and maximum amplitude 500mV (zero-to-peak) should be connected in series with LFXTAL_I pin. Minimum voltage should be larger than ground and maximum voltage smaller than VDDIO. The sine source does not need to be ac coupled externally as it is ac couples inside LFXO. LFXTAL_O is free to be used as a general purpose GPIO."]
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut W {
        self.variant(MODE_A::BUFEXTCLK)
    }
    #[doc = "An external 32KHz CMOS clock should be provided on LFXTAL_I. LFXTAL_O is free to be used as a general purpose GPIO."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(MODE_A::DIGEXTCLK)
    }
}
#[doc = "Field `TIMEOUT` reader - LFXO Start-up Delay"]
pub type TIMEOUT_R = crate::FieldReader<u8, TIMEOUT_A>;
#[doc = "LFXO Start-up Delay\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    CYCLES2 = 0,
    #[doc = "1: Timeout period of 256 cycles"]
    CYCLES256 = 1,
    #[doc = "2: Timeout period of 1024 cycles"]
    CYCLES1K = 2,
    #[doc = "3: Timeout period of 2048 cycles"]
    CYCLES2K = 3,
    #[doc = "4: Timeout period of 4096 cycles"]
    CYCLES4K = 4,
    #[doc = "5: Timeout period of 8192 cycles"]
    CYCLES8K = 5,
    #[doc = "6: Timeout period of 16384 cycles"]
    CYCLES16K = 6,
    #[doc = "7: Timeout period of 32768 cycles"]
    CYCLES32K = 7,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as _
    }
}
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            0 => TIMEOUT_A::CYCLES2,
            1 => TIMEOUT_A::CYCLES256,
            2 => TIMEOUT_A::CYCLES1K,
            3 => TIMEOUT_A::CYCLES2K,
            4 => TIMEOUT_A::CYCLES4K,
            5 => TIMEOUT_A::CYCLES8K,
            6 => TIMEOUT_A::CYCLES16K,
            7 => TIMEOUT_A::CYCLES32K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES2`"]
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == TIMEOUT_A::CYCLES2
    }
    #[doc = "Checks if the value of the field is `CYCLES256`"]
    #[inline(always)]
    pub fn is_cycles256(&self) -> bool {
        *self == TIMEOUT_A::CYCLES256
    }
    #[doc = "Checks if the value of the field is `CYCLES1K`"]
    #[inline(always)]
    pub fn is_cycles1k(&self) -> bool {
        *self == TIMEOUT_A::CYCLES1K
    }
    #[doc = "Checks if the value of the field is `CYCLES2K`"]
    #[inline(always)]
    pub fn is_cycles2k(&self) -> bool {
        *self == TIMEOUT_A::CYCLES2K
    }
    #[doc = "Checks if the value of the field is `CYCLES4K`"]
    #[inline(always)]
    pub fn is_cycles4k(&self) -> bool {
        *self == TIMEOUT_A::CYCLES4K
    }
    #[doc = "Checks if the value of the field is `CYCLES8K`"]
    #[inline(always)]
    pub fn is_cycles8k(&self) -> bool {
        *self == TIMEOUT_A::CYCLES8K
    }
    #[doc = "Checks if the value of the field is `CYCLES16K`"]
    #[inline(always)]
    pub fn is_cycles16k(&self) -> bool {
        *self == TIMEOUT_A::CYCLES16K
    }
    #[doc = "Checks if the value of the field is `CYCLES32K`"]
    #[inline(always)]
    pub fn is_cycles32k(&self) -> bool {
        *self == TIMEOUT_A::CYCLES32K
    }
}
#[doc = "Field `TIMEOUT` writer - LFXO Start-up Delay"]
pub type TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, TIMEOUT_A, 3, O>;
impl<'a, const O: u8> TIMEOUT_W<'a, O> {
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut W {
        self.variant(TIMEOUT_A::CYCLES2)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn cycles256(self) -> &'a mut W {
        self.variant(TIMEOUT_A::CYCLES256)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn cycles1k(self) -> &'a mut W {
        self.variant(TIMEOUT_A::CYCLES1K)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn cycles2k(self) -> &'a mut W {
        self.variant(TIMEOUT_A::CYCLES2K)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn cycles4k(self) -> &'a mut W {
        self.variant(TIMEOUT_A::CYCLES4K)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn cycles8k(self) -> &'a mut W {
        self.variant(TIMEOUT_A::CYCLES8K)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn cycles16k(self) -> &'a mut W {
        self.variant(TIMEOUT_A::CYCLES16K)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn cycles32k(self) -> &'a mut W {
        self.variant(TIMEOUT_A::CYCLES32K)
    }
}
impl R {
    #[doc = "Bit 0 - LFXO AGC Enable"]
    #[inline(always)]
    pub fn agc(&self) -> AGC_R {
        AGC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LFXO High Amplitude Enable"]
    #[inline(always)]
    pub fn highampl(&self) -> HIGHAMPL_R {
        HIGHAMPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - LFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - LFXO Start-up Delay"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LFXO AGC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn agc(&mut self) -> AGC_W<0> {
        AGC_W::new(self)
    }
    #[doc = "Bit 1 - LFXO High Amplitude Enable"]
    #[inline(always)]
    #[must_use]
    pub fn highampl(&mut self) -> HIGHAMPL_W<1> {
        HIGHAMPL_W::new(self)
    }
    #[doc = "Bits 4:5 - LFXO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    #[doc = "Bits 8:10 - LFXO Start-up Delay"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<8> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Do not write to this register unless the oscillator is forced off. The oscillator is forced off if DISONDEMAND is set and FORCEEN is cleared.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0x0701"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0701;
}
