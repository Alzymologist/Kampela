#[doc = "Register `SINGLEFIFOCFG` reader"]
pub struct R(crate::R<SINGLEFIFOCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLEFIFOCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLEFIFOCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLEFIFOCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SINGLEFIFOCFG` writer"]
pub struct W(crate::W<SINGLEFIFOCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLEFIFOCFG_SPEC>;
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
impl From<crate::W<SINGLEFIFOCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLEFIFOCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALIGNMENT` reader - Alignment"]
pub type ALIGNMENT_R = crate::FieldReader<u8, ALIGNMENT_A>;
#[doc = "Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALIGNMENT_A {
    #[doc = "0: ID\\[7:0\\], SIGN_EXT, DATA\\[11:0\\]"]
    RIGHT12 = 0,
    #[doc = "1: ID\\[7:0\\], SIGN_EXT, DATA\\[15:0\\]"]
    RIGHT16 = 1,
    #[doc = "2: ID\\[7:0\\], SIGN_EXT, DATA\\[19:0\\]"]
    RIGHT20 = 2,
    #[doc = "3: DATA\\[11:0\\], 000000000000, ID\\[7:0\\]"]
    LEFT12 = 3,
    #[doc = "4: DATA\\[15:0\\], 00000000, ID\\[7:0\\]"]
    LEFT16 = 4,
    #[doc = "5: DATA\\[19:0\\], 0000, ID\\[7:0\\]"]
    LEFT20 = 5,
}
impl From<ALIGNMENT_A> for u8 {
    #[inline(always)]
    fn from(variant: ALIGNMENT_A) -> Self {
        variant as _
    }
}
impl ALIGNMENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALIGNMENT_A> {
        match self.bits {
            0 => Some(ALIGNMENT_A::RIGHT12),
            1 => Some(ALIGNMENT_A::RIGHT16),
            2 => Some(ALIGNMENT_A::RIGHT20),
            3 => Some(ALIGNMENT_A::LEFT12),
            4 => Some(ALIGNMENT_A::LEFT16),
            5 => Some(ALIGNMENT_A::LEFT20),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT12`"]
    #[inline(always)]
    pub fn is_right12(&self) -> bool {
        *self == ALIGNMENT_A::RIGHT12
    }
    #[doc = "Checks if the value of the field is `RIGHT16`"]
    #[inline(always)]
    pub fn is_right16(&self) -> bool {
        *self == ALIGNMENT_A::RIGHT16
    }
    #[doc = "Checks if the value of the field is `RIGHT20`"]
    #[inline(always)]
    pub fn is_right20(&self) -> bool {
        *self == ALIGNMENT_A::RIGHT20
    }
    #[doc = "Checks if the value of the field is `LEFT12`"]
    #[inline(always)]
    pub fn is_left12(&self) -> bool {
        *self == ALIGNMENT_A::LEFT12
    }
    #[doc = "Checks if the value of the field is `LEFT16`"]
    #[inline(always)]
    pub fn is_left16(&self) -> bool {
        *self == ALIGNMENT_A::LEFT16
    }
    #[doc = "Checks if the value of the field is `LEFT20`"]
    #[inline(always)]
    pub fn is_left20(&self) -> bool {
        *self == ALIGNMENT_A::LEFT20
    }
}
#[doc = "Field `ALIGNMENT` writer - Alignment"]
pub type ALIGNMENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SINGLEFIFOCFG_SPEC, u8, ALIGNMENT_A, 3, O>;
impl<'a, const O: u8> ALIGNMENT_W<'a, O> {
    #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[11:0\\]"]
    #[inline(always)]
    pub fn right12(self) -> &'a mut W {
        self.variant(ALIGNMENT_A::RIGHT12)
    }
    #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[15:0\\]"]
    #[inline(always)]
    pub fn right16(self) -> &'a mut W {
        self.variant(ALIGNMENT_A::RIGHT16)
    }
    #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[19:0\\]"]
    #[inline(always)]
    pub fn right20(self) -> &'a mut W {
        self.variant(ALIGNMENT_A::RIGHT20)
    }
    #[doc = "DATA\\[11:0\\], 000000000000, ID\\[7:0\\]"]
    #[inline(always)]
    pub fn left12(self) -> &'a mut W {
        self.variant(ALIGNMENT_A::LEFT12)
    }
    #[doc = "DATA\\[15:0\\], 00000000, ID\\[7:0\\]"]
    #[inline(always)]
    pub fn left16(self) -> &'a mut W {
        self.variant(ALIGNMENT_A::LEFT16)
    }
    #[doc = "DATA\\[19:0\\], 0000, ID\\[7:0\\]"]
    #[inline(always)]
    pub fn left20(self) -> &'a mut W {
        self.variant(ALIGNMENT_A::LEFT20)
    }
}
#[doc = "Field `SHOWID` reader - Show ID"]
pub type SHOWID_R = crate::BitReader<bool>;
#[doc = "Field `SHOWID` writer - Show ID"]
pub type SHOWID_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINGLEFIFOCFG_SPEC, bool, O>;
#[doc = "Field `DVL` reader - Data Valid Level"]
pub type DVL_R = crate::FieldReader<u8, DVL_A>;
#[doc = "Data Valid Level\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVL_A {
    #[doc = "0: When 1 entry in the single FIFO is valid, set the SINGLEFIFODVL interrupt and request DMA."]
    VALID1 = 0,
    #[doc = "1: When 2 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    VALID2 = 1,
    #[doc = "2: When 3 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    VALID3 = 2,
    #[doc = "3: When 4 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    VALID4 = 3,
    #[doc = "4: When 5 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    VALID5 = 4,
    #[doc = "5: When 6 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    VALID6 = 5,
    #[doc = "6: When 7 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    VALID7 = 6,
    #[doc = "7: When 8 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    VALID8 = 7,
}
impl From<DVL_A> for u8 {
    #[inline(always)]
    fn from(variant: DVL_A) -> Self {
        variant as _
    }
}
impl DVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVL_A {
        match self.bits {
            0 => DVL_A::VALID1,
            1 => DVL_A::VALID2,
            2 => DVL_A::VALID3,
            3 => DVL_A::VALID4,
            4 => DVL_A::VALID5,
            5 => DVL_A::VALID6,
            6 => DVL_A::VALID7,
            7 => DVL_A::VALID8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALID1`"]
    #[inline(always)]
    pub fn is_valid1(&self) -> bool {
        *self == DVL_A::VALID1
    }
    #[doc = "Checks if the value of the field is `VALID2`"]
    #[inline(always)]
    pub fn is_valid2(&self) -> bool {
        *self == DVL_A::VALID2
    }
    #[doc = "Checks if the value of the field is `VALID3`"]
    #[inline(always)]
    pub fn is_valid3(&self) -> bool {
        *self == DVL_A::VALID3
    }
    #[doc = "Checks if the value of the field is `VALID4`"]
    #[inline(always)]
    pub fn is_valid4(&self) -> bool {
        *self == DVL_A::VALID4
    }
    #[doc = "Checks if the value of the field is `VALID5`"]
    #[inline(always)]
    pub fn is_valid5(&self) -> bool {
        *self == DVL_A::VALID5
    }
    #[doc = "Checks if the value of the field is `VALID6`"]
    #[inline(always)]
    pub fn is_valid6(&self) -> bool {
        *self == DVL_A::VALID6
    }
    #[doc = "Checks if the value of the field is `VALID7`"]
    #[inline(always)]
    pub fn is_valid7(&self) -> bool {
        *self == DVL_A::VALID7
    }
    #[doc = "Checks if the value of the field is `VALID8`"]
    #[inline(always)]
    pub fn is_valid8(&self) -> bool {
        *self == DVL_A::VALID8
    }
}
#[doc = "Field `DVL` writer - Data Valid Level"]
pub type DVL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SINGLEFIFOCFG_SPEC, u8, DVL_A, 3, O>;
impl<'a, const O: u8> DVL_W<'a, O> {
    #[doc = "When 1 entry in the single FIFO is valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid1(self) -> &'a mut W {
        self.variant(DVL_A::VALID1)
    }
    #[doc = "When 2 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid2(self) -> &'a mut W {
        self.variant(DVL_A::VALID2)
    }
    #[doc = "When 3 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid3(self) -> &'a mut W {
        self.variant(DVL_A::VALID3)
    }
    #[doc = "When 4 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid4(self) -> &'a mut W {
        self.variant(DVL_A::VALID4)
    }
    #[doc = "When 5 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid5(self) -> &'a mut W {
        self.variant(DVL_A::VALID5)
    }
    #[doc = "When 6 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid6(self) -> &'a mut W {
        self.variant(DVL_A::VALID6)
    }
    #[doc = "When 7 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid7(self) -> &'a mut W {
        self.variant(DVL_A::VALID7)
    }
    #[doc = "When 8 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid8(self) -> &'a mut W {
        self.variant(DVL_A::VALID8)
    }
}
#[doc = "Field `DMAWUFIFOSINGLE` reader - Single FIFO DMA wakeup."]
pub type DMAWUFIFOSINGLE_R = crate::BitReader<DMAWUFIFOSINGLE_A>;
#[doc = "Single FIFO DMA wakeup.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAWUFIFOSINGLE_A {
    #[doc = "0: While in EM2 or EM3, the DMA controller will not be requested."]
    DISABLED = 0,
    #[doc = "1: While in EM2 or EM3, the DMA controller will be requested when the single FIFO reaches its Data Valid Level. \\[DVL must be set to 0 (VALID1).\\]"]
    ENABLED = 1,
}
impl From<DMAWUFIFOSINGLE_A> for bool {
    #[inline(always)]
    fn from(variant: DMAWUFIFOSINGLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAWUFIFOSINGLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAWUFIFOSINGLE_A {
        match self.bits {
            false => DMAWUFIFOSINGLE_A::DISABLED,
            true => DMAWUFIFOSINGLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAWUFIFOSINGLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAWUFIFOSINGLE_A::ENABLED
    }
}
#[doc = "Field `DMAWUFIFOSINGLE` writer - Single FIFO DMA wakeup."]
pub type DMAWUFIFOSINGLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SINGLEFIFOCFG_SPEC, DMAWUFIFOSINGLE_A, O>;
impl<'a, const O: u8> DMAWUFIFOSINGLE_W<'a, O> {
    #[doc = "While in EM2 or EM3, the DMA controller will not be requested."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAWUFIFOSINGLE_A::DISABLED)
    }
    #[doc = "While in EM2 or EM3, the DMA controller will be requested when the single FIFO reaches its Data Valid Level. \\[DVL must be set to 0 (VALID1).\\]"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAWUFIFOSINGLE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:2 - Alignment"]
    #[inline(always)]
    pub fn alignment(&self) -> ALIGNMENT_R {
        ALIGNMENT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Show ID"]
    #[inline(always)]
    pub fn showid(&self) -> SHOWID_R {
        SHOWID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Data Valid Level"]
    #[inline(always)]
    pub fn dvl(&self) -> DVL_R {
        DVL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Single FIFO DMA wakeup."]
    #[inline(always)]
    pub fn dmawufifosingle(&self) -> DMAWUFIFOSINGLE_R {
        DMAWUFIFOSINGLE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alignment"]
    #[inline(always)]
    #[must_use]
    pub fn alignment(&mut self) -> ALIGNMENT_W<0> {
        ALIGNMENT_W::new(self)
    }
    #[doc = "Bit 3 - Show ID"]
    #[inline(always)]
    #[must_use]
    pub fn showid(&mut self) -> SHOWID_W<3> {
        SHOWID_W::new(self)
    }
    #[doc = "Bits 4:6 - Data Valid Level"]
    #[inline(always)]
    #[must_use]
    pub fn dvl(&mut self) -> DVL_W<4> {
        DVL_W::new(self)
    }
    #[doc = "Bit 8 - Single FIFO DMA wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn dmawufifosingle(&mut self) -> DMAWUFIFOSINGLE_W<8> {
        DMAWUFIFOSINGLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Single FIFO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlefifocfg](index.html) module"]
pub struct SINGLEFIFOCFG_SPEC;
impl crate::RegisterSpec for SINGLEFIFOCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singlefifocfg::R](R) reader structure"]
impl crate::Readable for SINGLEFIFOCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singlefifocfg::W](W) writer structure"]
impl crate::Writable for SINGLEFIFOCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SINGLEFIFOCFG to value 0x30"]
impl crate::Resettable for SINGLEFIFOCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
