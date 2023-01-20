#[doc = "Register `HFXOCAL` reader"]
pub struct R(crate::R<HFXOCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SHUNTBIASANA` reader - No Description"]
pub type SHUNTBIASANA_R = crate::FieldReader<u8, SHUNTBIASANA_A>;
#[doc = "No Description\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHUNTBIASANA_A {
    #[doc = "0: I20UA"]
    I20UA = 0,
    #[doc = "1: I30UA"]
    I30UA = 1,
    #[doc = "2: I40UA"]
    I40UA = 2,
    #[doc = "3: I50UA"]
    I50UA = 3,
    #[doc = "4: I60UA"]
    I60UA = 4,
    #[doc = "5: I70UA"]
    I70UA = 5,
    #[doc = "6: I80UA"]
    I80UA = 6,
    #[doc = "7: I90UA"]
    I90UA = 7,
    #[doc = "8: I100UA"]
    I100UA = 8,
    #[doc = "9: I110UA"]
    I110UA = 9,
    #[doc = "10: I120UA"]
    I120UA = 10,
    #[doc = "11: I130UA"]
    I130UA = 11,
    #[doc = "12: I140UA"]
    I140UA = 12,
    #[doc = "13: I150UA"]
    I150UA = 13,
    #[doc = "14: I160UA"]
    I160UA = 14,
    #[doc = "15: I170UA"]
    I170UA = 15,
}
impl From<SHUNTBIASANA_A> for u8 {
    #[inline(always)]
    fn from(variant: SHUNTBIASANA_A) -> Self {
        variant as _
    }
}
impl SHUNTBIASANA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHUNTBIASANA_A {
        match self.bits {
            0 => SHUNTBIASANA_A::I20UA,
            1 => SHUNTBIASANA_A::I30UA,
            2 => SHUNTBIASANA_A::I40UA,
            3 => SHUNTBIASANA_A::I50UA,
            4 => SHUNTBIASANA_A::I60UA,
            5 => SHUNTBIASANA_A::I70UA,
            6 => SHUNTBIASANA_A::I80UA,
            7 => SHUNTBIASANA_A::I90UA,
            8 => SHUNTBIASANA_A::I100UA,
            9 => SHUNTBIASANA_A::I110UA,
            10 => SHUNTBIASANA_A::I120UA,
            11 => SHUNTBIASANA_A::I130UA,
            12 => SHUNTBIASANA_A::I140UA,
            13 => SHUNTBIASANA_A::I150UA,
            14 => SHUNTBIASANA_A::I160UA,
            15 => SHUNTBIASANA_A::I170UA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `I20UA`"]
    #[inline(always)]
    pub fn is_i20ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I20UA
    }
    #[doc = "Checks if the value of the field is `I30UA`"]
    #[inline(always)]
    pub fn is_i30ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I30UA
    }
    #[doc = "Checks if the value of the field is `I40UA`"]
    #[inline(always)]
    pub fn is_i40ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I40UA
    }
    #[doc = "Checks if the value of the field is `I50UA`"]
    #[inline(always)]
    pub fn is_i50ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I50UA
    }
    #[doc = "Checks if the value of the field is `I60UA`"]
    #[inline(always)]
    pub fn is_i60ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I60UA
    }
    #[doc = "Checks if the value of the field is `I70UA`"]
    #[inline(always)]
    pub fn is_i70ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I70UA
    }
    #[doc = "Checks if the value of the field is `I80UA`"]
    #[inline(always)]
    pub fn is_i80ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I80UA
    }
    #[doc = "Checks if the value of the field is `I90UA`"]
    #[inline(always)]
    pub fn is_i90ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I90UA
    }
    #[doc = "Checks if the value of the field is `I100UA`"]
    #[inline(always)]
    pub fn is_i100ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I100UA
    }
    #[doc = "Checks if the value of the field is `I110UA`"]
    #[inline(always)]
    pub fn is_i110ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I110UA
    }
    #[doc = "Checks if the value of the field is `I120UA`"]
    #[inline(always)]
    pub fn is_i120ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I120UA
    }
    #[doc = "Checks if the value of the field is `I130UA`"]
    #[inline(always)]
    pub fn is_i130ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I130UA
    }
    #[doc = "Checks if the value of the field is `I140UA`"]
    #[inline(always)]
    pub fn is_i140ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I140UA
    }
    #[doc = "Checks if the value of the field is `I150UA`"]
    #[inline(always)]
    pub fn is_i150ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I150UA
    }
    #[doc = "Checks if the value of the field is `I160UA`"]
    #[inline(always)]
    pub fn is_i160ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I160UA
    }
    #[doc = "Checks if the value of the field is `I170UA`"]
    #[inline(always)]
    pub fn is_i170ua(&self) -> bool {
        *self == SHUNTBIASANA_A::I170UA
    }
}
#[doc = "Field `VTRTRIMANA` reader - No Description"]
pub type VTRTRIMANA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - No Description"]
    #[inline(always)]
    pub fn shuntbiasana(&self) -> SHUNTBIASANA_R {
        SHUNTBIASANA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - No Description"]
    #[inline(always)]
    pub fn vtrtrimana(&self) -> VTRTRIMANA_R {
        VTRTRIMANA_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "High Frequency Crystal Oscillator Calibration data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxocal](index.html) module"]
pub struct HFXOCAL_SPEC;
impl crate::RegisterSpec for HFXOCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxocal::R](R) reader structure"]
impl crate::Readable for HFXOCAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFXOCAL to value 0xffff_ff00"]
impl crate::Resettable for HFXOCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ff00;
}
