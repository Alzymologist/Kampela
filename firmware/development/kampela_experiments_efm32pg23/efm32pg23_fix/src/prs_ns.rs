#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - No Description"]
    pub async_swpulse: ASYNC_SWPULSE,
    #[doc = "0x0c - No Description"]
    pub async_swlevel: ASYNC_SWLEVEL,
    #[doc = "0x10 - No Description"]
    pub async_peek: ASYNC_PEEK,
    #[doc = "0x14 - No Description"]
    pub sync_peek: SYNC_PEEK,
    #[doc = "0x18 - No Description"]
    pub async_ch0_ctrl: ASYNC_CH0_CTRL,
    #[doc = "0x1c - No Description"]
    pub async_ch1_ctrl: ASYNC_CH1_CTRL,
    #[doc = "0x20 - No Description"]
    pub async_ch2_ctrl: ASYNC_CH2_CTRL,
    #[doc = "0x24 - No Description"]
    pub async_ch3_ctrl: ASYNC_CH3_CTRL,
    #[doc = "0x28 - No Description"]
    pub async_ch4_ctrl: ASYNC_CH4_CTRL,
    #[doc = "0x2c - No Description"]
    pub async_ch5_ctrl: ASYNC_CH5_CTRL,
    #[doc = "0x30 - No Description"]
    pub async_ch6_ctrl: ASYNC_CH6_CTRL,
    #[doc = "0x34 - No Description"]
    pub async_ch7_ctrl: ASYNC_CH7_CTRL,
    #[doc = "0x38 - No Description"]
    pub async_ch8_ctrl: ASYNC_CH8_CTRL,
    #[doc = "0x3c - No Description"]
    pub async_ch9_ctrl: ASYNC_CH9_CTRL,
    #[doc = "0x40 - No Description"]
    pub async_ch10_ctrl: ASYNC_CH10_CTRL,
    #[doc = "0x44 - No Description"]
    pub async_ch11_ctrl: ASYNC_CH11_CTRL,
    #[doc = "0x48 - No Description"]
    pub sync_ch0_ctrl: SYNC_CH0_CTRL,
    #[doc = "0x4c - No Description"]
    pub sync_ch1_ctrl: SYNC_CH1_CTRL,
    #[doc = "0x50 - No Description"]
    pub sync_ch2_ctrl: SYNC_CH2_CTRL,
    #[doc = "0x54 - No Description"]
    pub sync_ch3_ctrl: SYNC_CH3_CTRL,
    #[doc = "0x58 - CALDN consumer register"]
    pub consumer_cmu_caldn: CONSUMER_CMU_CALDN,
    #[doc = "0x5c - CALUP Consumer register"]
    pub consumer_cmu_calup: CONSUMER_CMU_CALUP,
    #[doc = "0x60 - CLK consumer register"]
    pub consumer_eusart0_clk: CONSUMER_EUSART0_CLK,
    #[doc = "0x64 - RX Consumer register"]
    pub consumer_eusart0_rx: CONSUMER_EUSART0_RX,
    #[doc = "0x68 - TRIGGER Consumer register"]
    pub consumer_eusart0_trigger: CONSUMER_EUSART0_TRIGGER,
    #[doc = "0x6c - CLK consumer register"]
    pub consumer_eusart1_clk: CONSUMER_EUSART1_CLK,
    #[doc = "0x70 - RX Consumer register"]
    pub consumer_eusart1_rx: CONSUMER_EUSART1_RX,
    #[doc = "0x74 - TRIGGER Consumer register"]
    pub consumer_eusart1_trigger: CONSUMER_EUSART1_TRIGGER,
    #[doc = "0x78 - CLK consumer register"]
    pub consumer_eusart2_clk: CONSUMER_EUSART2_CLK,
    #[doc = "0x7c - RX Consumer register"]
    pub consumer_eusart2_rx: CONSUMER_EUSART2_RX,
    #[doc = "0x80 - TRIGGER Consumer register"]
    pub consumer_eusart2_trigger: CONSUMER_EUSART2_TRIGGER,
    _reserved32: [u8; 0x04],
    #[doc = "0x88 - SCAN consumer register"]
    pub consumer_iadc0_scantrigger: CONSUMER_IADC0_SCANTRIGGER,
    #[doc = "0x8c - SINGLE Consumer register"]
    pub consumer_iadc0_singletrigger: CONSUMER_IADC0_SINGLETRIGGER,
    #[doc = "0x90 - DMAREQ0 consumer register"]
    pub consumer_ldmaxbar_dmareq0: CONSUMER_LDMAXBAR_DMAREQ0,
    #[doc = "0x94 - DMAREQ1 Consumer register"]
    pub consumer_ldmaxbar_dmareq1: CONSUMER_LDMAXBAR_DMAREQ1,
    _reserved36: [u8; 0x10],
    #[doc = "0xa8 - START Consumer register"]
    pub consumer_lesense_start: CONSUMER_LESENSE_START,
    #[doc = "0xac - CLEAR consumer register"]
    pub consumer_letimer0_clear: CONSUMER_LETIMER0_CLEAR,
    #[doc = "0xb0 - START Consumer register"]
    pub consumer_letimer0_start: CONSUMER_LETIMER0_START,
    #[doc = "0xb4 - STOP Consumer register"]
    pub consumer_letimer0_stop: CONSUMER_LETIMER0_STOP,
    _reserved40: [u8; 0x04],
    #[doc = "0xbc - S0IN consumer register"]
    pub consumer_pcnt0_s0in: CONSUMER_PCNT0_S0IN,
    #[doc = "0xc0 - S1IN Consumer register"]
    pub consumer_pcnt0_s1in: CONSUMER_PCNT0_S1IN,
    _reserved42: [u8; 0x50],
    #[doc = "0x114 - TAMPERSRC25 consumer register"]
    pub consumer_setamper_tampersrc25: CONSUMER_SETAMPER_TAMPERSRC25,
    #[doc = "0x118 - TAMPERSRC26 Consumer register"]
    pub consumer_setamper_tampersrc26: CONSUMER_SETAMPER_TAMPERSRC26,
    #[doc = "0x11c - TAMPERSRC27 Consumer register"]
    pub consumer_setamper_tampersrc27: CONSUMER_SETAMPER_TAMPERSRC27,
    #[doc = "0x120 - TAMPERSRC28 Consumer register"]
    pub consumer_setamper_tampersrc28: CONSUMER_SETAMPER_TAMPERSRC28,
    #[doc = "0x124 - TAMPERSRC29 Consumer register"]
    pub consumer_setamper_tampersrc29: CONSUMER_SETAMPER_TAMPERSRC29,
    #[doc = "0x128 - TAMPERSRC30 Consumer register"]
    pub consumer_setamper_tampersrc30: CONSUMER_SETAMPER_TAMPERSRC30,
    #[doc = "0x12c - TAMPERSRC31 Consumer register"]
    pub consumer_setamper_tampersrc31: CONSUMER_SETAMPER_TAMPERSRC31,
    #[doc = "0x130 - IN0 consumer register"]
    pub consumer_sysrtc0_in0: CONSUMER_SYSRTC0_IN0,
    #[doc = "0x134 - IN1 Consumer register"]
    pub consumer_sysrtc0_in1: CONSUMER_SYSRTC0_IN1,
    #[doc = "0x138 - OSCREQ consumer register"]
    pub consumer_hfxo0_oscreq: CONSUMER_HFXO0_OSCREQ,
    #[doc = "0x13c - TIMEOUT Consumer register"]
    pub consumer_hfxo0_timeout: CONSUMER_HFXO0_TIMEOUT,
    #[doc = "0x140 - CTI Consumer Register"]
    pub consumer_core_ctiin0: CONSUMER_CORE_CTIIN0,
    #[doc = "0x144 - CTI Consumer Register"]
    pub consumer_core_ctiin1: CONSUMER_CORE_CTIIN1,
    #[doc = "0x148 - CTI Consumer Register"]
    pub consumer_core_ctiin2: CONSUMER_CORE_CTIIN2,
    #[doc = "0x14c - CTI Consumer Register"]
    pub consumer_core_ctiin3: CONSUMER_CORE_CTIIN3,
    #[doc = "0x150 - M33 Consumer Register"]
    pub consumer_core_m33rxev: CONSUMER_CORE_M33RXEV,
    #[doc = "0x154 - CC0 consumer register"]
    pub consumer_timer0_cc0: CONSUMER_TIMER0_CC0,
    #[doc = "0x158 - CC1 Consumer register"]
    pub consumer_timer0_cc1: CONSUMER_TIMER0_CC1,
    #[doc = "0x15c - CC2 Consumer register"]
    pub consumer_timer0_cc2: CONSUMER_TIMER0_CC2,
    #[doc = "0x160 - DTI Consumer register"]
    pub consumer_timer0_dti: CONSUMER_TIMER0_DTI,
    #[doc = "0x164 - DTI Consumer register"]
    pub consumer_timer0_dtifs1: CONSUMER_TIMER0_DTIFS1,
    #[doc = "0x168 - DTI Consumer register"]
    pub consumer_timer0_dtifs2: CONSUMER_TIMER0_DTIFS2,
    #[doc = "0x16c - CC0 consumer register"]
    pub consumer_timer1_cc0: CONSUMER_TIMER1_CC0,
    #[doc = "0x170 - CC1 Consumer register"]
    pub consumer_timer1_cc1: CONSUMER_TIMER1_CC1,
    #[doc = "0x174 - CC2 Consumer register"]
    pub consumer_timer1_cc2: CONSUMER_TIMER1_CC2,
    #[doc = "0x178 - DTI Consumer register"]
    pub consumer_timer1_dti: CONSUMER_TIMER1_DTI,
    #[doc = "0x17c - DTI Consumer register"]
    pub consumer_timer1_dtifs1: CONSUMER_TIMER1_DTIFS1,
    #[doc = "0x180 - DTI Consumer register"]
    pub consumer_timer1_dtifs2: CONSUMER_TIMER1_DTIFS2,
    #[doc = "0x184 - CC0 consumer register"]
    pub consumer_timer2_cc0: CONSUMER_TIMER2_CC0,
    #[doc = "0x188 - CC1 Consumer register"]
    pub consumer_timer2_cc1: CONSUMER_TIMER2_CC1,
    #[doc = "0x18c - CC2 Consumer register"]
    pub consumer_timer2_cc2: CONSUMER_TIMER2_CC2,
    #[doc = "0x190 - DTI Consumer register"]
    pub consumer_timer2_dti: CONSUMER_TIMER2_DTI,
    #[doc = "0x194 - DTI Consumer register"]
    pub consumer_timer2_dtifs1: CONSUMER_TIMER2_DTIFS1,
    #[doc = "0x198 - DTI Consumer register"]
    pub consumer_timer2_dtifs2: CONSUMER_TIMER2_DTIFS2,
    #[doc = "0x19c - CC0 consumer register"]
    pub consumer_timer3_cc0: CONSUMER_TIMER3_CC0,
    #[doc = "0x1a0 - CC1 Consumer register"]
    pub consumer_timer3_cc1: CONSUMER_TIMER3_CC1,
    #[doc = "0x1a4 - CC2 Consumer register"]
    pub consumer_timer3_cc2: CONSUMER_TIMER3_CC2,
    #[doc = "0x1a8 - DTI Consumer register"]
    pub consumer_timer3_dti: CONSUMER_TIMER3_DTI,
    #[doc = "0x1ac - DTI Consumer register"]
    pub consumer_timer3_dtifs1: CONSUMER_TIMER3_DTIFS1,
    #[doc = "0x1b0 - DTI Consumer register"]
    pub consumer_timer3_dtifs2: CONSUMER_TIMER3_DTIFS2,
    #[doc = "0x1b4 - CC0 consumer register"]
    pub consumer_timer4_cc0: CONSUMER_TIMER4_CC0,
    #[doc = "0x1b8 - CC1 Consumer register"]
    pub consumer_timer4_cc1: CONSUMER_TIMER4_CC1,
    #[doc = "0x1bc - CC2 Consumer register"]
    pub consumer_timer4_cc2: CONSUMER_TIMER4_CC2,
    #[doc = "0x1c0 - DTI Consumer register"]
    pub consumer_timer4_dti: CONSUMER_TIMER4_DTI,
    #[doc = "0x1c4 - DTI Consumer register"]
    pub consumer_timer4_dtifs1: CONSUMER_TIMER4_DTIFS1,
    #[doc = "0x1c8 - DTI Consumer register"]
    pub consumer_timer4_dtifs2: CONSUMER_TIMER4_DTIFS2,
    #[doc = "0x1cc - CLK consumer register"]
    pub consumer_usart0_clk: CONSUMER_USART0_CLK,
    #[doc = "0x1d0 - IR Consumer register"]
    pub consumer_usart0_ir: CONSUMER_USART0_IR,
    #[doc = "0x1d4 - RX Consumer register"]
    pub consumer_usart0_rx: CONSUMER_USART0_RX,
    #[doc = "0x1d8 - TRIGGER Consumer register"]
    pub consumer_usart0_trigger: CONSUMER_USART0_TRIGGER,
    _reserved92: [u8; 0x0c],
    #[doc = "0x1e8 - ASYNCTRIG consumer register"]
    pub consumer_vdac0_asynctrigch0: CONSUMER_VDAC0_ASYNCTRIGCH0,
    #[doc = "0x1ec - ASYNCTRIG Consumer register"]
    pub consumer_vdac0_asynctrigch1: CONSUMER_VDAC0_ASYNCTRIGCH1,
    #[doc = "0x1f0 - SYNCTRIG Consumer register"]
    pub consumer_vdac0_synctrigch0: CONSUMER_VDAC0_SYNCTRIGCH0,
    #[doc = "0x1f4 - SYNCTRIG Consumer register"]
    pub consumer_vdac0_synctrigch1: CONSUMER_VDAC0_SYNCTRIGCH1,
    #[doc = "0x1f8 - SRC0 consumer register"]
    pub consumer_wdog0_src0: CONSUMER_WDOG0_SRC0,
    #[doc = "0x1fc - SRC1 Consumer register"]
    pub consumer_wdog0_src1: CONSUMER_WDOG0_SRC1,
    #[doc = "0x200 - SRC0 consumer register"]
    pub consumer_wdog1_src0: CONSUMER_WDOG1_SRC0,
    #[doc = "0x204 - SRC1 Consumer register"]
    pub consumer_wdog1_src1: CONSUMER_WDOG1_SRC1,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "ASYNC_SWPULSE (w) register accessor: an alias for `Reg<ASYNC_SWPULSE_SPEC>`"]
