#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDCTRL` reader - Update Data Control"]
pub type UDCTRL_R = crate::FieldReader<u8, UDCTRL_A>;
#[doc = "Update Data Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UDCTRL_A {
    #[doc = "0: The data transfer is controlled by SW. Transfer is performed as soon as possible on the next CTRL.PRESCALE clock. This is primarily available for debug only since only some of the new SEGMENT data may be ready by the time of the UPDATE. This should not be used with interrupts since partially updating SEGMENT data may have indeterminant results."]
    REGULAR = 0,
    #[doc = "1: Data is loaded continuously at every frame start"]
    FRAMESTART = 1,
    #[doc = "2: The data transfer is done at the next Frame Counter event"]
    FCEVENT = 2,
    #[doc = "3: The data transfer is done at the next Display Counter event"]
    DISPLAYEVENT = 3,
}
impl From<UDCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: UDCTRL_A) -> Self {
        variant as _
    }
}
impl UDCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDCTRL_A {
        match self.bits {
            0 => UDCTRL_A::REGULAR,
            1 => UDCTRL_A::FRAMESTART,
            2 => UDCTRL_A::FCEVENT,
            3 => UDCTRL_A::DISPLAYEVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == UDCTRL_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `FRAMESTART`"]
    #[inline(always)]
    pub fn is_framestart(&self) -> bool {
        *self == UDCTRL_A::FRAMESTART
    }
    #[doc = "Checks if the value of the field is `FCEVENT`"]
    #[inline(always)]
    pub fn is_fcevent(&self) -> bool {
        *self == UDCTRL_A::FCEVENT
    }
    #[doc = "Checks if the value of the field is `DISPLAYEVENT`"]
    #[inline(always)]
    pub fn is_displayevent(&self) -> bool {
        *self == UDCTRL_A::DISPLAYEVENT
    }
}
#[doc = "Field `UDCTRL` writer - Update Data Control"]
pub type UDCTRL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, UDCTRL_A, 2, O>;
impl<'a, const O: u8> UDCTRL_W<'a, O> {
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible on the next CTRL.PRESCALE clock. This is primarily available for debug only since only some of the new SEGMENT data may be ready by the time of the UPDATE. This should not be used with interrupts since partially updating SEGMENT data may have indeterminant results."]
    #[inline(always)]
    pub fn regular(self) -> &'a mut W {
        self.variant(UDCTRL_A::REGULAR)
    }
    #[doc = "Data is loaded continuously at every frame start"]
    #[inline(always)]
    pub fn framestart(self) -> &'a mut W {
        self.variant(UDCTRL_A::FRAMESTART)
    }
    #[doc = "The data transfer is done at the next Frame Counter event"]
    #[inline(always)]
    pub fn fcevent(self) -> &'a mut W {
        self.variant(UDCTRL_A::FCEVENT)
    }
    #[doc = "The data transfer is done at the next Display Counter event"]
    #[inline(always)]
    pub fn displayevent(self) -> &'a mut W {
        self.variant(UDCTRL_A::DISPLAYEVENT)
    }
}
#[doc = "Field `DSC` reader - Direct Segment Control"]
pub type DSC_R = crate::BitReader<DSC_A>;
#[doc = "Direct Segment Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSC_A {
    #[doc = "0: DSC disable"]
    DISABLE = 0,
    #[doc = "1: DSC enable"]
    ENABLE = 1,
}
impl From<DSC_A> for bool {
    #[inline(always)]
    fn from(variant: DSC_A) -> Self {
        variant as u8 != 0
    }
}
impl DSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSC_A {
        match self.bits {
            false => DSC_A::DISABLE,
            true => DSC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DSC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DSC_A::ENABLE
    }
}
#[doc = "Field `DSC` writer - Direct Segment Control"]
pub type DSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DSC_A, O>;
impl<'a, const O: u8> DSC_W<'a, O> {
    #[doc = "DSC disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DSC_A::DISABLE)
    }
    #[doc = "DSC enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DSC_A::ENABLE)
    }
}
#[doc = "Field `WARMUPDLY` reader - Warmup Delay"]
pub type WARMUPDLY_R = crate::FieldReader<u8, WARMUPDLY_A>;
#[doc = "Warmup Delay\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WARMUPDLY_A {
    #[doc = "0: 1mswarm up"]
    WARMUP1 = 0,
    #[doc = "1: 31ms warm up"]
    WARMUP31 = 1,
    #[doc = "2: 62ms warm up"]
    WARMUP63 = 2,
    #[doc = "3: 125ms warm up"]
    WARMUP125 = 3,
    #[doc = "4: 250ms warm up"]
    WARMUP250 = 4,
    #[doc = "5: 500ms warm up"]
    WARMUP500 = 5,
    #[doc = "6: 1000ms warm up"]
    WARMUP1000 = 6,
    #[doc = "7: 2000ms warm up"]
    WARMUP2000 = 7,
}
impl From<WARMUPDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMUPDLY_A) -> Self {
        variant as _
    }
}
impl WARMUPDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WARMUPDLY_A {
        match self.bits {
            0 => WARMUPDLY_A::WARMUP1,
            1 => WARMUPDLY_A::WARMUP31,
            2 => WARMUPDLY_A::WARMUP63,
            3 => WARMUPDLY_A::WARMUP125,
            4 => WARMUPDLY_A::WARMUP250,
            5 => WARMUPDLY_A::WARMUP500,
            6 => WARMUPDLY_A::WARMUP1000,
            7 => WARMUPDLY_A::WARMUP2000,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WARMUP1`"]
    #[inline(always)]
    pub fn is_warmup1(&self) -> bool {
        *self == WARMUPDLY_A::WARMUP1
    }
    #[doc = "Checks if the value of the field is `WARMUP31`"]
    #[inline(always)]
    pub fn is_warmup31(&self) -> bool {
        *self == WARMUPDLY_A::WARMUP31
    }
    #[doc = "Checks if the value of the field is `WARMUP63`"]
    #[inline(always)]
    pub fn is_warmup63(&self) -> bool {
        *self == WARMUPDLY_A::WARMUP63
    }
    #[doc = "Checks if the value of the field is `WARMUP125`"]
    #[inline(always)]
    pub fn is_warmup125(&self) -> bool {
        *self == WARMUPDLY_A::WARMUP125
    }
    #[doc = "Checks if the value of the field is `WARMUP250`"]
    #[inline(always)]
    pub fn is_warmup250(&self) -> bool {
        *self == WARMUPDLY_A::WARMUP250
    }
    #[doc = "Checks if the value of the field is `WARMUP500`"]
    #[inline(always)]
    pub fn is_warmup500(&self) -> bool {
        *self == WARMUPDLY_A::WARMUP500
    }
    #[doc = "Checks if the value of the field is `WARMUP1000`"]
    #[inline(always)]
    pub fn is_warmup1000(&self) -> bool {
        *self == WARMUPDLY_A::WARMUP1000
    }
    #[doc = "Checks if the value of the field is `WARMUP2000`"]
    #[inline(always)]
    pub fn is_warmup2000(&self) -> bool {
        *self == WARMUPDLY_A::WARMUP2000
    }
}
#[doc = "Field `WARMUPDLY` writer - Warmup Delay"]
pub type WARMUPDLY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, WARMUPDLY_A, 3, O>;
impl<'a, const O: u8> WARMUPDLY_W<'a, O> {
    #[doc = "1mswarm up"]
    #[inline(always)]
    pub fn warmup1(self) -> &'a mut W {
        self.variant(WARMUPDLY_A::WARMUP1)
    }
    #[doc = "31ms warm up"]
    #[inline(always)]
    pub fn warmup31(self) -> &'a mut W {
        self.variant(WARMUPDLY_A::WARMUP31)
    }
    #[doc = "62ms warm up"]
    #[inline(always)]
    pub fn warmup63(self) -> &'a mut W {
        self.variant(WARMUPDLY_A::WARMUP63)
    }
    #[doc = "125ms warm up"]
    #[inline(always)]
    pub fn warmup125(self) -> &'a mut W {
        self.variant(WARMUPDLY_A::WARMUP125)
    }
    #[doc = "250ms warm up"]
    #[inline(always)]
    pub fn warmup250(self) -> &'a mut W {
        self.variant(WARMUPDLY_A::WARMUP250)
    }
    #[doc = "500ms warm up"]
    #[inline(always)]
    pub fn warmup500(self) -> &'a mut W {
        self.variant(WARMUPDLY_A::WARMUP500)
    }
    #[doc = "1000ms warm up"]
    #[inline(always)]
    pub fn warmup1000(self) -> &'a mut W {
        self.variant(WARMUPDLY_A::WARMUP1000)
    }
    #[doc = "2000ms warm up"]
    #[inline(always)]
    pub fn warmup2000(self) -> &'a mut W {
        self.variant(WARMUPDLY_A::WARMUP2000)
    }
}
#[doc = "Field `PRESCALE` reader - Presclae"]
pub type PRESCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESCALE` writer - Presclae"]
pub type PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline(always)]
    pub fn udctrl(&self) -> UDCTRL_R {
        UDCTRL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 16 - Direct Segment Control"]
    #[inline(always)]
    pub fn dsc(&self) -> DSC_R {
        DSC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Warmup Delay"]
    #[inline(always)]
    pub fn warmupdly(&self) -> WARMUPDLY_R {
        WARMUPDLY_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 24:30 - Presclae"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline(always)]
    #[must_use]
    pub fn udctrl(&mut self) -> UDCTRL_W<1> {
        UDCTRL_W::new(self)
    }
    #[doc = "Bit 16 - Direct Segment Control"]
    #[inline(always)]
    #[must_use]
    pub fn dsc(&mut self) -> DSC_W<16> {
        DSC_W::new(self)
    }
    #[doc = "Bits 18:20 - Warmup Delay"]
    #[inline(always)]
    #[must_use]
    pub fn warmupdly(&mut self) -> WARMUPDLY_W<18> {
        WARMUPDLY_W::new(self)
    }
    #[doc = "Bits 24:30 - Presclae"]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PRESCALE_W<24> {
        PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0010_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0000;
}
