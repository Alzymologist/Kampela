#[doc = "Register `CONSUMER_USART0_IR` reader"]
pub struct R(crate::R<CONSUMER_USART0_IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSUMER_USART0_IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSUMER_USART0_IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSUMER_USART0_IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSUMER_USART0_IR` writer"]
pub struct W(crate::W<CONSUMER_USART0_IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSUMER_USART0_IR_SPEC>;
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
impl From<crate::W<CONSUMER_USART0_IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSUMER_USART0_IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRSSEL` reader - IR async channel select"]
pub type PRSSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRSSEL` writer - IR async channel select"]
pub type PRSSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSUMER_USART0_IR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - IR async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IR async channel select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<0> {
        PRSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IR Consumer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [consumer_usart0_ir](index.html) module"]
pub struct CONSUMER_USART0_IR_SPEC;
impl crate::RegisterSpec for CONSUMER_USART0_IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [consumer_usart0_ir::R](R) reader structure"]
impl crate::Readable for CONSUMER_USART0_IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [consumer_usart0_ir::W](W) writer structure"]
impl crate::Writable for CONSUMER_USART0_IR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSUMER_USART0_IR to value 0"]
impl crate::Resettable for CONSUMER_USART0_IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
