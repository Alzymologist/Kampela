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

#define PSRAM_CS_PORT gpioPortC
#define PSRAM_CS_PIN 4

#define PSRAM_MISO_PORT gpioPortC
#define PSRAM_MISO_PIN 5

#define PSRAM_MOSI_PORT gpioPortC
#define PSRAM_MOSI_PIN 6

#define PSRAM_SCK_PORT gpioPortC
#define PSRAM_SCK_PIN 7


#define UART_PSRAM EUSART2
#define UART_PSRAM_NO 2



uint8_t PsramWriteReadByte(uint8_t data);
void PsramSpiWriteArray(uint8_t * Data, size_t Len);
void PsramReadId(uint8_t * array);
void PsramSpiReadArray(uint8_t * Data, size_t Len);
void PsramReset();
void PsramRead(uint32_t Addr, uint8_t * data, size_t len);
void PsramWrite(uint32_t Addr, uint8_t * data, size_t len);
void PsramDmaRead(uint32_t page, uint8_t * data);
void PsramDmaWrite(uint32_t page, uint8_t * data);
void PsramInit();



//PSRAM
struct Psram_t
{
	uint8_t Buf[256];
	bool Found;
	uint32_t Size;
	bool Busy;
};

extern struct Psram_t Psram;











