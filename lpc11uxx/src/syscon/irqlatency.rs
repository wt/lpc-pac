#[doc = "Register `IRQLATENCY` reader"]
pub struct R(crate::R<IRQLATENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQLATENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQLATENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQLATENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQLATENCY` writer"]
pub struct W(crate::W<IRQLATENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQLATENCY_SPEC>;
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
impl From<crate::W<IRQLATENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQLATENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LATENCY` reader - 8-bit latency value"]
pub struct LATENCY_R(crate::FieldReader<u8, u8>);
impl LATENCY_R {
    pub(crate) fn new(bits: u8) -> Self {
        LATENCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATENCY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATENCY` writer - 8-bit latency value"]
pub struct LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> LATENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 8-bit latency value"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit latency value"]
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W {
        LATENCY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IQR delay. Allows trade-off between interrupt latency and determinism.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqlatency](index.html) module"]
pub struct IRQLATENCY_SPEC;
impl crate::RegisterSpec for IRQLATENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqlatency::R](R) reader structure"]
impl crate::Readable for IRQLATENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqlatency::W](W) writer structure"]
impl crate::Writable for IRQLATENCY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQLATENCY to value 0x10"]
impl crate::Resettable for IRQLATENCY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
