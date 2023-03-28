
#include "psram.h"
#include "em_ldma.h"

#include "dmadrv.h" 


#define PSRAM_CS(x) (x ? GPIO_PinOutSet : GPIO_PinOutClear)(PSRAM_CS_PORT, PSRAM_CS_PIN)

Ecode_t res;
	


uint8_t LocalBuf[16];
struct Psram_t Psram;

unsigned int DmaChannelPsram;

bool SPI_CpltCallback()
{
	Psram.Busy = false ;
	return false;
}




void PsramInit()
{
	
	EUSART_SpiInit_TypeDef init_SPI = EUSART_SPI_MASTER_INIT_DEFAULT_HF;
	EUSART_SpiAdvancedInit_TypeDef adv = EUSART_SPI_ADVANCED_INIT_DEFAULT;
	
	CMU_ClockEnable(cmuClock_GPIO, true);
	CMU_ClockEnable(cmuClock_EUSART2, true);

	//init_SPI.bitRate = 2000000;
	init_SPI.advancedSettings = &adv;
	adv.msbFirst = true; // Send msb first.
	init_SPI.clockMode = eusartClockMode0;
	
	
	GPIO_PinModeSet(PSRAM_CS_PORT, PSRAM_CS_PIN, gpioModePushPull, 1); //SPI CS
	GPIO_PinModeSet(PSRAM_MOSI_PORT, PSRAM_MOSI_PIN, gpioModePushPull, 0); //SPI TX
	GPIO_PinModeSet(PSRAM_MISO_PORT, PSRAM_MISO_PIN, gpioModeInputPull, 1); //SPI RX
	GPIO_PinModeSet(PSRAM_SCK_PORT, PSRAM_SCK_PIN, gpioModePushPull, 0); //SPI CK	

	GPIO->EUSARTROUTE[UART_PSRAM_NO].TXROUTE =
	(PSRAM_MOSI_PORT << _GPIO_EUSART_TXROUTE_PORT_SHIFT) | (PSRAM_MOSI_PIN << _GPIO_EUSART_TXROUTE_PIN_SHIFT);
	
	GPIO->EUSARTROUTE[UART_PSRAM_NO].RXROUTE =	
		(PSRAM_MISO_PORT << _GPIO_EUSART_RXROUTE_PORT_SHIFT) | (PSRAM_MISO_PIN << _GPIO_EUSART_RXROUTE_PIN_SHIFT);

	GPIO->EUSARTROUTE[UART_PSRAM_NO].SCLKROUTE =	
		(PSRAM_SCK_PORT << _GPIO_EUSART_RXROUTE_PORT_SHIFT) | (PSRAM_SCK_PIN << _GPIO_EUSART_RXROUTE_PIN_SHIFT);

	
	GPIO->EUSARTROUTE[UART_PSRAM_NO].ROUTEEN = 
		GPIO_EUSART_ROUTEEN_TXPEN | GPIO_EUSART_ROUTEEN_RXPEN | GPIO_EUSART_ROUTEEN_SCLKPEN;
	
	
	EUSART_SpiInit(UART_PSRAM, &init_SPI);
	


	
		

}

uint8_t PsramWriteReadByte(uint8_t data)
{
	
	 
	return EUSART_Spi_TxRx(UART_PSRAM, data);
//	while (HAL_DMA_GetState(&hdma_spi1_tx) != HAL_DMA_STATE_READY) ;
//
//	while (!(SPI1->SR & SPI_SR_TXE)) ;     
//	SPI1_DR_8bit = data;                       
//	while (!(SPI1->SR & SPI_SR_RXNE)) ;  
//	return (SPI1_DR_8bit); 
}


void PsramSpiWriteArray(uint8_t * Data, size_t Len)
{
	while (Len--)
	{
		PsramWriteReadByte(*Data++);
	}
}

void PsramSpiReadArray(uint8_t * Data, size_t Len)
{
	while (Len--)
	{
		*Data++ = PsramWriteReadByte(0xFF);
	}
}


