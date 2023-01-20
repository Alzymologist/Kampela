#[doc = "Register `LEGACY` reader"]
pub struct R(crate::R<LEGACY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEGACY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEGACY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEGACY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVICEFAMILY` reader - Device Family"]
pub type DEVICEFAMILY_R = crate::FieldReader<u8, DEVICEFAMILY_A>;
#[doc = "Device Family\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEVICEFAMILY_A {
    #[doc = "16: EFR32 Mighty Gecko Family Series 1 Device Config 1"]
    EFR32MG1P = 16,
    #[doc = "17: EFR32 Mighty Gecko Family Series 1 Device Config 1"]
    EFR32MG1B = 17,
    #[doc = "18: EFR32 Mighty Gecko Family Series 1 Device Config 1"]
    EFR32MG1V = 18,
    #[doc = "19: EFR32 Blue Gecko Family Series 1 Device Config 1"]
    EFR32BG1P = 19,
    #[doc = "20: EFR32 Blue Gecko Family Series 1 Device Config 1"]
    EFR32BG1B = 20,
    #[doc = "21: EFR32 Blue Gecko Family Series 1 Device Config 1"]
    EFR32BG1V = 21,
    #[doc = "25: EFR32 Flex Gecko Family Series 1 Device Config 1"]
    EFR32FG1P = 25,
    #[doc = "26: EFR32 Flex Gecko Family Series 1 Device Config 1"]
    EFR32FG1B = 26,
    #[doc = "27: EFR32 Flex Gecko Family Series 1 Device Config 1"]
    EFR32FG1V = 27,
    #[doc = "28: EFR32 Mighty Gecko Family Series 1 Device Config 2"]
    EFR32MG12P = 28,
    #[doc = "29: EFR32 Mighty Gecko Family Series 1 Device Config 2"]
    EFR32MG12B = 29,
    #[doc = "30: EFR32 Mighty Gecko Family Series 1 Device Config 2"]
    EFR32MG12V = 30,
    #[doc = "31: EFR32 Blue Gecko Family Series 1 Device Config 2"]
    EFR32BG12P = 31,
    #[doc = "32: EFR32 Blue Gecko Family Series 1 Device Config 2"]
    EFR32BG12B = 32,
    #[doc = "33: EFR32 Blue Gecko Family Series 1 Device Config 2"]
    EFR32BG12V = 33,
    #[doc = "37: EFR32 Flex Gecko Family Series 1 Device Config 2"]
    EFR32FG12P = 37,
    #[doc = "38: EFR32 Flex Gecko Family Series 1 Device Config 2"]
    EFR32FG12B = 38,
    #[doc = "39: EFR32 Flex Gecko Family Series 1 Device Config 2"]
    EFR32FG12V = 39,
    #[doc = "40: EFR32 Mighty Gecko Family Series 13 Device Config 3"]
    EFR32MG13P = 40,
    #[doc = "41: EFR32 Mighty Gecko Family Series 13 Device Config 3"]
    EFR32MG13B = 41,
    #[doc = "42: EFR32 Mighty Gecko Family Series 1 Device Config 3"]
    EFR32MG13V = 42,
    #[doc = "43: EFR32 Blue Gecko Family Series 1 Device Config 3"]
    EFR32BG13P = 43,
    #[doc = "44: EFR32 Blue Gecko Family Series 1 Device Config 3"]
    EFR32BG13B = 44,
    #[doc = "45: EFR32 Blue Gecko Family Series 1 Device Config 3"]
    EFR32BG13V = 45,
    #[doc = "49: EFR32 Flex Gecko Family Series 1 Device Config 3"]
    EFR32FG13P = 49,
    #[doc = "50: EFR32 Flex Gecko Family Series 1 Device Config 3"]
    EFR32FG13B = 50,
    #[doc = "51: EFR32 Flex Gecko Family Series 1 Device Config 3"]
    EFR32FG13V = 51,
    #[doc = "52: EFR32 Mighty Gecko Family Series 1 Device Config 4"]
    EFR32MG14P = 52,
    #[doc = "53: EFR32 Mighty Gecko Family Series 1 Device Config 4"]
    EFR32MG14B = 53,
    #[doc = "54: EFR32 Mighty Gecko Family Series 1 Device Config 4"]
    EFR32MG14V = 54,
    #[doc = "55: EFR32 Blue Gecko Family Series 1 Device Config 4"]
    EFR32BG14P = 55,
    #[doc = "56: EFR32 Blue Gecko Family Series 1 Device Config 4"]
    EFR32BG14B = 56,
    #[doc = "57: EFR32 Blue Gecko Family Series 1 Device Config 4"]
    EFR32BG14V = 57,
    #[doc = "61: EFR32 Flex Gecko Family Series 1 Device Config 4"]
    EFR32FG14P = 61,
    #[doc = "62: EFR32 Flex Gecko Family Series 1 Device Config 4"]
    EFR32FG14B = 62,
    #[doc = "63: EFR32 Flex Gecko Family Series 1 Device Config 4"]
    EFR32FG14V = 63,
    #[doc = "71: EFM32 Gecko Device Family"]
    EFM32G = 71,
    #[doc = "72: EFM32 Giant Gecko Device Family"]
    EFM32GG = 72,
    #[doc = "73: EFM32 Tiny Gecko Device Family"]
    EFM32TG = 73,
    #[doc = "74: EFM32 Leopard Gecko Device Family"]
    EFM32LG = 74,
    #[doc = "75: EFM32 Wonder Gecko Device Family"]
    EFM32WG = 75,
    #[doc = "76: EFM32 Zero Gecko Device Family"]
    EFM32ZG = 76,
    #[doc = "77: EFM32 Happy Gecko Device Family"]
    EFM32HG = 77,
    #[doc = "81: EFM32 Pearl Gecko Device Family Series 1 Device Config 1"]
    EFM32PG1B = 81,
    #[doc = "83: EFM32 Jade Gecko Device Family Series 1 Device Config 1"]
    EFM32JG1B = 83,
    #[doc = "85: EFM32 Pearl Gecko Device Family Series 1 Device Config 2"]
    EFM32PG12B = 85,
    #[doc = "87: EFM32 Jade Gecko Device Family Series 1 Device Config 2"]
    EFM32JG12B = 87,
    #[doc = "89: EFM32 Pearl Gecko Device Family Series 1 Device Config 3"]
    EFM32PG13B = 89,
    #[doc = "91: EFM32 Jade Gecko Device Family Series 1 Device Config 3"]
    EFM32JG13B = 91,
    #[doc = "100: EFM32 Giant Gecko Device Family Series 1 Device Config 1"]
    EFM32GG11B = 100,
    #[doc = "103: EFM32 Giant Gecko Device Family Series 1 Device Config 1"]
    EFM32TG11B = 103,
    #[doc = "120: EZR32 Leopard Gecko Device Family"]
    EZR32LG = 120,
    #[doc = "121: EZR32 Wonder Gecko Device Family"]
    EZR32WG = 121,
    #[doc = "122: EZR32 Happy Gecko Device Family"]
    EZR32HG = 122,
    #[doc = "128: DI page is encoded with the series 2 layout. Check alternate location."]
    SERIES2V0 = 128,
}
impl From<DEVICEFAMILY_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVICEFAMILY_A) -> Self {
        variant as _
    }
}
impl DEVICEFAMILY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEVICEFAMILY_A> {
        match self.bits {
            16 => Some(DEVICEFAMILY_A::EFR32MG1P),
            17 => Some(DEVICEFAMILY_A::EFR32MG1B),
            18 => Some(DEVICEFAMILY_A::EFR32MG1V),
            19 => Some(DEVICEFAMILY_A::EFR32BG1P),
            20 => Some(DEVICEFAMILY_A::EFR32BG1B),
            21 => Some(DEVICEFAMILY_A::EFR32BG1V),
            25 => Some(DEVICEFAMILY_A::EFR32FG1P),
            26 => Some(DEVICEFAMILY_A::EFR32FG1B),
            27 => Some(DEVICEFAMILY_A::EFR32FG1V),
            28 => Some(DEVICEFAMILY_A::EFR32MG12P),
            29 => Some(DEVICEFAMILY_A::EFR32MG12B),
            30 => Some(DEVICEFAMILY_A::EFR32MG12V),
            31 => Some(DEVICEFAMILY_A::EFR32BG12P),
            32 => Some(DEVICEFAMILY_A::EFR32BG12B),
            33 => Some(DEVICEFAMILY_A::EFR32BG12V),
            37 => Some(DEVICEFAMILY_A::EFR32FG12P),
            38 => Some(DEVICEFAMILY_A::EFR32FG12B),
            39 => Some(DEVICEFAMILY_A::EFR32FG12V),
            40 => Some(DEVICEFAMILY_A::EFR32MG13P),
            41 => Some(DEVICEFAMILY_A::EFR32MG13B),
            42 => Some(DEVICEFAMILY_A::EFR32MG13V),
            43 => Some(DEVICEFAMILY_A::EFR32BG13P),
            44 => Some(DEVICEFAMILY_A::EFR32BG13B),
            45 => Some(DEVICEFAMILY_A::EFR32BG13V),
            49 => Some(DEVICEFAMILY_A::EFR32FG13P),
            50 => Some(DEVICEFAMILY_A::EFR32FG13B),
            51 => Some(DEVICEFAMILY_A::EFR32FG13V),
            52 => Some(DEVICEFAMILY_A::EFR32MG14P),
            53 => Some(DEVICEFAMILY_A::EFR32MG14B),
            54 => Some(DEVICEFAMILY_A::EFR32MG14V),
            55 => Some(DEVICEFAMILY_A::EFR32BG14P),
            56 => Some(DEVICEFAMILY_A::EFR32BG14B),
            57 => Some(DEVICEFAMILY_A::EFR32BG14V),
            61 => Some(DEVICEFAMILY_A::EFR32FG14P),
            62 => Some(DEVICEFAMILY_A::EFR32FG14B),
            63 => Some(DEVICEFAMILY_A::EFR32FG14V),
            71 => Some(DEVICEFAMILY_A::EFM32G),
            72 => Some(DEVICEFAMILY_A::EFM32GG),
            73 => Some(DEVICEFAMILY_A::EFM32TG),
            74 => Some(DEVICEFAMILY_A::EFM32LG),
            75 => Some(DEVICEFAMILY_A::EFM32WG),
            76 => Some(DEVICEFAMILY_A::EFM32ZG),
            77 => Some(DEVICEFAMILY_A::EFM32HG),
            81 => Some(DEVICEFAMILY_A::EFM32PG1B),
            83 => Some(DEVICEFAMILY_A::EFM32JG1B),
            85 => Some(DEVICEFAMILY_A::EFM32PG12B),
            87 => Some(DEVICEFAMILY_A::EFM32JG12B),
            89 => Some(DEVICEFAMILY_A::EFM32PG13B),
            91 => Some(DEVICEFAMILY_A::EFM32JG13B),
            100 => Some(DEVICEFAMILY_A::EFM32GG11B),
            103 => Some(DEVICEFAMILY_A::EFM32TG11B),
            120 => Some(DEVICEFAMILY_A::EZR32LG),
            121 => Some(DEVICEFAMILY_A::EZR32WG),
            122 => Some(DEVICEFAMILY_A::EZR32HG),
            128 => Some(DEVICEFAMILY_A::SERIES2V0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EFR32MG1P`"]
    #[inline(always)]
    pub fn is_efr32mg1p(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32MG1P
    }
    #[doc = "Checks if the value of the field is `EFR32MG1B`"]
    #[inline(always)]
    pub fn is_efr32mg1b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32MG1B
    }
    #[doc = "Checks if the value of the field is `EFR32MG1V`"]
    #[inline(always)]
    pub fn is_efr32mg1v(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32MG1V
    }
    #[doc = "Checks if the value of the field is `EFR32BG1P`"]
    #[inline(always)]
    pub fn is_efr32bg1p(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32BG1P
    }
    #[doc = "Checks if the value of the field is `EFR32BG1B`"]
    #[inline(always)]
    pub fn is_efr32bg1b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32BG1B
    }
    #[doc = "Checks if the value of the field is `EFR32BG1V`"]
    #[inline(always)]
    pub fn is_efr32bg1v(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32BG1V
    }
    #[doc = "Checks if the value of the field is `EFR32FG1P`"]
    #[inline(always)]
    pub fn is_efr32fg1p(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32FG1P
    }
    #[doc = "Checks if the value of the field is `EFR32FG1B`"]
    #[inline(always)]
    pub fn is_efr32fg1b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32FG1B
    }
    #[doc = "Checks if the value of the field is `EFR32FG1V`"]
    #[inline(always)]
    pub fn is_efr32fg1v(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32FG1V
    }
    #[doc = "Checks if the value of the field is `EFR32MG12P`"]
    #[inline(always)]
    pub fn is_efr32mg12p(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32MG12P
    }
    #[doc = "Checks if the value of the field is `EFR32MG12B`"]
    #[inline(always)]
    pub fn is_efr32mg12b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32MG12B
    }
    #[doc = "Checks if the value of the field is `EFR32MG12V`"]
    #[inline(always)]
    pub fn is_efr32mg12v(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32MG12V
    }
    #[doc = "Checks if the value of the field is `EFR32BG12P`"]
    #[inline(always)]
    pub fn is_efr32bg12p(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32BG12P
    }
    #[doc = "Checks if the value of the field is `EFR32BG12B`"]
    #[inline(always)]
    pub fn is_efr32bg12b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32BG12B
    }
    #[doc = "Checks if the value of the field is `EFR32BG12V`"]
    #[inline(always)]
    pub fn is_efr32bg12v(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32BG12V
    }
    #[doc = "Checks if the value of the field is `EFR32FG12P`"]
    #[inline(always)]
    pub fn is_efr32fg12p(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32FG12P
    }
    #[doc = "Checks if the value of the field is `EFR32FG12B`"]
    #[inline(always)]
    pub fn is_efr32fg12b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32FG12B
    }
    #[doc = "Checks if the value of the field is `EFR32FG12V`"]
    #[inline(always)]
    pub fn is_efr32fg12v(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32FG12V
    }
    #[doc = "Checks if the value of the field is `EFR32MG13P`"]
    #[inline(always)]
    pub fn is_efr32mg13p(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32MG13P
    }
    #[doc = "Checks if the value of the field is `EFR32MG13B`"]
    #[inline(always)]
    pub fn is_efr32mg13b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32MG13B
    }
    #[doc = "Checks if the value of the field is `EFR32MG13V`"]
    #[inline(always)]
    pub fn is_efr32mg13v(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32MG13V
    }
    #[doc = "Checks if the value of the field is `EFR32BG13P`"]
    #[inline(always)]
    pub fn is_efr32bg13p(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32BG13P
    }
    #[doc = "Checks if the value of the field is `EFR32BG13B`"]
    #[inline(always)]
    pub fn is_efr32bg13b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32BG13B
    }
    #[doc = "Checks if the value of the field is `EFR32BG13V`"]
    #[inline(always)]
    pub fn is_efr32bg13v(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32BG13V
    }
    #[doc = "Checks if the value of the field is `EFR32FG13P`"]
    #[inline(always)]
    pub fn is_efr32fg13p(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32FG13P
    }
    #[doc = "Checks if the value of the field is `EFR32FG13B`"]
    #[inline(always)]
    pub fn is_efr32fg13b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32FG13B
    }
    #[doc = "Checks if the value of the field is `EFR32FG13V`"]
    #[inline(always)]
    pub fn is_efr32fg13v(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32FG13V
    }
    #[doc = "Checks if the value of the field is `EFR32MG14P`"]
    #[inline(always)]
    pub fn is_efr32mg14p(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32MG14P
    }
    #[doc = "Checks if the value of the field is `EFR32MG14B`"]
    #[inline(always)]
    pub fn is_efr32mg14b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32MG14B
    }
    #[doc = "Checks if the value of the field is `EFR32MG14V`"]
    #[inline(always)]
    pub fn is_efr32mg14v(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32MG14V
    }
    #[doc = "Checks if the value of the field is `EFR32BG14P`"]
    #[inline(always)]
    pub fn is_efr32bg14p(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32BG14P
    }
    #[doc = "Checks if the value of the field is `EFR32BG14B`"]
    #[inline(always)]
    pub fn is_efr32bg14b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32BG14B
    }
    #[doc = "Checks if the value of the field is `EFR32BG14V`"]
    #[inline(always)]
    pub fn is_efr32bg14v(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32BG14V
    }
    #[doc = "Checks if the value of the field is `EFR32FG14P`"]
    #[inline(always)]
    pub fn is_efr32fg14p(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32FG14P
    }
    #[doc = "Checks if the value of the field is `EFR32FG14B`"]
    #[inline(always)]
    pub fn is_efr32fg14b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32FG14B
    }
    #[doc = "Checks if the value of the field is `EFR32FG14V`"]
    #[inline(always)]
    pub fn is_efr32fg14v(&self) -> bool {
        *self == DEVICEFAMILY_A::EFR32FG14V
    }
    #[doc = "Checks if the value of the field is `EFM32G`"]
    #[inline(always)]
    pub fn is_efm32g(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32G
    }
    #[doc = "Checks if the value of the field is `EFM32GG`"]
    #[inline(always)]
    pub fn is_efm32gg(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32GG
    }
    #[doc = "Checks if the value of the field is `EFM32TG`"]
    #[inline(always)]
    pub fn is_efm32tg(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32TG
    }
    #[doc = "Checks if the value of the field is `EFM32LG`"]
    #[inline(always)]
    pub fn is_efm32lg(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32LG
    }
    #[doc = "Checks if the value of the field is `EFM32WG`"]
    #[inline(always)]
    pub fn is_efm32wg(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32WG
    }
    #[doc = "Checks if the value of the field is `EFM32ZG`"]
    #[inline(always)]
    pub fn is_efm32zg(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32ZG
    }
    #[doc = "Checks if the value of the field is `EFM32HG`"]
    #[inline(always)]
    pub fn is_efm32hg(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32HG
    }
    #[doc = "Checks if the value of the field is `EFM32PG1B`"]
    #[inline(always)]
    pub fn is_efm32pg1b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32PG1B
    }
    #[doc = "Checks if the value of the field is `EFM32JG1B`"]
    #[inline(always)]
    pub fn is_efm32jg1b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32JG1B
    }
    #[doc = "Checks if the value of the field is `EFM32PG12B`"]
    #[inline(always)]
    pub fn is_efm32pg12b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32PG12B
    }
    #[doc = "Checks if the value of the field is `EFM32JG12B`"]
    #[inline(always)]
    pub fn is_efm32jg12b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32JG12B
    }
    #[doc = "Checks if the value of the field is `EFM32PG13B`"]
    #[inline(always)]
    pub fn is_efm32pg13b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32PG13B
    }
    #[doc = "Checks if the value of the field is `EFM32JG13B`"]
    #[inline(always)]
    pub fn is_efm32jg13b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32JG13B
    }
    #[doc = "Checks if the value of the field is `EFM32GG11B`"]
    #[inline(always)]
    pub fn is_efm32gg11b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32GG11B
    }
    #[doc = "Checks if the value of the field is `EFM32TG11B`"]
    #[inline(always)]
    pub fn is_efm32tg11b(&self) -> bool {
        *self == DEVICEFAMILY_A::EFM32TG11B
    }
    #[doc = "Checks if the value of the field is `EZR32LG`"]
    #[inline(always)]
    pub fn is_ezr32lg(&self) -> bool {
        *self == DEVICEFAMILY_A::EZR32LG
    }
    #[doc = "Checks if the value of the field is `EZR32WG`"]
    #[inline(always)]
    pub fn is_ezr32wg(&self) -> bool {
        *self == DEVICEFAMILY_A::EZR32WG
    }
    #[doc = "Checks if the value of the field is `EZR32HG`"]
    #[inline(always)]
    pub fn is_ezr32hg(&self) -> bool {
        *self == DEVICEFAMILY_A::EZR32HG
    }
    #[doc = "Checks if the value of the field is `SERIES2V0`"]
    #[inline(always)]
    pub fn is_series2v0(&self) -> bool {
        *self == DEVICEFAMILY_A::SERIES2V0
    }
}
impl R {
    #[doc = "Bits 16:23 - Device Family"]
    #[inline(always)]
    pub fn devicefamily(&self) -> DEVICEFAMILY_R {
        DEVICEFAMILY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "This is the legacy device detection information for tools compatability\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [legacy](index.html) module"]
pub struct LEGACY_SPEC;
impl crate::RegisterSpec for LEGACY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [legacy::R](R) reader structure"]
impl crate::Readable for LEGACY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LEGACY to value 0x0080_0000"]
impl crate::Resettable for LEGACY_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0000;
}