pub type ASYNC_SWPULSE = crate::Reg<async_swpulse::ASYNC_SWPULSE_SPEC>;
#[doc = "No Description"]
pub mod async_swpulse;
#[doc = "ASYNC_SWLEVEL (rw) register accessor: an alias for `Reg<ASYNC_SWLEVEL_SPEC>`"]
pub type ASYNC_SWLEVEL = crate::Reg<async_swlevel::ASYNC_SWLEVEL_SPEC>;
#[doc = "No Description"]
pub mod async_swlevel;
#[doc = "ASYNC_PEEK (r) register accessor: an alias for `Reg<ASYNC_PEEK_SPEC>`"]
pub type ASYNC_PEEK = crate::Reg<async_peek::ASYNC_PEEK_SPEC>;
#[doc = "No Description"]
pub mod async_peek;
#[doc = "SYNC_PEEK (r) register accessor: an alias for `Reg<SYNC_PEEK_SPEC>`"]
pub type SYNC_PEEK = crate::Reg<sync_peek::SYNC_PEEK_SPEC>;
#[doc = "No Description"]
pub mod sync_peek;
#[doc = "ASYNC_CH0_CTRL (rw) register accessor: an alias for `Reg<ASYNC_CH0_CTRL_SPEC>`"]
pub type ASYNC_CH0_CTRL = crate::Reg<async_ch0_ctrl::ASYNC_CH0_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch0_ctrl;
#[doc = "ASYNC_CH1_CTRL (rw) register accessor: an alias for `Reg<ASYNC_CH1_CTRL_SPEC>`"]
pub type ASYNC_CH1_CTRL = crate::Reg<async_ch1_ctrl::ASYNC_CH1_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch1_ctrl;
#[doc = "ASYNC_CH2_CTRL (rw) register accessor: an alias for `Reg<ASYNC_CH2_CTRL_SPEC>`"]
pub type ASYNC_CH2_CTRL = crate::Reg<async_ch2_ctrl::ASYNC_CH2_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch2_ctrl;
#[doc = "ASYNC_CH3_CTRL (rw) register accessor: an alias for `Reg<ASYNC_CH3_CTRL_SPEC>`"]
pub type ASYNC_CH3_CTRL = crate::Reg<async_ch3_ctrl::ASYNC_CH3_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch3_ctrl;
#[doc = "ASYNC_CH4_CTRL (rw) register accessor: an alias for `Reg<ASYNC_CH4_CTRL_SPEC>`"]
pub type ASYNC_CH4_CTRL = crate::Reg<async_ch4_ctrl::ASYNC_CH4_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch4_ctrl;
#[doc = "ASYNC_CH5_CTRL (rw) register accessor: an alias for `Reg<ASYNC_CH5_CTRL_SPEC>`"]
pub type ASYNC_CH5_CTRL = crate::Reg<async_ch5_ctrl::ASYNC_CH5_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch5_ctrl;
#[doc = "ASYNC_CH6_CTRL (rw) register accessor: an alias for `Reg<ASYNC_CH6_CTRL_SPEC>`"]
pub type ASYNC_CH6_CTRL = crate::Reg<async_ch6_ctrl::ASYNC_CH6_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch6_ctrl;
#[doc = "ASYNC_CH7_CTRL (rw) register accessor: an alias for `Reg<ASYNC_CH7_CTRL_SPEC>`"]
pub type ASYNC_CH7_CTRL = crate::Reg<async_ch7_ctrl::ASYNC_CH7_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch7_ctrl;
#[doc = "ASYNC_CH8_CTRL (rw) register accessor: an alias for `Reg<ASYNC_CH8_CTRL_SPEC>`"]
pub type ASYNC_CH8_CTRL = crate::Reg<async_ch8_ctrl::ASYNC_CH8_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch8_ctrl;
#[doc = "ASYNC_CH9_CTRL (rw) register accessor: an alias for `Reg<ASYNC_CH9_CTRL_SPEC>`"]
pub type ASYNC_CH9_CTRL = crate::Reg<async_ch9_ctrl::ASYNC_CH9_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch9_ctrl;
#[doc = "ASYNC_CH10_CTRL (rw) register accessor: an alias for `Reg<ASYNC_CH10_CTRL_SPEC>`"]
pub type ASYNC_CH10_CTRL = crate::Reg<async_ch10_ctrl::ASYNC_CH10_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch10_ctrl;
#[doc = "ASYNC_CH11_CTRL (rw) register accessor: an alias for `Reg<ASYNC_CH11_CTRL_SPEC>`"]
pub type ASYNC_CH11_CTRL = crate::Reg<async_ch11_ctrl::ASYNC_CH11_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch11_ctrl;
#[doc = "SYNC_CH0_CTRL (rw) register accessor: an alias for `Reg<SYNC_CH0_CTRL_SPEC>`"]
pub type SYNC_CH0_CTRL = crate::Reg<sync_ch0_ctrl::SYNC_CH0_CTRL_SPEC>;
#[doc = "No Description"]
pub mod sync_ch0_ctrl;
#[doc = "SYNC_CH1_CTRL (rw) register accessor: an alias for `Reg<SYNC_CH1_CTRL_SPEC>`"]
pub type SYNC_CH1_CTRL = crate::Reg<sync_ch1_ctrl::SYNC_CH1_CTRL_SPEC>;
#[doc = "No Description"]
pub mod sync_ch1_ctrl;
#[doc = "SYNC_CH2_CTRL (rw) register accessor: an alias for `Reg<SYNC_CH2_CTRL_SPEC>`"]
pub type SYNC_CH2_CTRL = crate::Reg<sync_ch2_ctrl::SYNC_CH2_CTRL_SPEC>;
#[doc = "No Description"]
pub mod sync_ch2_ctrl;
#[doc = "SYNC_CH3_CTRL (rw) register accessor: an alias for `Reg<SYNC_CH3_CTRL_SPEC>`"]
pub type SYNC_CH3_CTRL = crate::Reg<sync_ch3_ctrl::SYNC_CH3_CTRL_SPEC>;
#[doc = "No Description"]
pub mod sync_ch3_ctrl;
#[doc = "CONSUMER_CMU_CALDN (rw) register accessor: an alias for `Reg<CONSUMER_CMU_CALDN_SPEC>`"]
pub type CONSUMER_CMU_CALDN = crate::Reg<consumer_cmu_caldn::CONSUMER_CMU_CALDN_SPEC>;
#[doc = "CALDN consumer register"]
pub mod consumer_cmu_caldn;
#[doc = "CONSUMER_CMU_CALUP (rw) register accessor: an alias for `Reg<CONSUMER_CMU_CALUP_SPEC>`"]
pub type CONSUMER_CMU_CALUP = crate::Reg<consumer_cmu_calup::CONSUMER_CMU_CALUP_SPEC>;
#[doc = "CALUP Consumer register"]
pub mod consumer_cmu_calup;
#[doc = "CONSUMER_EUSART0_CLK (rw) register accessor: an alias for `Reg<CONSUMER_EUSART0_CLK_SPEC>`"]
pub type CONSUMER_EUSART0_CLK = crate::Reg<consumer_eusart0_clk::CONSUMER_EUSART0_CLK_SPEC>;
#[doc = "CLK consumer register"]
pub mod consumer_eusart0_clk;
#[doc = "CONSUMER_EUSART0_RX (rw) register accessor: an alias for `Reg<CONSUMER_EUSART0_RX_SPEC>`"]
pub type CONSUMER_EUSART0_RX = crate::Reg<consumer_eusart0_rx::CONSUMER_EUSART0_RX_SPEC>;
#[doc = "RX Consumer register"]
pub mod consumer_eusart0_rx;
#[doc = "CONSUMER_EUSART0_TRIGGER (rw) register accessor: an alias for `Reg<CONSUMER_EUSART0_TRIGGER_SPEC>`"]
pub type CONSUMER_EUSART0_TRIGGER =
    crate::Reg<consumer_eusart0_trigger::CONSUMER_EUSART0_TRIGGER_SPEC>;
