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
#[doc = "Field `CLKDIV` reader - Clock Divider"]
pub type CLKDIV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CLKDIV` writer - Clock Divider"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u32, u32, 18, O>;
#[doc = "Field `SINGLEPRESS` reader - Single Press"]
pub type SINGLEPRESS_R = crate::BitReader<SINGLEPRESS_A>;
#[doc = "Single Press\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINGLEPRESS_A {
    #[doc = "0: After KEYIF is set and then cleared, scanning will continue. This can give multiple interrupts for the same key press, but allow multiple key presses to be detected. To use this mode for multi-key detection, the ISR should update a section of memory of COLNUM bytes on each interrupt, until key release is detected. After key release, the section of memory where key presses are recorded can be processed."]
    MULTIPRESS = 0,
    #[doc = "1: After KEYIF has been set and cleared, it will not set again until no key press is detected. This allows faster response since the ISR can start processing data as soon as the KEYIF is set."]
    SINGLEPRESS = 1,
}
impl From<SINGLEPRESS_A> for bool {
    #[inline(always)]
    fn from(variant: SINGLEPRESS_A) -> Self {
        variant as u8 != 0
    }
}
impl SINGLEPRESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINGLEPRESS_A {
        match self.bits {
            false => SINGLEPRESS_A::MULTIPRESS,
            true => SINGLEPRESS_A::SINGLEPRESS,
        }
    }
    #[doc = "Checks if the value of the field is `MULTIPRESS`"]
    #[inline(always)]
    pub fn is_multipress(&self) -> bool {
        *self == SINGLEPRESS_A::MULTIPRESS
    }
    #[doc = "Checks if the value of the field is `SINGLEPRESS`"]
    #[inline(always)]
    pub fn is_singlepress(&self) -> bool {
        *self == SINGLEPRESS_A::SINGLEPRESS
    }
}
#[doc = "Field `SINGLEPRESS` writer - Single Press"]
pub type SINGLEPRESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, SINGLEPRESS_A, O>;
impl<'a, const O: u8> SINGLEPRESS_W<'a, O> {
    #[doc = "After KEYIF is set and then cleared, scanning will continue. This can give multiple interrupts for the same key press, but allow multiple key presses to be detected. To use this mode for multi-key detection, the ISR should update a section of memory of COLNUM bytes on each interrupt, until key release is detected. After key release, the section of memory where key presses are recorded can be processed."]
    #[inline(always)]
    pub fn multipress(self) -> &'a mut W {
        self.variant(SINGLEPRESS_A::MULTIPRESS)
    }
    #[doc = "After KEYIF has been set and cleared, it will not set again until no key press is detected. This allows faster response since the ISR can start processing data as soon as the KEYIF is set."]
    #[inline(always)]
    pub fn singlepress(self) -> &'a mut W {
        self.variant(SINGLEPRESS_A::SINGLEPRESS)
    }
}
#[doc = "Field `AUTOSTART` reader - Automatically Start"]
pub type AUTOSTART_R = crate::BitReader<AUTOSTART_A>;
#[doc = "Automatically Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOSTART_A {
    #[doc = "0: Auto start is disabled"]
    AUTOSTARTDIS = 0,
    #[doc = "1: Auto start is enabled"]
    AUTOSTARTEN = 1,
}
impl From<AUTOSTART_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOSTART_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOSTART_A {
        match self.bits {
            false => AUTOSTART_A::AUTOSTARTDIS,
            true => AUTOSTART_A::AUTOSTARTEN,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOSTARTDIS`"]
    #[inline(always)]
    pub fn is_autostartdis(&self) -> bool {
        *self == AUTOSTART_A::AUTOSTARTDIS
    }
    #[doc = "Checks if the value of the field is `AUTOSTARTEN`"]
    #[inline(always)]
    pub fn is_autostarten(&self) -> bool {
        *self == AUTOSTART_A::AUTOSTARTEN
    }
}
#[doc = "Field `AUTOSTART` writer - Automatically Start"]
pub type AUTOSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, AUTOSTART_A, O>;
impl<'a, const O: u8> AUTOSTART_W<'a, O> {
    #[doc = "Auto start is disabled"]
    #[inline(always)]
    pub fn autostartdis(self) -> &'a mut W {
        self.variant(AUTOSTART_A::AUTOSTARTDIS)
    }
    #[doc = "Auto start is enabled"]
    #[inline(always)]
    pub fn autostarten(self) -> &'a mut W {
        self.variant(AUTOSTART_A::AUTOSTARTEN)
    }
}
#[doc = "Field `NUMROWS` reader - Number of Rows"]
pub type NUMROWS_R = crate::FieldReader<u8, NUMROWS_A>;
#[doc = "Number of Rows\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NUMROWS_A {
    #[doc = "0: 1 Row is not supported; defaults to 3 instead"]
    RSV1 = 0,
    #[doc = "1: 2 Rows are not supported; defaults to 3 instead"]
    RSV2 = 1,
    #[doc = "2: 3 Rows"]
    ROW3 = 2,
    #[doc = "3: 4 Rows"]
    ROW4 = 3,
    #[doc = "4: 5 Rows"]
    ROW5 = 4,
    #[doc = "5: 6 Rows"]
    ROW6 = 5,
}
impl From<NUMROWS_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMROWS_A) -> Self {
        variant as _
    }
}
impl NUMROWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NUMROWS_A> {
        match self.bits {
            0 => Some(NUMROWS_A::RSV1),
            1 => Some(NUMROWS_A::RSV2),
            2 => Some(NUMROWS_A::ROW3),
            3 => Some(NUMROWS_A::ROW4),
            4 => Some(NUMROWS_A::ROW5),
            5 => Some(NUMROWS_A::ROW6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RSV1`"]
    #[inline(always)]
    pub fn is_rsv1(&self) -> bool {
        *self == NUMROWS_A::RSV1
    }
    #[doc = "Checks if the value of the field is `RSV2`"]
    #[inline(always)]
    pub fn is_rsv2(&self) -> bool {
        *self == NUMROWS_A::RSV2
    }
    #[doc = "Checks if the value of the field is `ROW3`"]
    #[inline(always)]
    pub fn is_row3(&self) -> bool {
        *self == NUMROWS_A::ROW3
    }
    #[doc = "Checks if the value of the field is `ROW4`"]
    #[inline(always)]
    pub fn is_row4(&self) -> bool {
        *self == NUMROWS_A::ROW4
    }
    #[doc = "Checks if the value of the field is `ROW5`"]
    #[inline(always)]
    pub fn is_row5(&self) -> bool {
        *self == NUMROWS_A::ROW5
    }
    #[doc = "Checks if the value of the field is `ROW6`"]
    #[inline(always)]
    pub fn is_row6(&self) -> bool {
        *self == NUMROWS_A::ROW6
    }
}
#[doc = "Field `NUMROWS` writer - Number of Rows"]
pub type NUMROWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, NUMROWS_A, 3, O>;
impl<'a, const O: u8> NUMROWS_W<'a, O> {
    #[doc = "1 Row is not supported; defaults to 3 instead"]
    #[inline(always)]
    pub fn rsv1(self) -> &'a mut W {
        self.variant(NUMROWS_A::RSV1)
    }
    #[doc = "2 Rows are not supported; defaults to 3 instead"]
    #[inline(always)]
    pub fn rsv2(self) -> &'a mut W {
        self.variant(NUMROWS_A::RSV2)
    }
    #[doc = "3 Rows"]
    #[inline(always)]
    pub fn row3(self) -> &'a mut W {
        self.variant(NUMROWS_A::ROW3)
    }
    #[doc = "4 Rows"]
    #[inline(always)]
    pub fn row4(self) -> &'a mut W {
        self.variant(NUMROWS_A::ROW4)
    }
    #[doc = "5 Rows"]
    #[inline(always)]
    pub fn row5(self) -> &'a mut W {
        self.variant(NUMROWS_A::ROW5)
    }
    #[doc = "6 Rows"]
    #[inline(always)]
    pub fn row6(self) -> &'a mut W {
        self.variant(NUMROWS_A::ROW6)
    }
}
#[doc = "Field `NUMCOLS` reader - Number of Columns"]
pub type NUMCOLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMCOLS` writer - Number of Columns"]
pub type NUMCOLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:17 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bit 20 - Single Press"]
    #[inline(always)]
    pub fn singlepress(&self) -> SINGLEPRESS_R {
        SINGLEPRESS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Automatically Start"]
    #[inline(always)]
    pub fn autostart(&self) -> AUTOSTART_R {
        AUTOSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Number of Rows"]
    #[inline(always)]
    pub fn numrows(&self) -> NUMROWS_R {
        NUMROWS_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Number of Columns"]
    #[inline(always)]
    pub fn numcols(&self) -> NUMCOLS_R {
        NUMCOLS_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 20 - Single Press"]
    #[inline(always)]
    #[must_use]
    pub fn singlepress(&mut self) -> SINGLEPRESS_W<20> {
        SINGLEPRESS_W::new(self)
    }
    #[doc = "Bit 22 - Automatically Start"]
    #[inline(always)]
    #[must_use]
    pub fn autostart(&mut self) -> AUTOSTART_W<22> {
        AUTOSTART_W::new(self)
    }
    #[doc = "Bits 24:26 - Number of Rows"]
    #[inline(always)]
    #[must_use]
    pub fn numrows(&mut self) -> NUMROWS_W<24> {
        NUMROWS_W::new(self)
    }
    #[doc = "Bits 28:30 - Number of Columns"]
    #[inline(always)]
    #[must_use]
    pub fn numcols(&mut self) -> NUMCOLS_W<28> {
        NUMCOLS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CFG to value 0x2501_387f"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x2501_387f;
}