void PsramReadId(uint8_t * array)
{
	while (Psram.Busy) ;
	Psram.Busy = true;
	PSRAM_CS(0); //select PSRAM
	PsramWriteReadByte(0x9F);  //req ID
	PsramSpiWriteArray(LocalBuf, 3); //Addr don't care
	PsramSpiReadArray(array, 3); //read ID
	PSRAM_CS(1); //deselect PSRAM
	Psram.Busy = false;
}


void PsramReset()
{
	while (Psram.Busy) ;
	Psram.Busy = true;
	PSRAM_CS(1);
	PSRAM_CS(0); //select PSRAM
	PsramWriteReadByte(0x66); //Reset en
	PSRAM_CS(1); //deselect PSRAM
	PSRAM_CS(0); //select PSRAM
	PsramWriteReadByte(0x99); //Reset
	PSRAM_CS(1); //deselect PSRAM
	Psram.Busy = false;
}

void PsramRead(uint32_t Addr, uint8_t * data, size_t len)
{
	while (Psram.Busy) ;
	Psram.Busy = true;
	void PsramReset();
	
	PSRAM_CS(0); //select PSRAM

	PsramWriteReadByte(0x03); //read normal
	
	PsramWriteReadByte((Addr >> 16) & 0xFF);
	PsramWriteReadByte((Addr >> 8) & 0xFF);
	PsramWriteReadByte(Addr & 0xFF);
	PsramSpiReadArray(data, len); //read array
	PSRAM_CS(1); //deselect PSRAM
	Psram.Busy = false;

}

void PsramWrite(uint32_t Addr, uint8_t * data, size_t len)
{
	while (Psram.Busy) ;
	Psram.Busy = true;
	PSRAM_CS(0); //select PSRAM
	PsramWriteReadByte(0x02); //write normal
	PsramWriteReadByte((Addr >> 16) & 0xFF);
	PsramWriteReadByte((Addr >> 8) & 0xFF);
	PsramWriteReadByte(Addr & 0xFF);
	PsramSpiWriteArray(data, len); //read array
	PSRAM_CS(1); //deselect PSRAM
	Psram.Busy = false;

}

void PsramDmaRead(uint32_t page, uint8_t * data)
{
	uint32_t Addr = page << 8;
	//while (HAL_DMA_GetState(&hdma_spi1_tx) != HAL_DMA_STATE_READY) ;
	while (Psram.Busy) ;
	//Psram.Busy = true;
	PSRAM_CS(0); //select PSRAM
	PsramWriteReadByte(0x03); //read normal
	PsramWriteReadByte((Addr >> 16) & 0xFF);
	PsramWriteReadByte((Addr >> 8) & 0xFF);
	PsramWriteReadByte(Addr & 0xFF);
	PsramSpiReadArray(data, 256);
	PSRAM_CS(1); //deselect PSRAM

	//HAL_SPI_Receive_DMA(&hspi1, data, 256);
}

void PsramDmaWrite(uint32_t page, uint8_t * data)
{
	uint32_t Addr = page << 8;
	while (Psram.Busy) ;
	Psram.Busy = true;
	
	PSRAM_CS(0); //select PSRAM
	PsramWriteReadByte(0x02); //write normal
	PsramWriteReadByte((Addr >> 16) & 0xFF);
	PsramWriteReadByte((Addr >> 8) & 0xFF);
	PsramWriteReadByte(Addr & 0xFF);
	
//	PsramSpiWriteArray(data, 256);
//	PSRAM_CS(1); //deselect PSRAM
//	Psram.Busy = false;
	


	DMADRV_AllocateChannel(&DmaChannelPsram, NULL);
	
	DMADRV_MemoryPeripheral(DmaChannelPsram,
		dmadrvPeripheralSignal_EUSART2_TXBL,
		(void*)&(UART_PSRAM->TXDATA),
		data,
		true,
		256,
		dmadrvDataSize1,
		SPI_CpltCallback,
		NULL);


	while (Psram.Busy) ;
	//flush fifo
	
	while (EUSART2->STATUS & EUSART_STATUS_RXFL) 
	{
		EUSART2->RXDATA;	
	}


	DMADRV_FreeChannel(DmaChannelPsram);	
	PSRAM_CS(1); //deselect PSRAM	


}