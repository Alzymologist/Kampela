#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version of the device info structure being used"]
    pub info: INFO,
    #[doc = "0x04 - Part description"]
    pub part: PART,
    #[doc = "0x08 - Flash page size and misc. chip information"]
    pub meminfo: MEMINFO,
    #[doc = "0x0c - Flash and SRAM Memory size in kB"]
    pub msize: MSIZE,
    #[doc = "0x10 - Miscellaneous device information"]
    pub pkginfo: PKGINFO,
    #[doc = "0x14 - Custom information"]
    pub custominfo: CUSTOMINFO,
    #[doc = "0x18 - Used to track s/w workaround info"]
    pub swfix: SWFIX,
    #[doc = "0x1c - Software Capability Vector 0"]
    pub swcapa0: SWCAPA0,
    #[doc = "0x20 - Software Capability Vector 1"]
    pub swcapa1: SWCAPA1,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - External component description"]
    pub extinfo: EXTINFO,
    _reserved10: [u8; 0x14],
    #[doc = "0x40 - MA-L compliant EUI48 OUI (low bits) and Unique Identifier (24-bit)"]
    pub eui48l: EUI48L,
    #[doc = "0x44 - MA-L compliant EUI48 OUI (high bits)"]
    pub eui48h: EUI48H,
    #[doc = "0x48 - MA-L compliant EUI64 Unique Identifier (low bits)"]
    pub eui64l: EUI64L,
    #[doc = "0x4c - MA-L compliant EUI64 OUI and Unique Identifier (high bits)"]
    pub eui64h: EUI64H,
    #[doc = "0x50 - Calibration Temperature Information"]
    pub caltemp: CALTEMP,
    #[doc = "0x54 - EMU Temperature Sensor Calibration"]
    pub emutemp: EMUTEMP,
    #[doc = "0x58 - HFRCODPLL Calibration"]
    pub hfrcodpllcal0: HFRCODPLLCAL0,
    #[doc = "0x5c - HFRCODPLL Calibration"]
    pub hfrcodpllcal1: HFRCODPLLCAL1,
    #[doc = "0x60 - HFRCODPLL Calibration"]
    pub hfrcodpllcal2: HFRCODPLLCAL2,
    #[doc = "0x64 - HFRCODPLL Calibration"]
    pub hfrcodpllcal3: HFRCODPLLCAL3,
    #[doc = "0x68 - HFRCODPLL Calibration"]
    pub hfrcodpllcal4: HFRCODPLLCAL4,
    #[doc = "0x6c - HFRCODPLL Calibration"]
    pub hfrcodpllcal5: HFRCODPLLCAL5,
    #[doc = "0x70 - HFRCODPLL Calibration"]
    pub hfrcodpllcal6: HFRCODPLLCAL6,
    #[doc = "0x74 - HFRCODPLL Calibration"]
    pub hfrcodpllcal7: HFRCODPLLCAL7,
    #[doc = "0x78 - HFRCODPLL Calibration"]
    pub hfrcodpllcal8: HFRCODPLLCAL8,
    #[doc = "0x7c - HFRCODPLL Calibration"]
    pub hfrcodpllcal9: HFRCODPLLCAL9,
    #[doc = "0x80 - HFRCODPLL Calibration"]
    pub hfrcodpllcal10: HFRCODPLLCAL10,
    #[doc = "0x84 - HFRCODPLL Calibration"]
    pub hfrcodpllcal11: HFRCODPLLCAL11,
    #[doc = "0x88 - HFRCODPLL Calibration"]
    pub hfrcodpllcal12: HFRCODPLLCAL12,
    #[doc = "0x8c - HFRCODPLL Calibration"]
    pub hfrcodpllcal13: HFRCODPLLCAL13,
    #[doc = "0x90 - HFRCODPLL Calibration"]
    pub hfrcodpllcal14: HFRCODPLLCAL14,
    #[doc = "0x94 - HFRCODPLL Calibration"]
    pub hfrcodpllcal15: HFRCODPLLCAL15,
    #[doc = "0x98 - HFRCODPLL Calibration"]
    pub hfrcodpllcal16: HFRCODPLLCAL16,
    #[doc = "0x9c - HFRCODPLL Calibration"]
    pub hfrcodpllcal17: HFRCODPLLCAL17,
    #[doc = "0xa0 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal0: HFRCOEM23CAL0,
    #[doc = "0xa4 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal1: HFRCOEM23CAL1,
    #[doc = "0xa8 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal2: HFRCOEM23CAL2,
    #[doc = "0xac - HFRCOEM23 Calibration"]
    pub hfrcoem23cal3: HFRCOEM23CAL3,
    #[doc = "0xb0 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal4: HFRCOEM23CAL4,
    #[doc = "0xb4 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal5: HFRCOEM23CAL5,
    #[doc = "0xb8 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal6: HFRCOEM23CAL6,
    #[doc = "0xbc - HFRCOEM23 Calibration"]
    pub hfrcoem23cal7: HFRCOEM23CAL7,
    #[doc = "0xc0 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal8: HFRCOEM23CAL8,
    #[doc = "0xc4 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal9: HFRCOEM23CAL9,
    #[doc = "0xc8 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal10: HFRCOEM23CAL10,
    #[doc = "0xcc - HFRCOEM23 Calibration"]
    pub hfrcoem23cal11: HFRCOEM23CAL11,
    #[doc = "0xd0 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal12: HFRCOEM23CAL12,
    #[doc = "0xd4 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal13: HFRCOEM23CAL13,
    #[doc = "0xd8 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal14: HFRCOEM23CAL14,
    #[doc = "0xdc - HFRCOEM23 Calibration"]
    pub hfrcoem23cal15: HFRCOEM23CAL15,
    #[doc = "0xe0 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal16: HFRCOEM23CAL16,
    #[doc = "0xe4 - HFRCOEM23 Calibration"]
    pub hfrcoem23cal17: HFRCOEM23CAL17,
    _reserved52: [u8; 0x48],
    #[doc = "0x130 - Characters 1-4 of Module Name stored as a null terminated string"]
    pub modulename0: MODULENAME0,
    #[doc = "0x134 - Characters 5-8 of Module Name stored as a null terminated string"]
    pub modulename1: MODULENAME1,
    #[doc = "0x138 - Characters 9-12 of Module Name stored as a null terminated string"]
    pub modulename2: MODULENAME2,
    #[doc = "0x13c - Characters 13-16 of Module Name stored as a null terminated string"]
    pub modulename3: MODULENAME3,
    #[doc = "0x140 - Characters 17-20 of Module Name stored as a null terminated string"]
    pub modulename4: MODULENAME4,
    #[doc = "0x144 - Characters 21-24 of Module Name stored as a null terminated string"]
    pub modulename5: MODULENAME5,
    #[doc = "0x148 - Characters 25-26 of Module Name stored as a null terminated string"]
    pub modulename6: MODULENAME6,
    #[doc = "0x14c - Module Information"]
    pub moduleinfo: MODULEINFO,
    #[doc = "0x150 - Module Crystal Oscillator Calibration"]
    pub modxocal: MODXOCAL,
    _reserved61: [u8; 0x28],
    #[doc = "0x17c - High Frequency Crystal Oscillator Calibration data"]
    pub hfxocal: HFXOCAL,
    #[doc = "0x180 - IADC0 Gain Calibration Info"]
    pub iadc0gain0: IADC0GAIN0,
    #[doc = "0x184 - IADC0 Gain Calibration Info"]
    pub iadc0gain1: IADC0GAIN1,
    #[doc = "0x188 - IADC0 Offset Calibration Info"]
    pub iadc0offsetcal0: IADC0OFFSETCAL0,
    #[doc = "0x18c - IADC0 Normal Offset Calibration Info"]
    pub iadc0normaloffsetcal0: IADC0NORMALOFFSETCAL0,
    #[doc = "0x190 - IADC0 Normal Offset Calibration Info"]
    pub iadc0normaloffsetcal1: IADC0NORMALOFFSETCAL1,
    #[doc = "0x194 - IADC High Speed Offset Calibration Info"]
    pub iadc0hispdoffsetcal0: IADC0HISPDOFFSETCAL0,
    #[doc = "0x198 - IADC High Speed Offset Calibration Info"]
    pub iadc0hispdoffsetcal1: IADC0HISPDOFFSETCAL1,
    _reserved69: [u8; 0x60],
    #[doc = "0x1fc - This is the legacy device detection information for tools compatability"]
    pub legacy: LEGACY,
    _reserved70: [u8; 0x5c],
    #[doc = "0x25c - RTHERM"]
    pub rtherm: RTHERM,
}
#[doc = "INFO (r) register accessor: an alias for `Reg<INFO_SPEC>`"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "Version of the device info structure being used"]
pub mod info;
#[doc = "PART (r) register accessor: an alias for `Reg<PART_SPEC>`"]
pub type PART = crate::Reg<part::PART_SPEC>;
#[doc = "Part description"]
pub mod part;
#[doc = "MEMINFO (r) register accessor: an alias for `Reg<MEMINFO_SPEC>`"]
pub type MEMINFO = crate::Reg<meminfo::MEMINFO_SPEC>;
#[doc = "Flash page size and misc. chip information"]
pub mod meminfo;
#[doc = "MSIZE (r) register accessor: an alias for `Reg<MSIZE_SPEC>`"]
pub type MSIZE = crate::Reg<msize::MSIZE_SPEC>;
#[doc = "Flash and SRAM Memory size in kB"]
pub mod msize;
#[doc = "PKGINFO (r) register accessor: an alias for `Reg<PKGINFO_SPEC>`"]
pub type PKGINFO = crate::Reg<pkginfo::PKGINFO_SPEC>;
#[doc = "Miscellaneous device information"]
pub mod pkginfo;
#[doc = "CUSTOMINFO (r) register accessor: an alias for `Reg<CUSTOMINFO_SPEC>`"]
pub type CUSTOMINFO = crate::Reg<custominfo::CUSTOMINFO_SPEC>;
#[doc = "Custom information"]
pub mod custominfo;
#[doc = "SWFIX (r) register accessor: an alias for `Reg<SWFIX_SPEC>`"]
pub type SWFIX = crate::Reg<swfix::SWFIX_SPEC>;
#[doc = "Used to track s/w workaround info"]
pub mod swfix;
#[doc = "SWCAPA0 (r) register accessor: an alias for `Reg<SWCAPA0_SPEC>`"]
pub type SWCAPA0 = crate::Reg<swcapa0::SWCAPA0_SPEC>;
#[doc = "Software Capability Vector 0"]
pub mod swcapa0;
#[doc = "SWCAPA1 (r) register accessor: an alias for `Reg<SWCAPA1_SPEC>`"]
pub type SWCAPA1 = crate::Reg<swcapa1::SWCAPA1_SPEC>;
#[doc = "Software Capability Vector 1"]
pub mod swcapa1;
#[doc = "EXTINFO (r) register accessor: an alias for `Reg<EXTINFO_SPEC>`"]
pub type EXTINFO = crate::Reg<extinfo::EXTINFO_SPEC>;
#[doc = "External component description"]
pub mod extinfo;
#[doc = "EUI48L (r) register accessor: an alias for `Reg<EUI48L_SPEC>`"]
pub type EUI48L = crate::Reg<eui48l::EUI48L_SPEC>;
#[doc = "MA-L compliant EUI48 OUI (low bits) and Unique Identifier (24-bit)"]
pub mod eui48l;
#[doc = "EUI48H (r) register accessor: an alias for `Reg<EUI48H_SPEC>`"]
pub type EUI48H = crate::Reg<eui48h::EUI48H_SPEC>;
#[doc = "MA-L compliant EUI48 OUI (high bits)"]
pub mod eui48h;
#[doc = "EUI64L (r) register accessor: an alias for `Reg<EUI64L_SPEC>`"]
pub type EUI64L = crate::Reg<eui64l::EUI64L_SPEC>;
#[doc = "MA-L compliant EUI64 Unique Identifier (low bits)"]
pub mod eui64l;
#[doc = "EUI64H (r) register accessor: an alias for `Reg<EUI64H_SPEC>`"]
pub type EUI64H = crate::Reg<eui64h::EUI64H_SPEC>;
#[doc = "MA-L compliant EUI64 OUI and Unique Identifier (high bits)"]
pub mod eui64h;
#[doc = "CALTEMP (r) register accessor: an alias for `Reg<CALTEMP_SPEC>`"]
pub type CALTEMP = crate::Reg<caltemp::CALTEMP_SPEC>;
#[doc = "Calibration Temperature Information"]
pub mod caltemp;
#[doc = "EMUTEMP (r) register accessor: an alias for `Reg<EMUTEMP_SPEC>`"]
pub type EMUTEMP = crate::Reg<emutemp::EMUTEMP_SPEC>;
#[doc = "EMU Temperature Sensor Calibration"]
pub mod emutemp;
#[doc = "HFRCODPLLCAL0 (r) register accessor: an alias for `Reg<HFRCODPLLCAL0_SPEC>`"]
pub type HFRCODPLLCAL0 = crate::Reg<hfrcodpllcal0::HFRCODPLLCAL0_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal0;
#[doc = "HFRCODPLLCAL1 (r) register accessor: an alias for `Reg<HFRCODPLLCAL1_SPEC>`"]
pub type HFRCODPLLCAL1 = crate::Reg<hfrcodpllcal1::HFRCODPLLCAL1_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal1;
#[doc = "HFRCODPLLCAL2 (r) register accessor: an alias for `Reg<HFRCODPLLCAL2_SPEC>`"]
pub type HFRCODPLLCAL2 = crate::Reg<hfrcodpllcal2::HFRCODPLLCAL2_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal2;
#[doc = "HFRCODPLLCAL3 (r) register accessor: an alias for `Reg<HFRCODPLLCAL3_SPEC>`"]
pub type HFRCODPLLCAL3 = crate::Reg<hfrcodpllcal3::HFRCODPLLCAL3_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal3;
#[doc = "HFRCODPLLCAL4 (r) register accessor: an alias for `Reg<HFRCODPLLCAL4_SPEC>`"]
pub type HFRCODPLLCAL4 = crate::Reg<hfrcodpllcal4::HFRCODPLLCAL4_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal4;
#[doc = "HFRCODPLLCAL5 (r) register accessor: an alias for `Reg<HFRCODPLLCAL5_SPEC>`"]
pub type HFRCODPLLCAL5 = crate::Reg<hfrcodpllcal5::HFRCODPLLCAL5_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal5;
#[doc = "HFRCODPLLCAL6 (r) register accessor: an alias for `Reg<HFRCODPLLCAL6_SPEC>`"]
pub type HFRCODPLLCAL6 = crate::Reg<hfrcodpllcal6::HFRCODPLLCAL6_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal6;
#[doc = "HFRCODPLLCAL7 (r) register accessor: an alias for `Reg<HFRCODPLLCAL7_SPEC>`"]
pub type HFRCODPLLCAL7 = crate::Reg<hfrcodpllcal7::HFRCODPLLCAL7_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal7;
#[doc = "HFRCODPLLCAL8 (r) register accessor: an alias for `Reg<HFRCODPLLCAL8_SPEC>`"]
pub type HFRCODPLLCAL8 = crate::Reg<hfrcodpllcal8::HFRCODPLLCAL8_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal8;
#[doc = "HFRCODPLLCAL9 (r) register accessor: an alias for `Reg<HFRCODPLLCAL9_SPEC>`"]
pub type HFRCODPLLCAL9 = crate::Reg<hfrcodpllcal9::HFRCODPLLCAL9_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal9;
#[doc = "HFRCODPLLCAL10 (r) register accessor: an alias for `Reg<HFRCODPLLCAL10_SPEC>`"]
pub type HFRCODPLLCAL10 = crate::Reg<hfrcodpllcal10::HFRCODPLLCAL10_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal10;
#[doc = "HFRCODPLLCAL11 (r) register accessor: an alias for `Reg<HFRCODPLLCAL11_SPEC>`"]
pub type HFRCODPLLCAL11 = crate::Reg<hfrcodpllcal11::HFRCODPLLCAL11_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal11;
#[doc = "HFRCODPLLCAL12 (r) register accessor: an alias for `Reg<HFRCODPLLCAL12_SPEC>`"]
pub type HFRCODPLLCAL12 = crate::Reg<hfrcodpllcal12::HFRCODPLLCAL12_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal12;
#[doc = "HFRCODPLLCAL13 (r) register accessor: an alias for `Reg<HFRCODPLLCAL13_SPEC>`"]
pub type HFRCODPLLCAL13 = crate::Reg<hfrcodpllcal13::HFRCODPLLCAL13_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal13;
#[doc = "HFRCODPLLCAL14 (r) register accessor: an alias for `Reg<HFRCODPLLCAL14_SPEC>`"]
pub type HFRCODPLLCAL14 = crate::Reg<hfrcodpllcal14::HFRCODPLLCAL14_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal14;
#[doc = "HFRCODPLLCAL15 (r) register accessor: an alias for `Reg<HFRCODPLLCAL15_SPEC>`"]
pub type HFRCODPLLCAL15 = crate::Reg<hfrcodpllcal15::HFRCODPLLCAL15_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal15;
#[doc = "HFRCODPLLCAL16 (r) register accessor: an alias for `Reg<HFRCODPLLCAL16_SPEC>`"]
pub type HFRCODPLLCAL16 = crate::Reg<hfrcodpllcal16::HFRCODPLLCAL16_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal16;
#[doc = "HFRCODPLLCAL17 (r) register accessor: an alias for `Reg<HFRCODPLLCAL17_SPEC>`"]
pub type HFRCODPLLCAL17 = crate::Reg<hfrcodpllcal17::HFRCODPLLCAL17_SPEC>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal17;
#[doc = "HFRCOEM23CAL0 (r) register accessor: an alias for `Reg<HFRCOEM23CAL0_SPEC>`"]
pub type HFRCOEM23CAL0 = crate::Reg<hfrcoem23cal0::HFRCOEM23CAL0_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal0;
#[doc = "HFRCOEM23CAL1 (r) register accessor: an alias for `Reg<HFRCOEM23CAL1_SPEC>`"]
pub type HFRCOEM23CAL1 = crate::Reg<hfrcoem23cal1::HFRCOEM23CAL1_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal1;
#[doc = "HFRCOEM23CAL2 (r) register accessor: an alias for `Reg<HFRCOEM23CAL2_SPEC>`"]
pub type HFRCOEM23CAL2 = crate::Reg<hfrcoem23cal2::HFRCOEM23CAL2_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal2;
#[doc = "HFRCOEM23CAL3 (r) register accessor: an alias for `Reg<HFRCOEM23CAL3_SPEC>`"]
pub type HFRCOEM23CAL3 = crate::Reg<hfrcoem23cal3::HFRCOEM23CAL3_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal3;
#[doc = "HFRCOEM23CAL4 (r) register accessor: an alias for `Reg<HFRCOEM23CAL4_SPEC>`"]
pub type HFRCOEM23CAL4 = crate::Reg<hfrcoem23cal4::HFRCOEM23CAL4_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal4;
#[doc = "HFRCOEM23CAL5 (r) register accessor: an alias for `Reg<HFRCOEM23CAL5_SPEC>`"]
pub type HFRCOEM23CAL5 = crate::Reg<hfrcoem23cal5::HFRCOEM23CAL5_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal5;
#[doc = "HFRCOEM23CAL6 (r) register accessor: an alias for `Reg<HFRCOEM23CAL6_SPEC>`"]
pub type HFRCOEM23CAL6 = crate::Reg<hfrcoem23cal6::HFRCOEM23CAL6_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal6;
#[doc = "HFRCOEM23CAL7 (r) register accessor: an alias for `Reg<HFRCOEM23CAL7_SPEC>`"]
pub type HFRCOEM23CAL7 = crate::Reg<hfrcoem23cal7::HFRCOEM23CAL7_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal7;
#[doc = "HFRCOEM23CAL8 (r) register accessor: an alias for `Reg<HFRCOEM23CAL8_SPEC>`"]
pub type HFRCOEM23CAL8 = crate::Reg<hfrcoem23cal8::HFRCOEM23CAL8_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal8;
#[doc = "HFRCOEM23CAL9 (r) register accessor: an alias for `Reg<HFRCOEM23CAL9_SPEC>`"]
pub type HFRCOEM23CAL9 = crate::Reg<hfrcoem23cal9::HFRCOEM23CAL9_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal9;
#[doc = "HFRCOEM23CAL10 (r) register accessor: an alias for `Reg<HFRCOEM23CAL10_SPEC>`"]
pub type HFRCOEM23CAL10 = crate::Reg<hfrcoem23cal10::HFRCOEM23CAL10_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal10;
#[doc = "HFRCOEM23CAL11 (r) register accessor: an alias for `Reg<HFRCOEM23CAL11_SPEC>`"]
pub type HFRCOEM23CAL11 = crate::Reg<hfrcoem23cal11::HFRCOEM23CAL11_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal11;
#[doc = "HFRCOEM23CAL12 (r) register accessor: an alias for `Reg<HFRCOEM23CAL12_SPEC>`"]
pub type HFRCOEM23CAL12 = crate::Reg<hfrcoem23cal12::HFRCOEM23CAL12_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal12;
#[doc = "HFRCOEM23CAL13 (r) register accessor: an alias for `Reg<HFRCOEM23CAL13_SPEC>`"]
pub type HFRCOEM23CAL13 = crate::Reg<hfrcoem23cal13::HFRCOEM23CAL13_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal13;
#[doc = "HFRCOEM23CAL14 (r) register accessor: an alias for `Reg<HFRCOEM23CAL14_SPEC>`"]
pub type HFRCOEM23CAL14 = crate::Reg<hfrcoem23cal14::HFRCOEM23CAL14_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal14;
#[doc = "HFRCOEM23CAL15 (r) register accessor: an alias for `Reg<HFRCOEM23CAL15_SPEC>`"]
pub type HFRCOEM23CAL15 = crate::Reg<hfrcoem23cal15::HFRCOEM23CAL15_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal15;
#[doc = "HFRCOEM23CAL16 (r) register accessor: an alias for `Reg<HFRCOEM23CAL16_SPEC>`"]
pub type HFRCOEM23CAL16 = crate::Reg<hfrcoem23cal16::HFRCOEM23CAL16_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal16;
#[doc = "HFRCOEM23CAL17 (r) register accessor: an alias for `Reg<HFRCOEM23CAL17_SPEC>`"]
pub type HFRCOEM23CAL17 = crate::Reg<hfrcoem23cal17::HFRCOEM23CAL17_SPEC>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal17;
#[doc = "MODULENAME0 (r) register accessor: an alias for `Reg<MODULENAME0_SPEC>`"]
pub type MODULENAME0 = crate::Reg<modulename0::MODULENAME0_SPEC>;
#[doc = "Characters 1-4 of Module Name stored as a null terminated string"]
pub mod modulename0;
#[doc = "MODULENAME1 (r) register accessor: an alias for `Reg<MODULENAME1_SPEC>`"]
pub type MODULENAME1 = crate::Reg<modulename1::MODULENAME1_SPEC>;
#[doc = "Characters 5-8 of Module Name stored as a null terminated string"]
pub mod modulename1;
#[doc = "MODULENAME2 (r) register accessor: an alias for `Reg<MODULENAME2_SPEC>`"]
pub type MODULENAME2 = crate::Reg<modulename2::MODULENAME2_SPEC>;
#[doc = "Characters 9-12 of Module Name stored as a null terminated string"]
pub mod modulename2;
#[doc = "MODULENAME3 (r) register accessor: an alias for `Reg<MODULENAME3_SPEC>`"]
pub type MODULENAME3 = crate::Reg<modulename3::MODULENAME3_SPEC>;
#[doc = "Characters 13-16 of Module Name stored as a null terminated string"]
pub mod modulename3;
#[doc = "MODULENAME4 (r) register accessor: an alias for `Reg<MODULENAME4_SPEC>`"]
pub type MODULENAME4 = crate::Reg<modulename4::MODULENAME4_SPEC>;
#[doc = "Characters 17-20 of Module Name stored as a null terminated string"]
pub mod modulename4;
#[doc = "MODULENAME5 (r) register accessor: an alias for `Reg<MODULENAME5_SPEC>`"]
pub type MODULENAME5 = crate::Reg<modulename5::MODULENAME5_SPEC>;
#[doc = "Characters 21-24 of Module Name stored as a null terminated string"]
pub mod modulename5;
#[doc = "MODULENAME6 (r) register accessor: an alias for `Reg<MODULENAME6_SPEC>`"]
pub type MODULENAME6 = crate::Reg<modulename6::MODULENAME6_SPEC>;
#[doc = "Characters 25-26 of Module Name stored as a null terminated string"]
pub mod modulename6;
#[doc = "MODULEINFO (r) register accessor: an alias for `Reg<MODULEINFO_SPEC>`"]
pub type MODULEINFO = crate::Reg<moduleinfo::MODULEINFO_SPEC>;
#[doc = "Module Information"]
pub mod moduleinfo;
#[doc = "MODXOCAL (r) register accessor: an alias for `Reg<MODXOCAL_SPEC>`"]
pub type MODXOCAL = crate::Reg<modxocal::MODXOCAL_SPEC>;
#[doc = "Module Crystal Oscillator Calibration"]
pub mod modxocal;
#[doc = "HFXOCAL (r) register accessor: an alias for `Reg<HFXOCAL_SPEC>`"]
pub type HFXOCAL = crate::Reg<hfxocal::HFXOCAL_SPEC>;
#[doc = "High Frequency Crystal Oscillator Calibration data"]
pub mod hfxocal;
#[doc = "IADC0GAIN0 (r) register accessor: an alias for `Reg<IADC0GAIN0_SPEC>`"]
pub type IADC0GAIN0 = crate::Reg<iadc0gain0::IADC0GAIN0_SPEC>;
#[doc = "IADC0 Gain Calibration Info"]
pub mod iadc0gain0;
#[doc = "IADC0GAIN1 (r) register accessor: an alias for `Reg<IADC0GAIN1_SPEC>`"]
pub type IADC0GAIN1 = crate::Reg<iadc0gain1::IADC0GAIN1_SPEC>;
#[doc = "IADC0 Gain Calibration Info"]
pub mod iadc0gain1;
#[doc = "IADC0OFFSETCAL0 (r) register accessor: an alias for `Reg<IADC0OFFSETCAL0_SPEC>`"]
pub type IADC0OFFSETCAL0 = crate::Reg<iadc0offsetcal0::IADC0OFFSETCAL0_SPEC>;
#[doc = "IADC0 Offset Calibration Info"]
pub mod iadc0offsetcal0;
#[doc = "IADC0NORMALOFFSETCAL0 (r) register accessor: an alias for `Reg<IADC0NORMALOFFSETCAL0_SPEC>`"]
pub type IADC0NORMALOFFSETCAL0 = crate::Reg<iadc0normaloffsetcal0::IADC0NORMALOFFSETCAL0_SPEC>;
#[doc = "IADC0 Normal Offset Calibration Info"]
pub mod iadc0normaloffsetcal0;
#[doc = "IADC0NORMALOFFSETCAL1 (r) register accessor: an alias for `Reg<IADC0NORMALOFFSETCAL1_SPEC>`"]
pub type IADC0NORMALOFFSETCAL1 = crate::Reg<iadc0normaloffsetcal1::IADC0NORMALOFFSETCAL1_SPEC>;
#[doc = "IADC0 Normal Offset Calibration Info"]
pub mod iadc0normaloffsetcal1;
#[doc = "IADC0HISPDOFFSETCAL0 (r) register accessor: an alias for `Reg<IADC0HISPDOFFSETCAL0_SPEC>`"]
pub type IADC0HISPDOFFSETCAL0 = crate::Reg<iadc0hispdoffsetcal0::IADC0HISPDOFFSETCAL0_SPEC>;
#[doc = "IADC High Speed Offset Calibration Info"]
pub mod iadc0hispdoffsetcal0;
#[doc = "IADC0HISPDOFFSETCAL1 (r) register accessor: an alias for `Reg<IADC0HISPDOFFSETCAL1_SPEC>`"]
pub type IADC0HISPDOFFSETCAL1 = crate::Reg<iadc0hispdoffsetcal1::IADC0HISPDOFFSETCAL1_SPEC>;
#[doc = "IADC High Speed Offset Calibration Info"]
pub mod iadc0hispdoffsetcal1;
#[doc = "LEGACY (r) register accessor: an alias for `Reg<LEGACY_SPEC>`"]
pub type LEGACY = crate::Reg<legacy::LEGACY_SPEC>;
#[doc = "This is the legacy device detection information for tools compatability"]
pub mod legacy;
#[doc = "RTHERM (r) register accessor: an alias for `Reg<RTHERM_SPEC>`"]
pub type RTHERM = crate::Reg<rtherm::RTHERM_SPEC>;
#[doc = "RTHERM"]
pub mod rtherm;
