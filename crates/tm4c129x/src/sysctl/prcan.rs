#[doc = "Reader of register PRCAN"]
pub type R = crate::R<u32, super::PRCAN>;
#[doc = "Reader of field `R0`"]
pub type R0_R = crate::R<bool, bool>;
#[doc = "Reader of field `R1`"]
pub type R1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CAN Module 0 Peripheral Ready"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CAN Module 1 Peripheral Ready"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
