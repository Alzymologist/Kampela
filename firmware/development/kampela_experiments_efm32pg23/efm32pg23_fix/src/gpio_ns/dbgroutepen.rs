#[doc = "Register `DBGROUTEPEN` reader"]
pub struct R(crate::R<DBGROUTEPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGROUTEPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGROUTEPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGROUTEPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGROUTEPEN` writer"]
pub struct W(crate::W<DBGROUTEPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGROUTEPEN_SPEC>;
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
impl From<crate::W<DBGROUTEPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGROUTEPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWCLKTCKPEN` reader - Route Pin Enable"]
pub type SWCLKTCKPEN_R = crate::BitReader<bool>;
#[doc = "Field `SWCLKTCKPEN` writer - Route Pin Enable"]
pub type SWCLKTCKPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBGROUTEPEN_SPEC, bool, O>;
#[doc = "Field `SWDIOTMSPEN` reader - Route Location 0"]
pub type SWDIOTMSPEN_R = crate::BitReader<bool>;
#[doc = "Field `SWDIOTMSPEN` writer - Route Location 0"]
pub type SWDIOTMSPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBGROUTEPEN_SPEC, bool, O>;
#[doc = "Field `TDOPEN` reader - JTAG Test Debug Output Pin Enable"]
pub type TDOPEN_R = crate::BitReader<bool>;
#[doc = "Field `TDOPEN` writer - JTAG Test Debug Output Pin Enable"]
pub type TDOPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBGROUTEPEN_SPEC, bool, O>;
#[doc = "Field `TDIPEN` reader - JTAG Test Debug Input Pin Enable"]
pub type TDIPEN_R = crate::BitReader<bool>;
#[doc = "Field `TDIPEN` writer - JTAG Test Debug Input Pin Enable"]
pub type TDIPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBGROUTEPEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Route Pin Enable"]
    #[inline(always)]
    pub fn swclktckpen(&self) -> SWCLKTCKPEN_R {
        SWCLKTCKPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Route Location 0"]
    #[inline(always)]
    pub fn swdiotmspen(&self) -> SWDIOTMSPEN_R {
        SWDIOTMSPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    pub fn tdopen(&self) -> TDOPEN_R {
        TDOPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    pub fn tdipen(&self) -> TDIPEN_R {
        TDIPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Route Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swclktckpen(&mut self) -> SWCLKTCKPEN_W<0> {
        SWCLKTCKPEN_W::new(self)
    }
    #[doc = "Bit 1 - Route Location 0"]
    #[inline(always)]
    #[must_use]
    pub fn swdiotmspen(&mut self) -> SWDIOTMSPEN_W<1> {
        SWDIOTMSPEN_W::new(self)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdopen(&mut self) -> TDOPEN_W<2> {
        TDOPEN_W::new(self)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdipen(&mut self) -> TDIPEN_W<3> {
        TDIPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgroutepen](index.html) module"]
pub struct DBGROUTEPEN_SPEC;
impl crate::RegisterSpec for DBGROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgroutepen::R](R) reader structure"]
impl crate::Readable for DBGROUTEPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgroutepen::W](W) writer structure"]
impl crate::Writable for DBGROUTEPEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGROUTEPEN to value 0x0f"]
impl crate::Resettable for DBGROUTEPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
