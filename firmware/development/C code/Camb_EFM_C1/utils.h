#pragma once
#include <stdbool.h>
#include <string.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>

#include "em_usart.h"
#include "em_eusart.h"
#include "em_gpio.h"

#include <em_chip.h>
#include <em_cmu.h>
#include "em_emu.h"
#include <em_iadc.h>
#include "em_se.h"



 
#define		SetBit(reg, x)		reg |= (1<<x)
#define		ClearBit(reg, x)	reg &= (~(1<<x))
#define		InvBit(reg, x)		reg ^= (1<<x)
#define		BitIsSet(reg, x)	((reg & (1<<x)) != 0)
#define		BitIsClear(reg, x)	((reg & (1<<x)) == 0)
#define		bit(x)				(1 << (x))


#define		hibyte(x)			(uint8_t)((x>>8) & 0xFF)
#define		lobyte(x)			(uint8_t)((x) & 0xFF)

#define bitRead(value, bit) (((value) >> (bit)) & 0x01)
#define bitSet(value, bit) ((value) |= (1UL << (bit)))
#define bitClear(value, bit) ((value) &= ~(1UL << (bit)))
#define bitWrite(value, bit, bitvalue) ((bitvalue) ? bitSet(value, bit) : bitClear(value, bit))



  
#define SPI_PERF USART0
#define SPI_PERF_NO 0

#define UART_PERF EUSART1
#define UART_PERF_NO 1

#define UART_TX_PORT gpioPortB
#define UART_TX_PIN 5

#define UART_RX_PORT gpioPortB
#define UART_RX_PIN 6


//display


#define SPI_DC_PORT gpioPortD
#define SPI_DC_PIN 3

#define SPI_RES_PORT gpioPortA
#define SPI_RES_PIN 6

#define SPI_BUSY_PORT gpioPortB
#define SPI_BUSY_PIN 4


//2v8 en
#define POW_EN_PORT gpioPortA
#define POW_EN_PIN 9


//touch panel
#define SDA_PORT gpioPortA
#define SDA_PIN 5

#define SCL_PORT gpioPortA
#define SCL_PIN 3

#define I2CP_PORT gpioPortA
#define I2CP_PIN 4

#define INT_PORT gpioPortB
#define INT_PIN 1



#define POW_2V8(x) (x ? GPIO_PinOutSet : GPIO_PinOutClear)(POW_EN_PORT, POW_EN_PIN)
#define POW_I2C_T(x) (x ? GPIO_PinOutSet : GPIO_PinOutClear)(I2CP_PORT, I2CP_PIN)

#define DISP_BUSY GPIO_PinInGet(SPI_BUSY_PORT, SPI_BUSY_PIN)




	
#define IADC_PRS_CH 0 // Use specified PRS channel

	// Set CLK_ADC to 10MHz (this corresponds to a sample rate of 77K with OSR = 32)
	// CLK_SRC_ADC; largest division is by 4
#define CLK_SRC_ADC_FREQ        20000000

	// CLK_ADC; IADC_SCHEDx PRESCALE has 10 valid bits
#define CLK_ADC_FREQ            10000000

#define IADC_INPUT_0_PORT_PIN     iadcPosInputPortAPin0  // Chip PORT and PIN
#define IADC_INPUT_0_BUS          ABUSALLOC
#define IADC_INPUT_0_BUSALLOC     GPIO_ABUSALLOC_AEVEN0_ADC0 //EVEN PIN

#define SLI_SE_COMMAND_TRNG_GET_RANDOM          0x07000000UL


typedef struct sl_se_command_context_t 
{
	SE_Command_t  command; ///< SE mailbox command struct
} sl_se_command_context_t;

static sl_se_command_context_t cmd_ctx;





//I2C

void swap(uint16_t a, uint16_t b);
void uPrintf(const char *fmt, ...);
 



void Delay(uint32_t delayInTicks);
void ClockInit();
void SpiDispInit();
void UartInit();
bool TxCmpl();
void txDMA(char* str, size_t len);
void CheckTx();
void cPrintf(const char *fmt, ...);
void UartFifoSend();
void AdcInit();
uint32_t EfmGetRandom();



















