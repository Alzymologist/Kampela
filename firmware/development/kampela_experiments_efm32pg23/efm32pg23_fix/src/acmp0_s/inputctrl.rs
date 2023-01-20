#[doc = "Register `INPUTCTRL` reader"]
pub struct R(crate::R<INPUTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUTCTRL` writer"]
pub struct W(crate::W<INPUTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUTCTRL_SPEC>;
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
impl From<crate::W<INPUTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POSSEL` reader - Positive Input Select"]
pub type POSSEL_R = crate::FieldReader<u8, POSSEL_A>;
#[doc = "Positive Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POSSEL_A {
    #[doc = "0: VSS"]
    VSS = 0,
    #[doc = "16: Divided AVDD"]
    VREFDIVAVDD = 16,
    #[doc = "17: Low-Power Divided AVDD"]
    VREFDIVAVDDLP = 17,
    #[doc = "18: Divided 1V25 reference"]
    VREFDIV1V25 = 18,
    #[doc = "19: Low-power Divided 1V25 reference"]
    VREFDIV1V25LP = 19,
    #[doc = "20: Divided 2V5 reference"]
    VREFDIV2V5 = 20,
    #[doc = "21: Low-power Divided 2V5 reference"]
    VREFDIV2V5LP = 21,
    #[doc = "32: VSENSE0 divided by 4"]
    VSENSE01DIV4 = 32,
    #[doc = "33: Low-power VSENSE0 divided by 4"]
    VSENSE01DIV4LP = 33,
    #[doc = "34: VSENSE1 divided by 4"]
    VSENSE11DIV4 = 34,
    #[doc = "35: Low-power VSENSE1 divided by 4"]
    VSENSE11DIV4LP = 35,
    #[doc = "64: VDAC0 channel 0 output"]
    VDACOUT0 = 64,
    #[doc = "65: VDAC0 channel 1 output"]
    VDACOUT1 = 65,
    #[doc = "80: External interface, base is PA0."]
    EXTPA = 80,
    #[doc = "81: External interface, base is PB0."]
    EXTPB = 81,
    #[doc = "82: External interface, base is PC0."]
    EXTPC = 82,
    #[doc = "83: External interface, base is PD0."]
    EXTPD = 83,
    #[doc = "128: Port A, Pin0"]
    PA0 = 128,
    #[doc = "129: Port A, Pin1"]
    PA1 = 129,
    #[doc = "130: Port A, Pin2"]
    PA2 = 130,
    #[doc = "131: Port A, Pin3"]
    PA3 = 131,
    #[doc = "132: Port A, Pin4"]
    PA4 = 132,
    #[doc = "133: Port A, Pin5"]
    PA5 = 133,
    #[doc = "134: Port A, Pin6"]
    PA6 = 134,
    #[doc = "135: Port A, Pin7"]
    PA7 = 135,
    #[doc = "136: Port A, Pin8"]
    PA8 = 136,
    #[doc = "137: Port A, Pin9"]
    PA9 = 137,
    #[doc = "138: Port A, Pin10"]
    PA10 = 138,
    #[doc = "139: Port A, Pin11"]
    PA11 = 139,
    #[doc = "140: Port A, Pin12"]
    PA12 = 140,
    #[doc = "141: Port A, Pin13"]
    PA13 = 141,
    #[doc = "142: Port A, Pin14"]
    PA14 = 142,
    #[doc = "143: Port A, Pin15"]
    PA15 = 143,
    #[doc = "144: Port B, Pin0"]
    PB0 = 144,
    #[doc = "145: Port B, Pin1"]
    PB1 = 145,
    #[doc = "146: Port B, Pin2"]
    PB2 = 146,
    #[doc = "147: Port B, Pin3"]
    PB3 = 147,
    #[doc = "148: Port B, Pin4"]
    PB4 = 148,
    #[doc = "149: Port B, Pin5"]
    PB5 = 149,
    #[doc = "150: Port B, Pin6"]
    PB6 = 150,
    #[doc = "151: Port B, Pin7"]
    PB7 = 151,
    #[doc = "152: Port B, Pin8"]
    PB8 = 152,
    #[doc = "153: Port B, Pin9"]
    PB9 = 153,
    #[doc = "154: Port B, Pin10"]
    PB10 = 154,
    #[doc = "155: Port B, Pin11"]
    PB11 = 155,
    #[doc = "156: Port B, Pin12"]
    PB12 = 156,
    #[doc = "157: Port B, Pin13"]
    PB13 = 157,
    #[doc = "158: Port B, Pin14"]
    PB14 = 158,
    #[doc = "159: Port B, Pin15"]
    PB15 = 159,
    #[doc = "160: Port C, Pin0"]
    PC0 = 160,
    #[doc = "161: Port C, Pin1"]
    PC1 = 161,
    #[doc = "162: Port C, Pin2"]
    PC2 = 162,
    #[doc = "163: Port C, Pin3"]
    PC3 = 163,
    #[doc = "164: Port C, Pin4"]
    PC4 = 164,
    #[doc = "165: Port C, Pin5"]
    PC5 = 165,
    #[doc = "166: Port C, Pin6"]
    PC6 = 166,
    #[doc = "167: Port C, Pin7"]
    PC7 = 167,
    #[doc = "168: Port C, Pin8"]
    PC8 = 168,
    #[doc = "169: Port C, Pin9"]
    PC9 = 169,
    #[doc = "170: Port C, Pin10"]
    PC10 = 170,
    #[doc = "171: Port C, Pin11"]
    PC11 = 171,
    #[doc = "172: Port C, Pin12"]
    PC12 = 172,
    #[doc = "173: Port C, Pin13"]
    PC13 = 173,
    #[doc = "174: Port C, Pin14"]
    PC14 = 174,
    #[doc = "175: Port C, Pin15"]
    PC15 = 175,
    #[doc = "176: Port D, Pin0"]
    PD0 = 176,
    #[doc = "177: Port D, Pin1"]
    PD1 = 177,
    #[doc = "178: Port D, Pin2"]
    PD2 = 178,
    #[doc = "179: Port D, Pin3"]
    PD3 = 179,
    #[doc = "180: Port D, Pin4"]
    PD4 = 180,
    #[doc = "181: Port D, Pin5"]
    PD5 = 181,
    #[doc = "182: Port D, Pin6"]
    PD6 = 182,
    #[doc = "183: Port D, Pin7"]
    PD7 = 183,
    #[doc = "184: Port D, Pin8"]
    PD8 = 184,
    #[doc = "185: Port D, Pin9"]
    PD9 = 185,
    #[doc = "186: Port D, Pin10"]
    PD10 = 186,
    #[doc = "187: Port D, Pin11"]
    PD11 = 187,
    #[doc = "188: Port D, Pin12"]
    PD12 = 188,
    #[doc = "189: Port D, Pin13"]
    PD13 = 189,
    #[doc = "190: Port D, Pin14"]
    PD14 = 190,
    #[doc = "191: Port D, Pin15"]
    PD15 = 191,
}
impl From<POSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: POSSEL_A) -> Self {
        variant as _
    }
}
impl POSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POSSEL_A> {
        match self.bits {
            0 => Some(POSSEL_A::VSS),
            16 => Some(POSSEL_A::VREFDIVAVDD),
            17 => Some(POSSEL_A::VREFDIVAVDDLP),
            18 => Some(POSSEL_A::VREFDIV1V25),
            19 => Some(POSSEL_A::VREFDIV1V25LP),
            20 => Some(POSSEL_A::VREFDIV2V5),
            21 => Some(POSSEL_A::VREFDIV2V5LP),
            32 => Some(POSSEL_A::VSENSE01DIV4),
            33 => Some(POSSEL_A::VSENSE01DIV4LP),
            34 => Some(POSSEL_A::VSENSE11DIV4),
            35 => Some(POSSEL_A::VSENSE11DIV4LP),
            64 => Some(POSSEL_A::VDACOUT0),
            65 => Some(POSSEL_A::VDACOUT1),
            80 => Some(POSSEL_A::EXTPA),
            81 => Some(POSSEL_A::EXTPB),
            82 => Some(POSSEL_A::EXTPC),
            83 => Some(POSSEL_A::EXTPD),
            128 => Some(POSSEL_A::PA0),
            129 => Some(POSSEL_A::PA1),
            130 => Some(POSSEL_A::PA2),
            131 => Some(POSSEL_A::PA3),
            132 => Some(POSSEL_A::PA4),
            133 => Some(POSSEL_A::PA5),
            134 => Some(POSSEL_A::PA6),
            135 => Some(POSSEL_A::PA7),
            136 => Some(POSSEL_A::PA8),
            137 => Some(POSSEL_A::PA9),
            138 => Some(POSSEL_A::PA10),
            139 => Some(POSSEL_A::PA11),
            140 => Some(POSSEL_A::PA12),
            141 => Some(POSSEL_A::PA13),
            142 => Some(POSSEL_A::PA14),
            143 => Some(POSSEL_A::PA15),
            144 => Some(POSSEL_A::PB0),
            145 => Some(POSSEL_A::PB1),
            146 => Some(POSSEL_A::PB2),
            147 => Some(POSSEL_A::PB3),
            148 => Some(POSSEL_A::PB4),
            149 => Some(POSSEL_A::PB5),
            150 => Some(POSSEL_A::PB6),
            151 => Some(POSSEL_A::PB7),
            152 => Some(POSSEL_A::PB8),
            153 => Some(POSSEL_A::PB9),
            154 => Some(POSSEL_A::PB10),
            155 => Some(POSSEL_A::PB11),
            156 => Some(POSSEL_A::PB12),
            157 => Some(POSSEL_A::PB13),
            158 => Some(POSSEL_A::PB14),
            159 => Some(POSSEL_A::PB15),
            160 => Some(POSSEL_A::PC0),
            161 => Some(POSSEL_A::PC1),
            162 => Some(POSSEL_A::PC2),
            163 => Some(POSSEL_A::PC3),
            164 => Some(POSSEL_A::PC4),
            165 => Some(POSSEL_A::PC5),
            166 => Some(POSSEL_A::PC6),
            167 => Some(POSSEL_A::PC7),
            168 => Some(POSSEL_A::PC8),
            169 => Some(POSSEL_A::PC9),
            170 => Some(POSSEL_A::PC10),
            171 => Some(POSSEL_A::PC11),
            172 => Some(POSSEL_A::PC12),
            173 => Some(POSSEL_A::PC13),
            174 => Some(POSSEL_A::PC14),
            175 => Some(POSSEL_A::PC15),
            176 => Some(POSSEL_A::PD0),
            177 => Some(POSSEL_A::PD1),
            178 => Some(POSSEL_A::PD2),
            179 => Some(POSSEL_A::PD3),
            180 => Some(POSSEL_A::PD4),
            181 => Some(POSSEL_A::PD5),
            182 => Some(POSSEL_A::PD6),
            183 => Some(POSSEL_A::PD7),
            184 => Some(POSSEL_A::PD8),
            185 => Some(POSSEL_A::PD9),
            186 => Some(POSSEL_A::PD10),
            187 => Some(POSSEL_A::PD11),
            188 => Some(POSSEL_A::PD12),
            189 => Some(POSSEL_A::PD13),
            190 => Some(POSSEL_A::PD14),
            191 => Some(POSSEL_A::PD15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == POSSEL_A::VSS
    }
    #[doc = "Checks if the value of the field is `VREFDIVAVDD`"]
    #[inline(always)]
    pub fn is_vrefdivavdd(&self) -> bool {
        *self == POSSEL_A::VREFDIVAVDD
    }
    #[doc = "Checks if the value of the field is `VREFDIVAVDDLP`"]
    #[inline(always)]
    pub fn is_vrefdivavddlp(&self) -> bool {
        *self == POSSEL_A::VREFDIVAVDDLP
    }
    #[doc = "Checks if the value of the field is `VREFDIV1V25`"]
    #[inline(always)]
    pub fn is_vrefdiv1v25(&self) -> bool {
        *self == POSSEL_A::VREFDIV1V25
    }
    #[doc = "Checks if the value of the field is `VREFDIV1V25LP`"]
    #[inline(always)]
    pub fn is_vrefdiv1v25lp(&self) -> bool {
        *self == POSSEL_A::VREFDIV1V25LP
    }
    #[doc = "Checks if the value of the field is `VREFDIV2V5`"]
    #[inline(always)]
    pub fn is_vrefdiv2v5(&self) -> bool {
        *self == POSSEL_A::VREFDIV2V5
    }
    #[doc = "Checks if the value of the field is `VREFDIV2V5LP`"]
    #[inline(always)]
    pub fn is_vrefdiv2v5lp(&self) -> bool {
        *self == POSSEL_A::VREFDIV2V5LP
    }
    #[doc = "Checks if the value of the field is `VSENSE01DIV4`"]
    #[inline(always)]
    pub fn is_vsense01div4(&self) -> bool {
        *self == POSSEL_A::VSENSE01DIV4
    }
    #[doc = "Checks if the value of the field is `VSENSE01DIV4LP`"]
    #[inline(always)]
    pub fn is_vsense01div4lp(&self) -> bool {
        *self == POSSEL_A::VSENSE01DIV4LP
    }
    #[doc = "Checks if the value of the field is `VSENSE11DIV4`"]
    #[inline(always)]
    pub fn is_vsense11div4(&self) -> bool {
        *self == POSSEL_A::VSENSE11DIV4
    }
    #[doc = "Checks if the value of the field is `VSENSE11DIV4LP`"]
    #[inline(always)]
    pub fn is_vsense11div4lp(&self) -> bool {
        *self == POSSEL_A::VSENSE11DIV4LP
    }
    #[doc = "Checks if the value of the field is `VDACOUT0`"]
    #[inline(always)]
    pub fn is_vdacout0(&self) -> bool {
        *self == POSSEL_A::VDACOUT0
    }
    #[doc = "Checks if the value of the field is `VDACOUT1`"]
    #[inline(always)]
    pub fn is_vdacout1(&self) -> bool {
        *self == POSSEL_A::VDACOUT1
    }
    #[doc = "Checks if the value of the field is `EXTPA`"]
    #[inline(always)]
    pub fn is_extpa(&self) -> bool {
        *self == POSSEL_A::EXTPA
    }
    #[doc = "Checks if the value of the field is `EXTPB`"]
    #[inline(always)]
    pub fn is_extpb(&self) -> bool {
        *self == POSSEL_A::EXTPB
    }
    #[doc = "Checks if the value of the field is `EXTPC`"]
    #[inline(always)]
    pub fn is_extpc(&self) -> bool {
        *self == POSSEL_A::EXTPC
    }
    #[doc = "Checks if the value of the field is `EXTPD`"]
    #[inline(always)]
    pub fn is_extpd(&self) -> bool {
        *self == POSSEL_A::EXTPD
    }
    #[doc = "Checks if the value of the field is `PA0`"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == POSSEL_A::PA0
    }
    #[doc = "Checks if the value of the field is `PA1`"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == POSSEL_A::PA1
    }
    #[doc = "Checks if the value of the field is `PA2`"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == POSSEL_A::PA2
    }
    #[doc = "Checks if the value of the field is `PA3`"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == POSSEL_A::PA3
    }
    #[doc = "Checks if the value of the field is `PA4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == POSSEL_A::PA4
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == POSSEL_A::PA5
    }
    #[doc = "Checks if the value of the field is `PA6`"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        *self == POSSEL_A::PA6
    }
    #[doc = "Checks if the value of the field is `PA7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == POSSEL_A::PA7
    }
    #[doc = "Checks if the value of the field is `PA8`"]
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        *self == POSSEL_A::PA8
    }
    #[doc = "Checks if the value of the field is `PA9`"]
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        *self == POSSEL_A::PA9
    }
    #[doc = "Checks if the value of the field is `PA10`"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == POSSEL_A::PA10
    }
    #[doc = "Checks if the value of the field is `PA11`"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == POSSEL_A::PA11
    }
    #[doc = "Checks if the value of the field is `PA12`"]
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == POSSEL_A::PA12
    }
    #[doc = "Checks if the value of the field is `PA13`"]
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == POSSEL_A::PA13
    }
    #[doc = "Checks if the value of the field is `PA14`"]
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == POSSEL_A::PA14
    }
    #[doc = "Checks if the value of the field is `PA15`"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == POSSEL_A::PA15
    }
    #[doc = "Checks if the value of the field is `PB0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == POSSEL_A::PB0
    }
    #[doc = "Checks if the value of the field is `PB1`"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == POSSEL_A::PB1
    }
    #[doc = "Checks if the value of the field is `PB2`"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == POSSEL_A::PB2
    }
    #[doc = "Checks if the value of the field is `PB3`"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == POSSEL_A::PB3
    }
    #[doc = "Checks if the value of the field is `PB4`"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == POSSEL_A::PB4
    }
    #[doc = "Checks if the value of the field is `PB5`"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == POSSEL_A::PB5
    }
    #[doc = "Checks if the value of the field is `PB6`"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == POSSEL_A::PB6
    }
    #[doc = "Checks if the value of the field is `PB7`"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == POSSEL_A::PB7
    }
    #[doc = "Checks if the value of the field is `PB8`"]
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        *self == POSSEL_A::PB8
    }
    #[doc = "Checks if the value of the field is `PB9`"]
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        *self == POSSEL_A::PB9
    }
    #[doc = "Checks if the value of the field is `PB10`"]
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        *self == POSSEL_A::PB10
    }
    #[doc = "Checks if the value of the field is `PB11`"]
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        *self == POSSEL_A::PB11
    }
    #[doc = "Checks if the value of the field is `PB12`"]
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        *self == POSSEL_A::PB12
    }
    #[doc = "Checks if the value of the field is `PB13`"]
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        *self == POSSEL_A::PB13
    }
    #[doc = "Checks if the value of the field is `PB14`"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == POSSEL_A::PB14
    }
    #[doc = "Checks if the value of the field is `PB15`"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == POSSEL_A::PB15
    }
    #[doc = "Checks if the value of the field is `PC0`"]
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == POSSEL_A::PC0
    }
    #[doc = "Checks if the value of the field is `PC1`"]
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == POSSEL_A::PC1
    }
    #[doc = "Checks if the value of the field is `PC2`"]
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == POSSEL_A::PC2
    }
    #[doc = "Checks if the value of the field is `PC3`"]
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == POSSEL_A::PC3
    }
    #[doc = "Checks if the value of the field is `PC4`"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == POSSEL_A::PC4
    }
    #[doc = "Checks if the value of the field is `PC5`"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == POSSEL_A::PC5
    }
    #[doc = "Checks if the value of the field is `PC6`"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        *self == POSSEL_A::PC6
    }
    #[doc = "Checks if the value of the field is `PC7`"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        *self == POSSEL_A::PC7
    }
    #[doc = "Checks if the value of the field is `PC8`"]
    #[inline(always)]
    pub fn is_pc8(&self) -> bool {
        *self == POSSEL_A::PC8
    }
    #[doc = "Checks if the value of the field is `PC9`"]
    #[inline(always)]
    pub fn is_pc9(&self) -> bool {
        *self == POSSEL_A::PC9
    }
    #[doc = "Checks if the value of the field is `PC10`"]
    #[inline(always)]
    pub fn is_pc10(&self) -> bool {
        *self == POSSEL_A::PC10
    }
    #[doc = "Checks if the value of the field is `PC11`"]
    #[inline(always)]
    pub fn is_pc11(&self) -> bool {
        *self == POSSEL_A::PC11
    }
    #[doc = "Checks if the value of the field is `PC12`"]
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        *self == POSSEL_A::PC12
    }
    #[doc = "Checks if the value of the field is `PC13`"]
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == POSSEL_A::PC13
    }
    #[doc = "Checks if the value of the field is `PC14`"]
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == POSSEL_A::PC14
    }
    #[doc = "Checks if the value of the field is `PC15`"]
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == POSSEL_A::PC15
    }
    #[doc = "Checks if the value of the field is `PD0`"]
    #[inline(always)]
    pub fn is_pd0(&self) -> bool {
        *self == POSSEL_A::PD0
    }
    #[doc = "Checks if the value of the field is `PD1`"]
    #[inline(always)]
    pub fn is_pd1(&self) -> bool {
        *self == POSSEL_A::PD1
    }
    #[doc = "Checks if the value of the field is `PD2`"]
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == POSSEL_A::PD2
    }
    #[doc = "Checks if the value of the field is `PD3`"]
    #[inline(always)]
    pub fn is_pd3(&self) -> bool {
        *self == POSSEL_A::PD3
    }
    #[doc = "Checks if the value of the field is `PD4`"]
    #[inline(always)]
    pub fn is_pd4(&self) -> bool {
        *self == POSSEL_A::PD4
    }
    #[doc = "Checks if the value of the field is `PD5`"]
    #[inline(always)]
    pub fn is_pd5(&self) -> bool {
        *self == POSSEL_A::PD5
    }
    #[doc = "Checks if the value of the field is `PD6`"]
    #[inline(always)]
    pub fn is_pd6(&self) -> bool {
        *self == POSSEL_A::PD6
    }
    #[doc = "Checks if the value of the field is `PD7`"]
    #[inline(always)]
    pub fn is_pd7(&self) -> bool {
        *self == POSSEL_A::PD7
    }
    #[doc = "Checks if the value of the field is `PD8`"]
    #[inline(always)]
    pub fn is_pd8(&self) -> bool {
        *self == POSSEL_A::PD8
    }
    #[doc = "Checks if the value of the field is `PD9`"]
    #[inline(always)]
    pub fn is_pd9(&self) -> bool {
        *self == POSSEL_A::PD9
    }
    #[doc = "Checks if the value of the field is `PD10`"]
    #[inline(always)]
    pub fn is_pd10(&self) -> bool {
        *self == POSSEL_A::PD10
    }
    #[doc = "Checks if the value of the field is `PD11`"]
    #[inline(always)]
    pub fn is_pd11(&self) -> bool {
        *self == POSSEL_A::PD11
    }
    #[doc = "Checks if the value of the field is `PD12`"]
    #[inline(always)]
    pub fn is_pd12(&self) -> bool {
        *self == POSSEL_A::PD12
    }
    #[doc = "Checks if the value of the field is `PD13`"]
    #[inline(always)]
    pub fn is_pd13(&self) -> bool {
        *self == POSSEL_A::PD13
    }
    #[doc = "Checks if the value of the field is `PD14`"]
    #[inline(always)]
    pub fn is_pd14(&self) -> bool {
        *self == POSSEL_A::PD14
    }
    #[doc = "Checks if the value of the field is `PD15`"]
    #[inline(always)]
    pub fn is_pd15(&self) -> bool {
        *self == POSSEL_A::PD15
    }
}
#[doc = "Field `POSSEL` writer - Positive Input Select"]
pub type POSSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INPUTCTRL_SPEC, u8, POSSEL_A, 8, O>;
impl<'a, const O: u8> POSSEL_W<'a, O> {
    #[doc = "VSS"]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(POSSEL_A::VSS)
    }
    #[doc = "Divided AVDD"]
    #[inline(always)]
    pub fn vrefdivavdd(self) -> &'a mut W {
        self.variant(POSSEL_A::VREFDIVAVDD)
    }
    #[doc = "Low-Power Divided AVDD"]
    #[inline(always)]
    pub fn vrefdivavddlp(self) -> &'a mut W {
        self.variant(POSSEL_A::VREFDIVAVDDLP)
    }
    #[doc = "Divided 1V25 reference"]
    #[inline(always)]
    pub fn vrefdiv1v25(self) -> &'a mut W {
        self.variant(POSSEL_A::VREFDIV1V25)
    }
    #[doc = "Low-power Divided 1V25 reference"]
    #[inline(always)]
    pub fn vrefdiv1v25lp(self) -> &'a mut W {
        self.variant(POSSEL_A::VREFDIV1V25LP)
    }
    #[doc = "Divided 2V5 reference"]
    #[inline(always)]
    pub fn vrefdiv2v5(self) -> &'a mut W {
        self.variant(POSSEL_A::VREFDIV2V5)
    }
    #[doc = "Low-power Divided 2V5 reference"]
    #[inline(always)]
    pub fn vrefdiv2v5lp(self) -> &'a mut W {
        self.variant(POSSEL_A::VREFDIV2V5LP)
    }
    #[doc = "VSENSE0 divided by 4"]
    #[inline(always)]
    pub fn vsense01div4(self) -> &'a mut W {
        self.variant(POSSEL_A::VSENSE01DIV4)
    }
    #[doc = "Low-power VSENSE0 divided by 4"]
    #[inline(always)]
    pub fn vsense01div4lp(self) -> &'a mut W {
        self.variant(POSSEL_A::VSENSE01DIV4LP)
    }
    #[doc = "VSENSE1 divided by 4"]
    #[inline(always)]
    pub fn vsense11div4(self) -> &'a mut W {
        self.variant(POSSEL_A::VSENSE11DIV4)
    }
    #[doc = "Low-power VSENSE1 divided by 4"]
    #[inline(always)]
    pub fn vsense11div4lp(self) -> &'a mut W {
        self.variant(POSSEL_A::VSENSE11DIV4LP)
    }
    #[doc = "VDAC0 channel 0 output"]
    #[inline(always)]
    pub fn vdacout0(self) -> &'a mut W {
        self.variant(POSSEL_A::VDACOUT0)
    }
    #[doc = "VDAC0 channel 1 output"]
    #[inline(always)]
    pub fn vdacout1(self) -> &'a mut W {
        self.variant(POSSEL_A::VDACOUT1)
    }
    #[doc = "External interface, base is PA0."]
    #[inline(always)]
    pub fn extpa(self) -> &'a mut W {
        self.variant(POSSEL_A::EXTPA)
    }
    #[doc = "External interface, base is PB0."]
    #[inline(always)]
    pub fn extpb(self) -> &'a mut W {
        self.variant(POSSEL_A::EXTPB)
    }
    #[doc = "External interface, base is PC0."]
    #[inline(always)]
    pub fn extpc(self) -> &'a mut W {
        self.variant(POSSEL_A::EXTPC)
    }
    #[doc = "External interface, base is PD0."]
    #[inline(always)]
    pub fn extpd(self) -> &'a mut W {
        self.variant(POSSEL_A::EXTPD)
    }
    #[doc = "Port A, Pin0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut W {
        self.variant(POSSEL_A::PA0)
    }
    #[doc = "Port A, Pin1"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut W {
        self.variant(POSSEL_A::PA1)
    }
    #[doc = "Port A, Pin2"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut W {
        self.variant(POSSEL_A::PA2)
    }
    #[doc = "Port A, Pin3"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut W {
        self.variant(POSSEL_A::PA3)
    }
    #[doc = "Port A, Pin4"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(POSSEL_A::PA4)
    }
    #[doc = "Port A, Pin5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(POSSEL_A::PA5)
    }
    #[doc = "Port A, Pin6"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut W {
        self.variant(POSSEL_A::PA6)
    }
    #[doc = "Port A, Pin7"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(POSSEL_A::PA7)
    }
    #[doc = "Port A, Pin8"]
    #[inline(always)]
    pub fn pa8(self) -> &'a mut W {
        self.variant(POSSEL_A::PA8)
    }
    #[doc = "Port A, Pin9"]
    #[inline(always)]
    pub fn pa9(self) -> &'a mut W {
        self.variant(POSSEL_A::PA9)
    }
    #[doc = "Port A, Pin10"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut W {
        self.variant(POSSEL_A::PA10)
    }
    #[doc = "Port A, Pin11"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut W {
        self.variant(POSSEL_A::PA11)
    }
    #[doc = "Port A, Pin12"]
    #[inline(always)]
    pub fn pa12(self) -> &'a mut W {
        self.variant(POSSEL_A::PA12)
    }
    #[doc = "Port A, Pin13"]
    #[inline(always)]
    pub fn pa13(self) -> &'a mut W {
        self.variant(POSSEL_A::PA13)
    }
    #[doc = "Port A, Pin14"]
    #[inline(always)]
    pub fn pa14(self) -> &'a mut W {
        self.variant(POSSEL_A::PA14)
    }
    #[doc = "Port A, Pin15"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut W {
        self.variant(POSSEL_A::PA15)
    }
    #[doc = "Port B, Pin0"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(POSSEL_A::PB0)
    }
    #[doc = "Port B, Pin1"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut W {
        self.variant(POSSEL_A::PB1)
    }
    #[doc = "Port B, Pin2"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut W {
        self.variant(POSSEL_A::PB2)
    }
    #[doc = "Port B, Pin3"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut W {
        self.variant(POSSEL_A::PB3)
    }
    #[doc = "Port B, Pin4"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut W {
        self.variant(POSSEL_A::PB4)
    }
    #[doc = "Port B, Pin5"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut W {
        self.variant(POSSEL_A::PB5)
    }
    #[doc = "Port B, Pin6"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut W {
        self.variant(POSSEL_A::PB6)
    }
    #[doc = "Port B, Pin7"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut W {
        self.variant(POSSEL_A::PB7)
    }
    #[doc = "Port B, Pin8"]
    #[inline(always)]
    pub fn pb8(self) -> &'a mut W {
        self.variant(POSSEL_A::PB8)
    }
    #[doc = "Port B, Pin9"]
    #[inline(always)]
    pub fn pb9(self) -> &'a mut W {
        self.variant(POSSEL_A::PB9)
    }
    #[doc = "Port B, Pin10"]
    #[inline(always)]
    pub fn pb10(self) -> &'a mut W {
        self.variant(POSSEL_A::PB10)
    }
    #[doc = "Port B, Pin11"]
    #[inline(always)]
    pub fn pb11(self) -> &'a mut W {
        self.variant(POSSEL_A::PB11)
    }
    #[doc = "Port B, Pin12"]
    #[inline(always)]
    pub fn pb12(self) -> &'a mut W {
        self.variant(POSSEL_A::PB12)
    }
    #[doc = "Port B, Pin13"]
    #[inline(always)]
    pub fn pb13(self) -> &'a mut W {
        self.variant(POSSEL_A::PB13)
    }
    #[doc = "Port B, Pin14"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(POSSEL_A::PB14)
    }
    #[doc = "Port B, Pin15"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(POSSEL_A::PB15)
    }
    #[doc = "Port C, Pin0"]
    #[inline(always)]
    pub fn pc0(self) -> &'a mut W {
        self.variant(POSSEL_A::PC0)
    }
    #[doc = "Port C, Pin1"]
    #[inline(always)]
    pub fn pc1(self) -> &'a mut W {
        self.variant(POSSEL_A::PC1)
    }
    #[doc = "Port C, Pin2"]
    #[inline(always)]
    pub fn pc2(self) -> &'a mut W {
        self.variant(POSSEL_A::PC2)
    }
    #[doc = "Port C, Pin3"]
    #[inline(always)]
    pub fn pc3(self) -> &'a mut W {
        self.variant(POSSEL_A::PC3)
    }
    #[doc = "Port C, Pin4"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut W {
        self.variant(POSSEL_A::PC4)
    }
    #[doc = "Port C, Pin5"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut W {
        self.variant(POSSEL_A::PC5)
    }
    #[doc = "Port C, Pin6"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut W {
        self.variant(POSSEL_A::PC6)
    }
    #[doc = "Port C, Pin7"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut W {
        self.variant(POSSEL_A::PC7)
    }
    #[doc = "Port C, Pin8"]
    #[inline(always)]
    pub fn pc8(self) -> &'a mut W {
        self.variant(POSSEL_A::PC8)
    }
    #[doc = "Port C, Pin9"]
    #[inline(always)]
    pub fn pc9(self) -> &'a mut W {
        self.variant(POSSEL_A::PC9)
    }
    #[doc = "Port C, Pin10"]
    #[inline(always)]
    pub fn pc10(self) -> &'a mut W {
        self.variant(POSSEL_A::PC10)
    }
    #[doc = "Port C, Pin11"]
    #[inline(always)]
    pub fn pc11(self) -> &'a mut W {
        self.variant(POSSEL_A::PC11)
    }
    #[doc = "Port C, Pin12"]
    #[inline(always)]
    pub fn pc12(self) -> &'a mut W {
        self.variant(POSSEL_A::PC12)
    }
    #[doc = "Port C, Pin13"]
    #[inline(always)]
    pub fn pc13(self) -> &'a mut W {
        self.variant(POSSEL_A::PC13)
    }
    #[doc = "Port C, Pin14"]
    #[inline(always)]
    pub fn pc14(self) -> &'a mut W {
        self.variant(POSSEL_A::PC14)
    }
    #[doc = "Port C, Pin15"]
    #[inline(always)]
    pub fn pc15(self) -> &'a mut W {
        self.variant(POSSEL_A::PC15)
    }
    #[doc = "Port D, Pin0"]
    #[inline(always)]
    pub fn pd0(self) -> &'a mut W {
        self.variant(POSSEL_A::PD0)
    }
    #[doc = "Port D, Pin1"]
    #[inline(always)]
    pub fn pd1(self) -> &'a mut W {
        self.variant(POSSEL_A::PD1)
    }
    #[doc = "Port D, Pin2"]
    #[inline(always)]
    pub fn pd2(self) -> &'a mut W {
        self.variant(POSSEL_A::PD2)
    }
    #[doc = "Port D, Pin3"]
    #[inline(always)]
    pub fn pd3(self) -> &'a mut W {
        self.variant(POSSEL_A::PD3)
    }
    #[doc = "Port D, Pin4"]
    #[inline(always)]
    pub fn pd4(self) -> &'a mut W {
        self.variant(POSSEL_A::PD4)
    }
    #[doc = "Port D, Pin5"]
    #[inline(always)]
    pub fn pd5(self) -> &'a mut W {
        self.variant(POSSEL_A::PD5)
    }
    #[doc = "Port D, Pin6"]
    #[inline(always)]
    pub fn pd6(self) -> &'a mut W {
        self.variant(POSSEL_A::PD6)
    }
    #[doc = "Port D, Pin7"]
    #[inline(always)]
    pub fn pd7(self) -> &'a mut W {
        self.variant(POSSEL_A::PD7)
    }
    #[doc = "Port D, Pin8"]
    #[inline(always)]
    pub fn pd8(self) -> &'a mut W {
        self.variant(POSSEL_A::PD8)
    }
    #[doc = "Port D, Pin9"]
    #[inline(always)]
    pub fn pd9(self) -> &'a mut W {
        self.variant(POSSEL_A::PD9)
    }
    #[doc = "Port D, Pin10"]
    #[inline(always)]
    pub fn pd10(self) -> &'a mut W {
        self.variant(POSSEL_A::PD10)
    }
    #[doc = "Port D, Pin11"]
    #[inline(always)]
    pub fn pd11(self) -> &'a mut W {
        self.variant(POSSEL_A::PD11)
    }
    #[doc = "Port D, Pin12"]
    #[inline(always)]
    pub fn pd12(self) -> &'a mut W {
        self.variant(POSSEL_A::PD12)
    }
    #[doc = "Port D, Pin13"]
    #[inline(always)]
    pub fn pd13(self) -> &'a mut W {
        self.variant(POSSEL_A::PD13)
    }
    #[doc = "Port D, Pin14"]
    #[inline(always)]
    pub fn pd14(self) -> &'a mut W {
        self.variant(POSSEL_A::PD14)
    }
    #[doc = "Port D, Pin15"]
    #[inline(always)]
    pub fn pd15(self) -> &'a mut W {
        self.variant(POSSEL_A::PD15)
    }
}
#[doc = "Field `NEGSEL` reader - Negative Input Select"]
pub type NEGSEL_R = crate::FieldReader<u8, NEGSEL_A>;
#[doc = "Negative Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NEGSEL_A {
    #[doc = "0: VSS"]
    VSS = 0,
    #[doc = "16: Divided AVDD"]
    VREFDIVAVDD = 16,
    #[doc = "17: Low-Power Divided AVDD"]
    VREFDIVAVDDLP = 17,
    #[doc = "18: Divided 1V25 reference"]
    VREFDIV1V25 = 18,
    #[doc = "19: Low-power Divided 1V25 reference"]
    VREFDIV1V25LP = 19,
    #[doc = "20: Divided 2V5 reference"]
    VREFDIV2V5 = 20,
    #[doc = "21: Low-power Divided 2V5 reference"]
    VREFDIV2V5LP = 21,
    #[doc = "32: VSENSE0 divided by 4"]
    VSENSE01DIV4 = 32,
    #[doc = "33: Low-power VSENSE0 divided by 4"]
    VSENSE01DIV4LP = 33,
    #[doc = "34: VSENSE1 divided by 4"]
    VSENSE11DIV4 = 34,
    #[doc = "35: Low-power VSENSE1 divided by 4"]
    VSENSE11DIV4LP = 35,
    #[doc = "48: Capsense mode"]
    CAPSENSE = 48,
    #[doc = "64: VDAC0 channel 0 output"]
    VDACOUT0 = 64,
    #[doc = "65: VDAC0 channel 1 output"]
    VDACOUT1 = 65,
    #[doc = "128: Port A, Pin0"]
    PA0 = 128,
    #[doc = "129: Port A, Pin1"]
    PA1 = 129,
    #[doc = "130: Port A, Pin2"]
    PA2 = 130,
    #[doc = "131: Port A, Pin3"]
    PA3 = 131,
    #[doc = "132: Port A, Pin4"]
    PA4 = 132,
    #[doc = "133: Port A, Pin5"]
    PA5 = 133,
    #[doc = "134: Port A, Pin6"]
    PA6 = 134,
    #[doc = "135: Port A, Pin7"]
    PA7 = 135,
    #[doc = "136: Port A, Pin8"]
    PA8 = 136,
    #[doc = "137: Port A, Pin9"]
    PA9 = 137,
    #[doc = "138: Port A, Pin10"]
    PA10 = 138,
    #[doc = "139: Port A, Pin11"]
    PA11 = 139,
    #[doc = "140: Port A, Pin12"]
    PA12 = 140,
    #[doc = "141: Port A, Pin13"]
    PA13 = 141,
    #[doc = "142: Port A, Pin14"]
    PA14 = 142,
    #[doc = "143: Port A, Pin15"]
    PA15 = 143,
    #[doc = "144: Port B, Pin0"]
    PB0 = 144,
    #[doc = "145: Port B, Pin1"]
    PB1 = 145,
    #[doc = "146: Port B, Pin2"]
    PB2 = 146,
    #[doc = "147: Port B, Pin3"]
    PB3 = 147,
    #[doc = "148: Port B, Pin4"]
    PB4 = 148,
    #[doc = "149: Port B, Pin5"]
    PB5 = 149,
    #[doc = "150: Port B, Pin6"]
    PB6 = 150,
    #[doc = "151: Port B, Pin7"]
    PB7 = 151,
    #[doc = "152: Port B, Pin8"]
    PB8 = 152,
    #[doc = "153: Port B, Pin9"]
    PB9 = 153,
    #[doc = "154: Port B, Pin10"]
    PB10 = 154,
    #[doc = "155: Port B, Pin11"]
    PB11 = 155,
    #[doc = "156: Port B, Pin12"]
    PB12 = 156,
    #[doc = "157: Port B, Pin13"]
    PB13 = 157,
    #[doc = "158: Port B, Pin14"]
    PB14 = 158,
    #[doc = "159: Port B, Pin15"]
    PB15 = 159,
    #[doc = "160: Port C, Pin0"]
    PC0 = 160,
    #[doc = "161: Port C, Pin1"]
    PC1 = 161,
    #[doc = "162: Port C, Pin2"]
    PC2 = 162,
    #[doc = "163: Port C, Pin3"]
    PC3 = 163,
    #[doc = "164: Port C, Pin4"]
    PC4 = 164,
    #[doc = "165: Port C, Pin5"]
    PC5 = 165,
    #[doc = "166: Port C, Pin6"]
    PC6 = 166,
    #[doc = "167: Port C, Pin7"]
    PC7 = 167,
    #[doc = "168: Port C, Pin8"]
    PC8 = 168,
    #[doc = "169: Port C, Pin9"]
    PC9 = 169,
    #[doc = "170: Port C, Pin10"]
    PC10 = 170,
    #[doc = "171: Port C, Pin11"]
    PC11 = 171,
    #[doc = "172: Port C, Pin12"]
    PC12 = 172,
    #[doc = "173: Port C, Pin13"]
    PC13 = 173,
    #[doc = "174: Port C, Pin14"]
    PC14 = 174,
    #[doc = "175: Port C, Pin15"]
    PC15 = 175,
    #[doc = "176: Port D, Pin0"]
    PD0 = 176,
    #[doc = "177: Port D, Pin1"]
    PD1 = 177,
    #[doc = "178: Port D, Pin2"]
    PD2 = 178,
    #[doc = "179: Port D, Pin3"]
    PD3 = 179,
    #[doc = "180: Port D, Pin4"]
    PD4 = 180,
    #[doc = "181: Port D, Pin5"]
    PD5 = 181,
    #[doc = "182: Port D, Pin6"]
    PD6 = 182,
    #[doc = "183: Port D, Pin7"]
    PD7 = 183,
    #[doc = "184: Port D, Pin8"]
    PD8 = 184,
    #[doc = "185: Port D, Pin9"]
    PD9 = 185,
    #[doc = "186: Port D, Pin10"]
    PD10 = 186,
    #[doc = "187: Port D, Pin11"]
    PD11 = 187,
    #[doc = "188: Port D, Pin12"]
    PD12 = 188,
    #[doc = "189: Port D, Pin13"]
    PD13 = 189,
    #[doc = "190: Port D, Pin14"]
    PD14 = 190,
    #[doc = "191: Port D, Pin15"]
    PD15 = 191,
}
impl From<NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NEGSEL_A) -> Self {
        variant as _
    }
}
impl NEGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NEGSEL_A> {
        match self.bits {
            0 => Some(NEGSEL_A::VSS),
            16 => Some(NEGSEL_A::VREFDIVAVDD),
            17 => Some(NEGSEL_A::VREFDIVAVDDLP),
            18 => Some(NEGSEL_A::VREFDIV1V25),
            19 => Some(NEGSEL_A::VREFDIV1V25LP),
            20 => Some(NEGSEL_A::VREFDIV2V5),
            21 => Some(NEGSEL_A::VREFDIV2V5LP),
            32 => Some(NEGSEL_A::VSENSE01DIV4),
            33 => Some(NEGSEL_A::VSENSE01DIV4LP),
            34 => Some(NEGSEL_A::VSENSE11DIV4),
            35 => Some(NEGSEL_A::VSENSE11DIV4LP),
            48 => Some(NEGSEL_A::CAPSENSE),
            64 => Some(NEGSEL_A::VDACOUT0),
            65 => Some(NEGSEL_A::VDACOUT1),
            128 => Some(NEGSEL_A::PA0),
            129 => Some(NEGSEL_A::PA1),
            130 => Some(NEGSEL_A::PA2),
            131 => Some(NEGSEL_A::PA3),
            132 => Some(NEGSEL_A::PA4),
            133 => Some(NEGSEL_A::PA5),
            134 => Some(NEGSEL_A::PA6),
            135 => Some(NEGSEL_A::PA7),
            136 => Some(NEGSEL_A::PA8),
            137 => Some(NEGSEL_A::PA9),
            138 => Some(NEGSEL_A::PA10),
            139 => Some(NEGSEL_A::PA11),
            140 => Some(NEGSEL_A::PA12),
            141 => Some(NEGSEL_A::PA13),
            142 => Some(NEGSEL_A::PA14),
            143 => Some(NEGSEL_A::PA15),
            144 => Some(NEGSEL_A::PB0),
            145 => Some(NEGSEL_A::PB1),
            146 => Some(NEGSEL_A::PB2),
            147 => Some(NEGSEL_A::PB3),
            148 => Some(NEGSEL_A::PB4),
            149 => Some(NEGSEL_A::PB5),
            150 => Some(NEGSEL_A::PB6),
            151 => Some(NEGSEL_A::PB7),
            152 => Some(NEGSEL_A::PB8),
            153 => Some(NEGSEL_A::PB9),
            154 => Some(NEGSEL_A::PB10),
            155 => Some(NEGSEL_A::PB11),
            156 => Some(NEGSEL_A::PB12),
            157 => Some(NEGSEL_A::PB13),
            158 => Some(NEGSEL_A::PB14),
            159 => Some(NEGSEL_A::PB15),
            160 => Some(NEGSEL_A::PC0),
            161 => Some(NEGSEL_A::PC1),
            162 => Some(NEGSEL_A::PC2),
            163 => Some(NEGSEL_A::PC3),
            164 => Some(NEGSEL_A::PC4),
            165 => Some(NEGSEL_A::PC5),
            166 => Some(NEGSEL_A::PC6),
            167 => Some(NEGSEL_A::PC7),
            168 => Some(NEGSEL_A::PC8),
            169 => Some(NEGSEL_A::PC9),
            170 => Some(NEGSEL_A::PC10),
            171 => Some(NEGSEL_A::PC11),
            172 => Some(NEGSEL_A::PC12),
            173 => Some(NEGSEL_A::PC13),
            174 => Some(NEGSEL_A::PC14),
            175 => Some(NEGSEL_A::PC15),
            176 => Some(NEGSEL_A::PD0),
            177 => Some(NEGSEL_A::PD1),
            178 => Some(NEGSEL_A::PD2),
            179 => Some(NEGSEL_A::PD3),
            180 => Some(NEGSEL_A::PD4),
            181 => Some(NEGSEL_A::PD5),
            182 => Some(NEGSEL_A::PD6),
            183 => Some(NEGSEL_A::PD7),
            184 => Some(NEGSEL_A::PD8),
            185 => Some(NEGSEL_A::PD9),
            186 => Some(NEGSEL_A::PD10),
            187 => Some(NEGSEL_A::PD11),
            188 => Some(NEGSEL_A::PD12),
            189 => Some(NEGSEL_A::PD13),
            190 => Some(NEGSEL_A::PD14),
            191 => Some(NEGSEL_A::PD15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == NEGSEL_A::VSS
    }
    #[doc = "Checks if the value of the field is `VREFDIVAVDD`"]
    #[inline(always)]
    pub fn is_vrefdivavdd(&self) -> bool {
        *self == NEGSEL_A::VREFDIVAVDD
    }
    #[doc = "Checks if the value of the field is `VREFDIVAVDDLP`"]
    #[inline(always)]
    pub fn is_vrefdivavddlp(&self) -> bool {
        *self == NEGSEL_A::VREFDIVAVDDLP
    }
    #[doc = "Checks if the value of the field is `VREFDIV1V25`"]
    #[inline(always)]
    pub fn is_vrefdiv1v25(&self) -> bool {
        *self == NEGSEL_A::VREFDIV1V25
    }
    #[doc = "Checks if the value of the field is `VREFDIV1V25LP`"]
    #[inline(always)]
    pub fn is_vrefdiv1v25lp(&self) -> bool {
        *self == NEGSEL_A::VREFDIV1V25LP
    }
    #[doc = "Checks if the value of the field is `VREFDIV2V5`"]
    #[inline(always)]
    pub fn is_vrefdiv2v5(&self) -> bool {
        *self == NEGSEL_A::VREFDIV2V5
    }
    #[doc = "Checks if the value of the field is `VREFDIV2V5LP`"]
    #[inline(always)]
    pub fn is_vrefdiv2v5lp(&self) -> bool {
        *self == NEGSEL_A::VREFDIV2V5LP
    }
    #[doc = "Checks if the value of the field is `VSENSE01DIV4`"]
    #[inline(always)]
    pub fn is_vsense01div4(&self) -> bool {
        *self == NEGSEL_A::VSENSE01DIV4
    }
    #[doc = "Checks if the value of the field is `VSENSE01DIV4LP`"]
    #[inline(always)]
    pub fn is_vsense01div4lp(&self) -> bool {
        *self == NEGSEL_A::VSENSE01DIV4LP
    }
    #[doc = "Checks if the value of the field is `VSENSE11DIV4`"]
    #[inline(always)]
    pub fn is_vsense11div4(&self) -> bool {
        *self == NEGSEL_A::VSENSE11DIV4
    }
    #[doc = "Checks if the value of the field is `VSENSE11DIV4LP`"]
    #[inline(always)]
    pub fn is_vsense11div4lp(&self) -> bool {
        *self == NEGSEL_A::VSENSE11DIV4LP
    }
    #[doc = "Checks if the value of the field is `CAPSENSE`"]
    #[inline(always)]
    pub fn is_capsense(&self) -> bool {
        *self == NEGSEL_A::CAPSENSE
    }
    #[doc = "Checks if the value of the field is `VDACOUT0`"]
    #[inline(always)]
    pub fn is_vdacout0(&self) -> bool {
        *self == NEGSEL_A::VDACOUT0
    }
    #[doc = "Checks if the value of the field is `VDACOUT1`"]
    #[inline(always)]
    pub fn is_vdacout1(&self) -> bool {
        *self == NEGSEL_A::VDACOUT1
    }
    #[doc = "Checks if the value of the field is `PA0`"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == NEGSEL_A::PA0
    }
    #[doc = "Checks if the value of the field is `PA1`"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == NEGSEL_A::PA1
    }
    #[doc = "Checks if the value of the field is `PA2`"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == NEGSEL_A::PA2
    }
    #[doc = "Checks if the value of the field is `PA3`"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == NEGSEL_A::PA3
    }
    #[doc = "Checks if the value of the field is `PA4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == NEGSEL_A::PA4
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == NEGSEL_A::PA5
    }
    #[doc = "Checks if the value of the field is `PA6`"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        *self == NEGSEL_A::PA6
    }
    #[doc = "Checks if the value of the field is `PA7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == NEGSEL_A::PA7
    }
    #[doc = "Checks if the value of the field is `PA8`"]
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        *self == NEGSEL_A::PA8
    }
    #[doc = "Checks if the value of the field is `PA9`"]
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        *self == NEGSEL_A::PA9
    }
    #[doc = "Checks if the value of the field is `PA10`"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == NEGSEL_A::PA10
    }
    #[doc = "Checks if the value of the field is `PA11`"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == NEGSEL_A::PA11
    }
    #[doc = "Checks if the value of the field is `PA12`"]
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == NEGSEL_A::PA12
    }
    #[doc = "Checks if the value of the field is `PA13`"]
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == NEGSEL_A::PA13
    }
    #[doc = "Checks if the value of the field is `PA14`"]
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == NEGSEL_A::PA14
    }
    #[doc = "Checks if the value of the field is `PA15`"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == NEGSEL_A::PA15
    }
    #[doc = "Checks if the value of the field is `PB0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == NEGSEL_A::PB0
    }
    #[doc = "Checks if the value of the field is `PB1`"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == NEGSEL_A::PB1
    }
    #[doc = "Checks if the value of the field is `PB2`"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == NEGSEL_A::PB2
    }
    #[doc = "Checks if the value of the field is `PB3`"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == NEGSEL_A::PB3
    }
    #[doc = "Checks if the value of the field is `PB4`"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == NEGSEL_A::PB4
    }
    #[doc = "Checks if the value of the field is `PB5`"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == NEGSEL_A::PB5
    }
    #[doc = "Checks if the value of the field is `PB6`"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == NEGSEL_A::PB6
    }
    #[doc = "Checks if the value of the field is `PB7`"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == NEGSEL_A::PB7
    }
    #[doc = "Checks if the value of the field is `PB8`"]
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        *self == NEGSEL_A::PB8
    }
    #[doc = "Checks if the value of the field is `PB9`"]
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        *self == NEGSEL_A::PB9
    }
    #[doc = "Checks if the value of the field is `PB10`"]
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        *self == NEGSEL_A::PB10
    }
    #[doc = "Checks if the value of the field is `PB11`"]
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        *self == NEGSEL_A::PB11
    }
    #[doc = "Checks if the value of the field is `PB12`"]
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        *self == NEGSEL_A::PB12
    }
    #[doc = "Checks if the value of the field is `PB13`"]
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        *self == NEGSEL_A::PB13
    }
    #[doc = "Checks if the value of the field is `PB14`"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == NEGSEL_A::PB14
    }
    #[doc = "Checks if the value of the field is `PB15`"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == NEGSEL_A::PB15
    }
    #[doc = "Checks if the value of the field is `PC0`"]
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == NEGSEL_A::PC0
    }
    #[doc = "Checks if the value of the field is `PC1`"]
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == NEGSEL_A::PC1
    }
    #[doc = "Checks if the value of the field is `PC2`"]
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == NEGSEL_A::PC2
    }
    #[doc = "Checks if the value of the field is `PC3`"]
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == NEGSEL_A::PC3
    }
    #[doc = "Checks if the value of the field is `PC4`"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == NEGSEL_A::PC4
    }
    #[doc = "Checks if the value of the field is `PC5`"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == NEGSEL_A::PC5
    }
    #[doc = "Checks if the value of the field is `PC6`"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        *self == NEGSEL_A::PC6
    }
    #[doc = "Checks if the value of the field is `PC7`"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        *self == NEGSEL_A::PC7
    }
    #[doc = "Checks if the value of the field is `PC8`"]
    #[inline(always)]
    pub fn is_pc8(&self) -> bool {
        *self == NEGSEL_A::PC8
    }
    #[doc = "Checks if the value of the field is `PC9`"]
    #[inline(always)]
    pub fn is_pc9(&self) -> bool {
        *self == NEGSEL_A::PC9
    }
    #[doc = "Checks if the value of the field is `PC10`"]
    #[inline(always)]
    pub fn is_pc10(&self) -> bool {
        *self == NEGSEL_A::PC10
    }
    #[doc = "Checks if the value of the field is `PC11`"]
    #[inline(always)]
    pub fn is_pc11(&self) -> bool {
        *self == NEGSEL_A::PC11
    }
    #[doc = "Checks if the value of the field is `PC12`"]
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        *self == NEGSEL_A::PC12
    }
    #[doc = "Checks if the value of the field is `PC13`"]
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == NEGSEL_A::PC13
    }
    #[doc = "Checks if the value of the field is `PC14`"]
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == NEGSEL_A::PC14
    }
    #[doc = "Checks if the value of the field is `PC15`"]
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == NEGSEL_A::PC15
    }
    #[doc = "Checks if the value of the field is `PD0`"]
    #[inline(always)]
    pub fn is_pd0(&self) -> bool {
        *self == NEGSEL_A::PD0
    }
    #[doc = "Checks if the value of the field is `PD1`"]
    #[inline(always)]
    pub fn is_pd1(&self) -> bool {
        *self == NEGSEL_A::PD1
    }
    #[doc = "Checks if the value of the field is `PD2`"]
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == NEGSEL_A::PD2
    }
    #[doc = "Checks if the value of the field is `PD3`"]
    #[inline(always)]
    pub fn is_pd3(&self) -> bool {
        *self == NEGSEL_A::PD3
    }
    #[doc = "Checks if the value of the field is `PD4`"]
    #[inline(always)]
    pub fn is_pd4(&self) -> bool {
        *self == NEGSEL_A::PD4
    }
    #[doc = "Checks if the value of the field is `PD5`"]
    #[inline(always)]
    pub fn is_pd5(&self) -> bool {
        *self == NEGSEL_A::PD5
    }
    #[doc = "Checks if the value of the field is `PD6`"]
    #[inline(always)]
    pub fn is_pd6(&self) -> bool {
        *self == NEGSEL_A::PD6
    }
    #[doc = "Checks if the value of the field is `PD7`"]
    #[inline(always)]
    pub fn is_pd7(&self) -> bool {
        *self == NEGSEL_A::PD7
    }
    #[doc = "Checks if the value of the field is `PD8`"]
    #[inline(always)]
    pub fn is_pd8(&self) -> bool {
        *self == NEGSEL_A::PD8
    }
    #[doc = "Checks if the value of the field is `PD9`"]
    #[inline(always)]
    pub fn is_pd9(&self) -> bool {
        *self == NEGSEL_A::PD9
    }
    #[doc = "Checks if the value of the field is `PD10`"]
    #[inline(always)]
    pub fn is_pd10(&self) -> bool {
        *self == NEGSEL_A::PD10
    }
    #[doc = "Checks if the value of the field is `PD11`"]
    #[inline(always)]
    pub fn is_pd11(&self) -> bool {
        *self == NEGSEL_A::PD11
    }
    #[doc = "Checks if the value of the field is `PD12`"]
    #[inline(always)]
    pub fn is_pd12(&self) -> bool {
        *self == NEGSEL_A::PD12
    }
    #[doc = "Checks if the value of the field is `PD13`"]
    #[inline(always)]
    pub fn is_pd13(&self) -> bool {
        *self == NEGSEL_A::PD13
    }
    #[doc = "Checks if the value of the field is `PD14`"]
    #[inline(always)]
    pub fn is_pd14(&self) -> bool {
        *self == NEGSEL_A::PD14
    }
    #[doc = "Checks if the value of the field is `PD15`"]
    #[inline(always)]
    pub fn is_pd15(&self) -> bool {
        *self == NEGSEL_A::PD15
    }
}
#[doc = "Field `NEGSEL` writer - Negative Input Select"]
pub type NEGSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INPUTCTRL_SPEC, u8, NEGSEL_A, 8, O>;
impl<'a, const O: u8> NEGSEL_W<'a, O> {
    #[doc = "VSS"]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(NEGSEL_A::VSS)
    }
    #[doc = "Divided AVDD"]
    #[inline(always)]
    pub fn vrefdivavdd(self) -> &'a mut W {
        self.variant(NEGSEL_A::VREFDIVAVDD)
    }
    #[doc = "Low-Power Divided AVDD"]
    #[inline(always)]
    pub fn vrefdivavddlp(self) -> &'a mut W {
        self.variant(NEGSEL_A::VREFDIVAVDDLP)
    }
    #[doc = "Divided 1V25 reference"]
    #[inline(always)]
    pub fn vrefdiv1v25(self) -> &'a mut W {
        self.variant(NEGSEL_A::VREFDIV1V25)
    }
    #[doc = "Low-power Divided 1V25 reference"]
    #[inline(always)]
    pub fn vrefdiv1v25lp(self) -> &'a mut W {
        self.variant(NEGSEL_A::VREFDIV1V25LP)
    }
    #[doc = "Divided 2V5 reference"]
    #[inline(always)]
    pub fn vrefdiv2v5(self) -> &'a mut W {
        self.variant(NEGSEL_A::VREFDIV2V5)
    }
    #[doc = "Low-power Divided 2V5 reference"]
    #[inline(always)]
    pub fn vrefdiv2v5lp(self) -> &'a mut W {
        self.variant(NEGSEL_A::VREFDIV2V5LP)
    }
    #[doc = "VSENSE0 divided by 4"]
    #[inline(always)]
    pub fn vsense01div4(self) -> &'a mut W {
        self.variant(NEGSEL_A::VSENSE01DIV4)
    }
    #[doc = "Low-power VSENSE0 divided by 4"]
    #[inline(always)]
    pub fn vsense01div4lp(self) -> &'a mut W {
        self.variant(NEGSEL_A::VSENSE01DIV4LP)
    }
    #[doc = "VSENSE1 divided by 4"]
    #[inline(always)]
    pub fn vsense11div4(self) -> &'a mut W {
        self.variant(NEGSEL_A::VSENSE11DIV4)
    }
    #[doc = "Low-power VSENSE1 divided by 4"]
    #[inline(always)]
    pub fn vsense11div4lp(self) -> &'a mut W {
        self.variant(NEGSEL_A::VSENSE11DIV4LP)
    }
    #[doc = "Capsense mode"]
    #[inline(always)]
    pub fn capsense(self) -> &'a mut W {
        self.variant(NEGSEL_A::CAPSENSE)
    }
    #[doc = "VDAC0 channel 0 output"]
    #[inline(always)]
    pub fn vdacout0(self) -> &'a mut W {
        self.variant(NEGSEL_A::VDACOUT0)
    }
    #[doc = "VDAC0 channel 1 output"]
    #[inline(always)]
    pub fn vdacout1(self) -> &'a mut W {
        self.variant(NEGSEL_A::VDACOUT1)
    }
    #[doc = "Port A, Pin0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA0)
    }
    #[doc = "Port A, Pin1"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA1)
    }
    #[doc = "Port A, Pin2"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA2)
    }
    #[doc = "Port A, Pin3"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA3)
    }
    #[doc = "Port A, Pin4"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA4)
    }
    #[doc = "Port A, Pin5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA5)
    }
    #[doc = "Port A, Pin6"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA6)
    }
    #[doc = "Port A, Pin7"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA7)
    }
    #[doc = "Port A, Pin8"]
    #[inline(always)]
    pub fn pa8(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA8)
    }
    #[doc = "Port A, Pin9"]
    #[inline(always)]
    pub fn pa9(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA9)
    }
    #[doc = "Port A, Pin10"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA10)
    }
    #[doc = "Port A, Pin11"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA11)
    }
    #[doc = "Port A, Pin12"]
    #[inline(always)]
    pub fn pa12(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA12)
    }
    #[doc = "Port A, Pin13"]
    #[inline(always)]
    pub fn pa13(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA13)
    }
    #[doc = "Port A, Pin14"]
    #[inline(always)]
    pub fn pa14(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA14)
    }
    #[doc = "Port A, Pin15"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut W {
        self.variant(NEGSEL_A::PA15)
    }
    #[doc = "Port B, Pin0"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB0)
    }
    #[doc = "Port B, Pin1"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB1)
    }
    #[doc = "Port B, Pin2"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB2)
    }
    #[doc = "Port B, Pin3"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB3)
    }
    #[doc = "Port B, Pin4"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB4)
    }
    #[doc = "Port B, Pin5"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB5)
    }
    #[doc = "Port B, Pin6"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB6)
    }
    #[doc = "Port B, Pin7"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB7)
    }
    #[doc = "Port B, Pin8"]
    #[inline(always)]
    pub fn pb8(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB8)
    }
    #[doc = "Port B, Pin9"]
    #[inline(always)]
    pub fn pb9(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB9)
    }
    #[doc = "Port B, Pin10"]
    #[inline(always)]
    pub fn pb10(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB10)
    }
    #[doc = "Port B, Pin11"]
    #[inline(always)]
    pub fn pb11(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB11)
    }
    #[doc = "Port B, Pin12"]
    #[inline(always)]
    pub fn pb12(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB12)
    }
    #[doc = "Port B, Pin13"]
    #[inline(always)]
    pub fn pb13(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB13)
    }
    #[doc = "Port B, Pin14"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB14)
    }
    #[doc = "Port B, Pin15"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(NEGSEL_A::PB15)
    }
    #[doc = "Port C, Pin0"]
    #[inline(always)]
    pub fn pc0(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC0)
    }
    #[doc = "Port C, Pin1"]
    #[inline(always)]
    pub fn pc1(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC1)
    }
    #[doc = "Port C, Pin2"]
    #[inline(always)]
    pub fn pc2(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC2)
    }
    #[doc = "Port C, Pin3"]
    #[inline(always)]
    pub fn pc3(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC3)
    }
    #[doc = "Port C, Pin4"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC4)
    }
    #[doc = "Port C, Pin5"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC5)
    }
    #[doc = "Port C, Pin6"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC6)
    }
    #[doc = "Port C, Pin7"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC7)
    }
    #[doc = "Port C, Pin8"]
    #[inline(always)]
    pub fn pc8(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC8)
    }
    #[doc = "Port C, Pin9"]
    #[inline(always)]
    pub fn pc9(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC9)
    }
    #[doc = "Port C, Pin10"]
    #[inline(always)]
    pub fn pc10(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC10)
    }
    #[doc = "Port C, Pin11"]
    #[inline(always)]
    pub fn pc11(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC11)
    }
    #[doc = "Port C, Pin12"]
    #[inline(always)]
    pub fn pc12(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC12)
    }
    #[doc = "Port C, Pin13"]
    #[inline(always)]
    pub fn pc13(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC13)
    }
    #[doc = "Port C, Pin14"]
    #[inline(always)]
    pub fn pc14(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC14)
    }
    #[doc = "Port C, Pin15"]
    #[inline(always)]
    pub fn pc15(self) -> &'a mut W {
        self.variant(NEGSEL_A::PC15)
    }
    #[doc = "Port D, Pin0"]
    #[inline(always)]
    pub fn pd0(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD0)
    }
    #[doc = "Port D, Pin1"]
    #[inline(always)]
    pub fn pd1(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD1)
    }
    #[doc = "Port D, Pin2"]
    #[inline(always)]
    pub fn pd2(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD2)
    }
    #[doc = "Port D, Pin3"]
    #[inline(always)]
    pub fn pd3(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD3)
    }
    #[doc = "Port D, Pin4"]
    #[inline(always)]
    pub fn pd4(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD4)
    }
    #[doc = "Port D, Pin5"]
    #[inline(always)]
    pub fn pd5(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD5)
    }
    #[doc = "Port D, Pin6"]
    #[inline(always)]
    pub fn pd6(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD6)
    }
    #[doc = "Port D, Pin7"]
    #[inline(always)]
    pub fn pd7(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD7)
    }
    #[doc = "Port D, Pin8"]
    #[inline(always)]
    pub fn pd8(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD8)
    }
    #[doc = "Port D, Pin9"]
    #[inline(always)]
    pub fn pd9(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD9)
    }
    #[doc = "Port D, Pin10"]
    #[inline(always)]
    pub fn pd10(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD10)
    }
    #[doc = "Port D, Pin11"]
    #[inline(always)]
    pub fn pd11(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD11)
    }
    #[doc = "Port D, Pin12"]
    #[inline(always)]
    pub fn pd12(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD12)
    }
    #[doc = "Port D, Pin13"]
    #[inline(always)]
    pub fn pd13(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD13)
    }
    #[doc = "Port D, Pin14"]
    #[inline(always)]
    pub fn pd14(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD14)
    }
    #[doc = "Port D, Pin15"]
    #[inline(always)]
    pub fn pd15(self) -> &'a mut W {
        self.variant(NEGSEL_A::PD15)
    }
}
#[doc = "Field `VREFDIV` reader - VREF division"]
pub type VREFDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFDIV` writer - VREF division"]
pub type VREFDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INPUTCTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `CSRESSEL` reader - Capacitive Sense Mode Internal Resistor"]
pub type CSRESSEL_R = crate::FieldReader<u8, CSRESSEL_A>;
#[doc = "Capacitive Sense Mode Internal Resistor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSRESSEL_A {
    #[doc = "0: Internal capacitive sense resistor value 0"]
    RES0 = 0,
    #[doc = "1: Internal capacitive sense resistor value 1"]
    RES1 = 1,
    #[doc = "2: Internal capacitive sense resistor value 2"]
    RES2 = 2,
    #[doc = "3: Internal capacitive sense resistor value 3"]
    RES3 = 3,
    #[doc = "4: Internal capacitive sense resistor value 4"]
    RES4 = 4,
    #[doc = "5: Internal capacitive sense resistor value 5"]
    RES5 = 5,
    #[doc = "6: Internal capacitive sense resistor value 6"]
    RES6 = 6,
}
impl From<CSRESSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CSRESSEL_A) -> Self {
        variant as _
    }
}
impl CSRESSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSRESSEL_A> {
        match self.bits {
            0 => Some(CSRESSEL_A::RES0),
            1 => Some(CSRESSEL_A::RES1),
            2 => Some(CSRESSEL_A::RES2),
            3 => Some(CSRESSEL_A::RES3),
            4 => Some(CSRESSEL_A::RES4),
            5 => Some(CSRESSEL_A::RES5),
            6 => Some(CSRESSEL_A::RES6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == CSRESSEL_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == CSRESSEL_A::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == CSRESSEL_A::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == CSRESSEL_A::RES3
    }
    #[doc = "Checks if the value of the field is `RES4`"]
    #[inline(always)]
    pub fn is_res4(&self) -> bool {
        *self == CSRESSEL_A::RES4
    }
    #[doc = "Checks if the value of the field is `RES5`"]
    #[inline(always)]
    pub fn is_res5(&self) -> bool {
        *self == CSRESSEL_A::RES5
    }
    #[doc = "Checks if the value of the field is `RES6`"]
    #[inline(always)]
    pub fn is_res6(&self) -> bool {
        *self == CSRESSEL_A::RES6
    }
}
#[doc = "Field `CSRESSEL` writer - Capacitive Sense Mode Internal Resistor"]
pub type CSRESSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INPUTCTRL_SPEC, u8, CSRESSEL_A, 3, O>;
impl<'a, const O: u8> CSRESSEL_W<'a, O> {
    #[doc = "Internal capacitive sense resistor value 0"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES0)
    }
    #[doc = "Internal capacitive sense resistor value 1"]
    #[inline(always)]
    pub fn res1(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES1)
    }
    #[doc = "Internal capacitive sense resistor value 2"]
    #[inline(always)]
    pub fn res2(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES2)
    }
    #[doc = "Internal capacitive sense resistor value 3"]
    #[inline(always)]
    pub fn res3(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES3)
    }
    #[doc = "Internal capacitive sense resistor value 4"]
    #[inline(always)]
    pub fn res4(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES4)
    }
    #[doc = "Internal capacitive sense resistor value 5"]
    #[inline(always)]
    pub fn res5(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES5)
    }
    #[doc = "Internal capacitive sense resistor value 6"]
    #[inline(always)]
    pub fn res6(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES6)
    }
}
impl R {
    #[doc = "Bits 0:7 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - VREF division"]
    #[inline(always)]
    pub fn vrefdiv(&self) -> VREFDIV_R {
        VREFDIV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 28:30 - Capacitive Sense Mode Internal Resistor"]
    #[inline(always)]
    pub fn csressel(&self) -> CSRESSEL_R {
        CSRESSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Positive Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn possel(&mut self) -> POSSEL_W<0> {
        POSSEL_W::new(self)
    }
    #[doc = "Bits 8:15 - Negative Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn negsel(&mut self) -> NEGSEL_W<8> {
        NEGSEL_W::new(self)
    }
    #[doc = "Bits 16:21 - VREF division"]
    #[inline(always)]
    #[must_use]
    pub fn vrefdiv(&mut self) -> VREFDIV_W<16> {
        VREFDIV_W::new(self)
    }
    #[doc = "Bits 28:30 - Capacitive Sense Mode Internal Resistor"]
    #[inline(always)]
    #[must_use]
    pub fn csressel(&mut self) -> CSRESSEL_W<28> {
        CSRESSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputctrl](index.html) module"]
pub struct INPUTCTRL_SPEC;
impl crate::RegisterSpec for INPUTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inputctrl::R](R) reader structure"]
impl crate::Readable for INPUTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inputctrl::W](W) writer structure"]
impl crate::Writable for INPUTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTCTRL to value 0"]
impl crate::Resettable for INPUTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