#[doc = "TRIGGER Consumer register"]
pub mod consumer_eusart0_trigger;
#[doc = "CONSUMER_EUSART1_CLK (rw) register accessor: an alias for `Reg<CONSUMER_EUSART1_CLK_SPEC>`"]
pub type CONSUMER_EUSART1_CLK = crate::Reg<consumer_eusart1_clk::CONSUMER_EUSART1_CLK_SPEC>;
#[doc = "CLK consumer register"]
pub mod consumer_eusart1_clk;
#[doc = "CONSUMER_EUSART1_RX (rw) register accessor: an alias for `Reg<CONSUMER_EUSART1_RX_SPEC>`"]
pub type CONSUMER_EUSART1_RX = crate::Reg<consumer_eusart1_rx::CONSUMER_EUSART1_RX_SPEC>;
#[doc = "RX Consumer register"]
pub mod consumer_eusart1_rx;
#[doc = "CONSUMER_EUSART1_TRIGGER (rw) register accessor: an alias for `Reg<CONSUMER_EUSART1_TRIGGER_SPEC>`"]
pub type CONSUMER_EUSART1_TRIGGER =
    crate::Reg<consumer_eusart1_trigger::CONSUMER_EUSART1_TRIGGER_SPEC>;
#[doc = "TRIGGER Consumer register"]
pub mod consumer_eusart1_trigger;
#[doc = "CONSUMER_EUSART2_CLK (rw) register accessor: an alias for `Reg<CONSUMER_EUSART2_CLK_SPEC>`"]
pub type CONSUMER_EUSART2_CLK = crate::Reg<consumer_eusart2_clk::CONSUMER_EUSART2_CLK_SPEC>;
#[doc = "CLK consumer register"]
pub mod consumer_eusart2_clk;
#[doc = "CONSUMER_EUSART2_RX (rw) register accessor: an alias for `Reg<CONSUMER_EUSART2_RX_SPEC>`"]
pub type CONSUMER_EUSART2_RX = crate::Reg<consumer_eusart2_rx::CONSUMER_EUSART2_RX_SPEC>;
#[doc = "RX Consumer register"]
pub mod consumer_eusart2_rx;
#[doc = "CONSUMER_EUSART2_TRIGGER (rw) register accessor: an alias for `Reg<CONSUMER_EUSART2_TRIGGER_SPEC>`"]
pub type CONSUMER_EUSART2_TRIGGER =
    crate::Reg<consumer_eusart2_trigger::CONSUMER_EUSART2_TRIGGER_SPEC>;
