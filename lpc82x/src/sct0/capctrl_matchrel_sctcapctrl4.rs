#[doc = "Register `SCTCAPCTRL4` reader"]
pub struct R(crate::R<CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTCAPCTRL4` writer"]
pub struct W(crate::W<CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC>;
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
impl From<crate::W<CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPCONn_L` reader - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
pub struct CAPCONN_L_R(crate::FieldReader<u8, u8>);
impl CAPCONN_L_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPCONN_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPCONN_L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPCONn_L` writer - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
pub struct CAPCONN_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCONN_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CAPCONn_H` reader - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
pub struct CAPCONN_H_R(crate::FieldReader<u8, u8>);
impl CAPCONN_H_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPCONN_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPCONN_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPCONn_H` writer - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
pub struct CAPCONN_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCONN_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_l(&self) -> CAPCONN_L_R {
        CAPCONN_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_h(&self) -> CAPCONN_H_R {
        CAPCONN_H_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_l(&mut self) -> CAPCONN_L_W {
        CAPCONN_L_W { w: self }
    }
    #[doc = "Bits 16:23 - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_h(&mut self) -> CAPCONN_H_W {
        CAPCONN_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl_matchrel_sctcapctrl4](index.html) module"]
pub struct CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC;
impl crate::RegisterSpec for CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capctrl_matchrel_sctcapctrl4::R](R) reader structure"]
impl crate::Readable for CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [capctrl_matchrel_sctcapctrl4::W](W) writer structure"]
impl crate::Writable for CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCTCAPCTRL4 to value 0"]
impl crate::Resettable for CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
