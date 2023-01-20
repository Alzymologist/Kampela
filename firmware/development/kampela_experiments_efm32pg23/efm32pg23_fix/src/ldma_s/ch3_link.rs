#[doc = "Register `CH3_LINK` reader"]
pub struct R(crate::R<CH3_LINK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3_LINK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3_LINK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3_LINK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3_LINK` writer"]
pub struct W(crate::W<CH3_LINK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3_LINK_SPEC>;
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
impl From<crate::W<CH3_LINK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3_LINK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINKMODE` reader - Link Structure Addressing Mode"]
pub type LINKMODE_R = crate::BitReader<LINKMODE_A>;
#[doc = "Link Structure Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINKMODE_A {
    #[doc = "0: The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
    ABSOLUTE = 0,
    #[doc = "1: The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
    RELATIVE = 1,
}
impl From<LINKMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LINKMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LINKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINKMODE_A {
        match self.bits {
            false => LINKMODE_A::ABSOLUTE,
            true => LINKMODE_A::RELATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ABSOLUTE`"]
    #[inline(always)]
    pub fn is_absolute(&self) -> bool {
        *self == LINKMODE_A::ABSOLUTE
    }
    #[doc = "Checks if the value of the field is `RELATIVE`"]
    #[inline(always)]
    pub fn is_relative(&self) -> bool {
        *self == LINKMODE_A::RELATIVE
    }
}
#[doc = "Field `LINK` reader - Link Next Structure"]
pub type LINK_R = crate::BitReader<bool>;
#[doc = "Field `LINK` writer - Link Next Structure"]
pub type LINK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH3_LINK_SPEC, bool, O>;
#[doc = "Field `LINKADDR` reader - Link Structure Address"]
pub type LINKADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LINKADDR` writer - Link Structure Address"]
pub type LINKADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH3_LINK_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - Link Structure Addressing Mode"]
    #[inline(always)]
    pub fn linkmode(&self) -> LINKMODE_R {
        LINKMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Link Next Structure"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Link Structure Address"]
    #[inline(always)]
    pub fn linkaddr(&self) -> LINKADDR_R {
        LINKADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 1 - Link Next Structure"]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LINK_W<1> {
        LINK_W::new(self)
    }
    #[doc = "Bits 2:31 - Link Structure Address"]
    #[inline(always)]
    #[must_use]
    pub fn linkaddr(&mut self) -> LINKADDR_W<2> {
        LINKADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_link](index.html) module"]
pub struct CH3_LINK_SPEC;
impl crate::RegisterSpec for CH3_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3_link::R](R) reader structure"]
impl crate::Readable for CH3_LINK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3_link::W](W) writer structure"]
impl crate::Writable for CH3_LINK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3_LINK to value 0"]
impl crate::Resettable for CH3_LINK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
