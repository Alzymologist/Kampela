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
#[doc = "Field `MODE` reader - Timer Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Up-count mode"]
    UP = 0,
    #[doc = "1: Down-count mode"]
    DOWN = 1,
    #[doc = "2: Up/down-count mode"]
    UPDOWN = 2,
    #[doc = "3: Quadrature decoder mode"]
    QDEC = 3,
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
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::UP,
            1 => MODE_A::DOWN,
            2 => MODE_A::UPDOWN,
            3 => MODE_A::QDEC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == MODE_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == MODE_A::DOWN
    }
    #[doc = "Checks if the value of the field is `UPDOWN`"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == MODE_A::UPDOWN
    }
    #[doc = "Checks if the value of the field is `QDEC`"]
    #[inline(always)]
    pub fn is_qdec(&self) -> bool {
        *self == MODE_A::QDEC
    }
}
#[doc = "Field `MODE` writer - Timer Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Up-count mode"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(MODE_A::UP)
    }
    #[doc = "Down-count mode"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(MODE_A::DOWN)
    }
    #[doc = "Up/down-count mode"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut W {
        self.variant(MODE_A::UPDOWN)
    }
    #[doc = "Quadrature decoder mode"]
    #[inline(always)]
    pub fn qdec(self) -> &'a mut W {
        self.variant(MODE_A::QDEC)
    }
}
#[doc = "Field `SYNC` reader - Timer Start/Stop/Reload Synchronization"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
#[doc = "Timer Start/Stop/Reload Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    #[doc = "0: Timer operation is unaffected by other timers."]
    DISABLE = 0,
    #[doc = "1: Timer may be started, stopped and re-loaded from other timer instances."]
    ENABLE = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::DISABLE,
            true => SYNC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SYNC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SYNC_A::ENABLE
    }
}
#[doc = "Field `SYNC` writer - Timer Start/Stop/Reload Synchronization"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, SYNC_A, O>;
impl<'a, const O: u8> SYNC_W<'a, O> {
    #[doc = "Timer operation is unaffected by other timers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYNC_A::DISABLE)
    }
    #[doc = "Timer may be started, stopped and re-loaded from other timer instances."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYNC_A::ENABLE)
    }
}
#[doc = "Field `OSMEN` reader - One-shot Mode Enable"]
pub type OSMEN_R = crate::BitReader<bool>;
#[doc = "Field `OSMEN` writer - One-shot Mode Enable"]
pub type OSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `QDM` reader - Quadrature Decoder Mode Selection"]
pub type QDM_R = crate::BitReader<QDM_A>;
#[doc = "Quadrature Decoder Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QDM_A {
    #[doc = "0: X2 mode selected"]
    X2 = 0,
    #[doc = "1: X4 mode selected"]
    X4 = 1,
}
impl From<QDM_A> for bool {
    #[inline(always)]
    fn from(variant: QDM_A) -> Self {
        variant as u8 != 0
    }
}
impl QDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QDM_A {
        match self.bits {
            false => QDM_A::X2,
            true => QDM_A::X4,
        }
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == QDM_A::X2
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == QDM_A::X4
    }
}
#[doc = "Field `QDM` writer - Quadrature Decoder Mode Selection"]
pub type QDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, QDM_A, O>;
impl<'a, const O: u8> QDM_W<'a, O> {
    #[doc = "X2 mode selected"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(QDM_A::X2)
    }
    #[doc = "X4 mode selected"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(QDM_A::X4)
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader<DEBUGRUN_A>;
#[doc = "Debug Mode Run Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBUGRUN_A {
    #[doc = "0: Timer is halted in debug mode"]
    HALT = 0,
    #[doc = "1: Timer is running in debug mode"]
    RUN = 1,
}
impl From<DEBUGRUN_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUGRUN_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBUGRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUGRUN_A {
        match self.bits {
            false => DEBUGRUN_A::HALT,
            true => DEBUGRUN_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == DEBUGRUN_A::HALT
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == DEBUGRUN_A::RUN
    }
}
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, DEBUGRUN_A, O>;
impl<'a, const O: u8> DEBUGRUN_W<'a, O> {
    #[doc = "Timer is halted in debug mode"]
    #[inline(always)]
    pub fn halt(self) -> &'a mut W {
        self.variant(DEBUGRUN_A::HALT)
    }
    #[doc = "Timer is running in debug mode"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(DEBUGRUN_A::RUN)
    }
}
#[doc = "Field `DMACLRACT` reader - DMA Request Clear on Active"]
pub type DMACLRACT_R = crate::BitReader<bool>;
#[doc = "Field `DMACLRACT` writer - DMA Request Clear on Active"]
pub type DMACLRACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CLKSEL` reader - Clock Source Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Prescaled EM01GRPACLK"]
    PRESCEM01GRPACLK = 0,
    #[doc = "1: Compare/Capture Channel 1 Input"]
    CC1 = 1,
    #[doc = "2: Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    TIMEROUF = 2,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::PRESCEM01GRPACLK),
            1 => Some(CLKSEL_A::CC1),
            2 => Some(CLKSEL_A::TIMEROUF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRESCEM01GRPACLK`"]
    #[inline(always)]
    pub fn is_prescem01grpaclk(&self) -> bool {
        *self == CLKSEL_A::PRESCEM01GRPACLK
    }
    #[doc = "Checks if the value of the field is `CC1`"]
    #[inline(always)]
    pub fn is_cc1(&self) -> bool {
        *self == CLKSEL_A::CC1
    }
    #[doc = "Checks if the value of the field is `TIMEROUF`"]
    #[inline(always)]
    pub fn is_timerouf(&self) -> bool {
        *self == CLKSEL_A::TIMEROUF
    }
}
#[doc = "Field `CLKSEL` writer - Clock Source Select"]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "Prescaled EM01GRPACLK"]
    #[inline(always)]
    pub fn prescem01grpaclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::PRESCEM01GRPACLK)
    }
    #[doc = "Compare/Capture Channel 1 Input"]
    #[inline(always)]
    pub fn cc1(self) -> &'a mut W {
        self.variant(CLKSEL_A::CC1)
    }
    #[doc = "Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    #[inline(always)]
    pub fn timerouf(self) -> &'a mut W {
        self.variant(CLKSEL_A::TIMEROUF)
    }
}
#[doc = "Field `RETIMEEN` reader - PWM output retimed enable"]
pub type RETIMEEN_R = crate::BitReader<RETIMEEN_A>;
#[doc = "PWM output retimed enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RETIMEEN_A {
    #[doc = "0: PWM outputs are not re-timed."]
    DISABLE = 0,
    #[doc = "1: PWM outputs are re-timed."]
    ENABLE = 1,
}
impl From<RETIMEEN_A> for bool {
    #[inline(always)]
    fn from(variant: RETIMEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RETIMEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETIMEEN_A {
        match self.bits {
            false => RETIMEEN_A::DISABLE,
            true => RETIMEEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RETIMEEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RETIMEEN_A::ENABLE
    }
}
#[doc = "Field `RETIMEEN` writer - PWM output retimed enable"]
pub type RETIMEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, RETIMEEN_A, O>;
impl<'a, const O: u8> RETIMEEN_W<'a, O> {
    #[doc = "PWM outputs are not re-timed."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RETIMEEN_A::DISABLE)
    }
    #[doc = "PWM outputs are re-timed."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RETIMEEN_A::ENABLE)
    }
}
#[doc = "Field `DISSYNCOUT` reader - Disable Timer Start/Stop/Reload output"]
pub type DISSYNCOUT_R = crate::BitReader<DISSYNCOUT_A>;
#[doc = "Disable Timer Start/Stop/Reload output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISSYNCOUT_A {
    #[doc = "0: Timer can start/stop/reload other timers with SYNC bit set"]
    EN = 0,
    #[doc = "1: Timer cannot start/stop/reload other timers with SYNC bit set"]
    DIS = 1,
}
impl From<DISSYNCOUT_A> for bool {
    #[inline(always)]
    fn from(variant: DISSYNCOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl DISSYNCOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISSYNCOUT_A {
        match self.bits {
            false => DISSYNCOUT_A::EN,
            true => DISSYNCOUT_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DISSYNCOUT_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DISSYNCOUT_A::DIS
    }
}
#[doc = "Field `DISSYNCOUT` writer - Disable Timer Start/Stop/Reload output"]
pub type DISSYNCOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, DISSYNCOUT_A, O>;
impl<'a, const O: u8> DISSYNCOUT_W<'a, O> {
    #[doc = "Timer can start/stop/reload other timers with SYNC bit set"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DISSYNCOUT_A::EN)
    }
    #[doc = "Timer cannot start/stop/reload other timers with SYNC bit set"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DISSYNCOUT_A::DIS)
    }
}
#[doc = "Field `ATI` reader - Always Track Inputs"]
pub type ATI_R = crate::BitReader<bool>;
#[doc = "Field `ATI` writer - Always Track Inputs"]
pub type ATI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RSSCOIST` reader - Reload-Start Sets COIST"]
pub type RSSCOIST_R = crate::BitReader<bool>;
#[doc = "Field `RSSCOIST` writer - Reload-Start Sets COIST"]
pub type RSSCOIST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `PRESC` reader - Prescaler Setting"]
pub type PRESC_R = crate::FieldReader<u16, PRESC_A>;
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PRESC_A {
    #[doc = "0: No prescaling"]
    DIV1 = 0,
    #[doc = "1: Prescale by 2"]
    DIV2 = 1,
    #[doc = "3: Prescale by 4"]
    DIV4 = 3,
    #[doc = "7: Prescale by 8"]
    DIV8 = 7,
    #[doc = "15: Prescale by 16"]
    DIV16 = 15,
    #[doc = "31: Prescale by 32"]
    DIV32 = 31,
    #[doc = "63: Prescale by 64"]
    DIV64 = 63,
    #[doc = "127: Prescale by 128"]
    DIV128 = 127,
    #[doc = "255: Prescale by 256"]
    DIV256 = 255,
    #[doc = "511: Prescale by 512"]
    DIV512 = 511,
    #[doc = "1023: Prescale by 1024"]
    DIV1024 = 1023,
}
impl From<PRESC_A> for u16 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::DIV1),
            1 => Some(PRESC_A::DIV2),
            3 => Some(PRESC_A::DIV4),
            7 => Some(PRESC_A::DIV8),
            15 => Some(PRESC_A::DIV16),
            31 => Some(PRESC_A::DIV32),
            63 => Some(PRESC_A::DIV64),
            127 => Some(PRESC_A::DIV128),
            255 => Some(PRESC_A::DIV256),
            511 => Some(PRESC_A::DIV512),
            1023 => Some(PRESC_A::DIV1024),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESC_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PRESC_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESC_A::DIV1024
    }
}
#[doc = "Field `PRESC` writer - Prescaler Setting"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u16, PRESC_A, 10, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "No prescaling"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESC_A::DIV1)
    }
    #[doc = "Prescale by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::DIV2)
    }
    #[doc = "Prescale by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::DIV4)
    }
    #[doc = "Prescale by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESC_A::DIV8)
    }
    #[doc = "Prescale by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESC_A::DIV16)
    }
    #[doc = "Prescale by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESC_A::DIV32)
    }
    #[doc = "Prescale by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESC_A::DIV64)
    }
    #[doc = "Prescale by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESC_A::DIV128)
    }
    #[doc = "Prescale by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESC_A::DIV256)
    }
    #[doc = "Prescale by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESC_A::DIV512)
    }
    #[doc = "Prescale by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESC_A::DIV1024)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline(always)]
    pub fn osmen(&self) -> OSMEN_R {
        OSMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline(always)]
    pub fn qdm(&self) -> QDM_R {
        QDM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline(always)]
    pub fn dmaclract(&self) -> DMACLRACT_R {
        DMACLRACT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock Source Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - PWM output retimed enable"]
    #[inline(always)]
    pub fn retimeen(&self) -> RETIMEEN_R {
        RETIMEEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disable Timer Start/Stop/Reload output"]
    #[inline(always)]
    pub fn dissyncout(&self) -> DISSYNCOUT_R {
        DISSYNCOUT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Always Track Inputs"]
    #[inline(always)]
    pub fn ati(&self) -> ATI_R {
        ATI_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reload-Start Sets COIST"]
    #[inline(always)]
    pub fn rsscoist(&self) -> RSSCOIST_R {
        RSSCOIST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:27 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<3> {
        SYNC_W::new(self)
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osmen(&mut self) -> OSMEN_W<4> {
        OSMEN_W::new(self)
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn qdm(&mut self) -> QDM_W<5> {
        QDM_W::new(self)
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<6> {
        DEBUGRUN_W::new(self)
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline(always)]
    #[must_use]
    pub fn dmaclract(&mut self) -> DMACLRACT_W<7> {
        DMACLRACT_W::new(self)
    }
    #[doc = "Bits 8:9 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<8> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 10 - PWM output retimed enable"]
    #[inline(always)]
    #[must_use]
    pub fn retimeen(&mut self) -> RETIMEEN_W<10> {
        RETIMEEN_W::new(self)
    }
    #[doc = "Bit 11 - Disable Timer Start/Stop/Reload output"]
    #[inline(always)]
    #[must_use]
    pub fn dissyncout(&mut self) -> DISSYNCOUT_W<11> {
        DISSYNCOUT_W::new(self)
    }
    #[doc = "Bit 16 - Always Track Inputs"]
    #[inline(always)]
    #[must_use]
    pub fn ati(&mut self) -> ATI_W<16> {
        ATI_W::new(self)
    }
    #[doc = "Bit 17 - Reload-Start Sets COIST"]
    #[inline(always)]
    #[must_use]
    pub fn rsscoist(&mut self) -> RSSCOIST_W<17> {
        RSSCOIST_W::new(self)
    }
    #[doc = "Bits 18:27 - Prescaler Setting"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<18> {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
