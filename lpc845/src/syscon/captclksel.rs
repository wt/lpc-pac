#[doc = "Register `CAPTCLKSEL` reader"]
pub struct R(crate::R<CAPTCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPTCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPTCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPTCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAPTCLKSEL` writer"]
pub struct W(crate::W<CAPTCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPTCLKSEL_SPEC>;
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
impl From<crate::W<CAPTCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPTCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock source for CAPT clock\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO"]
    FRO = 0,
    #[doc = "1: main clock"]
    MAIN_CLK = 1,
    #[doc = "2: sys pll"]
    SYS_PLL = 2,
    #[doc = "3: FRO_DIV"]
    FRO_DIV = 3,
    #[doc = "4: Watchdog oscillator"]
    WDTOSC = 4,
    #[doc = "5: None"]
    NONE = 5,
    #[doc = "6: None"]
    NONE1 = 6,
    #[doc = "7: None"]
    NONE2 = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - Clock source for CAPT clock"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::FRO,
            1 => SEL_A::MAIN_CLK,
            2 => SEL_A::SYS_PLL,
            3 => SEL_A::FRO_DIV,
            4 => SEL_A::WDTOSC,
            5 => SEL_A::NONE,
            6 => SEL_A::NONE1,
            7 => SEL_A::NONE2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FRO`"]
    #[inline(always)]
    pub fn is_fro(&self) -> bool {
        **self == SEL_A::FRO
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        **self == SEL_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `SYS_PLL`"]
    #[inline(always)]
    pub fn is_sys_pll(&self) -> bool {
        **self == SEL_A::SYS_PLL
    }
    #[doc = "Checks if the value of the field is `FRO_DIV`"]
    #[inline(always)]
    pub fn is_fro_div(&self) -> bool {
        **self == SEL_A::FRO_DIV
    }
    #[doc = "Checks if the value of the field is `WDTOSC`"]
    #[inline(always)]
    pub fn is_wdtosc(&self) -> bool {
        **self == SEL_A::WDTOSC
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `NONE1`"]
    #[inline(always)]
    pub fn is_none1(&self) -> bool {
        **self == SEL_A::NONE1
    }
    #[doc = "Checks if the value of the field is `NONE2`"]
    #[inline(always)]
    pub fn is_none2(&self) -> bool {
        **self == SEL_A::NONE2
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Clock source for CAPT clock"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FRO"]
    #[inline(always)]
    pub fn fro(self) -> &'a mut W {
        self.variant(SEL_A::FRO)
    }
    #[doc = "main clock"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLK)
    }
    #[doc = "sys pll"]
    #[inline(always)]
    pub fn sys_pll(self) -> &'a mut W {
        self.variant(SEL_A::SYS_PLL)
    }
    #[doc = "FRO_DIV"]
    #[inline(always)]
    pub fn fro_div(self) -> &'a mut W {
        self.variant(SEL_A::FRO_DIV)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn wdtosc(self) -> &'a mut W {
        self.variant(SEL_A::WDTOSC)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none1(self) -> &'a mut W {
        self.variant(SEL_A::NONE1)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none2(self) -> &'a mut W {
        self.variant(SEL_A::NONE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock source for CAPT clock"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock source for CAPT clock"]
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
#[doc = "CAPT clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [captclksel](index.html) module"]
pub struct CAPTCLKSEL_SPEC;
impl crate::RegisterSpec for CAPTCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [captclksel::R](R) reader structure"]
impl crate::Readable for CAPTCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [captclksel::W](W) writer structure"]
impl crate::Writable for CAPTCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAPTCLKSEL to value 0x07"]
impl crate::Resettable for CAPTCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
