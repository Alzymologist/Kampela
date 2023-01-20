#[doc = "Register `CAL` reader"]
pub struct R(crate::R<CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL` writer"]
pub struct W(crate::W<CAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_SPEC>;
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
impl From<crate::W<CAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING` reader - Tuning Value"]
pub type TUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING` writer - Tuning Value"]
pub type TUNING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 7, O>;
#[doc = "Field `FINETUNING` reader - Fine Tuning Value"]
pub type FINETUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FINETUNING` writer - Fine Tuning Value"]
pub type FINETUNING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 6, O>;
#[doc = "Field `LDOHP` reader - LDO High Power Mode"]
pub type LDOHP_R = crate::BitReader<bool>;
#[doc = "Field `LDOHP` writer - LDO High Power Mode"]
pub type LDOHP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_SPEC, bool, O>;
#[doc = "Field `FREQRANGE` reader - Frequency Range"]
pub type FREQRANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FREQRANGE` writer - Frequency Range"]
pub type FREQRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `CMPBIAS` reader - Comparator Bias Current"]
pub type CMPBIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPBIAS` writer - Comparator Bias Current"]
pub type CMPBIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CLKDIV` reader - Locally Divide HFRCO Clock Output"]
pub type CLKDIV_R = crate::FieldReader<u8, CLKDIV_A>;
#[doc = "Locally Divide HFRCO Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKDIV_A {
    #[doc = "0: Divide by 1."]
    DIV1 = 0,
    #[doc = "1: Divide by 2."]
    DIV2 = 1,
    #[doc = "2: Divide by 4."]
    DIV4 = 2,
}
impl From<CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_A) -> Self {
        variant as _
    }
}
impl CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKDIV_A> {
        match self.bits {
            0 => Some(CLKDIV_A::DIV1),
            1 => Some(CLKDIV_A::DIV2),
            2 => Some(CLKDIV_A::DIV4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CLKDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CLKDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CLKDIV_A::DIV4
    }
}
#[doc = "Field `CLKDIV` writer - Locally Divide HFRCO Clock Output"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, CLKDIV_A, 2, O>;
impl<'a, const O: u8> CLKDIV_W<'a, O> {
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV4)
    }
}
#[doc = "Field `CMPSEL` reader - Comparator Load Select"]
pub type CMPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPSEL` writer - Comparator Load Select"]
pub type CMPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 2, O>;
#[doc = "Field `IREFTC` reader - Tempco Trim on Comparator Current"]
pub type IREFTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IREFTC` writer - Tempco Trim on Comparator Current"]
pub type IREFTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:6 - Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Fine Tuning Value"]
    #[inline(always)]
    pub fn finetuning(&self) -> FINETUNING_R {
        FINETUNING_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - LDO High Power Mode"]
    #[inline(always)]
    pub fn ldohp(&self) -> LDOHP_R {
        LDOHP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Frequency Range"]
    #[inline(always)]
    pub fn freqrange(&self) -> FREQRANGE_R {
        FREQRANGE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Comparator Bias Current"]
    #[inline(always)]
    pub fn cmpbias(&self) -> CMPBIAS_R {
        CMPBIAS_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Locally Divide HFRCO Clock Output"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Comparator Load Select"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CMPSEL_R {
        CMPSEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Tempco Trim on Comparator Current"]
    #[inline(always)]
    pub fn ireftc(&self) -> IREFTC_R {
        IREFTC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Tuning Value"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TUNING_W<0> {
        TUNING_W::new(self)
    }
    #[doc = "Bits 8:13 - Fine Tuning Value"]
    #[inline(always)]
    #[must_use]
    pub fn finetuning(&mut self) -> FINETUNING_W<8> {
        FINETUNING_W::new(self)
    }
    #[doc = "Bit 15 - LDO High Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ldohp(&mut self) -> LDOHP_W<15> {
        LDOHP_W::new(self)
    }
    #[doc = "Bits 16:20 - Frequency Range"]
    #[inline(always)]
    #[must_use]
    pub fn freqrange(&mut self) -> FREQRANGE_W<16> {
        FREQRANGE_W::new(self)
    }
    #[doc = "Bits 21:23 - Comparator Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn cmpbias(&mut self) -> CMPBIAS_W<21> {
        CMPBIAS_W::new(self)
    }
    #[doc = "Bits 24:25 - Locally Divide HFRCO Clock Output"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<24> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 26:27 - Comparator Load Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsel(&mut self) -> CMPSEL_W<26> {
        CMPSEL_W::new(self)
    }
    #[doc = "Bits 28:31 - Tempco Trim on Comparator Current"]
    #[inline(always)]
    #[must_use]
    pub fn ireftc(&mut self) -> IREFTC_W<28> {
        IREFTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal](index.html) module"]
pub struct CAL_SPEC;
impl crate::RegisterSpec for CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal::R](R) reader structure"]
impl crate::Readable for CAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal::W](W) writer structure"]
impl crate::Writable for CAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL to value 0xa868_9f7f"]
impl crate::Resettable for CAL_SPEC {
    const RESET_VALUE: Self::Ux = 0xa868_9f7f;
}
