#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MBOXIF0` reader - Mailbox Interupt Flag"]
pub type MBOXIF0_R = crate::BitReader<bool>;
#[doc = "Field `MBOXIF0` writer - Mailbox Interupt Flag"]
pub type MBOXIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `MBOXIF1` reader - Mailbox Interupt Flag"]
pub type MBOXIF1_R = crate::BitReader<bool>;
#[doc = "Field `MBOXIF1` writer - Mailbox Interupt Flag"]
pub type MBOXIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `MBOXIF2` reader - Mailbox Interupt Flag"]
pub type MBOXIF2_R = crate::BitReader<bool>;
#[doc = "Field `MBOXIF2` writer - Mailbox Interupt Flag"]
pub type MBOXIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `MBOXIF3` reader - Mailbox Interupt Flag"]
pub type MBOXIF3_R = crate::BitReader<bool>;
#[doc = "Field `MBOXIF3` writer - Mailbox Interupt Flag"]
pub type MBOXIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Mailbox Interupt Flag"]
    #[inline(always)]
    pub fn mboxif0(&self) -> MBOXIF0_R {
        MBOXIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox Interupt Flag"]
    #[inline(always)]
    pub fn mboxif1(&self) -> MBOXIF1_R {
        MBOXIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox Interupt Flag"]
    #[inline(always)]
    pub fn mboxif2(&self) -> MBOXIF2_R {
        MBOXIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox Interupt Flag"]
    #[inline(always)]
    pub fn mboxif3(&self) -> MBOXIF3_R {
        MBOXIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mailbox Interupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mboxif0(&mut self) -> MBOXIF0_W<0> {
        MBOXIF0_W::new(self)
    }
    #[doc = "Bit 1 - Mailbox Interupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mboxif1(&mut self) -> MBOXIF1_W<1> {
        MBOXIF1_W::new(self)
    }
    #[doc = "Bit 2 - Mailbox Interupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mboxif2(&mut self) -> MBOXIF2_W<2> {
        MBOXIF2_W::new(self)
    }
    #[doc = "Bit 3 - Mailbox Interupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mboxif3(&mut self) -> MBOXIF3_W<3> {
        MBOXIF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
