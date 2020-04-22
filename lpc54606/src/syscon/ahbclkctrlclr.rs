#[doc = "Writer for register AHBCLKCTRLCLR[%s]"]
pub type W = crate::W<u32, super::AHBCLKCTRLCLR>;
#[doc = "Register AHBCLKCTRLCLR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::AHBCLKCTRLCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CLK_CLR`"]
pub struct CLK_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register clears the corresponding bit or bits in the AHBCLKCTRLn register, if they are implemented. Bits that do not correspond to defined bits in AHBCLKCTRLn are reserved and only zeroes should be written to them."]
    #[inline(always)]
    pub fn clk_clr(&mut self) -> CLK_CLR_W {
        CLK_CLR_W { w: self }
    }
}
