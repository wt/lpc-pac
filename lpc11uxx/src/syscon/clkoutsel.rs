#[doc = "Register `CLKOUTSEL` reader"]
pub struct R(crate::R<CLKOUTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKOUTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKOUTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKOUTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKOUTSEL` writer"]
pub struct W(crate::W<CLKOUTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKOUTSEL_SPEC>;
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
impl From<crate::W<CLKOUTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKOUTSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CLKOUT clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC oscillator"]
    IRC_OSCILLATOR = 0,
    #[doc = "1: Crystal oscillator (SYSOSC)"]
    CRYSTAL_OSCILLATOR = 1,
    #[doc = "2: LF oscillator (watchdog oscillator)"]
    LF_OSCILLATOR_WATCH = 2,
    #[doc = "3: Main clock"]
    MAIN_CLOCK = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - CLKOUT clock source"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::IRC_OSCILLATOR,
            1 => SEL_A::CRYSTAL_OSCILLATOR,
            2 => SEL_A::LF_OSCILLATOR_WATCH,
            3 => SEL_A::MAIN_CLOCK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_irc_oscillator(&self) -> bool {
        **self == SEL_A::IRC_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `CRYSTAL_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_crystal_oscillator(&self) -> bool {
        **self == SEL_A::CRYSTAL_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `LF_OSCILLATOR_WATCH`"]
    #[inline(always)]
    pub fn is_lf_oscillator_watch(&self) -> bool {
        **self == SEL_A::LF_OSCILLATOR_WATCH
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        **self == SEL_A::MAIN_CLOCK
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - CLKOUT clock source"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "IRC oscillator"]
    #[inline(always)]
    pub fn irc_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::IRC_OSCILLATOR)
    }
    #[doc = "Crystal oscillator (SYSOSC)"]
    #[inline(always)]
    pub fn crystal_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::CRYSTAL_OSCILLATOR)
    }
    #[doc = "LF oscillator (watchdog oscillator)"]
    #[inline(always)]
    pub fn lf_oscillator_watch(self) -> &'a mut W {
        self.variant(SEL_A::LF_OSCILLATOR_WATCH)
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CLKOUT clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLKOUT clock source"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLKOUT clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutsel](index.html) module"]
pub struct CLKOUTSEL_SPEC;
impl crate::RegisterSpec for CLKOUTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkoutsel::R](R) reader structure"]
impl crate::Readable for CLKOUTSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkoutsel::W](W) writer structure"]
impl crate::Writable for CLKOUTSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKOUTSEL to value 0"]
impl crate::Resettable for CLKOUTSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
