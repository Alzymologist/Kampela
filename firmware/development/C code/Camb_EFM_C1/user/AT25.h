#pragma once
#include <stdbool.h>
#include <stdio.h>
#include "em_device.h"
#include "em_chip.h"
#include "em_cmu.h"
#include "em_emu.h"
#include "em_gpio.h"
#include "em_usart.h"
#include "em_eusart.h"
#include "utils.h"

#define F_CS_PORT gpioPortC
#define F_CS_PIN 0

#define E_MISO_PORT gpioPortC
#define E_MISO_PIN 1

#define E_MOSI_PORT gpioPortC
#define E_MOSI_PIN 2

#define E_SCK_PORT gpioPortC
#define E_SCK_PIN 3


#define SPI_DISP_FLASH USART0
#define SPI_DISP_FLASH_NO 0

#define SPI_CS_DISP_PORT gpioPortD
#define SPI_CS_DISP_PIN 2

#define FLASH_CS(x) (x ? GPIO_PinOutSet : GPIO_PinOutClear)(F_CS_PORT, F_CS_PIN)
#define DISP_CS(x) (x ? GPIO_PinOutSet : GPIO_PinOutClear)(SPI_CS_DISP_PORT, SPI_CS_DISP_PIN)
#define DISP_DC(x) (x ? GPIO_PinOutSet : GPIO_PinOutClear)(SPI_DC_PORT, SPI_DC_PIN)
#define DISP_RES(x) (x ? GPIO_PinOutSet : GPIO_PinOutClear)(SPI_RES_PORT, SPI_RES_PIN)


#define SIZEN_01M	0x14
#define SIZEN_02M	0x15
#define SIZEN_04M	0x16
#define SIZEN_08M	0x17
#define SIZEN_16M	0x18
#define SIZEN_32M	0x19
#define SIZEN_64M	0x20
#define SIZEN_CAL	0xff
#define SIZE_256B	0x100
#define SIZE_004K	0x1000
#define SIZE_064K	0x10000


/* SPI Flash Configuration Register(SFCR) (0xb800-1200) */
#define SFCR					0xb8001200			/*SPI Flash Configuration Register*/		
#define SFCR_SPI_CLK_DIV(val)	((val) << 29)
#define SFCR_RBO(val)			((val) << 28)
#define SFCR_WBO(val)			((val) << 27)
#define SFCR_SPI_TCS(val)		((val) << 23)			/*4 bit, 1111 */

/* SPI Flash Configuration Register(SFCR2) (0xb800-1204) */
#define SFCR2						0xb8001204
#define SFCR2_SFCMD(val)			((val) << 24)			/*8 bit, 1111_1111 */
#define SFCR2_SFSIZE(val)			((val) << 21)			/*3 bit, 111 */
#define SFCR2_RD_OPT(val)			((val) << 20)
#define SFCR2_CMD_IO(val)			((val) << 18)			/*2 bit, 11 */
#define SFCR2_ADDR_IO(val)			((val) << 16)			/*2 bit, 11 */
#define SFCR2_DUMMY_CYCLE(val)		((val) << 13)			/*3 bit, 111 */
#define SFCR2_DATA_IO(val)			((val) << 11)			/*2 bit, 11 */
#define SFCR2_HOLD_TILL_SFDR2(val)	((val) << 10)

/* SPI Flash Control and Status Register(SFCSR)(0xb800-1208) */
#define SFCSR					0xb8001208
#define SFCSR_SPI_CSB0(val)		((val) << 31)
#define SFCSR_SPI_CSB1(val)		((val) << 30)		
#define SFCSR_LEN(val)			((val) << 28)			/*2 bits*/
#define SFCSR_SPI_RDY(val)		((val) << 27)
#define SFCSR_IO_WIDTH(val)		((val) << 25)			/*2 bits*/
#define SFCSR_CHIP_SEL(val)		((val) << 24)
#define SFCSR_CMD_BYTE(val)		((val) << 16)			/*8 bit, 1111_1111 */

#define SFCSR_SPI_CSB(val)		((val) << 30)

/* SPI Flash Data Register(SFDR)(0xb800-120c) */
#define SFDR					0xb800120c

/* SPI Flash Data Register(SFDR2)(0xb8001210) */
#define SFDR2					0xb8001210

#define SPI_BLOCK_SIZE			0x10000				/* 64KB */
#define SPI_SECTOR_SIZE			0x1000				/* 4KB */
#define SPI_PAGE_SIZE			0x100				/* 256B */



