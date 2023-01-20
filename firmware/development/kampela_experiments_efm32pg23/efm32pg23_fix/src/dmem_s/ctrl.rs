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
#[doc = "Field `ECCEN` reader - Enable ECC functionality"]
pub type ECCEN_R = crate::BitReader<bool>;
#[doc = "Field `ECCEN` writer - Enable ECC functionality"]
pub type ECCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ECCWEN` reader - Enable ECC syndrome writes"]
pub type ECCWEN_R = crate::BitReader<bool>;
#[doc = "Field `ECCWEN` writer - Enable ECC syndrome writes"]
pub type ECCWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ECCERRFAULTEN` reader - ECC Error bus fault enable"]
pub type ECCERRFAULTEN_R = crate::BitReader<bool>;
#[doc = "Field `ECCERRFAULTEN` writer - ECC Error bus fault enable"]
pub type ECCERRFAULTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `AHBPORTPRIORITY` reader - AHB port arbitration priority"]
pub type AHBPORTPRIORITY_R = crate::FieldReader<u8, AHBPORTPRIORITY_A>;
#[doc = "AHB port arbitration priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AHBPORTPRIORITY_A {
    #[doc = "0: No AHB port have raised priority."]
    NONE = 0,
    #[doc = "1: AHB port 0 has raised priority."]
    PORT0 = 1,
    #[doc = "2: AHB port 1 has raised priority."]
    PORT1 = 2,
}
impl From<AHBPORTPRIORITY_A> for u8 {
    #[inline(always)]
    fn from(variant: AHBPORTPRIORITY_A) -> Self {
        variant as _
    }
}
impl AHBPORTPRIORITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AHBPORTPRIORITY_A> {
        match self.bits {
            0 => Some(AHBPORTPRIORITY_A::NONE),
            1 => Some(AHBPORTPRIORITY_A::PORT0),
            2 => Some(AHBPORTPRIORITY_A::PORT1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AHBPORTPRIORITY_A::NONE
    }
    #[doc = "Checks if the value of the field is `PORT0`"]
    #[inline(always)]
    pub fn is_port0(&self) -> bool {
        *self == AHBPORTPRIORITY_A::PORT0
    }
    #[doc = "Checks if the value of the field is `PORT1`"]
    #[inline(always)]
    pub fn is_port1(&self) -> bool {
        *self == AHBPORTPRIORITY_A::PORT1
    }
}
#[doc = "Field `AHBPORTPRIORITY` writer - AHB port arbitration priority"]
pub type AHBPORTPRIORITY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, AHBPORTPRIORITY_A, 3, O>;
impl<'a, const O: u8> AHBPORTPRIORITY_W<'a, O> {
    #[doc = "No AHB port have raised priority."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(AHBPORTPRIORITY_A::NONE)
    }
    #[doc = "AHB port 0 has raised priority."]
    #[inline(always)]
    pub fn port0(self) -> &'a mut W {
        self.variant(AHBPORTPRIORITY_A::PORT0)
    }
    #[doc = "AHB port 1 has raised priority."]
    #[inline(always)]
    pub fn port1(self) -> &'a mut W {
        self.variant(AHBPORTPRIORITY_A::PORT1)
    }
}
#[doc = "Field `ADDRFAULTEN` reader - Address fault bus fault enable"]
pub type ADDRFAULTEN_R = crate::BitReader<bool>;
#[doc = "Field `ADDRFAULTEN` writer - Address fault bus fault enable"]
pub type ADDRFAULTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable ECC functionality"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable ECC syndrome writes"]
    #[inline(always)]
    pub fn eccwen(&self) -> ECCWEN_R {
        ECCWEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECC Error bus fault enable"]
    #[inline(always)]
    pub fn eccerrfaulten(&self) -> ECCERRFAULTEN_R {
        ECCERRFAULTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - AHB port arbitration priority"]
    #[inline(always)]
    pub fn ahbportpriority(&self) -> AHBPORTPRIORITY_R {
        AHBPORTPRIORITY_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Address fault bus fault enable"]
    #[inline(always)]
    pub fn addrfaulten(&self) -> ADDRFAULTEN_R {
        ADDRFAULTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ECC functionality"]
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<0> {
        ECCEN_W::new(self)
    }
    #[doc = "Bit 1 - Enable ECC syndrome writes"]
    #[inline(always)]
    #[must_use]
    pub fn eccwen(&mut self) -> ECCWEN_W<1> {
        ECCWEN_W::new(self)
    }
    #[doc = "Bit 2 - ECC Error bus fault enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccerrfaulten(&mut self) -> ECCERRFAULTEN_W<2> {
        ECCERRFAULTEN_W::new(self)
    }
    #[doc = "Bits 3:5 - AHB port arbitration priority"]
    #[inline(always)]
    #[must_use]
    pub fn ahbportpriority(&mut self) -> AHBPORTPRIORITY_W<3> {
        AHBPORTPRIORITY_W::new(self)
    }
    #[doc = "Bit 6 - Address fault bus fault enable"]
    #[inline(always)]
    #[must_use]
    pub fn addrfaulten(&mut self) -> ADDRFAULTEN_W<6> {
        ADDRFAULTEN_W::new(self)
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
#[doc = "`reset()` method sets CTRL to value 0x40"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