#[doc = "TRIGGER Consumer register"]
pub mod consumer_eusart2_trigger;
#[doc = "CONSUMER_IADC0_SCANTRIGGER (rw) register accessor: an alias for `Reg<CONSUMER_IADC0_SCANTRIGGER_SPEC>`"]
pub type CONSUMER_IADC0_SCANTRIGGER =
    crate::Reg<consumer_iadc0_scantrigger::CONSUMER_IADC0_SCANTRIGGER_SPEC>;
#[doc = "SCAN consumer register"]
pub mod consumer_iadc0_scantrigger;
#[doc = "CONSUMER_IADC0_SINGLETRIGGER (rw) register accessor: an alias for `Reg<CONSUMER_IADC0_SINGLETRIGGER_SPEC>`"]
pub type CONSUMER_IADC0_SINGLETRIGGER =
    crate::Reg<consumer_iadc0_singletrigger::CONSUMER_IADC0_SINGLETRIGGER_SPEC>;
#[doc = "SINGLE Consumer register"]
pub mod consumer_iadc0_singletrigger;
#[doc = "CONSUMER_LDMAXBAR_DMAREQ0 (rw) register accessor: an alias for `Reg<CONSUMER_LDMAXBAR_DMAREQ0_SPEC>`"]
pub type CONSUMER_LDMAXBAR_DMAREQ0 =
    crate::Reg<consumer_ldmaxbar_dmareq0::CONSUMER_LDMAXBAR_DMAREQ0_SPEC>;
