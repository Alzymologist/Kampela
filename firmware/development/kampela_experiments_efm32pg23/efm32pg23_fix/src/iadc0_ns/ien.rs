#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINGLEFIFODVL` reader - Single FIFO Data Valid Level Enable"]
pub type SINGLEFIFODVL_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEFIFODVL` writer - Single FIFO Data Valid Level Enable"]
pub type SINGLEFIFODVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SCANFIFODVL` reader - Scan FIFO Data Valid Level Enable"]
pub type SCANFIFODVL_R = crate::BitReader<bool>;
#[doc = "Field `SCANFIFODVL` writer - Scan FIFO Data Valid Level Enable"]
pub type SCANFIFODVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SINGLECMP` reader - Single Result Window Compare Enable"]
pub type SINGLECMP_R = crate::BitReader<bool>;
#[doc = "Field `SINGLECMP` writer - Single Result Window Compare Enable"]
pub type SINGLECMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SCANCMP` reader - Scan Result Window Compare Enable"]
pub type SCANCMP_R = crate::BitReader<bool>;
#[doc = "Field `SCANCMP` writer - Scan Result Window Compare Enable"]
pub type SCANCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SCANENTRYDONE` reader - Scan Entry Done Enable"]
pub type SCANENTRYDONE_R = crate::BitReader<bool>;
#[doc = "Field `SCANENTRYDONE` writer - Scan Entry Done Enable"]
pub type SCANENTRYDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SCANTABLEDONE` reader - Scan Table Done Enable"]
pub type SCANTABLEDONE_R = crate::BitReader<bool>;
#[doc = "Field `SCANTABLEDONE` writer - Scan Table Done Enable"]
pub type SCANTABLEDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SINGLEDONE` reader - Single Conversion Done Enable"]
pub type SINGLEDONE_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEDONE` writer - Single Conversion Done Enable"]
pub type SINGLEDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `POLARITYERR` reader - Polarity Error Enable"]
pub type POLARITYERR_R = crate::BitReader<bool>;
#[doc = "Field `POLARITYERR` writer - Polarity Error Enable"]
pub type POLARITYERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `PORTALLOCERR` reader - Port Allocation Error Enable"]
pub type PORTALLOCERR_R = crate::BitReader<bool>;
#[doc = "Field `PORTALLOCERR` writer - Port Allocation Error Enable"]
pub type PORTALLOCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SINGLEFIFOOF` reader - Single FIFO Overflow Enable"]
pub type SINGLEFIFOOF_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEFIFOOF` writer - Single FIFO Overflow Enable"]
pub type SINGLEFIFOOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SCANFIFOOF` reader - Scan FIFO Overflow Enable"]
pub type SCANFIFOOF_R = crate::BitReader<bool>;
#[doc = "Field `SCANFIFOOF` writer - Scan FIFO Overflow Enable"]
pub type SCANFIFOOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SINGLEFIFOUF` reader - Single FIFO Underflow Enable"]
pub type SINGLEFIFOUF_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEFIFOUF` writer - Single FIFO Underflow Enable"]
pub type SINGLEFIFOUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SCANFIFOUF` reader - Scan FIFO Underflow Enable"]
pub type SCANFIFOUF_R = crate::BitReader<bool>;
#[doc = "Field `SCANFIFOUF` writer - Scan FIFO Underflow Enable"]
pub type SCANFIFOUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM23ABORTERROR` reader - EM2/3 Abort Error Enable"]
pub type EM23ABORTERROR_R = crate::BitReader<bool>;
#[doc = "Field `EM23ABORTERROR` writer - EM2/3 Abort Error Enable"]
pub type EM23ABORTERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Single FIFO Data Valid Level Enable"]
    #[inline(always)]
    pub fn singlefifodvl(&self) -> SINGLEFIFODVL_R {
        SINGLEFIFODVL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan FIFO Data Valid Level Enable"]
    #[inline(always)]
    pub fn scanfifodvl(&self) -> SCANFIFODVL_R {
        SCANFIFODVL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single Result Window Compare Enable"]
    #[inline(always)]
    pub fn singlecmp(&self) -> SINGLECMP_R {
        SINGLECMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Result Window Compare Enable"]
    #[inline(always)]
    pub fn scancmp(&self) -> SCANCMP_R {
        SCANCMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan Entry Done Enable"]
    #[inline(always)]
    pub fn scanentrydone(&self) -> SCANENTRYDONE_R {
        SCANENTRYDONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Table Done Enable"]
    #[inline(always)]
    pub fn scantabledone(&self) -> SCANTABLEDONE_R {
        SCANTABLEDONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Single Conversion Done Enable"]
    #[inline(always)]
    pub fn singledone(&self) -> SINGLEDONE_R {
        SINGLEDONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Polarity Error Enable"]
    #[inline(always)]
    pub fn polarityerr(&self) -> POLARITYERR_R {
        POLARITYERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Allocation Error Enable"]
    #[inline(always)]
    pub fn portallocerr(&self) -> PORTALLOCERR_R {
        PORTALLOCERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Single FIFO Overflow Enable"]
    #[inline(always)]
    pub fn singlefifoof(&self) -> SINGLEFIFOOF_R {
        SINGLEFIFOOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Scan FIFO Overflow Enable"]
    #[inline(always)]
    pub fn scanfifoof(&self) -> SCANFIFOOF_R {
        SCANFIFOOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Single FIFO Underflow Enable"]
    #[inline(always)]
    pub fn singlefifouf(&self) -> SINGLEFIFOUF_R {
        SINGLEFIFOUF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Scan FIFO Underflow Enable"]
    #[inline(always)]
    pub fn scanfifouf(&self) -> SCANFIFOUF_R {
        SCANFIFOUF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - EM2/3 Abort Error Enable"]
    #[inline(always)]
    pub fn em23aborterror(&self) -> EM23ABORTERROR_R {
        EM23ABORTERROR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single FIFO Data Valid Level Enable"]
    #[inline(always)]
    #[must_use]
    pub fn singlefifodvl(&mut self) -> SINGLEFIFODVL_W<0> {
        SINGLEFIFODVL_W::new(self)
    }
    #[doc = "Bit 1 - Scan FIFO Data Valid Level Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scanfifodvl(&mut self) -> SCANFIFODVL_W<1> {
        SCANFIFODVL_W::new(self)
    }
    #[doc = "Bit 2 - Single Result Window Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn singlecmp(&mut self) -> SINGLECMP_W<2> {
        SINGLECMP_W::new(self)
    }
    #[doc = "Bit 3 - Scan Result Window Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scancmp(&mut self) -> SCANCMP_W<3> {
        SCANCMP_W::new(self)
    }
    #[doc = "Bit 7 - Scan Entry Done Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scanentrydone(&mut self) -> SCANENTRYDONE_W<7> {
        SCANENTRYDONE_W::new(self)
    }
    #[doc = "Bit 8 - Scan Table Done Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scantabledone(&mut self) -> SCANTABLEDONE_W<8> {
        SCANTABLEDONE_W::new(self)
    }
    #[doc = "Bit 9 - Single Conversion Done Enable"]
    #[inline(always)]
    #[must_use]
    pub fn singledone(&mut self) -> SINGLEDONE_W<9> {
        SINGLEDONE_W::new(self)
    }
    #[doc = "Bit 12 - Polarity Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn polarityerr(&mut self) -> POLARITYERR_W<12> {
        POLARITYERR_W::new(self)
    }
    #[doc = "Bit 13 - Port Allocation Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn portallocerr(&mut self) -> PORTALLOCERR_W<13> {
        PORTALLOCERR_W::new(self)
    }
    #[doc = "Bit 16 - Single FIFO Overflow Enable"]
    #[inline(always)]
    #[must_use]
    pub fn singlefifoof(&mut self) -> SINGLEFIFOOF_W<16> {
        SINGLEFIFOOF_W::new(self)
    }
    #[doc = "Bit 17 - Scan FIFO Overflow Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scanfifoof(&mut self) -> SCANFIFOOF_W<17> {
        SCANFIFOOF_W::new(self)
    }
    #[doc = "Bit 18 - Single FIFO Underflow Enable"]
    #[inline(always)]
    #[must_use]
    pub fn singlefifouf(&mut self) -> SINGLEFIFOUF_W<18> {
        SINGLEFIFOUF_W::new(self)
    }
    #[doc = "Bit 19 - Scan FIFO Underflow Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scanfifouf(&mut self) -> SCANFIFOUF_W<19> {
        SCANFIFOUF_W::new(self)
    }
    #[doc = "Bit 31 - EM2/3 Abort Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em23aborterror(&mut self) -> EM23ABORTERROR_W<31> {
        EM23ABORTERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
