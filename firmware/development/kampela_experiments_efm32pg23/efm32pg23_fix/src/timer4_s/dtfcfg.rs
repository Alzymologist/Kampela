#[doc = "Register `DTFCFG` reader"]
pub struct R(crate::R<DTFCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTFCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTFCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTFCFG` writer"]
pub struct W(crate::W<DTFCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTFCFG_SPEC>;
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
impl From<crate::W<DTFCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTFCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTFA` reader - DTI Fault Action"]
pub type DTFA_R = crate::FieldReader<u8, DTFA_A>;
#[doc = "DTI Fault Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTFA_A {
    #[doc = "0: No action on fault"]
    NONE = 0,
    #[doc = "1: Set outputs inactive"]
    INACTIVE = 1,
    #[doc = "2: Clear outputs"]
    CLEAR = 2,
    #[doc = "3: Tristate outputs"]
    TRISTATE = 3,
}
impl From<DTFA_A> for u8 {
    #[inline(always)]
    fn from(variant: DTFA_A) -> Self {
        variant as _
    }
}
impl DTFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTFA_A {
        match self.bits {
            0 => DTFA_A::NONE,
            1 => DTFA_A::INACTIVE,
            2 => DTFA_A::CLEAR,
            3 => DTFA_A::TRISTATE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DTFA_A::NONE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DTFA_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DTFA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == DTFA_A::TRISTATE
    }
}
#[doc = "Field `DTFA` writer - DTI Fault Action"]
pub type DTFA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DTFCFG_SPEC, u8, DTFA_A, 2, O>;
impl<'a, const O: u8> DTFA_W<'a, O> {
    #[doc = "No action on fault"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DTFA_A::NONE)
    }
    #[doc = "Set outputs inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DTFA_A::INACTIVE)
    }
    #[doc = "Clear outputs"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DTFA_A::CLEAR)
    }
    #[doc = "Tristate outputs"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(DTFA_A::TRISTATE)
    }
}
#[doc = "Field `DTPRS0FEN` reader - DTI PRS 0 Fault Enable"]
pub type DTPRS0FEN_R = crate::BitReader<bool>;
#[doc = "Field `DTPRS0FEN` writer - DTI PRS 0 Fault Enable"]
pub type DTPRS0FEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTFCFG_SPEC, bool, O>;
#[doc = "Field `DTPRS1FEN` reader - DTI PRS 1 Fault Enable"]
pub type DTPRS1FEN_R = crate::BitReader<bool>;
#[doc = "Field `DTPRS1FEN` writer - DTI PRS 1 Fault Enable"]
pub type DTPRS1FEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTFCFG_SPEC, bool, O>;
#[doc = "Field `DTDBGFEN` reader - DTI Debugger Fault Enable"]
pub type DTDBGFEN_R = crate::BitReader<bool>;
#[doc = "Field `DTDBGFEN` writer - DTI Debugger Fault Enable"]
pub type DTDBGFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTFCFG_SPEC, bool, O>;
#[doc = "Field `DTLOCKUPFEN` reader - DTI Lockup Fault Enable"]
pub type DTLOCKUPFEN_R = crate::BitReader<bool>;
#[doc = "Field `DTLOCKUPFEN` writer - DTI Lockup Fault Enable"]
pub type DTLOCKUPFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTFCFG_SPEC, bool, O>;
#[doc = "Field `DTEM23FEN` reader - DTI EM23 Fault Enable"]
pub type DTEM23FEN_R = crate::BitReader<bool>;
#[doc = "Field `DTEM23FEN` writer - DTI EM23 Fault Enable"]
pub type DTEM23FEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTFCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    pub fn dtfa(&self) -> DTFA_R {
        DTFA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    pub fn dtprs0fen(&self) -> DTPRS0FEN_R {
        DTPRS0FEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    pub fn dtprs1fen(&self) -> DTPRS1FEN_R {
        DTPRS1FEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    pub fn dtdbgfen(&self) -> DTDBGFEN_R {
        DTDBGFEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    pub fn dtlockupfen(&self) -> DTLOCKUPFEN_R {
        DTLOCKUPFEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DTI EM23 Fault Enable"]
    #[inline(always)]
    pub fn dtem23fen(&self) -> DTEM23FEN_R {
        DTEM23FEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    #[must_use]
    pub fn dtfa(&mut self) -> DTFA_W<16> {
        DTFA_W::new(self)
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtprs0fen(&mut self) -> DTPRS0FEN_W<24> {
        DTPRS0FEN_W::new(self)
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtprs1fen(&mut self) -> DTPRS1FEN_W<25> {
        DTPRS1FEN_W::new(self)
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtdbgfen(&mut self) -> DTDBGFEN_W<26> {
        DTDBGFEN_W::new(self)
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtlockupfen(&mut self) -> DTLOCKUPFEN_W<27> {
        DTLOCKUPFEN_W::new(self)
    }
    #[doc = "Bit 28 - DTI EM23 Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtem23fen(&mut self) -> DTEM23FEN_W<28> {
        DTEM23FEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtfcfg](index.html) module"]
pub struct DTFCFG_SPEC;
impl crate::RegisterSpec for DTFCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtfcfg::R](R) reader structure"]
impl crate::Readable for DTFCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtfcfg::W](W) writer structure"]
impl crate::Writable for DTFCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTFCFG to value 0"]
impl crate::Resettable for DTFCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
