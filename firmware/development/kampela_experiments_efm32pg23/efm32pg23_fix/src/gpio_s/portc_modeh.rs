#[doc = "Register `PORTC_MODEH` reader"]
pub struct R(crate::R<PORTC_MODEH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTC_MODEH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTC_MODEH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTC_MODEH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTC_MODEH` writer"]
pub struct W(crate::W<PORTC_MODEH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTC_MODEH_SPEC>;
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
impl From<crate::W<PORTC_MODEH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTC_MODEH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE0` reader - MODE n"]
pub type MODE0_R = crate::FieldReader<u8, MODE0_A>;
#[doc = "MODE n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE0_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set."]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction."]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction."]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output."]
    PUSHPULL = 4,
    #[doc = "5: Push-pull using alternate control."]
    PUSHPULLALT = 5,
    #[doc = "6: Wired-or output."]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down."]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output."]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter."]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup."]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup."]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output using alternate control."]
    WIREDANDALT = 12,
    #[doc = "13: Open-drain output using alternate control with filter."]
    WIREDANDALTFILTER = 13,
    #[doc = "14: Open-drain output using alternate control with pullup."]
    WIREDANDALTPULLUP = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup."]
    WIREDANDALTPULLUPFILTER = 15,
}
impl From<MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE0_A) -> Self {
        variant as _
    }
}
impl MODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE0_A {
        match self.bits {
            0 => MODE0_A::DISABLED,
            1 => MODE0_A::INPUT,
            2 => MODE0_A::INPUTPULL,
            3 => MODE0_A::INPUTPULLFILTER,
            4 => MODE0_A::PUSHPULL,
            5 => MODE0_A::PUSHPULLALT,
            6 => MODE0_A::WIREDOR,
            7 => MODE0_A::WIREDORPULLDOWN,
            8 => MODE0_A::WIREDAND,
            9 => MODE0_A::WIREDANDFILTER,
            10 => MODE0_A::WIREDANDPULLUP,
            11 => MODE0_A::WIREDANDPULLUPFILTER,
            12 => MODE0_A::WIREDANDALT,
            13 => MODE0_A::WIREDANDALTFILTER,
            14 => MODE0_A::WIREDANDALTPULLUP,
            15 => MODE0_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE0_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE0_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE0_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE0_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE0_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE0_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE0_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE0_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE0_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE0_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE0_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE0_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE0_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE0_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Field `MODE0` writer - MODE n"]
pub type MODE0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PORTC_MODEH_SPEC, u8, MODE0_A, 4, O>;
impl<'a, const O: u8> MODE0_W<'a, O> {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE0_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE0_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE0_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE0_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE0_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDALTPULLUPFILTER)
    }
}
#[doc = "Field `MODE1` reader - MODE n"]
pub type MODE1_R = crate::FieldReader<u8, MODE1_A>;
#[doc = "MODE n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set."]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction."]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction."]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output."]
    PUSHPULL = 4,
    #[doc = "5: Push-pull using alternate control."]
    PUSHPULLALT = 5,
    #[doc = "6: Wired-or output."]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down."]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output."]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter."]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup."]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup."]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output using alternate control."]
    WIREDANDALT = 12,
    #[doc = "13: Open-drain output using alternate control with filter."]
    WIREDANDALTFILTER = 13,
    #[doc = "14: Open-drain output using alternate control with pullup."]
    WIREDANDALTPULLUP = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup."]
    WIREDANDALTPULLUPFILTER = 15,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::DISABLED,
            1 => MODE1_A::INPUT,
            2 => MODE1_A::INPUTPULL,
            3 => MODE1_A::INPUTPULLFILTER,
            4 => MODE1_A::PUSHPULL,
            5 => MODE1_A::PUSHPULLALT,
            6 => MODE1_A::WIREDOR,
            7 => MODE1_A::WIREDORPULLDOWN,
            8 => MODE1_A::WIREDAND,
            9 => MODE1_A::WIREDANDFILTER,
            10 => MODE1_A::WIREDANDPULLUP,
            11 => MODE1_A::WIREDANDPULLUPFILTER,
            12 => MODE1_A::WIREDANDALT,
            13 => MODE1_A::WIREDANDALTFILTER,
            14 => MODE1_A::WIREDANDALTPULLUP,
            15 => MODE1_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE1_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE1_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE1_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE1_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE1_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE1_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE1_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE1_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE1_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE1_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE1_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE1_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE1_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE1_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE1_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Field `MODE1` writer - MODE n"]
pub type MODE1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PORTC_MODEH_SPEC, u8, MODE1_A, 4, O>;
impl<'a, const O: u8> MODE1_W<'a, O> {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE1_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE1_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE1_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE1_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE1_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE1_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDALTPULLUPFILTER)
    }
}
impl R {
    #[doc = "Bits 0:3 - MODE n"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MODE n"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - MODE n"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE0_W<0> {
        MODE0_W::new(self)
    }
    #[doc = "Bits 4:7 - MODE n"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<4> {
        MODE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mode high\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portc_modeh](index.html) module"]
pub struct PORTC_MODEH_SPEC;
impl crate::RegisterSpec for PORTC_MODEH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portc_modeh::R](R) reader structure"]
impl crate::Readable for PORTC_MODEH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portc_modeh::W](W) writer structure"]
impl crate::Writable for PORTC_MODEH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTC_MODEH to value 0"]
impl crate::Resettable for PORTC_MODEH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