#[doc = "DMAREQ0 consumer register"]
pub mod consumer_ldmaxbar_dmareq0;
#[doc = "CONSUMER_LDMAXBAR_DMAREQ1 (rw) register accessor: an alias for `Reg<CONSUMER_LDMAXBAR_DMAREQ1_SPEC>`"]
pub type CONSUMER_LDMAXBAR_DMAREQ1 =
    crate::Reg<consumer_ldmaxbar_dmareq1::CONSUMER_LDMAXBAR_DMAREQ1_SPEC>;
#[doc = "DMAREQ1 Consumer register"]
pub mod consumer_ldmaxbar_dmareq1;
#[doc = "CONSUMER_LESENSE_START (rw) register accessor: an alias for `Reg<CONSUMER_LESENSE_START_SPEC>`"]
pub type CONSUMER_LESENSE_START = crate::Reg<consumer_lesense_start::CONSUMER_LESENSE_START_SPEC>;
#[doc = "START Consumer register"]
pub mod consumer_lesense_start;
#[doc = "CONSUMER_LETIMER0_CLEAR (rw) register accessor: an alias for `Reg<CONSUMER_LETIMER0_CLEAR_SPEC>`"]
pub type CONSUMER_LETIMER0_CLEAR =
    crate::Reg<consumer_letimer0_clear::CONSUMER_LETIMER0_CLEAR_SPEC>;
#[doc = "CLEAR consumer register"]
pub mod consumer_letimer0_clear;
#[doc = "CONSUMER_LETIMER0_START (rw) register accessor: an alias for `Reg<CONSUMER_LETIMER0_START_SPEC>`"]
pub type CONSUMER_LETIMER0_START =
    crate::Reg<consumer_letimer0_start::CONSUMER_LETIMER0_START_SPEC>;
#[doc = "START Consumer register"]
pub mod consumer_letimer0_start;
#[doc = "CONSUMER_LETIMER0_STOP (rw) register accessor: an alias for `Reg<CONSUMER_LETIMER0_STOP_SPEC>`"]
pub type CONSUMER_LETIMER0_STOP = crate::Reg<consumer_letimer0_stop::CONSUMER_LETIMER0_STOP_SPEC>;
#[doc = "STOP Consumer register"]
pub mod consumer_letimer0_stop;
#[doc = "CONSUMER_PCNT0_S0IN (rw) register accessor: an alias for `Reg<CONSUMER_PCNT0_S0IN_SPEC>`"]
pub type CONSUMER_PCNT0_S0IN = crate::Reg<consumer_pcnt0_s0in::CONSUMER_PCNT0_S0IN_SPEC>;
#[doc = "S0IN consumer register"]
pub mod consumer_pcnt0_s0in;
#[doc = "CONSUMER_PCNT0_S1IN (rw) register accessor: an alias for `Reg<CONSUMER_PCNT0_S1IN_SPEC>`"]
pub type CONSUMER_PCNT0_S1IN = crate::Reg<consumer_pcnt0_s1in::CONSUMER_PCNT0_S1IN_SPEC>;
#[doc = "S1IN Consumer register"]
pub mod consumer_pcnt0_s1in;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC25 (rw) register accessor: an alias for `Reg<CONSUMER_SETAMPER_TAMPERSRC25_SPEC>`"]
pub type CONSUMER_SETAMPER_TAMPERSRC25 =
    crate::Reg<consumer_setamper_tampersrc25::CONSUMER_SETAMPER_TAMPERSRC25_SPEC>;
#[doc = "TAMPERSRC25 consumer register"]
pub mod consumer_setamper_tampersrc25;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC26 (rw) register accessor: an alias for `Reg<CONSUMER_SETAMPER_TAMPERSRC26_SPEC>`"]
pub type CONSUMER_SETAMPER_TAMPERSRC26 =
    crate::Reg<consumer_setamper_tampersrc26::CONSUMER_SETAMPER_TAMPERSRC26_SPEC>;
#[doc = "TAMPERSRC26 Consumer register"]
pub mod consumer_setamper_tampersrc26;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC27 (rw) register accessor: an alias for `Reg<CONSUMER_SETAMPER_TAMPERSRC27_SPEC>`"]
pub type CONSUMER_SETAMPER_TAMPERSRC27 =
    crate::Reg<consumer_setamper_tampersrc27::CONSUMER_SETAMPER_TAMPERSRC27_SPEC>;
#[doc = "TAMPERSRC27 Consumer register"]
pub mod consumer_setamper_tampersrc27;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC28 (rw) register accessor: an alias for `Reg<CONSUMER_SETAMPER_TAMPERSRC28_SPEC>`"]
pub type CONSUMER_SETAMPER_TAMPERSRC28 =
    crate::Reg<consumer_setamper_tampersrc28::CONSUMER_SETAMPER_TAMPERSRC28_SPEC>;
#[doc = "TAMPERSRC28 Consumer register"]
pub mod consumer_setamper_tampersrc28;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC29 (rw) register accessor: an alias for `Reg<CONSUMER_SETAMPER_TAMPERSRC29_SPEC>`"]
pub type CONSUMER_SETAMPER_TAMPERSRC29 =
    crate::Reg<consumer_setamper_tampersrc29::CONSUMER_SETAMPER_TAMPERSRC29_SPEC>;
