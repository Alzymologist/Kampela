
#include "AT25.h"



void SpiFlashDispInit()
{
	
	USART_InitSync_TypeDef init_SPI = USART_INITSYNC_DEFAULT;

	
	init_SPI.msbf = true;
	init_SPI.baudrate = 10000000;
	
	CMU_ClockEnable(cmuClock_USART0, true); //SPI port
	CMU_ClockEnable(cmuClock_GPIO, true);
	
	//SPI shares port with flash
	GPIO_PinModeSet(SPI_DC_PORT, SPI_DC_PIN, gpioModePushPull, 0); //DISP DC
	GPIO_PinModeSet(SPI_RES_PORT, SPI_RES_PIN, gpioModePushPull, 0); //DISP RES
	GPIO_PinModeSet(SPI_BUSY_PORT, SPI_BUSY_PIN, gpioModeInput, 0); //DISP BUSY
	GPIO_PinModeSet(SPI_CS_DISP_PORT, SPI_CS_DISP_PIN, gpioModeInputPull, 1); //DISP CS
	
	
	GPIO_PinModeSet(F_CS_PORT, F_CS_PIN, gpioModePushPull, 1); //SPI CS
	GPIO_PinModeSet(E_MOSI_PORT, E_MOSI_PIN, gpioModePushPull, 1); //SPI TX
	GPIO_PinModeSet(E_MISO_PORT, E_MISO_PIN, gpioModeInputPull, 1); //SPI RX
	GPIO_PinModeSet(E_SCK_PORT, E_SCK_PIN, gpioModePushPull, 0); //SPI CK	


	USART_InitSync(SPI_DISP_FLASH, &init_SPI);
	
	GPIO->USARTROUTE[SPI_DISP_FLASH_NO].TXROUTE =
		(E_MOSI_PORT << _GPIO_USART_TXROUTE_PORT_SHIFT) | (E_MOSI_PIN << _GPIO_USART_TXROUTE_PIN_SHIFT);

	GPIO->USARTROUTE[SPI_DISP_FLASH_NO].RXROUTE =
		(E_MISO_PORT << _GPIO_USART_RXROUTE_PORT_SHIFT) | (E_MISO_PIN << _GPIO_USART_RXROUTE_PIN_SHIFT);

	
	GPIO->USARTROUTE[SPI_DISP_FLASH_NO].CLKROUTE =
		(E_SCK_PORT << _GPIO_USART_CLKROUTE_PORT_SHIFT) | (E_SCK_PIN << _GPIO_USART_CLKROUTE_PIN_SHIFT);

	GPIO->USARTROUTE[SPI_DISP_FLASH_NO].ROUTEEN = 
		(GPIO_USART_ROUTEEN_TXPEN | GPIO_USART_ROUTEEN_RXPEN | GPIO_USART_ROUTEEN_CLKPEN);

	

	

}

//The Ultra-Deep Power-Down 
void SpiFlashUDPD() 	
{
	FLASH_CS(1);
	FLASH_CS(0);
	USART_SpiTransfer(SPI_DISP_FLASH, SPICMD_UDPD);
	FLASH_CS(1);
}

void SpiFlashExitSleep() 	
{
	FLASH_CS(1);
	FLASH_CS(0);
	USART_SpiTransfer(SPI_DISP_FLASH, SPICMD_RDP);
	FLASH_CS(1);
	Delay(1); //flash wakeup time
		
}

uint8_t SpiFlashReadStatus()
{
	static uint8_t i;
	FLASH_CS(0);
	USART_SpiTransfer(SPI_DISP_FLASH, SPICMD_RDSR);
	i = USART_SpiTransfer(SPI_DISP_FLASH, 0x00);
	FLASH_CS(1);	
	return i;
}

void SpiFlashCmd(uint8_t cmd)
{
	FLASH_CS(0);
	USART_SpiTransfer(SPI_DISP_FLASH, cmd);
	FLASH_CS(1);	
}


//read 256 bytes
void SpiFlashReadPage(uint8_t* data, uint32_t addr)
{
	size_t len = 256;
	FLASH_CS(0);
	USART_SpiTransfer(SPI_DISP_FLASH, SPICMD_READ);
		
	USART_SpiTransfer(SPI_DISP_FLASH, (addr >> 16) & 0xFF);
	USART_SpiTransfer(SPI_DISP_FLASH, (addr >> 8) & 0xFF);
	USART_SpiTransfer(SPI_DISP_FLASH, addr & 0xFF);
	while (len--)
	{
		*data++ = USART_SpiTransfer(SPI_DISP_FLASH, 0x00);
	}
	
	FLASH_CS(1);	
}

//write 256 bytes
void SpiFlashWritePage(uint8_t* data, uint32_t addr)
{
	size_t len = 256;
	FLASH_CS(0);
	USART_SpiTransfer(SPI_DISP_FLASH, SPICMD_PP);
	
	USART_SpiTransfer(SPI_DISP_FLASH, (addr >> 16) & 0xFF);
	USART_SpiTransfer(SPI_DISP_FLASH, (addr >> 8) & 0xFF);
	USART_SpiTransfer(SPI_DISP_FLASH, addr & 0xFF);
	while (len--)
	{
		USART_SpiTransfer(SPI_DISP_FLASH, *data++);
	}
	
	FLASH_CS(1);	
}

//erase 256 bytes
void SpiFlashErasePage(uint32_t addr)
{
	FLASH_CS(0);
	USART_SpiTransfer(SPI_DISP_FLASH, SPICMD_PE);
	USART_SpiTransfer(SPI_DISP_FLASH, (addr >> 16) & 0xFF);
	USART_SpiTransfer(SPI_DISP_FLASH, (addr >> 8) & 0xFF);
	USART_SpiTransfer(SPI_DISP_FLASH, addr & 0xFF);
	FLASH_CS(1);	
}

uint32_t SpiFlashGetSize()
{
	uint8_t i;
	uint8_t SizeH;
	uint32_t r = 0;

	FLASH_CS(0);

	USART_SpiTransfer(SPI_DISP_FLASH, SPICMD_SFDP); //req JEDEC table
	
	for (i = 0; i < 11; i++)
	{
		SizeH = 1 + USART_SpiTransfer(SPI_DISP_FLASH, 0); //header size
	}

	USART_SpiTransfer(SPI_DISP_FLASH, 0); //block align
	for (i = 0; i < (SizeH << 3) + 4; i++) //jump to memsize
	{
		USART_SpiTransfer(SPI_DISP_FLASH, 0);
	}
	for (i = 0; i < 4; i++)
	{
		r >>= 8;
		r |= (USART_SpiTransfer(SPI_DISP_FLASH, 0) << 24);
	}
	r++;
	FLASH_CS(1);
	return r >> 13;  //KBytes
}