#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: IPVERSION,
    _reserved1: [u8; 0x2c],
    #[doc = "0x30 - Port control"]
    pub porta_ctrl: PORTA_CTRL,
    #[doc = "0x34 - mode low"]
    pub porta_model: PORTA_MODEL,
    _reserved3: [u8; 0x04],
    #[doc = "0x3c - mode high"]
    pub porta_modeh: PORTA_MODEH,
    #[doc = "0x40 - data out"]
    pub porta_dout: PORTA_DOUT,
    #[doc = "0x44 - data in"]
    pub porta_din: PORTA_DIN,
    _reserved6: [u8; 0x18],
    #[doc = "0x60 - Port control"]
    pub portb_ctrl: PORTB_CTRL,
    #[doc = "0x64 - mode low"]
    pub portb_model: PORTB_MODEL,
    _reserved8: [u8; 0x08],
    #[doc = "0x70 - data out"]
    pub portb_dout: PORTB_DOUT,
    #[doc = "0x74 - data in"]
    pub portb_din: PORTB_DIN,
    _reserved10: [u8; 0x18],
    #[doc = "0x90 - Port control"]
    pub portc_ctrl: PORTC_CTRL,
    #[doc = "0x94 - mode low"]
    pub portc_model: PORTC_MODEL,
    _reserved12: [u8; 0x04],
    #[doc = "0x9c - mode high"]
    pub portc_modeh: PORTC_MODEH,
    #[doc = "0xa0 - data out"]
    pub portc_dout: PORTC_DOUT,
    #[doc = "0xa4 - data in"]
    pub portc_din: PORTC_DIN,
    _reserved15: [u8; 0x18],
    #[doc = "0xc0 - Port control"]
    pub portd_ctrl: PORTD_CTRL,
    #[doc = "0xc4 - mode low"]
    pub portd_model: PORTD_MODEL,
    _reserved17: [u8; 0x08],
    #[doc = "0xd0 - data out"]
    pub portd_dout: PORTD_DOUT,
    #[doc = "0xd4 - data in"]
    pub portd_din: PORTD_DIN,
    _reserved19: [u8; 0x0228],
    #[doc = "0x300 - No Description"]
    pub lock: LOCK,
    _reserved20: [u8; 0x0c],
    #[doc = "0x310 - No Description"]
    pub gpiolockstatus: GPIOLOCKSTATUS,
    _reserved21: [u8; 0x0c],
    #[doc = "0x320 - A Bus allocation"]
    pub abusalloc: ABUSALLOC,
    #[doc = "0x324 - B Bus allocation"]
    pub bbusalloc: BBUSALLOC,
    #[doc = "0x328 - CD Bus allocation"]
    pub cdbusalloc: CDBUSALLOC,
    _reserved24: [u8; 0xd4],
    #[doc = "0x400 - External Interrupt Port Select Low"]
    pub extipsell: EXTIPSELL,
    #[doc = "0x404 - External interrupt Port Select High"]
    pub extipselh: EXTIPSELH,
    #[doc = "0x408 - External Interrupt Pin Select Low"]
    pub extipinsell: EXTIPINSELL,
    #[doc = "0x40c - External Interrupt Pin Select High"]
    pub extipinselh: EXTIPINSELH,
    #[doc = "0x410 - External Interrupt Rising Edge Trigger"]
    pub extirise: EXTIRISE,
    #[doc = "0x414 - External Interrupt Falling Edge Trigger"]
    pub extifall: EXTIFALL,
    _reserved30: [u8; 0x08],
    #[doc = "0x420 - Interrupt Flag"]
    pub if_: IF,
    #[doc = "0x424 - Interrupt Enable"]
    pub ien: IEN,
    _reserved32: [u8; 0x04],
    #[doc = "0x42c - No Description"]
    pub em4wuen: EM4WUEN,
    #[doc = "0x430 - No Description"]
    pub em4wupol: EM4WUPOL,
    _reserved34: [u8; 0x0c],
    #[doc = "0x440 - No Description"]
    pub dbgroutepen: DBGROUTEPEN,
    #[doc = "0x444 - No Description"]
    pub traceroutepen: TRACEROUTEPEN,
    _reserved36: [u8; 0x18],
    #[doc = "0x460 - LCD Segment Enable"]
    pub lcdseg: LCDSEG,
    _reserved37: [u8; 0x0c],
    #[doc = "0x470 - LCD Common Enable"]
    pub lcdcom: LCDCOM,
    _reserved38: [u8; 0x0c],
    #[doc = "0x480 - ACMP0 pin enable"]
    pub acmp0_routeen: ACMP0_ROUTEEN,
    #[doc = "0x484 - ACMPOUT port/pin select"]
    pub acmp0_acmpoutroute: ACMP0_ACMPOUTROUTE,
    _reserved40: [u8; 0x04],
    #[doc = "0x48c - ACMP1 pin enable"]
    pub acmp1_routeen: ACMP1_ROUTEEN,
    #[doc = "0x490 - ACMPOUT port/pin select"]
    pub acmp1_acmpoutroute: ACMP1_ACMPOUTROUTE,
    _reserved42: [u8; 0x04],
    #[doc = "0x498 - CMU pin enable"]
    pub cmu_routeen: CMU_ROUTEEN,
    #[doc = "0x49c - CLKIN0 port/pin select"]
    pub cmu_clkin0route: CMU_CLKIN0ROUTE,
    #[doc = "0x4a0 - CLKOUT0 port/pin select"]
    pub cmu_clkout0route: CMU_CLKOUT0ROUTE,
    #[doc = "0x4a4 - CLKOUT1 port/pin select"]
    pub cmu_clkout1route: CMU_CLKOUT1ROUTE,
    #[doc = "0x4a8 - CLKOUT2 port/pin select"]
    pub cmu_clkout2route: CMU_CLKOUT2ROUTE,
    _reserved47: [u8; 0x18],
    #[doc = "0x4c4 - EUSART0 pin enable"]
    pub eusart0_routeen: EUSART0_ROUTEEN,
    #[doc = "0x4c8 - CS port/pin select"]
    pub eusart0_csroute: EUSART0_CSROUTE,
    #[doc = "0x4cc - CTS port/pin select"]
    pub eusart0_ctsroute: EUSART0_CTSROUTE,
    #[doc = "0x4d0 - RTS port/pin select"]
    pub eusart0_rtsroute: EUSART0_RTSROUTE,
    #[doc = "0x4d4 - RX port/pin select"]
    pub eusart0_rxroute: EUSART0_RXROUTE,
    #[doc = "0x4d8 - SCLK port/pin select"]
    pub eusart0_sclkroute: EUSART0_SCLKROUTE,
    #[doc = "0x4dc - TX port/pin select"]
    pub eusart0_txroute: EUSART0_TXROUTE,
    _reserved54: [u8; 0x04],
    #[doc = "0x4e4 - EUSART1 pin enable"]
    pub eusart1_routeen: EUSART1_ROUTEEN,
    #[doc = "0x4e8 - CS port/pin select"]
    pub eusart1_csroute: EUSART1_CSROUTE,
    #[doc = "0x4ec - CTS port/pin select"]
    pub eusart1_ctsroute: EUSART1_CTSROUTE,
    #[doc = "0x4f0 - RTS port/pin select"]
    pub eusart1_rtsroute: EUSART1_RTSROUTE,
    #[doc = "0x4f4 - RX port/pin select"]
    pub eusart1_rxroute: EUSART1_RXROUTE,
    #[doc = "0x4f8 - SCLK port/pin select"]
    pub eusart1_sclkroute: EUSART1_SCLKROUTE,
    #[doc = "0x4fc - TX port/pin select"]
    pub eusart1_txroute: EUSART1_TXROUTE,
    _reserved61: [u8; 0x04],
    #[doc = "0x504 - EUSART2 pin enable"]
    pub eusart2_routeen: EUSART2_ROUTEEN,
    #[doc = "0x508 - CS port/pin select"]
    pub eusart2_csroute: EUSART2_CSROUTE,
    #[doc = "0x50c - CTS port/pin select"]
    pub eusart2_ctsroute: EUSART2_CTSROUTE,
    #[doc = "0x510 - RTS port/pin select"]
    pub eusart2_rtsroute: EUSART2_RTSROUTE,
    #[doc = "0x514 - RX port/pin select"]
    pub eusart2_rxroute: EUSART2_RXROUTE,
    #[doc = "0x518 - SCLK port/pin select"]
    pub eusart2_sclkroute: EUSART2_SCLKROUTE,
    #[doc = "0x51c - TX port/pin select"]
    pub eusart2_txroute: EUSART2_TXROUTE,
    _reserved68: [u8; 0x18],
    #[doc = "0x538 - I2C0 pin enable"]
    pub i2c0_routeen: I2C0_ROUTEEN,
    #[doc = "0x53c - SCL port/pin select"]
    pub i2c0_sclroute: I2C0_SCLROUTE,
    #[doc = "0x540 - SDA port/pin select"]
    pub i2c0_sdaroute: I2C0_SDAROUTE,
    _reserved71: [u8; 0x04],
    #[doc = "0x548 - I2C1 pin enable"]
    pub i2c1_routeen: I2C1_ROUTEEN,
    #[doc = "0x54c - SCL port/pin select"]
    pub i2c1_sclroute: I2C1_SCLROUTE,
    #[doc = "0x550 - SDA port/pin select"]
    pub i2c1_sdaroute: I2C1_SDAROUTE,
    _reserved74: [u8; 0x04],
    #[doc = "0x558 - KEYSCAN pin enable"]
    pub keyscan_routeen: KEYSCAN_ROUTEEN,
    #[doc = "0x55c - COLOUT0 port/pin select"]
    pub keyscan_colout0route: KEYSCAN_COLOUT0ROUTE,
    #[doc = "0x560 - COLOUT1 port/pin select"]
    pub keyscan_colout1route: KEYSCAN_COLOUT1ROUTE,
    #[doc = "0x564 - COLOUT2 port/pin select"]
    pub keyscan_colout2route: KEYSCAN_COLOUT2ROUTE,
    #[doc = "0x568 - COLOUT3 port/pin select"]
    pub keyscan_colout3route: KEYSCAN_COLOUT3ROUTE,
    #[doc = "0x56c - COLOUT4 port/pin select"]
    pub keyscan_colout4route: KEYSCAN_COLOUT4ROUTE,
    #[doc = "0x570 - COLOUT5 port/pin select"]
    pub keyscan_colout5route: KEYSCAN_COLOUT5ROUTE,
    #[doc = "0x574 - COLOUT6 port/pin select"]
    pub keyscan_colout6route: KEYSCAN_COLOUT6ROUTE,
    #[doc = "0x578 - COLOUT7 port/pin select"]
    pub keyscan_colout7route: KEYSCAN_COLOUT7ROUTE,
    #[doc = "0x57c - ROWSENSE0 port/pin select"]
    pub keyscan_rowsense0route: KEYSCAN_ROWSENSE0ROUTE,
    #[doc = "0x580 - ROWSENSE1 port/pin select"]
    pub keyscan_rowsense1route: KEYSCAN_ROWSENSE1ROUTE,
    #[doc = "0x584 - ROWSENSE2 port/pin select"]
    pub keyscan_rowsense2route: KEYSCAN_ROWSENSE2ROUTE,
    #[doc = "0x588 - ROWSENSE3 port/pin select"]
    pub keyscan_rowsense3route: KEYSCAN_ROWSENSE3ROUTE,
    #[doc = "0x58c - ROWSENSE4 port/pin select"]
    pub keyscan_rowsense4route: KEYSCAN_ROWSENSE4ROUTE,
    #[doc = "0x590 - ROWSENSE5 port/pin select"]
    pub keyscan_rowsense5route: KEYSCAN_ROWSENSE5ROUTE,
    _reserved89: [u8; 0x04],
    #[doc = "0x598 - LESENSE pin enable"]
    pub lesense_routeen: LESENSE_ROUTEEN,
    #[doc = "0x59c - CH0OUT port/pin select"]
    pub lesense_ch0outroute: LESENSE_CH0OUTROUTE,
    #[doc = "0x5a0 - CH1OUT port/pin select"]
    pub lesense_ch1outroute: LESENSE_CH1OUTROUTE,
    #[doc = "0x5a4 - CH2OUT port/pin select"]
    pub lesense_ch2outroute: LESENSE_CH2OUTROUTE,
    #[doc = "0x5a8 - CH3OUT port/pin select"]
    pub lesense_ch3outroute: LESENSE_CH3OUTROUTE,
    #[doc = "0x5ac - CH4OUT port/pin select"]
    pub lesense_ch4outroute: LESENSE_CH4OUTROUTE,
    #[doc = "0x5b0 - CH5OUT port/pin select"]
    pub lesense_ch5outroute: LESENSE_CH5OUTROUTE,
    #[doc = "0x5b4 - CH6OUT port/pin select"]
    pub lesense_ch6outroute: LESENSE_CH6OUTROUTE,
    #[doc = "0x5b8 - CH7OUT port/pin select"]
    pub lesense_ch7outroute: LESENSE_CH7OUTROUTE,
    #[doc = "0x5bc - CH8OUT port/pin select"]
    pub lesense_ch8outroute: LESENSE_CH8OUTROUTE,
    #[doc = "0x5c0 - CH9OUT port/pin select"]
    pub lesense_ch9outroute: LESENSE_CH9OUTROUTE,
    #[doc = "0x5c4 - CH10OUT port/pin select"]
    pub lesense_ch10outroute: LESENSE_CH10OUTROUTE,
    #[doc = "0x5c8 - CH11OUT port/pin select"]
    pub lesense_ch11outroute: LESENSE_CH11OUTROUTE,
    #[doc = "0x5cc - CH12OUT port/pin select"]
    pub lesense_ch12outroute: LESENSE_CH12OUTROUTE,
    #[doc = "0x5d0 - CH13OUT port/pin select"]
    pub lesense_ch13outroute: LESENSE_CH13OUTROUTE,
    #[doc = "0x5d4 - CH14OUT port/pin select"]
    pub lesense_ch14outroute: LESENSE_CH14OUTROUTE,
    #[doc = "0x5d8 - CH15OUT port/pin select"]
    pub lesense_ch15outroute: LESENSE_CH15OUTROUTE,
    _reserved106: [u8; 0x04],
    #[doc = "0x5e0 - LETIMER pin enable"]
    pub letimer_routeen: LETIMER_ROUTEEN,
    #[doc = "0x5e4 - OUT0 port/pin select"]
    pub letimer_out0route: LETIMER_OUT0ROUTE,
    #[doc = "0x5e8 - OUT1 port/pin select"]
    pub letimer_out1route: LETIMER_OUT1ROUTE,
    _reserved109: [u8; 0x50],
    #[doc = "0x63c - S0IN port/pin select"]
    pub pcnt0_s0inroute: PCNT0_S0INROUTE,
    #[doc = "0x640 - S1IN port/pin select"]
    pub pcnt0_s1inroute: PCNT0_S1INROUTE,
    _reserved111: [u8; 0x04],
    #[doc = "0x648 - PRS0 pin enable"]
    pub prs0_routeen: PRS0_ROUTEEN,
    #[doc = "0x64c - ASYNCH0 port/pin select"]
    pub prs0_asynch0route: PRS0_ASYNCH0ROUTE,
    #[doc = "0x650 - ASYNCH1 port/pin select"]
    pub prs0_asynch1route: PRS0_ASYNCH1ROUTE,
    #[doc = "0x654 - ASYNCH2 port/pin select"]
    pub prs0_asynch2route: PRS0_ASYNCH2ROUTE,
    #[doc = "0x658 - ASYNCH3 port/pin select"]
    pub prs0_asynch3route: PRS0_ASYNCH3ROUTE,
    #[doc = "0x65c - ASYNCH4 port/pin select"]
    pub prs0_asynch4route: PRS0_ASYNCH4ROUTE,
    #[doc = "0x660 - ASYNCH5 port/pin select"]
    pub prs0_asynch5route: PRS0_ASYNCH5ROUTE,
    #[doc = "0x664 - ASYNCH6 port/pin select"]
    pub prs0_asynch6route: PRS0_ASYNCH6ROUTE,
    #[doc = "0x668 - ASYNCH7 port/pin select"]
    pub prs0_asynch7route: PRS0_ASYNCH7ROUTE,
    #[doc = "0x66c - ASYNCH8 port/pin select"]
    pub prs0_asynch8route: PRS0_ASYNCH8ROUTE,
    #[doc = "0x670 - ASYNCH9 port/pin select"]
    pub prs0_asynch9route: PRS0_ASYNCH9ROUTE,
    #[doc = "0x674 - ASYNCH10 port/pin select"]
    pub prs0_asynch10route: PRS0_ASYNCH10ROUTE,
    #[doc = "0x678 - ASYNCH11 port/pin select"]
    pub prs0_asynch11route: PRS0_ASYNCH11ROUTE,
    #[doc = "0x67c - SYNCH0 port/pin select"]
    pub prs0_synch0route: PRS0_SYNCH0ROUTE,
    #[doc = "0x680 - SYNCH1 port/pin select"]
    pub prs0_synch1route: PRS0_SYNCH1ROUTE,
    #[doc = "0x684 - SYNCH2 port/pin select"]
    pub prs0_synch2route: PRS0_SYNCH2ROUTE,
    #[doc = "0x688 - SYNCH3 port/pin select"]
    pub prs0_synch3route: PRS0_SYNCH3ROUTE,
    _reserved128: [u8; 0x64],
    #[doc = "0x6f0 - BUFOUTREQINASYNC port/pin select"]
    pub syxo0_bufoutreqinasyncroute: SYXO0_BUFOUTREQINASYNCROUTE,
    _reserved129: [u8; 0x04],
    #[doc = "0x6f8 - TIMER0 pin enable"]
    pub timer0_routeen: TIMER0_ROUTEEN,
    #[doc = "0x6fc - CC0 port/pin select"]
    pub timer0_cc0route: TIMER0_CC0ROUTE,
    #[doc = "0x700 - CC1 port/pin select"]
    pub timer0_cc1route: TIMER0_CC1ROUTE,
    #[doc = "0x704 - CC2 port/pin select"]
    pub timer0_cc2route: TIMER0_CC2ROUTE,
    #[doc = "0x708 - CDTI0 port/pin select"]
    pub timer0_cdti0route: TIMER0_CDTI0ROUTE,
    #[doc = "0x70c - CDTI1 port/pin select"]
    pub timer0_cdti1route: TIMER0_CDTI1ROUTE,
    #[doc = "0x710 - CDTI2 port/pin select"]
    pub timer0_cdti2route: TIMER0_CDTI2ROUTE,
    _reserved136: [u8; 0x04],
    #[doc = "0x718 - TIMER1 pin enable"]
    pub timer1_routeen: TIMER1_ROUTEEN,
    #[doc = "0x71c - CC0 port/pin select"]
    pub timer1_cc0route: TIMER1_CC0ROUTE,
    #[doc = "0x720 - CC1 port/pin select"]
    pub timer1_cc1route: TIMER1_CC1ROUTE,
    #[doc = "0x724 - CC2 port/pin select"]
    pub timer1_cc2route: TIMER1_CC2ROUTE,
    #[doc = "0x728 - CDTI0 port/pin select"]
    pub timer1_cdti0route: TIMER1_CDTI0ROUTE,
    #[doc = "0x72c - CDTI1 port/pin select"]
    pub timer1_cdti1route: TIMER1_CDTI1ROUTE,
    #[doc = "0x730 - CDTI2 port/pin select"]
    pub timer1_cdti2route: TIMER1_CDTI2ROUTE,
    _reserved143: [u8; 0x04],
    #[doc = "0x738 - TIMER2 pin enable"]
    pub timer2_routeen: TIMER2_ROUTEEN,
    #[doc = "0x73c - CC0 port/pin select"]
    pub timer2_cc0route: TIMER2_CC0ROUTE,
    #[doc = "0x740 - CC1 port/pin select"]
    pub timer2_cc1route: TIMER2_CC1ROUTE,
    #[doc = "0x744 - CC2 port/pin select"]
    pub timer2_cc2route: TIMER2_CC2ROUTE,
    #[doc = "0x748 - CDTI0 port/pin select"]
    pub timer2_cdti0route: TIMER2_CDTI0ROUTE,
    #[doc = "0x74c - CDTI1 port/pin select"]
    pub timer2_cdti1route: TIMER2_CDTI1ROUTE,
    #[doc = "0x750 - CDTI2 port/pin select"]
    pub timer2_cdti2route: TIMER2_CDTI2ROUTE,
    _reserved150: [u8; 0x04],
    #[doc = "0x758 - TIMER3 pin enable"]
    pub timer3_routeen: TIMER3_ROUTEEN,
    #[doc = "0x75c - CC0 port/pin select"]
    pub timer3_cc0route: TIMER3_CC0ROUTE,
    #[doc = "0x760 - CC1 port/pin select"]
    pub timer3_cc1route: TIMER3_CC1ROUTE,
    #[doc = "0x764 - CC2 port/pin select"]
    pub timer3_cc2route: TIMER3_CC2ROUTE,
    #[doc = "0x768 - CDTI0 port/pin select"]
    pub timer3_cdti0route: TIMER3_CDTI0ROUTE,
    #[doc = "0x76c - CDTI1 port/pin select"]
    pub timer3_cdti1route: TIMER3_CDTI1ROUTE,
    #[doc = "0x770 - CDTI2 port/pin select"]
    pub timer3_cdti2route: TIMER3_CDTI2ROUTE,
    _reserved157: [u8; 0x04],
    #[doc = "0x778 - TIMER4 pin enable"]
    pub timer4_routeen: TIMER4_ROUTEEN,
    #[doc = "0x77c - CC0 port/pin select"]
    pub timer4_cc0route: TIMER4_CC0ROUTE,
    #[doc = "0x780 - CC1 port/pin select"]
    pub timer4_cc1route: TIMER4_CC1ROUTE,
    #[doc = "0x784 - CC2 port/pin select"]
    pub timer4_cc2route: TIMER4_CC2ROUTE,
    #[doc = "0x788 - CDTI0 port/pin select"]
    pub timer4_cdti0route: TIMER4_CDTI0ROUTE,
    #[doc = "0x78c - CDTI1 port/pin select"]
    pub timer4_cdti1route: TIMER4_CDTI1ROUTE,
    #[doc = "0x790 - CDTI2 port/pin select"]
    pub timer4_cdti2route: TIMER4_CDTI2ROUTE,
    _reserved164: [u8; 0x04],
    #[doc = "0x798 - USART0 pin enable"]
    pub usart0_routeen: USART0_ROUTEEN,
    #[doc = "0x79c - CS port/pin select"]
    pub usart0_csroute: USART0_CSROUTE,
    #[doc = "0x7a0 - CTS port/pin select"]
    pub usart0_ctsroute: USART0_CTSROUTE,
    #[doc = "0x7a4 - RTS port/pin select"]
    pub usart0_rtsroute: USART0_RTSROUTE,
    #[doc = "0x7a8 - RX port/pin select"]
    pub usart0_rxroute: USART0_RXROUTE,
    #[doc = "0x7ac - SCLK port/pin select"]
    pub usart0_clkroute: USART0_CLKROUTE,
    #[doc = "0x7b0 - TX port/pin select"]
    pub usart0_txroute: USART0_TXROUTE,
}
#[doc = "IPVERSION (r) register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "PORTA_CTRL (rw) register accessor: an alias for `Reg<PORTA_CTRL_SPEC>`"]
pub type PORTA_CTRL = crate::Reg<porta_ctrl::PORTA_CTRL_SPEC>;
#[doc = "Port control"]
pub mod porta_ctrl;
#[doc = "PORTA_MODEL (rw) register accessor: an alias for `Reg<PORTA_MODEL_SPEC>`"]
pub type PORTA_MODEL = crate::Reg<porta_model::PORTA_MODEL_SPEC>;
#[doc = "mode low"]
pub mod porta_model;
#[doc = "PORTA_MODEH (rw) register accessor: an alias for `Reg<PORTA_MODEH_SPEC>`"]
pub type PORTA_MODEH = crate::Reg<porta_modeh::PORTA_MODEH_SPEC>;
#[doc = "mode high"]
pub mod porta_modeh;
#[doc = "PORTA_DOUT (rw) register accessor: an alias for `Reg<PORTA_DOUT_SPEC>`"]
pub type PORTA_DOUT = crate::Reg<porta_dout::PORTA_DOUT_SPEC>;
#[doc = "data out"]
pub mod porta_dout;
#[doc = "PORTA_DIN (r) register accessor: an alias for `Reg<PORTA_DIN_SPEC>`"]
pub type PORTA_DIN = crate::Reg<porta_din::PORTA_DIN_SPEC>;
#[doc = "data in"]
pub mod porta_din;
#[doc = "PORTB_CTRL (rw) register accessor: an alias for `Reg<PORTB_CTRL_SPEC>`"]
pub type PORTB_CTRL = crate::Reg<portb_ctrl::PORTB_CTRL_SPEC>;
#[doc = "Port control"]
pub mod portb_ctrl;
#[doc = "PORTB_MODEL (rw) register accessor: an alias for `Reg<PORTB_MODEL_SPEC>`"]
pub type PORTB_MODEL = crate::Reg<portb_model::PORTB_MODEL_SPEC>;
#[doc = "mode low"]
pub mod portb_model;
#[doc = "PORTB_DOUT (rw) register accessor: an alias for `Reg<PORTB_DOUT_SPEC>`"]
pub type PORTB_DOUT = crate::Reg<portb_dout::PORTB_DOUT_SPEC>;
#[doc = "data out"]
pub mod portb_dout;
#[doc = "PORTB_DIN (r) register accessor: an alias for `Reg<PORTB_DIN_SPEC>`"]
pub type PORTB_DIN = crate::Reg<portb_din::PORTB_DIN_SPEC>;
#[doc = "data in"]
pub mod portb_din;
#[doc = "PORTC_CTRL (rw) register accessor: an alias for `Reg<PORTC_CTRL_SPEC>`"]
pub type PORTC_CTRL = crate::Reg<portc_ctrl::PORTC_CTRL_SPEC>;
#[doc = "Port control"]
pub mod portc_ctrl;
#[doc = "PORTC_MODEL (rw) register accessor: an alias for `Reg<PORTC_MODEL_SPEC>`"]
pub type PORTC_MODEL = crate::Reg<portc_model::PORTC_MODEL_SPEC>;
#[doc = "mode low"]
pub mod portc_model;
#[doc = "PORTC_MODEH (rw) register accessor: an alias for `Reg<PORTC_MODEH_SPEC>`"]
pub type PORTC_MODEH = crate::Reg<portc_modeh::PORTC_MODEH_SPEC>;
#[doc = "mode high"]
pub mod portc_modeh;
#[doc = "PORTC_DOUT (rw) register accessor: an alias for `Reg<PORTC_DOUT_SPEC>`"]
pub type PORTC_DOUT = crate::Reg<portc_dout::PORTC_DOUT_SPEC>;
#[doc = "data out"]
pub mod portc_dout;
#[doc = "PORTC_DIN (r) register accessor: an alias for `Reg<PORTC_DIN_SPEC>`"]
pub type PORTC_DIN = crate::Reg<portc_din::PORTC_DIN_SPEC>;
#[doc = "data in"]
pub mod portc_din;
#[doc = "PORTD_CTRL (rw) register accessor: an alias for `Reg<PORTD_CTRL_SPEC>`"]
pub type PORTD_CTRL = crate::Reg<portd_ctrl::PORTD_CTRL_SPEC>;
#[doc = "Port control"]
pub mod portd_ctrl;
#[doc = "PORTD_MODEL (rw) register accessor: an alias for `Reg<PORTD_MODEL_SPEC>`"]
pub type PORTD_MODEL = crate::Reg<portd_model::PORTD_MODEL_SPEC>;
#[doc = "mode low"]
pub mod portd_model;
#[doc = "PORTD_DOUT (rw) register accessor: an alias for `Reg<PORTD_DOUT_SPEC>`"]
pub type PORTD_DOUT = crate::Reg<portd_dout::PORTD_DOUT_SPEC>;
#[doc = "data out"]
pub mod portd_dout;
#[doc = "PORTD_DIN (r) register accessor: an alias for `Reg<PORTD_DIN_SPEC>`"]
pub type PORTD_DIN = crate::Reg<portd_din::PORTD_DIN_SPEC>;
#[doc = "data in"]
pub mod portd_din;
#[doc = "LOCK (w) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
#[doc = "GPIOLOCKSTATUS (r) register accessor: an alias for `Reg<GPIOLOCKSTATUS_SPEC>`"]
pub type GPIOLOCKSTATUS = crate::Reg<gpiolockstatus::GPIOLOCKSTATUS_SPEC>;
#[doc = "No Description"]
pub mod gpiolockstatus;
#[doc = "ABUSALLOC (rw) register accessor: an alias for `Reg<ABUSALLOC_SPEC>`"]
pub type ABUSALLOC = crate::Reg<abusalloc::ABUSALLOC_SPEC>;
#[doc = "A Bus allocation"]
pub mod abusalloc;
#[doc = "BBUSALLOC (rw) register accessor: an alias for `Reg<BBUSALLOC_SPEC>`"]
pub type BBUSALLOC = crate::Reg<bbusalloc::BBUSALLOC_SPEC>;
#[doc = "B Bus allocation"]
pub mod bbusalloc;
#[doc = "CDBUSALLOC (rw) register accessor: an alias for `Reg<CDBUSALLOC_SPEC>`"]
pub type CDBUSALLOC = crate::Reg<cdbusalloc::CDBUSALLOC_SPEC>;
#[doc = "CD Bus allocation"]
pub mod cdbusalloc;
#[doc = "EXTIPSELL (rw) register accessor: an alias for `Reg<EXTIPSELL_SPEC>`"]
pub type EXTIPSELL = crate::Reg<extipsell::EXTIPSELL_SPEC>;
#[doc = "External Interrupt Port Select Low"]
pub mod extipsell;
#[doc = "EXTIPSELH (rw) register accessor: an alias for `Reg<EXTIPSELH_SPEC>`"]
pub type EXTIPSELH = crate::Reg<extipselh::EXTIPSELH_SPEC>;
#[doc = "External interrupt Port Select High"]
pub mod extipselh;
#[doc = "EXTIPINSELL (rw) register accessor: an alias for `Reg<EXTIPINSELL_SPEC>`"]
pub type EXTIPINSELL = crate::Reg<extipinsell::EXTIPINSELL_SPEC>;
#[doc = "External Interrupt Pin Select Low"]
pub mod extipinsell;
#[doc = "EXTIPINSELH (rw) register accessor: an alias for `Reg<EXTIPINSELH_SPEC>`"]
pub type EXTIPINSELH = crate::Reg<extipinselh::EXTIPINSELH_SPEC>;
#[doc = "External Interrupt Pin Select High"]
pub mod extipinselh;
#[doc = "EXTIRISE (rw) register accessor: an alias for `Reg<EXTIRISE_SPEC>`"]
pub type EXTIRISE = crate::Reg<extirise::EXTIRISE_SPEC>;
#[doc = "External Interrupt Rising Edge Trigger"]
pub mod extirise;
#[doc = "EXTIFALL (rw) register accessor: an alias for `Reg<EXTIFALL_SPEC>`"]
pub type EXTIFALL = crate::Reg<extifall::EXTIFALL_SPEC>;
#[doc = "External Interrupt Falling Edge Trigger"]
pub mod extifall;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag"]
pub mod if_;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ien;
#[doc = "EM4WUEN (rw) register accessor: an alias for `Reg<EM4WUEN_SPEC>`"]
pub type EM4WUEN = crate::Reg<em4wuen::EM4WUEN_SPEC>;
#[doc = "No Description"]
pub mod em4wuen;
#[doc = "EM4WUPOL (rw) register accessor: an alias for `Reg<EM4WUPOL_SPEC>`"]
pub type EM4WUPOL = crate::Reg<em4wupol::EM4WUPOL_SPEC>;
#[doc = "No Description"]
pub mod em4wupol;
#[doc = "DBGROUTEPEN (rw) register accessor: an alias for `Reg<DBGROUTEPEN_SPEC>`"]
pub type DBGROUTEPEN = crate::Reg<dbgroutepen::DBGROUTEPEN_SPEC>;
#[doc = "No Description"]
pub mod dbgroutepen;
#[doc = "TRACEROUTEPEN (rw) register accessor: an alias for `Reg<TRACEROUTEPEN_SPEC>`"]
pub type TRACEROUTEPEN = crate::Reg<traceroutepen::TRACEROUTEPEN_SPEC>;
#[doc = "No Description"]
pub mod traceroutepen;
#[doc = "LCDSEG (rw) register accessor: an alias for `Reg<LCDSEG_SPEC>`"]
pub type LCDSEG = crate::Reg<lcdseg::LCDSEG_SPEC>;
#[doc = "LCD Segment Enable"]
pub mod lcdseg;
#[doc = "LCDCOM (rw) register accessor: an alias for `Reg<LCDCOM_SPEC>`"]
pub type LCDCOM = crate::Reg<lcdcom::LCDCOM_SPEC>;
#[doc = "LCD Common Enable"]
pub mod lcdcom;
#[doc = "ACMP0_ROUTEEN (rw) register accessor: an alias for `Reg<ACMP0_ROUTEEN_SPEC>`"]
pub type ACMP0_ROUTEEN = crate::Reg<acmp0_routeen::ACMP0_ROUTEEN_SPEC>;
#[doc = "ACMP0 pin enable"]
pub mod acmp0_routeen;
#[doc = "ACMP0_ACMPOUTROUTE (rw) register accessor: an alias for `Reg<ACMP0_ACMPOUTROUTE_SPEC>`"]
pub type ACMP0_ACMPOUTROUTE = crate::Reg<acmp0_acmpoutroute::ACMP0_ACMPOUTROUTE_SPEC>;
#[doc = "ACMPOUT port/pin select"]
pub mod acmp0_acmpoutroute;
#[doc = "ACMP1_ROUTEEN (rw) register accessor: an alias for `Reg<ACMP1_ROUTEEN_SPEC>`"]
pub type ACMP1_ROUTEEN = crate::Reg<acmp1_routeen::ACMP1_ROUTEEN_SPEC>;
#[doc = "ACMP1 pin enable"]
pub mod acmp1_routeen;
#[doc = "ACMP1_ACMPOUTROUTE (rw) register accessor: an alias for `Reg<ACMP1_ACMPOUTROUTE_SPEC>`"]
pub type ACMP1_ACMPOUTROUTE = crate::Reg<acmp1_acmpoutroute::ACMP1_ACMPOUTROUTE_SPEC>;
#[doc = "ACMPOUT port/pin select"]
pub mod acmp1_acmpoutroute;
#[doc = "CMU_ROUTEEN (rw) register accessor: an alias for `Reg<CMU_ROUTEEN_SPEC>`"]
pub type CMU_ROUTEEN = crate::Reg<cmu_routeen::CMU_ROUTEEN_SPEC>;
#[doc = "CMU pin enable"]
pub mod cmu_routeen;
#[doc = "CMU_CLKIN0ROUTE (rw) register accessor: an alias for `Reg<CMU_CLKIN0ROUTE_SPEC>`"]
pub type CMU_CLKIN0ROUTE = crate::Reg<cmu_clkin0route::CMU_CLKIN0ROUTE_SPEC>;
#[doc = "CLKIN0 port/pin select"]
pub mod cmu_clkin0route;
#[doc = "CMU_CLKOUT0ROUTE (rw) register accessor: an alias for `Reg<CMU_CLKOUT0ROUTE_SPEC>`"]
pub type CMU_CLKOUT0ROUTE = crate::Reg<cmu_clkout0route::CMU_CLKOUT0ROUTE_SPEC>;
#[doc = "CLKOUT0 port/pin select"]
pub mod cmu_clkout0route;
#[doc = "CMU_CLKOUT1ROUTE (rw) register accessor: an alias for `Reg<CMU_CLKOUT1ROUTE_SPEC>`"]
pub type CMU_CLKOUT1ROUTE = crate::Reg<cmu_clkout1route::CMU_CLKOUT1ROUTE_SPEC>;
#[doc = "CLKOUT1 port/pin select"]
pub mod cmu_clkout1route;
#[doc = "CMU_CLKOUT2ROUTE (rw) register accessor: an alias for `Reg<CMU_CLKOUT2ROUTE_SPEC>`"]
pub type CMU_CLKOUT2ROUTE = crate::Reg<cmu_clkout2route::CMU_CLKOUT2ROUTE_SPEC>;
#[doc = "CLKOUT2 port/pin select"]
pub mod cmu_clkout2route;
#[doc = "EUSART0_ROUTEEN (rw) register accessor: an alias for `Reg<EUSART0_ROUTEEN_SPEC>`"]
pub type EUSART0_ROUTEEN = crate::Reg<eusart0_routeen::EUSART0_ROUTEEN_SPEC>;
#[doc = "EUSART0 pin enable"]
pub mod eusart0_routeen;
#[doc = "EUSART0_CSROUTE (rw) register accessor: an alias for `Reg<EUSART0_CSROUTE_SPEC>`"]
pub type EUSART0_CSROUTE = crate::Reg<eusart0_csroute::EUSART0_CSROUTE_SPEC>;
#[doc = "CS port/pin select"]
pub mod eusart0_csroute;
#[doc = "EUSART0_CTSROUTE (rw) register accessor: an alias for `Reg<EUSART0_CTSROUTE_SPEC>`"]
pub type EUSART0_CTSROUTE = crate::Reg<eusart0_ctsroute::EUSART0_CTSROUTE_SPEC>;
#[doc = "CTS port/pin select"]
pub mod eusart0_ctsroute;
#[doc = "EUSART0_RTSROUTE (rw) register accessor: an alias for `Reg<EUSART0_RTSROUTE_SPEC>`"]
pub type EUSART0_RTSROUTE = crate::Reg<eusart0_rtsroute::EUSART0_RTSROUTE_SPEC>;
#[doc = "RTS port/pin select"]
pub mod eusart0_rtsroute;
#[doc = "EUSART0_RXROUTE (rw) register accessor: an alias for `Reg<EUSART0_RXROUTE_SPEC>`"]
pub type EUSART0_RXROUTE = crate::Reg<eusart0_rxroute::EUSART0_RXROUTE_SPEC>;
#[doc = "RX port/pin select"]
pub mod eusart0_rxroute;
#[doc = "EUSART0_SCLKROUTE (rw) register accessor: an alias for `Reg<EUSART0_SCLKROUTE_SPEC>`"]
pub type EUSART0_SCLKROUTE = crate::Reg<eusart0_sclkroute::EUSART0_SCLKROUTE_SPEC>;
#[doc = "SCLK port/pin select"]
pub mod eusart0_sclkroute;
#[doc = "EUSART0_TXROUTE (rw) register accessor: an alias for `Reg<EUSART0_TXROUTE_SPEC>`"]
pub type EUSART0_TXROUTE = crate::Reg<eusart0_txroute::EUSART0_TXROUTE_SPEC>;
#[doc = "TX port/pin select"]
pub mod eusart0_txroute;
#[doc = "EUSART1_ROUTEEN (rw) register accessor: an alias for `Reg<EUSART1_ROUTEEN_SPEC>`"]
pub type EUSART1_ROUTEEN = crate::Reg<eusart1_routeen::EUSART1_ROUTEEN_SPEC>;
#[doc = "EUSART1 pin enable"]
pub mod eusart1_routeen;
#[doc = "EUSART1_CSROUTE (rw) register accessor: an alias for `Reg<EUSART1_CSROUTE_SPEC>`"]
pub type EUSART1_CSROUTE = crate::Reg<eusart1_csroute::EUSART1_CSROUTE_SPEC>;
#[doc = "CS port/pin select"]
pub mod eusart1_csroute;
#[doc = "EUSART1_CTSROUTE (rw) register accessor: an alias for `Reg<EUSART1_CTSROUTE_SPEC>`"]
pub type EUSART1_CTSROUTE = crate::Reg<eusart1_ctsroute::EUSART1_CTSROUTE_SPEC>;
#[doc = "CTS port/pin select"]
pub mod eusart1_ctsroute;
#[doc = "EUSART1_RTSROUTE (rw) register accessor: an alias for `Reg<EUSART1_RTSROUTE_SPEC>`"]
pub type EUSART1_RTSROUTE = crate::Reg<eusart1_rtsroute::EUSART1_RTSROUTE_SPEC>;
#[doc = "RTS port/pin select"]
pub mod eusart1_rtsroute;
#[doc = "EUSART1_RXROUTE (rw) register accessor: an alias for `Reg<EUSART1_RXROUTE_SPEC>`"]
pub type EUSART1_RXROUTE = crate::Reg<eusart1_rxroute::EUSART1_RXROUTE_SPEC>;
#[doc = "RX port/pin select"]
pub mod eusart1_rxroute;
#[doc = "EUSART1_SCLKROUTE (rw) register accessor: an alias for `Reg<EUSART1_SCLKROUTE_SPEC>`"]
pub type EUSART1_SCLKROUTE = crate::Reg<eusart1_sclkroute::EUSART1_SCLKROUTE_SPEC>;
#[doc = "SCLK port/pin select"]
pub mod eusart1_sclkroute;
#[doc = "EUSART1_TXROUTE (rw) register accessor: an alias for `Reg<EUSART1_TXROUTE_SPEC>`"]
pub type EUSART1_TXROUTE = crate::Reg<eusart1_txroute::EUSART1_TXROUTE_SPEC>;
#[doc = "TX port/pin select"]
pub mod eusart1_txroute;
#[doc = "EUSART2_ROUTEEN (rw) register accessor: an alias for `Reg<EUSART2_ROUTEEN_SPEC>`"]
pub type EUSART2_ROUTEEN = crate::Reg<eusart2_routeen::EUSART2_ROUTEEN_SPEC>;
#[doc = "EUSART2 pin enable"]
pub mod eusart2_routeen;
#[doc = "EUSART2_CSROUTE (rw) register accessor: an alias for `Reg<EUSART2_CSROUTE_SPEC>`"]
pub type EUSART2_CSROUTE = crate::Reg<eusart2_csroute::EUSART2_CSROUTE_SPEC>;
#[doc = "CS port/pin select"]
pub mod eusart2_csroute;
#[doc = "EUSART2_CTSROUTE (rw) register accessor: an alias for `Reg<EUSART2_CTSROUTE_SPEC>`"]
pub type EUSART2_CTSROUTE = crate::Reg<eusart2_ctsroute::EUSART2_CTSROUTE_SPEC>;
#[doc = "CTS port/pin select"]
pub mod eusart2_ctsroute;
#[doc = "EUSART2_RTSROUTE (rw) register accessor: an alias for `Reg<EUSART2_RTSROUTE_SPEC>`"]
pub type EUSART2_RTSROUTE = crate::Reg<eusart2_rtsroute::EUSART2_RTSROUTE_SPEC>;
#[doc = "RTS port/pin select"]
pub mod eusart2_rtsroute;
#[doc = "EUSART2_RXROUTE (rw) register accessor: an alias for `Reg<EUSART2_RXROUTE_SPEC>`"]
pub type EUSART2_RXROUTE = crate::Reg<eusart2_rxroute::EUSART2_RXROUTE_SPEC>;
#[doc = "RX port/pin select"]
pub mod eusart2_rxroute;
#[doc = "EUSART2_SCLKROUTE (rw) register accessor: an alias for `Reg<EUSART2_SCLKROUTE_SPEC>`"]
pub type EUSART2_SCLKROUTE = crate::Reg<eusart2_sclkroute::EUSART2_SCLKROUTE_SPEC>;
#[doc = "SCLK port/pin select"]
pub mod eusart2_sclkroute;
#[doc = "EUSART2_TXROUTE (rw) register accessor: an alias for `Reg<EUSART2_TXROUTE_SPEC>`"]
pub type EUSART2_TXROUTE = crate::Reg<eusart2_txroute::EUSART2_TXROUTE_SPEC>;
#[doc = "TX port/pin select"]
pub mod eusart2_txroute;
#[doc = "I2C0_ROUTEEN (rw) register accessor: an alias for `Reg<I2C0_ROUTEEN_SPEC>`"]
pub type I2C0_ROUTEEN = crate::Reg<i2c0_routeen::I2C0_ROUTEEN_SPEC>;
#[doc = "I2C0 pin enable"]
pub mod i2c0_routeen;
#[doc = "I2C0_SCLROUTE (rw) register accessor: an alias for `Reg<I2C0_SCLROUTE_SPEC>`"]
pub type I2C0_SCLROUTE = crate::Reg<i2c0_sclroute::I2C0_SCLROUTE_SPEC>;
#[doc = "SCL port/pin select"]
pub mod i2c0_sclroute;
#[doc = "I2C0_SDAROUTE (rw) register accessor: an alias for `Reg<I2C0_SDAROUTE_SPEC>`"]
pub type I2C0_SDAROUTE = crate::Reg<i2c0_sdaroute::I2C0_SDAROUTE_SPEC>;
#[doc = "SDA port/pin select"]
pub mod i2c0_sdaroute;
#[doc = "I2C1_ROUTEEN (rw) register accessor: an alias for `Reg<I2C1_ROUTEEN_SPEC>`"]
pub type I2C1_ROUTEEN = crate::Reg<i2c1_routeen::I2C1_ROUTEEN_SPEC>;
#[doc = "I2C1 pin enable"]
pub mod i2c1_routeen;
#[doc = "I2C1_SCLROUTE (rw) register accessor: an alias for `Reg<I2C1_SCLROUTE_SPEC>`"]
pub type I2C1_SCLROUTE = crate::Reg<i2c1_sclroute::I2C1_SCLROUTE_SPEC>;
#[doc = "SCL port/pin select"]
pub mod i2c1_sclroute;
#[doc = "I2C1_SDAROUTE (rw) register accessor: an alias for `Reg<I2C1_SDAROUTE_SPEC>`"]
pub type I2C1_SDAROUTE = crate::Reg<i2c1_sdaroute::I2C1_SDAROUTE_SPEC>;
#[doc = "SDA port/pin select"]
pub mod i2c1_sdaroute;
#[doc = "KEYSCAN_ROUTEEN (rw) register accessor: an alias for `Reg<KEYSCAN_ROUTEEN_SPEC>`"]
pub type KEYSCAN_ROUTEEN = crate::Reg<keyscan_routeen::KEYSCAN_ROUTEEN_SPEC>;
#[doc = "KEYSCAN pin enable"]
pub mod keyscan_routeen;
#[doc = "KEYSCAN_COLOUT0ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_COLOUT0ROUTE_SPEC>`"]
pub type KEYSCAN_COLOUT0ROUTE = crate::Reg<keyscan_colout0route::KEYSCAN_COLOUT0ROUTE_SPEC>;
#[doc = "COLOUT0 port/pin select"]
pub mod keyscan_colout0route;
#[doc = "KEYSCAN_COLOUT1ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_COLOUT1ROUTE_SPEC>`"]
pub type KEYSCAN_COLOUT1ROUTE = crate::Reg<keyscan_colout1route::KEYSCAN_COLOUT1ROUTE_SPEC>;
#[doc = "COLOUT1 port/pin select"]
pub mod keyscan_colout1route;
#[doc = "KEYSCAN_COLOUT2ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_COLOUT2ROUTE_SPEC>`"]
pub type KEYSCAN_COLOUT2ROUTE = crate::Reg<keyscan_colout2route::KEYSCAN_COLOUT2ROUTE_SPEC>;
#[doc = "COLOUT2 port/pin select"]
pub mod keyscan_colout2route;
#[doc = "KEYSCAN_COLOUT3ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_COLOUT3ROUTE_SPEC>`"]
pub type KEYSCAN_COLOUT3ROUTE = crate::Reg<keyscan_colout3route::KEYSCAN_COLOUT3ROUTE_SPEC>;
#[doc = "COLOUT3 port/pin select"]
pub mod keyscan_colout3route;
#[doc = "KEYSCAN_COLOUT4ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_COLOUT4ROUTE_SPEC>`"]
pub type KEYSCAN_COLOUT4ROUTE = crate::Reg<keyscan_colout4route::KEYSCAN_COLOUT4ROUTE_SPEC>;
#[doc = "COLOUT4 port/pin select"]
pub mod keyscan_colout4route;
#[doc = "KEYSCAN_COLOUT5ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_COLOUT5ROUTE_SPEC>`"]
pub type KEYSCAN_COLOUT5ROUTE = crate::Reg<keyscan_colout5route::KEYSCAN_COLOUT5ROUTE_SPEC>;
#[doc = "COLOUT5 port/pin select"]
pub mod keyscan_colout5route;
#[doc = "KEYSCAN_COLOUT6ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_COLOUT6ROUTE_SPEC>`"]
pub type KEYSCAN_COLOUT6ROUTE = crate::Reg<keyscan_colout6route::KEYSCAN_COLOUT6ROUTE_SPEC>;
#[doc = "COLOUT6 port/pin select"]
pub mod keyscan_colout6route;
#[doc = "KEYSCAN_COLOUT7ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_COLOUT7ROUTE_SPEC>`"]
pub type KEYSCAN_COLOUT7ROUTE = crate::Reg<keyscan_colout7route::KEYSCAN_COLOUT7ROUTE_SPEC>;
#[doc = "COLOUT7 port/pin select"]
pub mod keyscan_colout7route;
#[doc = "KEYSCAN_ROWSENSE0ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_ROWSENSE0ROUTE_SPEC>`"]
pub type KEYSCAN_ROWSENSE0ROUTE = crate::Reg<keyscan_rowsense0route::KEYSCAN_ROWSENSE0ROUTE_SPEC>;
#[doc = "ROWSENSE0 port/pin select"]
pub mod keyscan_rowsense0route;
#[doc = "KEYSCAN_ROWSENSE1ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_ROWSENSE1ROUTE_SPEC>`"]
pub type KEYSCAN_ROWSENSE1ROUTE = crate::Reg<keyscan_rowsense1route::KEYSCAN_ROWSENSE1ROUTE_SPEC>;
#[doc = "ROWSENSE1 port/pin select"]
pub mod keyscan_rowsense1route;
#[doc = "KEYSCAN_ROWSENSE2ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_ROWSENSE2ROUTE_SPEC>`"]
pub type KEYSCAN_ROWSENSE2ROUTE = crate::Reg<keyscan_rowsense2route::KEYSCAN_ROWSENSE2ROUTE_SPEC>;
#[doc = "ROWSENSE2 port/pin select"]
pub mod keyscan_rowsense2route;
#[doc = "KEYSCAN_ROWSENSE3ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_ROWSENSE3ROUTE_SPEC>`"]
pub type KEYSCAN_ROWSENSE3ROUTE = crate::Reg<keyscan_rowsense3route::KEYSCAN_ROWSENSE3ROUTE_SPEC>;
#[doc = "ROWSENSE3 port/pin select"]
pub mod keyscan_rowsense3route;
#[doc = "KEYSCAN_ROWSENSE4ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_ROWSENSE4ROUTE_SPEC>`"]
pub type KEYSCAN_ROWSENSE4ROUTE = crate::Reg<keyscan_rowsense4route::KEYSCAN_ROWSENSE4ROUTE_SPEC>;
#[doc = "ROWSENSE4 port/pin select"]
pub mod keyscan_rowsense4route;
#[doc = "KEYSCAN_ROWSENSE5ROUTE (rw) register accessor: an alias for `Reg<KEYSCAN_ROWSENSE5ROUTE_SPEC>`"]
pub type KEYSCAN_ROWSENSE5ROUTE = crate::Reg<keyscan_rowsense5route::KEYSCAN_ROWSENSE5ROUTE_SPEC>;
#[doc = "ROWSENSE5 port/pin select"]
pub mod keyscan_rowsense5route;
#[doc = "LESENSE_ROUTEEN (rw) register accessor: an alias for `Reg<LESENSE_ROUTEEN_SPEC>`"]
pub type LESENSE_ROUTEEN = crate::Reg<lesense_routeen::LESENSE_ROUTEEN_SPEC>;
#[doc = "LESENSE pin enable"]
pub mod lesense_routeen;
#[doc = "LESENSE_CH0OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH0OUTROUTE_SPEC>`"]
pub type LESENSE_CH0OUTROUTE = crate::Reg<lesense_ch0outroute::LESENSE_CH0OUTROUTE_SPEC>;
#[doc = "CH0OUT port/pin select"]
pub mod lesense_ch0outroute;
#[doc = "LESENSE_CH1OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH1OUTROUTE_SPEC>`"]
pub type LESENSE_CH1OUTROUTE = crate::Reg<lesense_ch1outroute::LESENSE_CH1OUTROUTE_SPEC>;
#[doc = "CH1OUT port/pin select"]
pub mod lesense_ch1outroute;
#[doc = "LESENSE_CH2OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH2OUTROUTE_SPEC>`"]
pub type LESENSE_CH2OUTROUTE = crate::Reg<lesense_ch2outroute::LESENSE_CH2OUTROUTE_SPEC>;
#[doc = "CH2OUT port/pin select"]
pub mod lesense_ch2outroute;
#[doc = "LESENSE_CH3OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH3OUTROUTE_SPEC>`"]
pub type LESENSE_CH3OUTROUTE = crate::Reg<lesense_ch3outroute::LESENSE_CH3OUTROUTE_SPEC>;
#[doc = "CH3OUT port/pin select"]
pub mod lesense_ch3outroute;
#[doc = "LESENSE_CH4OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH4OUTROUTE_SPEC>`"]
pub type LESENSE_CH4OUTROUTE = crate::Reg<lesense_ch4outroute::LESENSE_CH4OUTROUTE_SPEC>;
#[doc = "CH4OUT port/pin select"]
pub mod lesense_ch4outroute;
#[doc = "LESENSE_CH5OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH5OUTROUTE_SPEC>`"]
pub type LESENSE_CH5OUTROUTE = crate::Reg<lesense_ch5outroute::LESENSE_CH5OUTROUTE_SPEC>;
#[doc = "CH5OUT port/pin select"]
pub mod lesense_ch5outroute;
#[doc = "LESENSE_CH6OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH6OUTROUTE_SPEC>`"]
pub type LESENSE_CH6OUTROUTE = crate::Reg<lesense_ch6outroute::LESENSE_CH6OUTROUTE_SPEC>;
#[doc = "CH6OUT port/pin select"]
pub mod lesense_ch6outroute;
#[doc = "LESENSE_CH7OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH7OUTROUTE_SPEC>`"]
pub type LESENSE_CH7OUTROUTE = crate::Reg<lesense_ch7outroute::LESENSE_CH7OUTROUTE_SPEC>;
#[doc = "CH7OUT port/pin select"]
pub mod lesense_ch7outroute;
#[doc = "LESENSE_CH8OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH8OUTROUTE_SPEC>`"]
pub type LESENSE_CH8OUTROUTE = crate::Reg<lesense_ch8outroute::LESENSE_CH8OUTROUTE_SPEC>;
#[doc = "CH8OUT port/pin select"]
pub mod lesense_ch8outroute;
#[doc = "LESENSE_CH9OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH9OUTROUTE_SPEC>`"]
pub type LESENSE_CH9OUTROUTE = crate::Reg<lesense_ch9outroute::LESENSE_CH9OUTROUTE_SPEC>;
#[doc = "CH9OUT port/pin select"]
pub mod lesense_ch9outroute;
#[doc = "LESENSE_CH10OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH10OUTROUTE_SPEC>`"]
pub type LESENSE_CH10OUTROUTE = crate::Reg<lesense_ch10outroute::LESENSE_CH10OUTROUTE_SPEC>;
#[doc = "CH10OUT port/pin select"]
pub mod lesense_ch10outroute;
#[doc = "LESENSE_CH11OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH11OUTROUTE_SPEC>`"]
pub type LESENSE_CH11OUTROUTE = crate::Reg<lesense_ch11outroute::LESENSE_CH11OUTROUTE_SPEC>;
#[doc = "CH11OUT port/pin select"]
pub mod lesense_ch11outroute;
#[doc = "LESENSE_CH12OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH12OUTROUTE_SPEC>`"]
pub type LESENSE_CH12OUTROUTE = crate::Reg<lesense_ch12outroute::LESENSE_CH12OUTROUTE_SPEC>;
#[doc = "CH12OUT port/pin select"]
pub mod lesense_ch12outroute;
#[doc = "LESENSE_CH13OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH13OUTROUTE_SPEC>`"]
pub type LESENSE_CH13OUTROUTE = crate::Reg<lesense_ch13outroute::LESENSE_CH13OUTROUTE_SPEC>;
#[doc = "CH13OUT port/pin select"]
pub mod lesense_ch13outroute;
#[doc = "LESENSE_CH14OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH14OUTROUTE_SPEC>`"]
pub type LESENSE_CH14OUTROUTE = crate::Reg<lesense_ch14outroute::LESENSE_CH14OUTROUTE_SPEC>;
#[doc = "CH14OUT port/pin select"]
pub mod lesense_ch14outroute;
#[doc = "LESENSE_CH15OUTROUTE (rw) register accessor: an alias for `Reg<LESENSE_CH15OUTROUTE_SPEC>`"]
pub type LESENSE_CH15OUTROUTE = crate::Reg<lesense_ch15outroute::LESENSE_CH15OUTROUTE_SPEC>;
#[doc = "CH15OUT port/pin select"]
pub mod lesense_ch15outroute;
#[doc = "LETIMER_ROUTEEN (rw) register accessor: an alias for `Reg<LETIMER_ROUTEEN_SPEC>`"]
pub type LETIMER_ROUTEEN = crate::Reg<letimer_routeen::LETIMER_ROUTEEN_SPEC>;
#[doc = "LETIMER pin enable"]
pub mod letimer_routeen;
#[doc = "LETIMER_OUT0ROUTE (rw) register accessor: an alias for `Reg<LETIMER_OUT0ROUTE_SPEC>`"]
pub type LETIMER_OUT0ROUTE = crate::Reg<letimer_out0route::LETIMER_OUT0ROUTE_SPEC>;
#[doc = "OUT0 port/pin select"]
pub mod letimer_out0route;
#[doc = "LETIMER_OUT1ROUTE (rw) register accessor: an alias for `Reg<LETIMER_OUT1ROUTE_SPEC>`"]
pub type LETIMER_OUT1ROUTE = crate::Reg<letimer_out1route::LETIMER_OUT1ROUTE_SPEC>;
#[doc = "OUT1 port/pin select"]
pub mod letimer_out1route;
#[doc = "PCNT0_S0INROUTE (rw) register accessor: an alias for `Reg<PCNT0_S0INROUTE_SPEC>`"]
pub type PCNT0_S0INROUTE = crate::Reg<pcnt0_s0inroute::PCNT0_S0INROUTE_SPEC>;
#[doc = "S0IN port/pin select"]
pub mod pcnt0_s0inroute;
#[doc = "PCNT0_S1INROUTE (rw) register accessor: an alias for `Reg<PCNT0_S1INROUTE_SPEC>`"]
pub type PCNT0_S1INROUTE = crate::Reg<pcnt0_s1inroute::PCNT0_S1INROUTE_SPEC>;
#[doc = "S1IN port/pin select"]
pub mod pcnt0_s1inroute;
#[doc = "PRS0_ROUTEEN (rw) register accessor: an alias for `Reg<PRS0_ROUTEEN_SPEC>`"]
pub type PRS0_ROUTEEN = crate::Reg<prs0_routeen::PRS0_ROUTEEN_SPEC>;
#[doc = "PRS0 pin enable"]
pub mod prs0_routeen;
#[doc = "PRS0_ASYNCH0ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH0ROUTE_SPEC>`"]
pub type PRS0_ASYNCH0ROUTE = crate::Reg<prs0_asynch0route::PRS0_ASYNCH0ROUTE_SPEC>;
#[doc = "ASYNCH0 port/pin select"]
pub mod prs0_asynch0route;
#[doc = "PRS0_ASYNCH1ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH1ROUTE_SPEC>`"]
pub type PRS0_ASYNCH1ROUTE = crate::Reg<prs0_asynch1route::PRS0_ASYNCH1ROUTE_SPEC>;
#[doc = "ASYNCH1 port/pin select"]
pub mod prs0_asynch1route;
#[doc = "PRS0_ASYNCH2ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH2ROUTE_SPEC>`"]
pub type PRS0_ASYNCH2ROUTE = crate::Reg<prs0_asynch2route::PRS0_ASYNCH2ROUTE_SPEC>;
#[doc = "ASYNCH2 port/pin select"]
pub mod prs0_asynch2route;
#[doc = "PRS0_ASYNCH3ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH3ROUTE_SPEC>`"]
pub type PRS0_ASYNCH3ROUTE = crate::Reg<prs0_asynch3route::PRS0_ASYNCH3ROUTE_SPEC>;
#[doc = "ASYNCH3 port/pin select"]
pub mod prs0_asynch3route;
#[doc = "PRS0_ASYNCH4ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH4ROUTE_SPEC>`"]
pub type PRS0_ASYNCH4ROUTE = crate::Reg<prs0_asynch4route::PRS0_ASYNCH4ROUTE_SPEC>;
#[doc = "ASYNCH4 port/pin select"]
pub mod prs0_asynch4route;
#[doc = "PRS0_ASYNCH5ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH5ROUTE_SPEC>`"]
pub type PRS0_ASYNCH5ROUTE = crate::Reg<prs0_asynch5route::PRS0_ASYNCH5ROUTE_SPEC>;
#[doc = "ASYNCH5 port/pin select"]
pub mod prs0_asynch5route;
#[doc = "PRS0_ASYNCH6ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH6ROUTE_SPEC>`"]
pub type PRS0_ASYNCH6ROUTE = crate::Reg<prs0_asynch6route::PRS0_ASYNCH6ROUTE_SPEC>;
#[doc = "ASYNCH6 port/pin select"]
pub mod prs0_asynch6route;
#[doc = "PRS0_ASYNCH7ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH7ROUTE_SPEC>`"]
pub type PRS0_ASYNCH7ROUTE = crate::Reg<prs0_asynch7route::PRS0_ASYNCH7ROUTE_SPEC>;
#[doc = "ASYNCH7 port/pin select"]
pub mod prs0_asynch7route;
#[doc = "PRS0_ASYNCH8ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH8ROUTE_SPEC>`"]
pub type PRS0_ASYNCH8ROUTE = crate::Reg<prs0_asynch8route::PRS0_ASYNCH8ROUTE_SPEC>;
#[doc = "ASYNCH8 port/pin select"]
pub mod prs0_asynch8route;
#[doc = "PRS0_ASYNCH9ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH9ROUTE_SPEC>`"]
pub type PRS0_ASYNCH9ROUTE = crate::Reg<prs0_asynch9route::PRS0_ASYNCH9ROUTE_SPEC>;
#[doc = "ASYNCH9 port/pin select"]
pub mod prs0_asynch9route;
#[doc = "PRS0_ASYNCH10ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH10ROUTE_SPEC>`"]
pub type PRS0_ASYNCH10ROUTE = crate::Reg<prs0_asynch10route::PRS0_ASYNCH10ROUTE_SPEC>;
#[doc = "ASYNCH10 port/pin select"]
pub mod prs0_asynch10route;
#[doc = "PRS0_ASYNCH11ROUTE (rw) register accessor: an alias for `Reg<PRS0_ASYNCH11ROUTE_SPEC>`"]
pub type PRS0_ASYNCH11ROUTE = crate::Reg<prs0_asynch11route::PRS0_ASYNCH11ROUTE_SPEC>;
#[doc = "ASYNCH11 port/pin select"]
pub mod prs0_asynch11route;
#[doc = "PRS0_SYNCH0ROUTE (rw) register accessor: an alias for `Reg<PRS0_SYNCH0ROUTE_SPEC>`"]
pub type PRS0_SYNCH0ROUTE = crate::Reg<prs0_synch0route::PRS0_SYNCH0ROUTE_SPEC>;
#[doc = "SYNCH0 port/pin select"]
pub mod prs0_synch0route;
#[doc = "PRS0_SYNCH1ROUTE (rw) register accessor: an alias for `Reg<PRS0_SYNCH1ROUTE_SPEC>`"]
pub type PRS0_SYNCH1ROUTE = crate::Reg<prs0_synch1route::PRS0_SYNCH1ROUTE_SPEC>;
#[doc = "SYNCH1 port/pin select"]
pub mod prs0_synch1route;
#[doc = "PRS0_SYNCH2ROUTE (rw) register accessor: an alias for `Reg<PRS0_SYNCH2ROUTE_SPEC>`"]
pub type PRS0_SYNCH2ROUTE = crate::Reg<prs0_synch2route::PRS0_SYNCH2ROUTE_SPEC>;
#[doc = "SYNCH2 port/pin select"]
pub mod prs0_synch2route;
#[doc = "PRS0_SYNCH3ROUTE (rw) register accessor: an alias for `Reg<PRS0_SYNCH3ROUTE_SPEC>`"]
pub type PRS0_SYNCH3ROUTE = crate::Reg<prs0_synch3route::PRS0_SYNCH3ROUTE_SPEC>;
#[doc = "SYNCH3 port/pin select"]
pub mod prs0_synch3route;
#[doc = "SYXO0_BUFOUTREQINASYNCROUTE (rw) register accessor: an alias for `Reg<SYXO0_BUFOUTREQINASYNCROUTE_SPEC>`"]
pub type SYXO0_BUFOUTREQINASYNCROUTE =
    crate::Reg<syxo0_bufoutreqinasyncroute::SYXO0_BUFOUTREQINASYNCROUTE_SPEC>;