#define SPICMD_WREN			0x06	/* 06 xx xx xx xx sets the (WEL) write enable latch bit */
#define SPICMD_WRDI			0x04	/* 04 xx xx xx xx resets the (WEL) write enable latch bit*/
#define SPICMD_RDID			0x9f	/* 9f xx xx xx xx outputs JEDEC ID: 1 byte manufacturer ID & 2 byte device ID */
#define SPICMD_SFDP			0x5A	/* 5A xx xx xx xx Serial Flash Discoverable Parameters */
#define SPICMD_RDSR			0x05	/* 05 xx xx xx xx to read out the values of the status register */
#define SPICMD_WRSR			0x01	/* 01 xx xx xx xx to write new values to the status register */
#define SPICMD_READ			0x03	/* 03 a1 a2 a3 xx n bytes read out until CS# goes high */
#define SPICMD_FASTREAD		0x0b	/* 0b a1 a2 a3 dd n bytes read out until CS# goes high */
#define SPICMD_2READ		0xbb	/* bb 12 3d xx xx n bytes read out by 2 I/O until CS# goes high */
#define SPICMD_4READ		0xeb	/* eb 3a 3d xx xx n bytes read out by 4 x I/O until CS# goes high */

#define SPICMD_PE			0x81	/* 20 a1 a2 a3 xx to erase the selected page 256 bytes */
#define SPICMD_SE			0x20	/* 20 a1 a2 a3 xx to erase the selected sector */
#define SPICMD_BE			0xd8	/* d8 a1 a2 a3 xx to erase the selected block */
#define SPICMD_CE			0x60	/* 60 xx xx xx xx to erase whole chip (cmd or 0xc7) */

#define SPICMD_PP			0x02	/* 02 a1 a2 a3 xx to program the selected page */

#define SPICMD_4PP			0x38	/* 38 3a 3d xx xx quad input to program the selected page */
#define SPICMD_CP			0xad	/* ad a1 a2 a3 xx continously program whole chip, the address is automaticlly increase */
#define SPICMD_DP			0xb9	/* b9 xx xx xx xx enters deep power down mode */
#define SPICMD_UDPD			0x79	/* 79 Ultra-Deep Power-Down mode */
#define SPICMD_RDP			0xab	/* ab xx xx xx xx release from deep power down mode */
#define SPICMD_RES			0xab	/* ab ?? ?? ?? xx to read out 1 byte device ID */
#define SPICMD_REMS_90		0x90	/* 90 ?? ?? ?? xx output the manufacter ID & device ID */
#define SPICMD_REMS_EF		0xef	/* ef ?? ?? ?? xx output the manufacter ID & device ID */
#define SPICMD_REMS_DF		0xdf	/* df ?? ?? ?? ?? output the manufacture ID & device ID */
#define SPICMD_ENSO			0xb1	/* b1 xx xx xx xx to enter the 512 bit secured OTP mode */
#define SPICMD_EXSO			0xc1	/* c1 xx xx xx xx to exit the 512 bit secured OTP mode */
#define SPICMD_RDSCUR		0x2b	/* 2b xx xx xx xx to read value of secured register */
#define SPICMD_WRSCUR		0x2f	/* 2f xx xx xx xx to set the lock down bit as "1" (once lock down, can not be updated) */
#define SPICMD_ESRY			0x70	/* 70 xx xx xx xx to enable SO to output RY/BY# during CP mode */
#define SPICMD_DSRY			0x80	/* 80 xx xx xx xx to disable SO to output RY/BY# during CP mode */

#define SPI_STATUS_SRP0		0x80	/* Status Register Protect 0 */
#define SPI_STATUS_BPSIZE	0x40	/* Block Protect Size */
#define SPI_STATUS_TB 		0x20	/* Top/Bottom */
#define SPI_STATUS_BP2		0x10	/* level of protected block */
#define SPI_STATUS_BP1		0x08	/* level of protected block */
#define SPI_STATUS_BP0		0x04	/* level of protected block */
#define SPI_STATUS_WEL		0x02	/* write enable latch */
#define SPI_STATUS_BSY		0x01	/* Ready/Busy Status */



void SpiFlashDispInit();
void SpiFlashUDPD();
void SpiFlashExitSleep();
uint32_t SpiFlashGetSize();
uint8_t SpiFlashReadStatus();
void SpiFlashCmd(uint8_t cmd);
void SpiFlashReadPage(uint8_t* data, uint32_t addr);
void SpiFlashWritePage(uint8_t* data, uint32_t addr);
void SpiFlashErasePage(uint32_t addr);



















