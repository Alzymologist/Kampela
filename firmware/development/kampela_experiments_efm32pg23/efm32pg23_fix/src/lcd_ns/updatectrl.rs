#[doc = "Register `UPDATECTRL` reader"]
pub struct R(crate::R<UPDATECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPDATECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPDATECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPDATECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPDATECTRL` writer"]
pub struct W(crate::W<UPDATECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPDATECTRL_SPEC>;
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
impl From<crate::W<UPDATECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPDATECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTOLOAD` reader - Auto Load"]
pub type AUTOLOAD_R = crate::BitReader<AUTOLOAD_A>;
#[doc = "Auto Load\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOLOAD_A {
    #[doc = "0: CLK_BUS register to CLK_PER register loads must be done manually with a write to CMD.LOAD."]
    MANUAL = 0,
    #[doc = "1: CLK_BUS register to CLK_PER register loads will be started automatically after a write to the register in UPDATECTRL.LOADADDR is detected."]
    AUTO = 1,
}
impl From<AUTOLOAD_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOLOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOLOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOLOAD_A {
        match self.bits {
            false => AUTOLOAD_A::MANUAL,
            true => AUTOLOAD_A::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == AUTOLOAD_A::MANUAL
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == AUTOLOAD_A::AUTO
    }
}
#[doc = "Field `AUTOLOAD` writer - Auto Load"]
pub type AUTOLOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPDATECTRL_SPEC, AUTOLOAD_A, O>;
impl<'a, const O: u8> AUTOLOAD_W<'a, O> {
    #[doc = "CLK_BUS register to CLK_PER register loads must be done manually with a write to CMD.LOAD."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(AUTOLOAD_A::MANUAL)
    }
    #[doc = "CLK_BUS register to CLK_PER register loads will be started automatically after a write to the register in UPDATECTRL.LOADADDR is detected."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(AUTOLOAD_A::AUTO)
    }
}
#[doc = "Field `LOADADDR` reader - Load Address"]
pub type LOADADDR_R = crate::FieldReader<u8, LOADADDR_A>;
#[doc = "Load Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOADADDR_A {
    #[doc = "0: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to BACTRL. Use with UPDATECTRL.AUTOLOAD"]
    BACTRLWR = 0,
    #[doc = "1: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to AREGA. Use with UPDATECTRL.AUTOLOAD"]
    AREGAWR = 1,
    #[doc = "2: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to AREGB. Use with UPDATECTRL.AUTOLOAD"]
    AREGBWR = 2,
    #[doc = "3: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD0. Use with UPDATECTRL.AUTOLOAD"]
    SEGD0WR = 3,
    #[doc = "4: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD1. Use with UPDATECTRL.AUTOLOAD"]
    SEGD1WR = 4,
    #[doc = "5: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD2. Use with UPDATECTRL.AUTOLOAD"]
    SEGD2WR = 5,
    #[doc = "6: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD3. Use with UPDATECTRL.AUTOLOAD"]
    SEGD3WR = 6,
}
impl From<LOADADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: LOADADDR_A) -> Self {
        variant as _
    }
}
impl LOADADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOADADDR_A> {
        match self.bits {
            0 => Some(LOADADDR_A::BACTRLWR),
            1 => Some(LOADADDR_A::AREGAWR),
            2 => Some(LOADADDR_A::AREGBWR),
            3 => Some(LOADADDR_A::SEGD0WR),
            4 => Some(LOADADDR_A::SEGD1WR),
            5 => Some(LOADADDR_A::SEGD2WR),
            6 => Some(LOADADDR_A::SEGD3WR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BACTRLWR`"]
    #[inline(always)]
    pub fn is_bactrlwr(&self) -> bool {
        *self == LOADADDR_A::BACTRLWR
    }
    #[doc = "Checks if the value of the field is `AREGAWR`"]
    #[inline(always)]
    pub fn is_aregawr(&self) -> bool {
        *self == LOADADDR_A::AREGAWR
    }
    #[doc = "Checks if the value of the field is `AREGBWR`"]
    #[inline(always)]
    pub fn is_aregbwr(&self) -> bool {
        *self == LOADADDR_A::AREGBWR
    }
    #[doc = "Checks if the value of the field is `SEGD0WR`"]
    #[inline(always)]
    pub fn is_segd0wr(&self) -> bool {
        *self == LOADADDR_A::SEGD0WR
    }
    #[doc = "Checks if the value of the field is `SEGD1WR`"]
    #[inline(always)]
    pub fn is_segd1wr(&self) -> bool {
        *self == LOADADDR_A::SEGD1WR
    }
    #[doc = "Checks if the value of the field is `SEGD2WR`"]
    #[inline(always)]
    pub fn is_segd2wr(&self) -> bool {
        *self == LOADADDR_A::SEGD2WR
    }
    #[doc = "Checks if the value of the field is `SEGD3WR`"]
    #[inline(always)]
    pub fn is_segd3wr(&self) -> bool {
        *self == LOADADDR_A::SEGD3WR
    }
}
#[doc = "Field `LOADADDR` writer - Load Address"]
pub type LOADADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UPDATECTRL_SPEC, u8, LOADADDR_A, 4, O>;
impl<'a, const O: u8> LOADADDR_W<'a, O> {
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to BACTRL. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn bactrlwr(self) -> &'a mut W {
        self.variant(LOADADDR_A::BACTRLWR)
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to AREGA. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn aregawr(self) -> &'a mut W {
        self.variant(LOADADDR_A::AREGAWR)
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to AREGB. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn aregbwr(self) -> &'a mut W {
        self.variant(LOADADDR_A::AREGBWR)
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD0. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn segd0wr(self) -> &'a mut W {
        self.variant(LOADADDR_A::SEGD0WR)
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD1. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn segd1wr(self) -> &'a mut W {
        self.variant(LOADADDR_A::SEGD1WR)
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD2. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn segd2wr(self) -> &'a mut W {
        self.variant(LOADADDR_A::SEGD2WR)
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD3. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn segd3wr(self) -> &'a mut W {
        self.variant(LOADADDR_A::SEGD3WR)
    }
}
impl R {
    #[doc = "Bit 8 - Auto Load"]
    #[inline(always)]
    pub fn autoload(&self) -> AUTOLOAD_R {
        AUTOLOAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Load Address"]
    #[inline(always)]
    pub fn loadaddr(&self) -> LOADADDR_R {
        LOADADDR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Auto Load"]
    #[inline(always)]
    #[must_use]
    pub fn autoload(&mut self) -> AUTOLOAD_W<8> {
        AUTOLOAD_W::new(self)
    }
    #[doc = "Bits 13:16 - Load Address"]
    #[inline(always)]
    #[must_use]
    pub fn loadaddr(&mut self) -> LOADADDR_W<13> {
        LOADADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [updatectrl](index.html) module"]
pub struct UPDATECTRL_SPEC;
impl crate::RegisterSpec for UPDATECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [updatectrl::R](R) reader structure"]
impl crate::Readable for UPDATECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [updatectrl::W](W) writer structure"]
impl crate::Writable for UPDATECTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPDATECTRL to value 0"]
impl crate::Resettable for UPDATECTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