#[doc = "TAMPERSRC29 Consumer register"]
pub mod consumer_setamper_tampersrc29;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC30 (rw) register accessor: an alias for `Reg<CONSUMER_SETAMPER_TAMPERSRC30_SPEC>`"]
pub type CONSUMER_SETAMPER_TAMPERSRC30 =
    crate::Reg<consumer_setamper_tampersrc30::CONSUMER_SETAMPER_TAMPERSRC30_SPEC>;
#[doc = "TAMPERSRC30 Consumer register"]
pub mod consumer_setamper_tampersrc30;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC31 (rw) register accessor: an alias for `Reg<CONSUMER_SETAMPER_TAMPERSRC31_SPEC>`"]
pub type CONSUMER_SETAMPER_TAMPERSRC31 =
    crate::Reg<consumer_setamper_tampersrc31::CONSUMER_SETAMPER_TAMPERSRC31_SPEC>;
#[doc = "TAMPERSRC31 Consumer register"]
pub mod consumer_setamper_tampersrc31;
#[doc = "CONSUMER_SYSRTC0_IN0 (rw) register accessor: an alias for `Reg<CONSUMER_SYSRTC0_IN0_SPEC>`"]
pub type CONSUMER_SYSRTC0_IN0 = crate::Reg<consumer_sysrtc0_in0::CONSUMER_SYSRTC0_IN0_SPEC>;
#[doc = "IN0 consumer register"]
pub mod consumer_sysrtc0_in0;
#[doc = "CONSUMER_SYSRTC0_IN1 (rw) register accessor: an alias for `Reg<CONSUMER_SYSRTC0_IN1_SPEC>`"]
pub type CONSUMER_SYSRTC0_IN1 = crate::Reg<consumer_sysrtc0_in1::CONSUMER_SYSRTC0_IN1_SPEC>;
#[doc = "IN1 Consumer register"]
pub mod consumer_sysrtc0_in1;
#[doc = "CONSUMER_HFXO0_OSCREQ (rw) register accessor: an alias for `Reg<CONSUMER_HFXO0_OSCREQ_SPEC>`"]
pub type CONSUMER_HFXO0_OSCREQ = crate::Reg<consumer_hfxo0_oscreq::CONSUMER_HFXO0_OSCREQ_SPEC>;
#[doc = "OSCREQ consumer register"]
pub mod consumer_hfxo0_oscreq;
#[doc = "CONSUMER_HFXO0_TIMEOUT (rw) register accessor: an alias for `Reg<CONSUMER_HFXO0_TIMEOUT_SPEC>`"]
pub type CONSUMER_HFXO0_TIMEOUT = crate::Reg<consumer_hfxo0_timeout::CONSUMER_HFXO0_TIMEOUT_SPEC>;
#[doc = "TIMEOUT Consumer register"]
pub mod consumer_hfxo0_timeout;
#[doc = "CONSUMER_CORE_CTIIN0 (rw) register accessor: an alias for `Reg<CONSUMER_CORE_CTIIN0_SPEC>`"]
pub type CONSUMER_CORE_CTIIN0 = crate::Reg<consumer_core_ctiin0::CONSUMER_CORE_CTIIN0_SPEC>;
#[doc = "CTI Consumer Register"]
pub mod consumer_core_ctiin0;
#[doc = "CONSUMER_CORE_CTIIN1 (rw) register accessor: an alias for `Reg<CONSUMER_CORE_CTIIN1_SPEC>`"]
pub type CONSUMER_CORE_CTIIN1 = crate::Reg<consumer_core_ctiin1::CONSUMER_CORE_CTIIN1_SPEC>;
#[doc = "CTI Consumer Register"]
pub mod consumer_core_ctiin1;
#[doc = "CONSUMER_CORE_CTIIN2 (rw) register accessor: an alias for `Reg<CONSUMER_CORE_CTIIN2_SPEC>`"]
pub type CONSUMER_CORE_CTIIN2 = crate::Reg<consumer_core_ctiin2::CONSUMER_CORE_CTIIN2_SPEC>;
#[doc = "CTI Consumer Register"]
pub mod consumer_core_ctiin2;
#[doc = "CONSUMER_CORE_CTIIN3 (rw) register accessor: an alias for `Reg<CONSUMER_CORE_CTIIN3_SPEC>`"]
pub type CONSUMER_CORE_CTIIN3 = crate::Reg<consumer_core_ctiin3::CONSUMER_CORE_CTIIN3_SPEC>;
#[doc = "CTI Consumer Register"]
pub mod consumer_core_ctiin3;
#[doc = "CONSUMER_CORE_M33RXEV (rw) register accessor: an alias for `Reg<CONSUMER_CORE_M33RXEV_SPEC>`"]
pub type CONSUMER_CORE_M33RXEV = crate::Reg<consumer_core_m33rxev::CONSUMER_CORE_M33RXEV_SPEC>;
#[doc = "M33 Consumer Register"]
pub mod consumer_core_m33rxev;
#[doc = "CONSUMER_TIMER0_CC0 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER0_CC0_SPEC>`"]
pub type CONSUMER_TIMER0_CC0 = crate::Reg<consumer_timer0_cc0::CONSUMER_TIMER0_CC0_SPEC>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer0_cc0;
#[doc = "CONSUMER_TIMER0_CC1 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER0_CC1_SPEC>`"]
pub type CONSUMER_TIMER0_CC1 = crate::Reg<consumer_timer0_cc1::CONSUMER_TIMER0_CC1_SPEC>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer0_cc1;
#[doc = "CONSUMER_TIMER0_CC2 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER0_CC2_SPEC>`"]
pub type CONSUMER_TIMER0_CC2 = crate::Reg<consumer_timer0_cc2::CONSUMER_TIMER0_CC2_SPEC>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer0_cc2;
#[doc = "CONSUMER_TIMER0_DTI (rw) register accessor: an alias for `Reg<CONSUMER_TIMER0_DTI_SPEC>`"]
pub type CONSUMER_TIMER0_DTI = crate::Reg<consumer_timer0_dti::CONSUMER_TIMER0_DTI_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer0_dti;
#[doc = "CONSUMER_TIMER0_DTIFS1 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER0_DTIFS1_SPEC>`"]
pub type CONSUMER_TIMER0_DTIFS1 = crate::Reg<consumer_timer0_dtifs1::CONSUMER_TIMER0_DTIFS1_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer0_dtifs1;
#[doc = "CONSUMER_TIMER0_DTIFS2 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER0_DTIFS2_SPEC>`"]
pub type CONSUMER_TIMER0_DTIFS2 = crate::Reg<consumer_timer0_dtifs2::CONSUMER_TIMER0_DTIFS2_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer0_dtifs2;
#[doc = "CONSUMER_TIMER1_CC0 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER1_CC0_SPEC>`"]
pub type CONSUMER_TIMER1_CC0 = crate::Reg<consumer_timer1_cc0::CONSUMER_TIMER1_CC0_SPEC>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer1_cc0;
#[doc = "CONSUMER_TIMER1_CC1 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER1_CC1_SPEC>`"]
pub type CONSUMER_TIMER1_CC1 = crate::Reg<consumer_timer1_cc1::CONSUMER_TIMER1_CC1_SPEC>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer1_cc1;
#[doc = "CONSUMER_TIMER1_CC2 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER1_CC2_SPEC>`"]
pub type CONSUMER_TIMER1_CC2 = crate::Reg<consumer_timer1_cc2::CONSUMER_TIMER1_CC2_SPEC>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer1_cc2;
#[doc = "CONSUMER_TIMER1_DTI (rw) register accessor: an alias for `Reg<CONSUMER_TIMER1_DTI_SPEC>`"]
pub type CONSUMER_TIMER1_DTI = crate::Reg<consumer_timer1_dti::CONSUMER_TIMER1_DTI_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer1_dti;
#[doc = "CONSUMER_TIMER1_DTIFS1 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER1_DTIFS1_SPEC>`"]
pub type CONSUMER_TIMER1_DTIFS1 = crate::Reg<consumer_timer1_dtifs1::CONSUMER_TIMER1_DTIFS1_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer1_dtifs1;
#[doc = "CONSUMER_TIMER1_DTIFS2 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER1_DTIFS2_SPEC>`"]
pub type CONSUMER_TIMER1_DTIFS2 = crate::Reg<consumer_timer1_dtifs2::CONSUMER_TIMER1_DTIFS2_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer1_dtifs2;
#[doc = "CONSUMER_TIMER2_CC0 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER2_CC0_SPEC>`"]
pub type CONSUMER_TIMER2_CC0 = crate::Reg<consumer_timer2_cc0::CONSUMER_TIMER2_CC0_SPEC>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer2_cc0;
#[doc = "CONSUMER_TIMER2_CC1 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER2_CC1_SPEC>`"]
pub type CONSUMER_TIMER2_CC1 = crate::Reg<consumer_timer2_cc1::CONSUMER_TIMER2_CC1_SPEC>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer2_cc1;
#[doc = "CONSUMER_TIMER2_CC2 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER2_CC2_SPEC>`"]
pub type CONSUMER_TIMER2_CC2 = crate::Reg<consumer_timer2_cc2::CONSUMER_TIMER2_CC2_SPEC>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer2_cc2;
#[doc = "CONSUMER_TIMER2_DTI (rw) register accessor: an alias for `Reg<CONSUMER_TIMER2_DTI_SPEC>`"]
pub type CONSUMER_TIMER2_DTI = crate::Reg<consumer_timer2_dti::CONSUMER_TIMER2_DTI_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer2_dti;
#[doc = "CONSUMER_TIMER2_DTIFS1 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER2_DTIFS1_SPEC>`"]
pub type CONSUMER_TIMER2_DTIFS1 = crate::Reg<consumer_timer2_dtifs1::CONSUMER_TIMER2_DTIFS1_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer2_dtifs1;
#[doc = "CONSUMER_TIMER2_DTIFS2 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER2_DTIFS2_SPEC>`"]
pub type CONSUMER_TIMER2_DTIFS2 = crate::Reg<consumer_timer2_dtifs2::CONSUMER_TIMER2_DTIFS2_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer2_dtifs2;
#[doc = "CONSUMER_TIMER3_CC0 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER3_CC0_SPEC>`"]
pub type CONSUMER_TIMER3_CC0 = crate::Reg<consumer_timer3_cc0::CONSUMER_TIMER3_CC0_SPEC>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer3_cc0;
#[doc = "CONSUMER_TIMER3_CC1 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER3_CC1_SPEC>`"]
pub type CONSUMER_TIMER3_CC1 = crate::Reg<consumer_timer3_cc1::CONSUMER_TIMER3_CC1_SPEC>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer3_cc1;
#[doc = "CONSUMER_TIMER3_CC2 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER3_CC2_SPEC>`"]
pub type CONSUMER_TIMER3_CC2 = crate::Reg<consumer_timer3_cc2::CONSUMER_TIMER3_CC2_SPEC>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer3_cc2;
#[doc = "CONSUMER_TIMER3_DTI (rw) register accessor: an alias for `Reg<CONSUMER_TIMER3_DTI_SPEC>`"]
pub type CONSUMER_TIMER3_DTI = crate::Reg<consumer_timer3_dti::CONSUMER_TIMER3_DTI_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer3_dti;
#[doc = "CONSUMER_TIMER3_DTIFS1 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER3_DTIFS1_SPEC>`"]
pub type CONSUMER_TIMER3_DTIFS1 = crate::Reg<consumer_timer3_dtifs1::CONSUMER_TIMER3_DTIFS1_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer3_dtifs1;
#[doc = "CONSUMER_TIMER3_DTIFS2 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER3_DTIFS2_SPEC>`"]
pub type CONSUMER_TIMER3_DTIFS2 = crate::Reg<consumer_timer3_dtifs2::CONSUMER_TIMER3_DTIFS2_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer3_dtifs2;
#[doc = "CONSUMER_TIMER4_CC0 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER4_CC0_SPEC>`"]
pub type CONSUMER_TIMER4_CC0 = crate::Reg<consumer_timer4_cc0::CONSUMER_TIMER4_CC0_SPEC>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer4_cc0;
#[doc = "CONSUMER_TIMER4_CC1 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER4_CC1_SPEC>`"]
pub type CONSUMER_TIMER4_CC1 = crate::Reg<consumer_timer4_cc1::CONSUMER_TIMER4_CC1_SPEC>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer4_cc1;
#[doc = "CONSUMER_TIMER4_CC2 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER4_CC2_SPEC>`"]
pub type CONSUMER_TIMER4_CC2 = crate::Reg<consumer_timer4_cc2::CONSUMER_TIMER4_CC2_SPEC>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer4_cc2;
#[doc = "CONSUMER_TIMER4_DTI (rw) register accessor: an alias for `Reg<CONSUMER_TIMER4_DTI_SPEC>`"]
pub type CONSUMER_TIMER4_DTI = crate::Reg<consumer_timer4_dti::CONSUMER_TIMER4_DTI_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer4_dti;
#[doc = "CONSUMER_TIMER4_DTIFS1 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER4_DTIFS1_SPEC>`"]
pub type CONSUMER_TIMER4_DTIFS1 = crate::Reg<consumer_timer4_dtifs1::CONSUMER_TIMER4_DTIFS1_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer4_dtifs1;
#[doc = "CONSUMER_TIMER4_DTIFS2 (rw) register accessor: an alias for `Reg<CONSUMER_TIMER4_DTIFS2_SPEC>`"]
pub type CONSUMER_TIMER4_DTIFS2 = crate::Reg<consumer_timer4_dtifs2::CONSUMER_TIMER4_DTIFS2_SPEC>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer4_dtifs2;
#[doc = "CONSUMER_USART0_CLK (rw) register accessor: an alias for `Reg<CONSUMER_USART0_CLK_SPEC>`"]
pub type CONSUMER_USART0_CLK = crate::Reg<consumer_usart0_clk::CONSUMER_USART0_CLK_SPEC>;
#[doc = "CLK consumer register"]
pub mod consumer_usart0_clk;
#[doc = "CONSUMER_USART0_IR (rw) register accessor: an alias for `Reg<CONSUMER_USART0_IR_SPEC>`"]
pub type CONSUMER_USART0_IR = crate::Reg<consumer_usart0_ir::CONSUMER_USART0_IR_SPEC>;
#[doc = "IR Consumer register"]
pub mod consumer_usart0_ir;
#[doc = "CONSUMER_USART0_RX (rw) register accessor: an alias for `Reg<CONSUMER_USART0_RX_SPEC>`"]
pub type CONSUMER_USART0_RX = crate::Reg<consumer_usart0_rx::CONSUMER_USART0_RX_SPEC>;
#[doc = "RX Consumer register"]
pub mod consumer_usart0_rx;
#[doc = "CONSUMER_USART0_TRIGGER (rw) register accessor: an alias for `Reg<CONSUMER_USART0_TRIGGER_SPEC>`"]
pub type CONSUMER_USART0_TRIGGER =
    crate::Reg<consumer_usart0_trigger::CONSUMER_USART0_TRIGGER_SPEC>;
