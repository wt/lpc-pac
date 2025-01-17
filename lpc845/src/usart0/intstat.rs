#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTAT` writer"]
pub struct W(crate::W<INTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTAT_SPEC>;
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
impl From<crate::W<INTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDY` reader - Receiver Ready flag."]
pub struct RXRDY_R(crate::FieldReader<bool, bool>);
impl RXRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY` reader - Transmitter Ready flag."]
pub struct TXRDY_R(crate::FieldReader<bool, bool>);
impl TXRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIDLE` reader - Transmitter idle status."]
pub struct TXIDLE_R(crate::FieldReader<bool, bool>);
impl TXIDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXIDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXIDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DELTACTS` reader - This bit is set when a change in the state of the CTS input is detected."]
pub struct DELTACTS_R(crate::FieldReader<bool, bool>);
impl DELTACTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DELTACTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELTACTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDISINT` reader - Transmitter Disabled Interrupt flag."]
pub struct TXDISINT_R(crate::FieldReader<bool, bool>);
impl TXDISINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDISINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDISINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUNINT` reader - Overrun Error interrupt flag."]
pub struct OVERRUNINT_R(crate::FieldReader<bool, bool>);
impl OVERRUNINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUNINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUNINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DELTARXBRK` reader - This bit is set when a change in the state of receiver break detection occurs."]
pub struct DELTARXBRK_R(crate::FieldReader<bool, bool>);
impl DELTARXBRK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DELTARXBRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELTARXBRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` reader - This bit is set when a start is detected on the receiver input."]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMERRINT` reader - Framing Error interrupt flag."]
pub struct FRAMERRINT_R(crate::FieldReader<bool, bool>);
impl FRAMERRINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRAMERRINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMERRINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITYERRINT` reader - Parity Error interrupt flag."]
pub struct PARITYERRINT_R(crate::FieldReader<bool, bool>);
impl PARITYERRINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARITYERRINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITYERRINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXNOISEINT` reader - Received Noise interrupt flag."]
pub struct RXNOISEINT_R(crate::FieldReader<bool, bool>);
impl RXNOISEINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNOISEINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNOISEINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABERR` reader - Autobaud Error flag."]
pub struct ABERR_R(crate::FieldReader<bool, bool>);
impl ABERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Ready flag."]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitter Ready flag."]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmitter idle status."]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is set when a change in the state of the CTS input is detected."]
    #[inline(always)]
    pub fn deltacts(&self) -> DELTACTS_R {
        DELTACTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmitter Disabled Interrupt flag."]
    #[inline(always)]
    pub fn txdisint(&self) -> TXDISINT_R {
        TXDISINT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Overrun Error interrupt flag."]
    #[inline(always)]
    pub fn overrunint(&self) -> OVERRUNINT_R {
        OVERRUNINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit is set when a change in the state of receiver break detection occurs."]
    #[inline(always)]
    pub fn deltarxbrk(&self) -> DELTARXBRK_R {
        DELTARXBRK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit is set when a start is detected on the receiver input."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Framing Error interrupt flag."]
    #[inline(always)]
    pub fn framerrint(&self) -> FRAMERRINT_R {
        FRAMERRINT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Parity Error interrupt flag."]
    #[inline(always)]
    pub fn parityerrint(&self) -> PARITYERRINT_R {
        PARITYERRINT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Received Noise interrupt flag."]
    #[inline(always)]
    pub fn rxnoiseint(&self) -> RXNOISEINT_R {
        RXNOISEINT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Autobaud Error flag."]
    #[inline(always)]
    pub fn aberr(&self) -> ABERR_R {
        ABERR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status register. Reflects interrupts that are currently enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intstat::W](W) writer structure"]
impl crate::Writable for INTSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSTAT to value 0x0c"]
impl crate::Resettable for INTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
