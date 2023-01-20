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
#[doc = "Field `ADDRFAULTEN` reader - Invalid Address Bus Fault Response Enabl"]
pub type ADDRFAULTEN_R = crate::BitReader<bool>;
#[doc = "Field `ADDRFAULTEN` writer - Invalid Address Bus Fault Response Enabl"]
pub type ADDRFAULTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CLKDISFAULTEN` reader - Disabled Clkbus Bus Fault Enable"]
pub type CLKDISFAULTEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKDISFAULTEN` writer - Disabled Clkbus Bus Fault Enable"]
pub type CLKDISFAULTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RAMECCERRFAULTEN` reader - Two bit ECC error bus fault response ena"]
pub type RAMECCERRFAULTEN_R = crate::BitReader<bool>;
#[doc = "Field `RAMECCERRFAULTEN` writer - Two bit ECC error bus fault response ena"]
pub type RAMECCERRFAULTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enabl"]
    #[inline(always)]
    pub fn addrfaulten(&self) -> ADDRFAULTEN_R {
        ADDRFAULTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disabled Clkbus Bus Fault Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&self) -> CLKDISFAULTEN_R {
        CLKDISFAULTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Two bit ECC error bus fault response ena"]
    #[inline(always)]
    pub fn rameccerrfaulten(&self) -> RAMECCERRFAULTEN_R {
        RAMECCERRFAULTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enabl"]
    #[inline(always)]
    #[must_use]
    pub fn addrfaulten(&mut self) -> ADDRFAULTEN_W<0> {
        ADDRFAULTEN_W::new(self)
    }
    #[doc = "Bit 1 - Disabled Clkbus Bus Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkdisfaulten(&mut self) -> CLKDISFAULTEN_W<1> {
        CLKDISFAULTEN_W::new(self)
    }
    #[doc = "Bit 5 - Two bit ECC error bus fault response ena"]
    #[inline(always)]
    #[must_use]
    pub fn rameccerrfaulten(&mut self) -> RAMECCERRFAULTEN_W<5> {
        RAMECCERRFAULTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure to provide general RAM configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0x23"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x23;
}