#[doc = "TRIGGER Consumer register"]
pub mod consumer_usart0_trigger;
#[doc = "CONSUMER_VDAC0_ASYNCTRIGCH0 (rw) register accessor: an alias for `Reg<CONSUMER_VDAC0_ASYNCTRIGCH0_SPEC>`"]
pub type CONSUMER_VDAC0_ASYNCTRIGCH0 =
    crate::Reg<consumer_vdac0_asynctrigch0::CONSUMER_VDAC0_ASYNCTRIGCH0_SPEC>;
#[doc = "ASYNCTRIG consumer register"]
pub mod consumer_vdac0_asynctrigch0;
#[doc = "CONSUMER_VDAC0_ASYNCTRIGCH1 (rw) register accessor: an alias for `Reg<CONSUMER_VDAC0_ASYNCTRIGCH1_SPEC>`"]
pub type CONSUMER_VDAC0_ASYNCTRIGCH1 =
    crate::Reg<consumer_vdac0_asynctrigch1::CONSUMER_VDAC0_ASYNCTRIGCH1_SPEC>;
#[doc = "ASYNCTRIG Consumer register"]
pub mod consumer_vdac0_asynctrigch1;
#[doc = "CONSUMER_VDAC0_SYNCTRIGCH0 (rw) register accessor: an alias for `Reg<CONSUMER_VDAC0_SYNCTRIGCH0_SPEC>`"]
pub type CONSUMER_VDAC0_SYNCTRIGCH0 =
    crate::Reg<consumer_vdac0_synctrigch0::CONSUMER_VDAC0_SYNCTRIGCH0_SPEC>;