#[doc = "BUFOUTREQINASYNC port/pin select"]
pub mod syxo0_bufoutreqinasyncroute;
#[doc = "TIMER0_ROUTEEN (rw) register accessor: an alias for `Reg<TIMER0_ROUTEEN_SPEC>`"]
pub type TIMER0_ROUTEEN = crate::Reg<timer0_routeen::TIMER0_ROUTEEN_SPEC>;
#[doc = "TIMER0 pin enable"]
pub mod timer0_routeen;
#[doc = "TIMER0_CC0ROUTE (rw) register accessor: an alias for `Reg<TIMER0_CC0ROUTE_SPEC>`"]
pub type TIMER0_CC0ROUTE = crate::Reg<timer0_cc0route::TIMER0_CC0ROUTE_SPEC>;
#[doc = "CC0 port/pin select"]
pub mod timer0_cc0route;
#[doc = "TIMER0_CC1ROUTE (rw) register accessor: an alias for `Reg<TIMER0_CC1ROUTE_SPEC>`"]
pub type TIMER0_CC1ROUTE = crate::Reg<timer0_cc1route::TIMER0_CC1ROUTE_SPEC>;
#[doc = "CC1 port/pin select"]
pub mod timer0_cc1route;
#[doc = "TIMER0_CC2ROUTE (rw) register accessor: an alias for `Reg<TIMER0_CC2ROUTE_SPEC>`"]
pub type TIMER0_CC2ROUTE = crate::Reg<timer0_cc2route::TIMER0_CC2ROUTE_SPEC>;
#[doc = "CC2 port/pin select"]
pub mod timer0_cc2route;
#[doc = "TIMER0_CDTI0ROUTE (rw) register accessor: an alias for `Reg<TIMER0_CDTI0ROUTE_SPEC>`"]
pub type TIMER0_CDTI0ROUTE = crate::Reg<timer0_cdti0route::TIMER0_CDTI0ROUTE_SPEC>;
#[doc = "CDTI0 port/pin select"]
pub mod timer0_cdti0route;
#[doc = "TIMER0_CDTI1ROUTE (rw) register accessor: an alias for `Reg<TIMER0_CDTI1ROUTE_SPEC>`"]
pub type TIMER0_CDTI1ROUTE = crate::Reg<timer0_cdti1route::TIMER0_CDTI1ROUTE_SPEC>;
#[doc = "CDTI1 port/pin select"]
pub mod timer0_cdti1route;
#[doc = "TIMER0_CDTI2ROUTE (rw) register accessor: an alias for `Reg<TIMER0_CDTI2ROUTE_SPEC>`"]
pub type TIMER0_CDTI2ROUTE = crate::Reg<timer0_cdti2route::TIMER0_CDTI2ROUTE_SPEC>;
#[doc = "CDTI2 port/pin select"]
pub mod timer0_cdti2route;
#[doc = "TIMER1_ROUTEEN (rw) register accessor: an alias for `Reg<TIMER1_ROUTEEN_SPEC>`"]
pub type TIMER1_ROUTEEN = crate::Reg<timer1_routeen::TIMER1_ROUTEEN_SPEC>;
#[doc = "TIMER1 pin enable"]
pub mod timer1_routeen;
#[doc = "TIMER1_CC0ROUTE (rw) register accessor: an alias for `Reg<TIMER1_CC0ROUTE_SPEC>`"]
pub type TIMER1_CC0ROUTE = crate::Reg<timer1_cc0route::TIMER1_CC0ROUTE_SPEC>;
#[doc = "CC0 port/pin select"]
pub mod timer1_cc0route;
#[doc = "TIMER1_CC1ROUTE (rw) register accessor: an alias for `Reg<TIMER1_CC1ROUTE_SPEC>`"]
pub type TIMER1_CC1ROUTE = crate::Reg<timer1_cc1route::TIMER1_CC1ROUTE_SPEC>;
#[doc = "CC1 port/pin select"]
pub mod timer1_cc1route;
#[doc = "TIMER1_CC2ROUTE (rw) register accessor: an alias for `Reg<TIMER1_CC2ROUTE_SPEC>`"]
pub type TIMER1_CC2ROUTE = crate::Reg<timer1_cc2route::TIMER1_CC2ROUTE_SPEC>;
#[doc = "CC2 port/pin select"]
pub mod timer1_cc2route;
#[doc = "TIMER1_CDTI0ROUTE (rw) register accessor: an alias for `Reg<TIMER1_CDTI0ROUTE_SPEC>`"]
pub type TIMER1_CDTI0ROUTE = crate::Reg<timer1_cdti0route::TIMER1_CDTI0ROUTE_SPEC>;
#[doc = "CDTI0 port/pin select"]
pub mod timer1_cdti0route;
#[doc = "TIMER1_CDTI1ROUTE (rw) register accessor: an alias for `Reg<TIMER1_CDTI1ROUTE_SPEC>`"]
pub type TIMER1_CDTI1ROUTE = crate::Reg<timer1_cdti1route::TIMER1_CDTI1ROUTE_SPEC>;
#[doc = "CDTI1 port/pin select"]
pub mod timer1_cdti1route;
#[doc = "TIMER1_CDTI2ROUTE (rw) register accessor: an alias for `Reg<TIMER1_CDTI2ROUTE_SPEC>`"]
pub type TIMER1_CDTI2ROUTE = crate::Reg<timer1_cdti2route::TIMER1_CDTI2ROUTE_SPEC>;
#[doc = "CDTI2 port/pin select"]
pub mod timer1_cdti2route;
#[doc = "TIMER2_ROUTEEN (rw) register accessor: an alias for `Reg<TIMER2_ROUTEEN_SPEC>`"]
pub type TIMER2_ROUTEEN = crate::Reg<timer2_routeen::TIMER2_ROUTEEN_SPEC>;
#[doc = "TIMER2 pin enable"]
pub mod timer2_routeen;
#[doc = "TIMER2_CC0ROUTE (rw) register accessor: an alias for `Reg<TIMER2_CC0ROUTE_SPEC>`"]
pub type TIMER2_CC0ROUTE = crate::Reg<timer2_cc0route::TIMER2_CC0ROUTE_SPEC>;
#[doc = "CC0 port/pin select"]
pub mod timer2_cc0route;
#[doc = "TIMER2_CC1ROUTE (rw) register accessor: an alias for `Reg<TIMER2_CC1ROUTE_SPEC>`"]
pub type TIMER2_CC1ROUTE = crate::Reg<timer2_cc1route::TIMER2_CC1ROUTE_SPEC>;
#[doc = "CC1 port/pin select"]
pub mod timer2_cc1route;
#[doc = "TIMER2_CC2ROUTE (rw) register accessor: an alias for `Reg<TIMER2_CC2ROUTE_SPEC>`"]
pub type TIMER2_CC2ROUTE = crate::Reg<timer2_cc2route::TIMER2_CC2ROUTE_SPEC>;
#[doc = "CC2 port/pin select"]
pub mod timer2_cc2route;
#[doc = "TIMER2_CDTI0ROUTE (rw) register accessor: an alias for `Reg<TIMER2_CDTI0ROUTE_SPEC>`"]
pub type TIMER2_CDTI0ROUTE = crate::Reg<timer2_cdti0route::TIMER2_CDTI0ROUTE_SPEC>;
#[doc = "CDTI0 port/pin select"]
pub mod timer2_cdti0route;
#[doc = "TIMER2_CDTI1ROUTE (rw) register accessor: an alias for `Reg<TIMER2_CDTI1ROUTE_SPEC>`"]
pub type TIMER2_CDTI1ROUTE = crate::Reg<timer2_cdti1route::TIMER2_CDTI1ROUTE_SPEC>;
#[doc = "CDTI1 port/pin select"]
pub mod timer2_cdti1route;
#[doc = "TIMER2_CDTI2ROUTE (rw) register accessor: an alias for `Reg<TIMER2_CDTI2ROUTE_SPEC>`"]
pub type TIMER2_CDTI2ROUTE = crate::Reg<timer2_cdti2route::TIMER2_CDTI2ROUTE_SPEC>;
#[doc = "CDTI2 port/pin select"]
pub mod timer2_cdti2route;
#[doc = "TIMER3_ROUTEEN (rw) register accessor: an alias for `Reg<TIMER3_ROUTEEN_SPEC>`"]
pub type TIMER3_ROUTEEN = crate::Reg<timer3_routeen::TIMER3_ROUTEEN_SPEC>;
#[doc = "TIMER3 pin enable"]
pub mod timer3_routeen;
#[doc = "TIMER3_CC0ROUTE (rw) register accessor: an alias for `Reg<TIMER3_CC0ROUTE_SPEC>`"]
pub type TIMER3_CC0ROUTE = crate::Reg<timer3_cc0route::TIMER3_CC0ROUTE_SPEC>;
#[doc = "CC0 port/pin select"]
pub mod timer3_cc0route;
#[doc = "TIMER3_CC1ROUTE (rw) register accessor: an alias for `Reg<TIMER3_CC1ROUTE_SPEC>`"]
pub type TIMER3_CC1ROUTE = crate::Reg<timer3_cc1route::TIMER3_CC1ROUTE_SPEC>;
#[doc = "CC1 port/pin select"]
pub mod timer3_cc1route;
#[doc = "TIMER3_CC2ROUTE (rw) register accessor: an alias for `Reg<TIMER3_CC2ROUTE_SPEC>`"]
pub type TIMER3_CC2ROUTE = crate::Reg<timer3_cc2route::TIMER3_CC2ROUTE_SPEC>;
#[doc = "CC2 port/pin select"]
pub mod timer3_cc2route;
#[doc = "TIMER3_CDTI0ROUTE (rw) register accessor: an alias for `Reg<TIMER3_CDTI0ROUTE_SPEC>`"]
pub type TIMER3_CDTI0ROUTE = crate::Reg<timer3_cdti0route::TIMER3_CDTI0ROUTE_SPEC>;
#[doc = "CDTI0 port/pin select"]
pub mod timer3_cdti0route;
#[doc = "TIMER3_CDTI1ROUTE (rw) register accessor: an alias for `Reg<TIMER3_CDTI1ROUTE_SPEC>`"]
pub type TIMER3_CDTI1ROUTE = crate::Reg<timer3_cdti1route::TIMER3_CDTI1ROUTE_SPEC>;
#[doc = "CDTI1 port/pin select"]
pub mod timer3_cdti1route;
#[doc = "TIMER3_CDTI2ROUTE (rw) register accessor: an alias for `Reg<TIMER3_CDTI2ROUTE_SPEC>`"]
pub type TIMER3_CDTI2ROUTE = crate::Reg<timer3_cdti2route::TIMER3_CDTI2ROUTE_SPEC>;
#[doc = "CDTI2 port/pin select"]
pub mod timer3_cdti2route;
#[doc = "TIMER4_ROUTEEN (rw) register accessor: an alias for `Reg<TIMER4_ROUTEEN_SPEC>`"]
pub type TIMER4_ROUTEEN = crate::Reg<timer4_routeen::TIMER4_ROUTEEN_SPEC>;
#[doc = "TIMER4 pin enable"]
pub mod timer4_routeen;
#[doc = "TIMER4_CC0ROUTE (rw) register accessor: an alias for `Reg<TIMER4_CC0ROUTE_SPEC>`"]
pub type TIMER4_CC0ROUTE = crate::Reg<timer4_cc0route::TIMER4_CC0ROUTE_SPEC>;
#[doc = "CC0 port/pin select"]
pub mod timer4_cc0route;
#[doc = "TIMER4_CC1ROUTE (rw) register accessor: an alias for `Reg<TIMER4_CC1ROUTE_SPEC>`"]
pub type TIMER4_CC1ROUTE = crate::Reg<timer4_cc1route::TIMER4_CC1ROUTE_SPEC>;
#[doc = "CC1 port/pin select"]
pub mod timer4_cc1route;
#[doc = "TIMER4_CC2ROUTE (rw) register accessor: an alias for `Reg<TIMER4_CC2ROUTE_SPEC>`"]
pub type TIMER4_CC2ROUTE = crate::Reg<timer4_cc2route::TIMER4_CC2ROUTE_SPEC>;
#[doc = "CC2 port/pin select"]
pub mod timer4_cc2route;
#[doc = "TIMER4_CDTI0ROUTE (rw) register accessor: an alias for `Reg<TIMER4_CDTI0ROUTE_SPEC>`"]
pub type TIMER4_CDTI0ROUTE = crate::Reg<timer4_cdti0route::TIMER4_CDTI0ROUTE_SPEC>;
#[doc = "CDTI0 port/pin select"]
pub mod timer4_cdti0route;
#[doc = "TIMER4_CDTI1ROUTE (rw) register accessor: an alias for `Reg<TIMER4_CDTI1ROUTE_SPEC>`"]
pub type TIMER4_CDTI1ROUTE = crate::Reg<timer4_cdti1route::TIMER4_CDTI1ROUTE_SPEC>;
#[doc = "CDTI1 port/pin select"]
pub mod timer4_cdti1route;
#[doc = "TIMER4_CDTI2ROUTE (rw) register accessor: an alias for `Reg<TIMER4_CDTI2ROUTE_SPEC>`"]
pub type TIMER4_CDTI2ROUTE = crate::Reg<timer4_cdti2route::TIMER4_CDTI2ROUTE_SPEC>;
#[doc = "CDTI2 port/pin select"]
pub mod timer4_cdti2route;
#[doc = "USART0_ROUTEEN (rw) register accessor: an alias for `Reg<USART0_ROUTEEN_SPEC>`"]
pub type USART0_ROUTEEN = crate::Reg<usart0_routeen::USART0_ROUTEEN_SPEC>;
#[doc = "USART0 pin enable"]
pub mod usart0_routeen;
#[doc = "USART0_CSROUTE (rw) register accessor: an alias for `Reg<USART0_CSROUTE_SPEC>`"]
pub type USART0_CSROUTE = crate::Reg<usart0_csroute::USART0_CSROUTE_SPEC>;
#[doc = "CS port/pin select"]
pub mod usart0_csroute;
#[doc = "USART0_CTSROUTE (rw) register accessor: an alias for `Reg<USART0_CTSROUTE_SPEC>`"]
pub type USART0_CTSROUTE = crate::Reg<usart0_ctsroute::USART0_CTSROUTE_SPEC>;
#[doc = "CTS port/pin select"]
pub mod usart0_ctsroute;
#[doc = "USART0_RTSROUTE (rw) register accessor: an alias for `Reg<USART0_RTSROUTE_SPEC>`"]
pub type USART0_RTSROUTE = crate::Reg<usart0_rtsroute::USART0_RTSROUTE_SPEC>;
#[doc = "RTS port/pin select"]
pub mod usart0_rtsroute;
#[doc = "USART0_RXROUTE (rw) register accessor: an alias for `Reg<USART0_RXROUTE_SPEC>`"]
pub type USART0_RXROUTE = crate::Reg<usart0_rxroute::USART0_RXROUTE_SPEC>;
#[doc = "RX port/pin select"]
pub mod usart0_rxroute;
#[doc = "USART0_CLKROUTE (rw) register accessor: an alias for `Reg<USART0_CLKROUTE_SPEC>`"]
pub type USART0_CLKROUTE = crate::Reg<usart0_clkroute::USART0_CLKROUTE_SPEC>;
#[doc = "SCLK port/pin select"]
pub mod usart0_clkroute;
#[doc = "USART0_TXROUTE (rw) register accessor: an alias for `Reg<USART0_TXROUTE_SPEC>`"]
pub type USART0_TXROUTE = crate::Reg<usart0_txroute::USART0_TXROUTE_SPEC>;
#[doc = "TX port/pin select"]
pub mod usart0_txroute;
