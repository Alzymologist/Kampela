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
#[doc = "Field `BUFOUTFREEZE` reader - Freeze BUFOUT Controls"]
pub type BUFOUTFREEZE_R = crate::BitReader<bool>;
#[doc = "Field `BUFOUTFREEZE` writer - Freeze BUFOUT Controls"]
pub type BUFOUTFREEZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `KEEPWARM` reader - Keep Warm"]
pub type KEEPWARM_R = crate::BitReader<bool>;
#[doc = "Field `KEEPWARM` writer - Keep Warm"]
pub type KEEPWARM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EM23ONDEMAND` reader - On-demand During EM23"]
pub type EM23ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `EM23ONDEMAND` writer - On-demand During EM23"]
pub type EM23ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FORCEXI2GNDANA` reader - Force XI Pin to Ground"]
pub type FORCEXI2GNDANA_R = crate::BitReader<FORCEXI2GNDANA_A>;
#[doc = "Force XI Pin to Ground\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCEXI2GNDANA_A {
    #[doc = "0: Disabled (not pulled)"]
    DISABLE = 0,
    #[doc = "1: Enabled (pulled)"]
    ENABLE = 1,
}
impl From<FORCEXI2GNDANA_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEXI2GNDANA_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCEXI2GNDANA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEXI2GNDANA_A {
        match self.bits {
            false => FORCEXI2GNDANA_A::DISABLE,
            true => FORCEXI2GNDANA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FORCEXI2GNDANA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FORCEXI2GNDANA_A::ENABLE
    }
}
#[doc = "Field `FORCEXI2GNDANA` writer - Force XI Pin to Ground"]
pub type FORCEXI2GNDANA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SPEC, FORCEXI2GNDANA_A, O>;
impl<'a, const O: u8> FORCEXI2GNDANA_W<'a, O> {
    #[doc = "Disabled (not pulled)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FORCEXI2GNDANA_A::DISABLE)
    }
    #[doc = "Enabled (pulled)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FORCEXI2GNDANA_A::ENABLE)
    }
}
#[doc = "Field `FORCEXO2GNDANA` reader - Force XO Pin to Ground"]
pub type FORCEXO2GNDANA_R = crate::BitReader<FORCEXO2GNDANA_A>;
#[doc = "Force XO Pin to Ground\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCEXO2GNDANA_A {
    #[doc = "0: Disabled (not pulled)"]
    DISABLE = 0,
    #[doc = "1: Enabled (pulled)"]
    ENABLE = 1,
}
impl From<FORCEXO2GNDANA_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEXO2GNDANA_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCEXO2GNDANA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEXO2GNDANA_A {
        match self.bits {
            false => FORCEXO2GNDANA_A::DISABLE,
            true => FORCEXO2GNDANA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FORCEXO2GNDANA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FORCEXO2GNDANA_A::ENABLE
    }
}
#[doc = "Field `FORCEXO2GNDANA` writer - Force XO Pin to Ground"]
pub type FORCEXO2GNDANA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SPEC, FORCEXO2GNDANA_A, O>;
impl<'a, const O: u8> FORCEXO2GNDANA_W<'a, O> {
    #[doc = "Disabled (not pulled)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FORCEXO2GNDANA_A::DISABLE)
    }
    #[doc = "Enabled (pulled)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FORCEXO2GNDANA_A::ENABLE)
    }
}
#[doc = "Field `FORCECTUNEMAX` reader - Force Tuning Cap to Max Value"]
pub type FORCECTUNEMAX_R = crate::BitReader<bool>;
#[doc = "Field `FORCECTUNEMAX` writer - Force Tuning Cap to Max Value"]
pub type FORCECTUNEMAX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PRSSTATUSSEL0` reader - PRS Status 0 Output Select"]
pub type PRSSTATUSSEL0_R = crate::FieldReader<u8, PRSSTATUSSEL0_A>;
#[doc = "PRS Status 0 Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTATUSSEL0_A {
    #[doc = "0: PRS mux outputs 0"]
    DISABLED = 0,
    #[doc = "1: PRS mux outputs enabled status"]
    ENS = 1,
    #[doc = "2: PRS mux outputs core bias optimization ready status"]
    COREBIASOPTRDY = 2,
    #[doc = "3: PRS mux outputs ready status"]
    RDY = 3,
    #[doc = "4: PRS mux outputs PRS ready status"]
    PRSRDY = 4,
    #[doc = "5: PRS mux outputs BUFOUT ready status"]
    BUFOUTRDY = 5,
    #[doc = "8: PRS mux outputs oscillator requested by digital clock status"]
    HWREQ = 8,
    #[doc = "9: PRS mux outputs oscillator requested by PRS request status"]
    PRSHWREQ = 9,
    #[doc = "10: PRS mux outputs oscillator requested by BUFOUT request status"]
    BUFOUTHWREQ = 10,
}
impl From<PRSSTATUSSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTATUSSEL0_A) -> Self {
        variant as _
    }
}
impl PRSSTATUSSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSSTATUSSEL0_A> {
        match self.bits {
            0 => Some(PRSSTATUSSEL0_A::DISABLED),
            1 => Some(PRSSTATUSSEL0_A::ENS),
            2 => Some(PRSSTATUSSEL0_A::COREBIASOPTRDY),
            3 => Some(PRSSTATUSSEL0_A::RDY),
            4 => Some(PRSSTATUSSEL0_A::PRSRDY),
            5 => Some(PRSSTATUSSEL0_A::BUFOUTRDY),
            8 => Some(PRSSTATUSSEL0_A::HWREQ),
            9 => Some(PRSSTATUSSEL0_A::PRSHWREQ),
            10 => Some(PRSSTATUSSEL0_A::BUFOUTHWREQ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRSSTATUSSEL0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENS`"]
    #[inline(always)]
    pub fn is_ens(&self) -> bool {
        *self == PRSSTATUSSEL0_A::ENS
    }
    #[doc = "Checks if the value of the field is `COREBIASOPTRDY`"]
    #[inline(always)]
    pub fn is_corebiasoptrdy(&self) -> bool {
        *self == PRSSTATUSSEL0_A::COREBIASOPTRDY
    }
    #[doc = "Checks if the value of the field is `RDY`"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == PRSSTATUSSEL0_A::RDY
    }
    #[doc = "Checks if the value of the field is `PRSRDY`"]
    #[inline(always)]
    pub fn is_prsrdy(&self) -> bool {
        *self == PRSSTATUSSEL0_A::PRSRDY
    }
    #[doc = "Checks if the value of the field is `BUFOUTRDY`"]
    #[inline(always)]
    pub fn is_bufoutrdy(&self) -> bool {
        *self == PRSSTATUSSEL0_A::BUFOUTRDY
    }
    #[doc = "Checks if the value of the field is `HWREQ`"]
    #[inline(always)]
    pub fn is_hwreq(&self) -> bool {
        *self == PRSSTATUSSEL0_A::HWREQ
    }
    #[doc = "Checks if the value of the field is `PRSHWREQ`"]
    #[inline(always)]
    pub fn is_prshwreq(&self) -> bool {
        *self == PRSSTATUSSEL0_A::PRSHWREQ
    }
    #[doc = "Checks if the value of the field is `BUFOUTHWREQ`"]
    #[inline(always)]
    pub fn is_bufouthwreq(&self) -> bool {
        *self == PRSSTATUSSEL0_A::BUFOUTHWREQ
    }
}
#[doc = "Field `PRSSTATUSSEL0` writer - PRS Status 0 Output Select"]
pub type PRSSTATUSSEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, PRSSTATUSSEL0_A, 4, O>;
impl<'a, const O: u8> PRSSTATUSSEL0_W<'a, O> {
    #[doc = "PRS mux outputs 0"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL0_A::DISABLED)
    }
    #[doc = "PRS mux outputs enabled status"]
    #[inline(always)]
    pub fn ens(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL0_A::ENS)
    }
    #[doc = "PRS mux outputs core bias optimization ready status"]
    #[inline(always)]
    pub fn corebiasoptrdy(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL0_A::COREBIASOPTRDY)
    }
    #[doc = "PRS mux outputs ready status"]
    #[inline(always)]
    pub fn rdy(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL0_A::RDY)
    }
    #[doc = "PRS mux outputs PRS ready status"]
    #[inline(always)]
    pub fn prsrdy(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL0_A::PRSRDY)
    }
    #[doc = "PRS mux outputs BUFOUT ready status"]
    #[inline(always)]
    pub fn bufoutrdy(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL0_A::BUFOUTRDY)
    }
    #[doc = "PRS mux outputs oscillator requested by digital clock status"]
    #[inline(always)]
    pub fn hwreq(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL0_A::HWREQ)
    }
    #[doc = "PRS mux outputs oscillator requested by PRS request status"]
    #[inline(always)]
    pub fn prshwreq(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL0_A::PRSHWREQ)
    }
    #[doc = "PRS mux outputs oscillator requested by BUFOUT request status"]
    #[inline(always)]
    pub fn bufouthwreq(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL0_A::BUFOUTHWREQ)
    }
}
#[doc = "Field `PRSSTATUSSEL1` reader - PRS Status 1 Output Select"]
pub type PRSSTATUSSEL1_R = crate::FieldReader<u8, PRSSTATUSSEL1_A>;
#[doc = "PRS Status 1 Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTATUSSEL1_A {
    #[doc = "0: PRS mux outputs 0"]
    DISABLED = 0,
    #[doc = "1: PRS mux outputs enabled status"]
    ENS = 1,
    #[doc = "2: PRS mux outputs core bias optimization ready status"]
    COREBIASOPTRDY = 2,
    #[doc = "3: PRS mux outputs ready status"]
    RDY = 3,
    #[doc = "4: PRS mux outputs PRS ready status"]
    PRSRDY = 4,
    #[doc = "5: PRS mux outputs BUFOUT ready status"]
    BUFOUTRDY = 5,
    #[doc = "8: PRS mux outputs oscillator requested by digital clock status"]
    HWREQ = 8,
    #[doc = "9: PRS mux outputs oscillator requested by PRS request status"]
    PRSHWREQ = 9,
    #[doc = "10: PRS mux outputs oscillator requested by BUFOUT request status"]
    BUFOUTHWREQ = 10,
}
impl From<PRSSTATUSSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTATUSSEL1_A) -> Self {
        variant as _
    }
}
impl PRSSTATUSSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSSTATUSSEL1_A> {
        match self.bits {
            0 => Some(PRSSTATUSSEL1_A::DISABLED),
            1 => Some(PRSSTATUSSEL1_A::ENS),
            2 => Some(PRSSTATUSSEL1_A::COREBIASOPTRDY),
            3 => Some(PRSSTATUSSEL1_A::RDY),
            4 => Some(PRSSTATUSSEL1_A::PRSRDY),
            5 => Some(PRSSTATUSSEL1_A::BUFOUTRDY),
            8 => Some(PRSSTATUSSEL1_A::HWREQ),
            9 => Some(PRSSTATUSSEL1_A::PRSHWREQ),
            10 => Some(PRSSTATUSSEL1_A::BUFOUTHWREQ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRSSTATUSSEL1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENS`"]
    #[inline(always)]
    pub fn is_ens(&self) -> bool {
        *self == PRSSTATUSSEL1_A::ENS
    }
    #[doc = "Checks if the value of the field is `COREBIASOPTRDY`"]
    #[inline(always)]
    pub fn is_corebiasoptrdy(&self) -> bool {
        *self == PRSSTATUSSEL1_A::COREBIASOPTRDY
    }
    #[doc = "Checks if the value of the field is `RDY`"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == PRSSTATUSSEL1_A::RDY
    }
    #[doc = "Checks if the value of the field is `PRSRDY`"]
    #[inline(always)]
    pub fn is_prsrdy(&self) -> bool {
        *self == PRSSTATUSSEL1_A::PRSRDY
    }
    #[doc = "Checks if the value of the field is `BUFOUTRDY`"]
    #[inline(always)]
    pub fn is_bufoutrdy(&self) -> bool {
        *self == PRSSTATUSSEL1_A::BUFOUTRDY
    }
    #[doc = "Checks if the value of the field is `HWREQ`"]
    #[inline(always)]
    pub fn is_hwreq(&self) -> bool {
        *self == PRSSTATUSSEL1_A::HWREQ
    }
    #[doc = "Checks if the value of the field is `PRSHWREQ`"]
    #[inline(always)]
    pub fn is_prshwreq(&self) -> bool {
        *self == PRSSTATUSSEL1_A::PRSHWREQ
    }
    #[doc = "Checks if the value of the field is `BUFOUTHWREQ`"]
    #[inline(always)]
    pub fn is_bufouthwreq(&self) -> bool {
        *self == PRSSTATUSSEL1_A::BUFOUTHWREQ
    }
}
#[doc = "Field `PRSSTATUSSEL1` writer - PRS Status 1 Output Select"]
pub type PRSSTATUSSEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, PRSSTATUSSEL1_A, 4, O>;
impl<'a, const O: u8> PRSSTATUSSEL1_W<'a, O> {
    #[doc = "PRS mux outputs 0"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL1_A::DISABLED)
    }
    #[doc = "PRS mux outputs enabled status"]
    #[inline(always)]
    pub fn ens(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL1_A::ENS)
    }
    #[doc = "PRS mux outputs core bias optimization ready status"]
    #[inline(always)]
    pub fn corebiasoptrdy(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL1_A::COREBIASOPTRDY)
    }
    #[doc = "PRS mux outputs ready status"]
    #[inline(always)]
    pub fn rdy(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL1_A::RDY)
    }
    #[doc = "PRS mux outputs PRS ready status"]
    #[inline(always)]
    pub fn prsrdy(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL1_A::PRSRDY)
    }
    #[doc = "PRS mux outputs BUFOUT ready status"]
    #[inline(always)]
    pub fn bufoutrdy(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL1_A::BUFOUTRDY)
    }
    #[doc = "PRS mux outputs oscillator requested by digital clock status"]
    #[inline(always)]
    pub fn hwreq(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL1_A::HWREQ)
    }
    #[doc = "PRS mux outputs oscillator requested by PRS request status"]
    #[inline(always)]
    pub fn prshwreq(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL1_A::PRSHWREQ)
    }
    #[doc = "PRS mux outputs oscillator requested by BUFOUT request status"]
    #[inline(always)]
    pub fn bufouthwreq(self) -> &'a mut W {
        self.variant(PRSSTATUSSEL1_A::BUFOUTHWREQ)
    }
}
#[doc = "Field `FORCEEN` reader - Force Digital Clock Request"]
pub type FORCEEN_R = crate::BitReader<bool>;
#[doc = "Field `FORCEEN` writer - Force Digital Clock Request"]
pub type FORCEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FORCEENPRS` reader - Force PRS Oscillator Request"]
pub type FORCEENPRS_R = crate::BitReader<bool>;
#[doc = "Field `FORCEENPRS` writer - Force PRS Oscillator Request"]
pub type FORCEENPRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FORCEENBUFOUT` reader - Force BUFOUT Request"]
pub type FORCEENBUFOUT_R = crate::BitReader<bool>;
#[doc = "Field `FORCEENBUFOUT` writer - Force BUFOUT Request"]
pub type FORCEENBUFOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DISONDEMAND` reader - Disable On-demand For Digital Clock"]
pub type DISONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `DISONDEMAND` writer - Disable On-demand For Digital Clock"]
pub type DISONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DISONDEMANDPRS` reader - Disable On-demand For PRS"]
pub type DISONDEMANDPRS_R = crate::BitReader<bool>;
#[doc = "Field `DISONDEMANDPRS` writer - Disable On-demand For PRS"]
pub type DISONDEMANDPRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DISONDEMANDBUFOUT` reader - Disable On-demand For BUFOUT"]
pub type DISONDEMANDBUFOUT_R = crate::BitReader<bool>;
#[doc = "Field `DISONDEMANDBUFOUT` writer - Disable On-demand For BUFOUT"]
pub type DISONDEMANDBUFOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Freeze BUFOUT Controls"]
    #[inline(always)]
    pub fn bufoutfreeze(&self) -> BUFOUTFREEZE_R {
        BUFOUTFREEZE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Keep Warm"]
    #[inline(always)]
    pub fn keepwarm(&self) -> KEEPWARM_R {
        KEEPWARM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - On-demand During EM23"]
    #[inline(always)]
    pub fn em23ondemand(&self) -> EM23ONDEMAND_R {
        EM23ONDEMAND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force XI Pin to Ground"]
    #[inline(always)]
    pub fn forcexi2gndana(&self) -> FORCEXI2GNDANA_R {
        FORCEXI2GNDANA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force XO Pin to Ground"]
    #[inline(always)]
    pub fn forcexo2gndana(&self) -> FORCEXO2GNDANA_R {
        FORCEXO2GNDANA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Tuning Cap to Max Value"]
    #[inline(always)]
    pub fn forcectunemax(&self) -> FORCECTUNEMAX_R {
        FORCECTUNEMAX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - PRS Status 0 Output Select"]
    #[inline(always)]
    pub fn prsstatussel0(&self) -> PRSSTATUSSEL0_R {
        PRSSTATUSSEL0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PRS Status 1 Output Select"]
    #[inline(always)]
    pub fn prsstatussel1(&self) -> PRSSTATUSSEL1_R {
        PRSSTATUSSEL1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Force Digital Clock Request"]
    #[inline(always)]
    pub fn forceen(&self) -> FORCEEN_R {
        FORCEEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force PRS Oscillator Request"]
    #[inline(always)]
    pub fn forceenprs(&self) -> FORCEENPRS_R {
        FORCEENPRS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Force BUFOUT Request"]
    #[inline(always)]
    pub fn forceenbufout(&self) -> FORCEENBUFOUT_R {
        FORCEENBUFOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable On-demand For Digital Clock"]
    #[inline(always)]
    pub fn disondemand(&self) -> DISONDEMAND_R {
        DISONDEMAND_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Disable On-demand For PRS"]
    #[inline(always)]
    pub fn disondemandprs(&self) -> DISONDEMANDPRS_R {
        DISONDEMANDPRS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable On-demand For BUFOUT"]
    #[inline(always)]
    pub fn disondemandbufout(&self) -> DISONDEMANDBUFOUT_R {
        DISONDEMANDBUFOUT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Freeze BUFOUT Controls"]
    #[inline(always)]
    #[must_use]
    pub fn bufoutfreeze(&mut self) -> BUFOUTFREEZE_W<0> {
        BUFOUTFREEZE_W::new(self)
    }
    #[doc = "Bit 2 - Keep Warm"]
    #[inline(always)]
    #[must_use]
    pub fn keepwarm(&mut self) -> KEEPWARM_W<2> {
        KEEPWARM_W::new(self)
    }
    #[doc = "Bit 3 - On-demand During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn em23ondemand(&mut self) -> EM23ONDEMAND_W<3> {
        EM23ONDEMAND_W::new(self)
    }
    #[doc = "Bit 4 - Force XI Pin to Ground"]
    #[inline(always)]
    #[must_use]
    pub fn forcexi2gndana(&mut self) -> FORCEXI2GNDANA_W<4> {
        FORCEXI2GNDANA_W::new(self)
    }
    #[doc = "Bit 5 - Force XO Pin to Ground"]
    #[inline(always)]
    #[must_use]
    pub fn forcexo2gndana(&mut self) -> FORCEXO2GNDANA_W<5> {
        FORCEXO2GNDANA_W::new(self)
    }
    #[doc = "Bit 6 - Force Tuning Cap to Max Value"]
    #[inline(always)]
    #[must_use]
    pub fn forcectunemax(&mut self) -> FORCECTUNEMAX_W<6> {
        FORCECTUNEMAX_W::new(self)
    }
    #[doc = "Bits 8:11 - PRS Status 0 Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn prsstatussel0(&mut self) -> PRSSTATUSSEL0_W<8> {
        PRSSTATUSSEL0_W::new(self)
    }
    #[doc = "Bits 12:15 - PRS Status 1 Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn prsstatussel1(&mut self) -> PRSSTATUSSEL1_W<12> {
        PRSSTATUSSEL1_W::new(self)
    }
    #[doc = "Bit 16 - Force Digital Clock Request"]
    #[inline(always)]
    #[must_use]
    pub fn forceen(&mut self) -> FORCEEN_W<16> {
        FORCEEN_W::new(self)
    }
    #[doc = "Bit 17 - Force PRS Oscillator Request"]
    #[inline(always)]
    #[must_use]
    pub fn forceenprs(&mut self) -> FORCEENPRS_W<17> {
        FORCEENPRS_W::new(self)
    }
    #[doc = "Bit 18 - Force BUFOUT Request"]
    #[inline(always)]
    #[must_use]
    pub fn forceenbufout(&mut self) -> FORCEENBUFOUT_W<18> {
        FORCEENBUFOUT_W::new(self)
    }
    #[doc = "Bit 24 - Disable On-demand For Digital Clock"]
    #[inline(always)]
    #[must_use]
    pub fn disondemand(&mut self) -> DISONDEMAND_W<24> {
        DISONDEMAND_W::new(self)
    }
    #[doc = "Bit 25 - Disable On-demand For PRS"]
    #[inline(always)]
    #[must_use]
    pub fn disondemandprs(&mut self) -> DISONDEMANDPRS_W<25> {
        DISONDEMANDPRS_W::new(self)
    }
    #[doc = "Bit 26 - Disable On-demand For BUFOUT"]
    #[inline(always)]
    #[must_use]
    pub fn disondemandbufout(&mut self) -> DISONDEMANDBUFOUT_W<26> {
        DISONDEMANDBUFOUT_W::new(self)
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
#[doc = "`reset()` method sets CTRL to value 0x0700_0040"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0700_0040;
}