#[doc = "SYNCTRIG Consumer register"]
pub mod consumer_vdac0_synctrigch0;
#[doc = "CONSUMER_VDAC0_SYNCTRIGCH1 (rw) register accessor: an alias for `Reg<CONSUMER_VDAC0_SYNCTRIGCH1_SPEC>`"]
pub type CONSUMER_VDAC0_SYNCTRIGCH1 =
    crate::Reg<consumer_vdac0_synctrigch1::CONSUMER_VDAC0_SYNCTRIGCH1_SPEC>;
#[doc = "SYNCTRIG Consumer register"]
pub mod consumer_vdac0_synctrigch1;
#[doc = "CONSUMER_WDOG0_SRC0 (rw) register accessor: an alias for `Reg<CONSUMER_WDOG0_SRC0_SPEC>`"]
pub type CONSUMER_WDOG0_SRC0 = crate::Reg<consumer_wdog0_src0::CONSUMER_WDOG0_SRC0_SPEC>;
#[doc = "SRC0 consumer register"]
pub mod consumer_wdog0_src0;
#[doc = "CONSUMER_WDOG0_SRC1 (rw) register accessor: an alias for `Reg<CONSUMER_WDOG0_SRC1_SPEC>`"]
pub type CONSUMER_WDOG0_SRC1 = crate::Reg<consumer_wdog0_src1::CONSUMER_WDOG0_SRC1_SPEC>;
#[doc = "SRC1 Consumer register"]
pub mod consumer_wdog0_src1;
#[doc = "CONSUMER_WDOG1_SRC0 (rw) register accessor: an alias for `Reg<CONSUMER_WDOG1_SRC0_SPEC>`"]
pub type CONSUMER_WDOG1_SRC0 = crate::Reg<consumer_wdog1_src0::CONSUMER_WDOG1_SRC0_SPEC>;
#[doc = "SRC0 consumer register"]
pub mod consumer_wdog1_src0;
#[doc = "CONSUMER_WDOG1_SRC1 (rw) register accessor: an alias for `Reg<CONSUMER_WDOG1_SRC1_SPEC>`"]
pub type CONSUMER_WDOG1_SRC1 = crate::Reg<consumer_wdog1_src1::CONSUMER_WDOG1_SRC1_SPEC>;
#[doc = "SRC1 Consumer register"]
pub mod consumer_wdog1_src1;
