#[doc = "Register `WRITECMD` writer"]
pub struct W(crate::W<WRITECMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITECMD_SPEC>;
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
impl From<crate::W<WRITECMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITECMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASEPAGE` writer - Erase Page"]
pub type ERASEPAGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `WRITEEND` writer - End Write Mode"]
pub type WRITEEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `ERASERANGE` writer - Erase range of pages"]
pub type ERASERANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `ERASEABORT` writer - Abort erase sequence"]
pub type ERASEABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `ERASEMAIN0` writer - Mass erase region 0"]
pub type ERASEMAIN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `CLEARWDATA` writer - Clear WDATA state"]
pub type CLEARWDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 1 - Erase Page"]
    #[inline(always)]
    #[must_use]
    pub fn erasepage(&mut self) -> ERASEPAGE_W<1> {
        ERASEPAGE_W::new(self)
    }
    #[doc = "Bit 2 - End Write Mode"]
    #[inline(always)]
    #[must_use]
    pub fn writeend(&mut self) -> WRITEEND_W<2> {
        WRITEEND_W::new(self)
    }
    #[doc = "Bit 4 - Erase range of pages"]
    #[inline(always)]
    #[must_use]
    pub fn eraserange(&mut self) -> ERASERANGE_W<4> {
        ERASERANGE_W::new(self)
    }
    #[doc = "Bit 5 - Abort erase sequence"]
    #[inline(always)]
    #[must_use]
    pub fn eraseabort(&mut self) -> ERASEABORT_W<5> {
        ERASEABORT_W::new(self)
    }
    #[doc = "Bit 8 - Mass erase region 0"]
    #[inline(always)]
    #[must_use]
    pub fn erasemain0(&mut self) -> ERASEMAIN0_W<8> {
        ERASEMAIN0_W::new(self)
    }
    #[doc = "Bit 12 - Clear WDATA state"]
    #[inline(always)]
    #[must_use]
    pub fn clearwdata(&mut self) -> CLEARWDATA_W<12> {
        CLEARWDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writecmd](index.html) module"]
pub struct WRITECMD_SPEC;
impl crate::RegisterSpec for WRITECMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [writecmd::W](W) writer structure"]
impl crate::Writable for WRITECMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRITECMD to value 0"]
impl crate::Resettable for WRITECMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
