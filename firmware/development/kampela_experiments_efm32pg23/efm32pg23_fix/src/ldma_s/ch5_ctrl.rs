#[doc = "Register `CH5_CTRL` reader"]
pub struct R(crate::R<CH5_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH5_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH5_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH5_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH5_CTRL` writer"]
pub struct W(crate::W<CH5_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH5_CTRL_SPEC>;
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
impl From<crate::W<CH5_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH5_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRUCTTYPE` reader - DMA Structure Type"]
pub type STRUCTTYPE_R = crate::FieldReader<u8, STRUCTTYPE_A>;
#[doc = "DMA Structure Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STRUCTTYPE_A {
    #[doc = "0: DMA transfer structure type selected."]
    TRANSFER = 0,
    #[doc = "1: Synchronization structure type selected."]
    SYNCHRONIZE = 1,
    #[doc = "2: Write immediate value structure type selected."]
    WRITE = 2,
}
impl From<STRUCTTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: STRUCTTYPE_A) -> Self {
        variant as _
    }
}
impl STRUCTTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STRUCTTYPE_A> {
        match self.bits {
            0 => Some(STRUCTTYPE_A::TRANSFER),
            1 => Some(STRUCTTYPE_A::SYNCHRONIZE),
            2 => Some(STRUCTTYPE_A::WRITE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSFER`"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == STRUCTTYPE_A::TRANSFER
    }
    #[doc = "Checks if the value of the field is `SYNCHRONIZE`"]
    #[inline(always)]
    pub fn is_synchronize(&self) -> bool {
        *self == STRUCTTYPE_A::SYNCHRONIZE
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == STRUCTTYPE_A::WRITE
    }
}
#[doc = "Field `STRUCTTYPE` writer - DMA Structure Type"]
pub type STRUCTTYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH5_CTRL_SPEC, u8, STRUCTTYPE_A, 2, O>;
impl<'a, const O: u8> STRUCTTYPE_W<'a, O> {
    #[doc = "DMA transfer structure type selected."]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut W {
        self.variant(STRUCTTYPE_A::TRANSFER)
    }
    #[doc = "Synchronization structure type selected."]
    #[inline(always)]
    pub fn synchronize(self) -> &'a mut W {
        self.variant(STRUCTTYPE_A::SYNCHRONIZE)
    }
    #[doc = "Write immediate value structure type selected."]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(STRUCTTYPE_A::WRITE)
    }
}
#[doc = "Field `STRUCTREQ` reader - Structure DMA Transfer Request"]
pub type STRUCTREQ_R = crate::BitReader<bool>;
#[doc = "Field `XFERCNT` reader - DMA Unit Data Transfer Count"]
pub type XFERCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XFERCNT` writer - DMA Unit Data Transfer Count"]
pub type XFERCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH5_CTRL_SPEC, u16, u16, 11, O>;
#[doc = "Field `BYTESWAP` reader - Endian Byte Swap"]
pub type BYTESWAP_R = crate::BitReader<bool>;
#[doc = "Field `BYTESWAP` writer - Endian Byte Swap"]
pub type BYTESWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH5_CTRL_SPEC, bool, O>;
#[doc = "Field `BLOCKSIZE` reader - Block Transfer Size"]
pub type BLOCKSIZE_R = crate::FieldReader<u8, BLOCKSIZE_A>;
#[doc = "Block Transfer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLOCKSIZE_A {
    #[doc = "0: One unit transfer per arbitration"]
    UNIT1 = 0,
    #[doc = "1: Two unit transfers per arbitration"]
    UNIT2 = 1,
    #[doc = "2: Three unit transfers per arbitration"]
    UNIT3 = 2,
    #[doc = "3: Four unit transfers per arbitration"]
    UNIT4 = 3,
    #[doc = "4: Six unit transfers per arbitration"]
    UNIT6 = 4,
    #[doc = "5: Eight unit transfers per arbitration"]
    UNIT8 = 5,
    #[doc = "7: Sixteen unit transfers per arbitration"]
    UNIT16 = 7,
    #[doc = "9: 32 unit transfers per arbitration"]
    UNIT32 = 9,
    #[doc = "10: 64 unit transfers per arbitration"]
    UNIT64 = 10,
    #[doc = "11: 128 unit transfers per arbitration"]
    UNIT128 = 11,
    #[doc = "12: 256 unit transfers per arbitration"]
    UNIT256 = 12,
    #[doc = "13: 512 unit transfers per arbitration"]
    UNIT512 = 13,
    #[doc = "14: 1024 unit transfers per arbitration"]
    UNIT1024 = 14,
    #[doc = "15: Transfer all units as specified by the XFRCNT field"]
    ALL = 15,
}
impl From<BLOCKSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: BLOCKSIZE_A) -> Self {
        variant as _
    }
}
impl BLOCKSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLOCKSIZE_A> {
        match self.bits {
            0 => Some(BLOCKSIZE_A::UNIT1),
            1 => Some(BLOCKSIZE_A::UNIT2),
            2 => Some(BLOCKSIZE_A::UNIT3),
            3 => Some(BLOCKSIZE_A::UNIT4),
            4 => Some(BLOCKSIZE_A::UNIT6),
            5 => Some(BLOCKSIZE_A::UNIT8),
            7 => Some(BLOCKSIZE_A::UNIT16),
            9 => Some(BLOCKSIZE_A::UNIT32),
            10 => Some(BLOCKSIZE_A::UNIT64),
            11 => Some(BLOCKSIZE_A::UNIT128),
            12 => Some(BLOCKSIZE_A::UNIT256),
            13 => Some(BLOCKSIZE_A::UNIT512),
            14 => Some(BLOCKSIZE_A::UNIT1024),
            15 => Some(BLOCKSIZE_A::ALL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNIT1`"]
    #[inline(always)]
    pub fn is_unit1(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT1
    }
    #[doc = "Checks if the value of the field is `UNIT2`"]
    #[inline(always)]
    pub fn is_unit2(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT2
    }
    #[doc = "Checks if the value of the field is `UNIT3`"]
    #[inline(always)]
    pub fn is_unit3(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT3
    }
    #[doc = "Checks if the value of the field is `UNIT4`"]
    #[inline(always)]
    pub fn is_unit4(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT4
    }
    #[doc = "Checks if the value of the field is `UNIT6`"]
    #[inline(always)]
    pub fn is_unit6(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT6
    }
    #[doc = "Checks if the value of the field is `UNIT8`"]
    #[inline(always)]
    pub fn is_unit8(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT8
    }
    #[doc = "Checks if the value of the field is `UNIT16`"]
    #[inline(always)]
    pub fn is_unit16(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT16
    }
    #[doc = "Checks if the value of the field is `UNIT32`"]
    #[inline(always)]
    pub fn is_unit32(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT32
    }
    #[doc = "Checks if the value of the field is `UNIT64`"]
    #[inline(always)]
    pub fn is_unit64(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT64
    }
    #[doc = "Checks if the value of the field is `UNIT128`"]
    #[inline(always)]
    pub fn is_unit128(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT128
    }
    #[doc = "Checks if the value of the field is `UNIT256`"]
    #[inline(always)]
    pub fn is_unit256(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT256
    }
    #[doc = "Checks if the value of the field is `UNIT512`"]
    #[inline(always)]
    pub fn is_unit512(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT512
    }
    #[doc = "Checks if the value of the field is `UNIT1024`"]
    #[inline(always)]
    pub fn is_unit1024(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT1024
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == BLOCKSIZE_A::ALL
    }
}
#[doc = "Field `BLOCKSIZE` writer - Block Transfer Size"]
pub type BLOCKSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH5_CTRL_SPEC, u8, BLOCKSIZE_A, 4, O>;
impl<'a, const O: u8> BLOCKSIZE_W<'a, O> {
    #[doc = "One unit transfer per arbitration"]
    #[inline(always)]
    pub fn unit1(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT1)
    }
    #[doc = "Two unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit2(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT2)
    }
    #[doc = "Three unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit3(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT3)
    }
    #[doc = "Four unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit4(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT4)
    }
    #[doc = "Six unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit6(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT6)
    }
    #[doc = "Eight unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit8(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT8)
    }
    #[doc = "Sixteen unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit16(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT16)
    }
    #[doc = "32 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit32(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT32)
    }
    #[doc = "64 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit64(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT64)
    }
    #[doc = "128 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit128(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT128)
    }
    #[doc = "256 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit256(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT256)
    }
    #[doc = "512 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit512(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT512)
    }
    #[doc = "1024 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit1024(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT1024)
    }
    #[doc = "Transfer all units as specified by the XFRCNT field"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::ALL)
    }
}
#[doc = "Field `DONEIEN` reader - DMA Operation Done Interrupt Flag Set En"]
pub type DONEIEN_R = crate::BitReader<bool>;
#[doc = "Field `DONEIEN` writer - DMA Operation Done Interrupt Flag Set En"]
pub type DONEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH5_CTRL_SPEC, bool, O>;
#[doc = "Field `REQMODE` reader - DMA Request Transfer Mode Select"]
pub type REQMODE_R = crate::BitReader<REQMODE_A>;
#[doc = "DMA Request Transfer Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REQMODE_A {
    #[doc = "0: The LDMA transfers one BLOCKSIZE per transfer request."]
    BLOCK = 0,
    #[doc = "1: One transfer request transfers all units as defined by the XFRCNT field."]
    ALL = 1,
}
impl From<REQMODE_A> for bool {
    #[inline(always)]
    fn from(variant: REQMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl REQMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQMODE_A {
        match self.bits {
            false => REQMODE_A::BLOCK,
            true => REQMODE_A::ALL,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == REQMODE_A::BLOCK
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == REQMODE_A::ALL
    }
}
#[doc = "Field `REQMODE` writer - DMA Request Transfer Mode Select"]
pub type REQMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH5_CTRL_SPEC, REQMODE_A, O>;
impl<'a, const O: u8> REQMODE_W<'a, O> {
    #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
    #[inline(always)]
    pub fn block(self) -> &'a mut W {
        self.variant(REQMODE_A::BLOCK)
    }
    #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(REQMODE_A::ALL)
    }
}
#[doc = "Field `DECLOOPCNT` reader - Decrement Loop Count"]
pub type DECLOOPCNT_R = crate::BitReader<bool>;
#[doc = "Field `DECLOOPCNT` writer - Decrement Loop Count"]
pub type DECLOOPCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH5_CTRL_SPEC, bool, O>;
#[doc = "Field `IGNORESREQ` reader - Ignore Sreq"]
pub type IGNORESREQ_R = crate::BitReader<bool>;
#[doc = "Field `IGNORESREQ` writer - Ignore Sreq"]
pub type IGNORESREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH5_CTRL_SPEC, bool, O>;
#[doc = "Field `SRCINC` reader - Source Address Increment Size"]
pub type SRCINC_R = crate::FieldReader<u8, SRCINC_A>;
#[doc = "Source Address Increment Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRCINC_A {
    #[doc = "0: Increment source address by one unit data size after each read"]
    ONE = 0,
    #[doc = "1: Increment source address by two unit data sizes after each read"]
    TWO = 1,
    #[doc = "2: Increment source address by four unit data sizes after each read"]
    FOUR = 2,
    #[doc = "3: Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    NONE = 3,
}
impl From<SRCINC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCINC_A) -> Self {
        variant as _
    }
}
impl SRCINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCINC_A {
        match self.bits {
            0 => SRCINC_A::ONE,
            1 => SRCINC_A::TWO,
            2 => SRCINC_A::FOUR,
            3 => SRCINC_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SRCINC_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == SRCINC_A::TWO
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == SRCINC_A::FOUR
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SRCINC_A::NONE
    }
}
#[doc = "Field `SRCINC` writer - Source Address Increment Size"]
pub type SRCINC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CH5_CTRL_SPEC, u8, SRCINC_A, 2, O>;
impl<'a, const O: u8> SRCINC_W<'a, O> {
    #[doc = "Increment source address by one unit data size after each read"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SRCINC_A::ONE)
    }
    #[doc = "Increment source address by two unit data sizes after each read"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(SRCINC_A::TWO)
    }
    #[doc = "Increment source address by four unit data sizes after each read"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(SRCINC_A::FOUR)
    }
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SRCINC_A::NONE)
    }
}
#[doc = "Field `SIZE` reader - Unit Data Transfer Size"]
pub type SIZE_R = crate::FieldReader<u8, SIZE_A>;
#[doc = "Unit Data Transfer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: Each unit transfer is a byte"]
    BYTE = 0,
    #[doc = "1: Each unit transfer is a half-word"]
    HALFWORD = 1,
    #[doc = "2: Each unit transfer is a word"]
    WORD = 2,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIZE_A> {
        match self.bits {
            0 => Some(SIZE_A::BYTE),
            1 => Some(SIZE_A::HALFWORD),
            2 => Some(SIZE_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SIZE_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == SIZE_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SIZE_A::WORD
    }
}
#[doc = "Field `SIZE` writer - Unit Data Transfer Size"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH5_CTRL_SPEC, u8, SIZE_A, 2, O>;
impl<'a, const O: u8> SIZE_W<'a, O> {
    #[doc = "Each unit transfer is a byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(SIZE_A::BYTE)
    }
    #[doc = "Each unit transfer is a half-word"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut W {
        self.variant(SIZE_A::HALFWORD)
    }
    #[doc = "Each unit transfer is a word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(SIZE_A::WORD)
    }
}
#[doc = "Field `DSTINC` reader - Destination Address Increment Size"]
pub type DSTINC_R = crate::FieldReader<u8, DSTINC_A>;
#[doc = "Destination Address Increment Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSTINC_A {
    #[doc = "0: Increment destination address by one unit data size after each write"]
    ONE = 0,
    #[doc = "1: Increment destination address by two unit data sizes after each write"]
    TWO = 1,
    #[doc = "2: Increment destination address by four unit data sizes after each write"]
    FOUR = 2,
    #[doc = "3: Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    NONE = 3,
}
impl From<DSTINC_A> for u8 {
    #[inline(always)]
    fn from(variant: DSTINC_A) -> Self {
        variant as _
    }
}
impl DSTINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSTINC_A {
        match self.bits {
            0 => DSTINC_A::ONE,
            1 => DSTINC_A::TWO,
            2 => DSTINC_A::FOUR,
            3 => DSTINC_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == DSTINC_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == DSTINC_A::TWO
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == DSTINC_A::FOUR
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DSTINC_A::NONE
    }
}
#[doc = "Field `DSTINC` writer - Destination Address Increment Size"]
pub type DSTINC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CH5_CTRL_SPEC, u8, DSTINC_A, 2, O>;
impl<'a, const O: u8> DSTINC_W<'a, O> {
    #[doc = "Increment destination address by one unit data size after each write"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(DSTINC_A::ONE)
    }
    #[doc = "Increment destination address by two unit data sizes after each write"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(DSTINC_A::TWO)
    }
    #[doc = "Increment destination address by four unit data sizes after each write"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(DSTINC_A::FOUR)
    }
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DSTINC_A::NONE)
    }
}
#[doc = "Field `SRCMODE` reader - Source Addressing Mode"]
pub type SRCMODE_R = crate::BitReader<SRCMODE_A>;
#[doc = "Source Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRCMODE_A {
    #[doc = "0: The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
    ABSOLUTE = 0,
    #[doc = "1: The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
    RELATIVE = 1,
}
impl From<SRCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: SRCMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl SRCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCMODE_A {
        match self.bits {
            false => SRCMODE_A::ABSOLUTE,
            true => SRCMODE_A::RELATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ABSOLUTE`"]
    #[inline(always)]
    pub fn is_absolute(&self) -> bool {
        *self == SRCMODE_A::ABSOLUTE
    }
    #[doc = "Checks if the value of the field is `RELATIVE`"]
    #[inline(always)]
    pub fn is_relative(&self) -> bool {
        *self == SRCMODE_A::RELATIVE
    }
}
#[doc = "Field `DSTMODE` reader - Destination Addressing Mode"]
pub type DSTMODE_R = crate::BitReader<DSTMODE_A>;
#[doc = "Destination Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSTMODE_A {
    #[doc = "0: The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
    ABSOLUTE = 0,
    #[doc = "1: The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
    RELATIVE = 1,
}
impl From<DSTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DSTMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DSTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSTMODE_A {
        match self.bits {
            false => DSTMODE_A::ABSOLUTE,
            true => DSTMODE_A::RELATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ABSOLUTE`"]
    #[inline(always)]
    pub fn is_absolute(&self) -> bool {
        *self == DSTMODE_A::ABSOLUTE
    }
    #[doc = "Checks if the value of the field is `RELATIVE`"]
    #[inline(always)]
    pub fn is_relative(&self) -> bool {
        *self == DSTMODE_A::RELATIVE
    }
}
impl R {
    #[doc = "Bits 0:1 - DMA Structure Type"]
    #[inline(always)]
    pub fn structtype(&self) -> STRUCTTYPE_R {
        STRUCTTYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Structure DMA Transfer Request"]
    #[inline(always)]
    pub fn structreq(&self) -> STRUCTREQ_R {
        STRUCTREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:14 - DMA Unit Data Transfer Count"]
    #[inline(always)]
    pub fn xfercnt(&self) -> XFERCNT_R {
        XFERCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Endian Byte Swap"]
    #[inline(always)]
    pub fn byteswap(&self) -> BYTESWAP_R {
        BYTESWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Block Transfer Size"]
    #[inline(always)]
    pub fn blocksize(&self) -> BLOCKSIZE_R {
        BLOCKSIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - DMA Operation Done Interrupt Flag Set En"]
    #[inline(always)]
    pub fn doneien(&self) -> DONEIEN_R {
        DONEIEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA Request Transfer Mode Select"]
    #[inline(always)]
    pub fn reqmode(&self) -> REQMODE_R {
        REQMODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Decrement Loop Count"]
    #[inline(always)]
    pub fn decloopcnt(&self) -> DECLOOPCNT_R {
        DECLOOPCNT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Ignore Sreq"]
    #[inline(always)]
    pub fn ignoresreq(&self) -> IGNORESREQ_R {
        IGNORESREQ_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Source Address Increment Size"]
    #[inline(always)]
    pub fn srcinc(&self) -> SRCINC_R {
        SRCINC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Unit Data Transfer Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Destination Address Increment Size"]
    #[inline(always)]
    pub fn dstinc(&self) -> DSTINC_R {
        DSTINC_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Source Addressing Mode"]
    #[inline(always)]
    pub fn srcmode(&self) -> SRCMODE_R {
        SRCMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Destination Addressing Mode"]
    #[inline(always)]
    pub fn dstmode(&self) -> DSTMODE_R {
        DSTMODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DMA Structure Type"]
    #[inline(always)]
    #[must_use]
    pub fn structtype(&mut self) -> STRUCTTYPE_W<0> {
        STRUCTTYPE_W::new(self)
    }
    #[doc = "Bits 4:14 - DMA Unit Data Transfer Count"]
    #[inline(always)]
    #[must_use]
    pub fn xfercnt(&mut self) -> XFERCNT_W<4> {
        XFERCNT_W::new(self)
    }
    #[doc = "Bit 15 - Endian Byte Swap"]
    #[inline(always)]
    #[must_use]
    pub fn byteswap(&mut self) -> BYTESWAP_W<15> {
        BYTESWAP_W::new(self)
    }
    #[doc = "Bits 16:19 - Block Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn blocksize(&mut self) -> BLOCKSIZE_W<16> {
        BLOCKSIZE_W::new(self)
    }
    #[doc = "Bit 20 - DMA Operation Done Interrupt Flag Set En"]
    #[inline(always)]
    #[must_use]
    pub fn doneien(&mut self) -> DONEIEN_W<20> {
        DONEIEN_W::new(self)
    }
    #[doc = "Bit 21 - DMA Request Transfer Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn reqmode(&mut self) -> REQMODE_W<21> {
        REQMODE_W::new(self)
    }
    #[doc = "Bit 22 - Decrement Loop Count"]
    #[inline(always)]
    #[must_use]
    pub fn decloopcnt(&mut self) -> DECLOOPCNT_W<22> {
        DECLOOPCNT_W::new(self)
    }
    #[doc = "Bit 23 - Ignore Sreq"]
    #[inline(always)]
    #[must_use]
    pub fn ignoresreq(&mut self) -> IGNORESREQ_W<23> {
        IGNORESREQ_W::new(self)
    }
    #[doc = "Bits 24:25 - Source Address Increment Size"]
    #[inline(always)]
    #[must_use]
    pub fn srcinc(&mut self) -> SRCINC_W<24> {
        SRCINC_W::new(self)
    }
    #[doc = "Bits 26:27 - Unit Data Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<26> {
        SIZE_W::new(self)
    }
    #[doc = "Bits 28:29 - Destination Address Increment Size"]
    #[inline(always)]
    #[must_use]
    pub fn dstinc(&mut self) -> DSTINC_W<28> {
        DSTINC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_ctrl](index.html) module"]
pub struct CH5_CTRL_SPEC;
impl crate::RegisterSpec for CH5_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch5_ctrl::R](R) reader structure"]
impl crate::Readable for CH5_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch5_ctrl::W](W) writer structure"]
impl crate::Writable for CH5_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH5_CTRL to value 0"]
impl crate::Resettable for CH5_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
