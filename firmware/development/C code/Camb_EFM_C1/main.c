

#include <em_chip.h>
#include <em_cmu.h>
#include <em_core.h>
#include <em_timer.h>




#include "main.h"

#include "utils.h" 

#include "epd27.h"
#include "FT6336.h"
#include "em_ldma.h"

#include "dmadrv.h" 
#include "config\dmadrv_config.h"

#include "AT25.h"
#include "psram.h"

#include "NfcDecoder.h"
#include "sl_status.h"
#include <math.h>
#include "alzlogo.h"
#include "em_acmp.h"


#define SHOWPACK



uint8_t FlashtoEpd[1024];
uint16_t FlashEpdP;
uint32_t FBp;
uint32_t PicBase;

#define LDMA_CH_MASK        (1 << 0)
unsigned int channelRX;
uint8_t PrintPos = 0;
uint8_t AddDataPos = 0;
bool PackLoaded = false;
uint16_t PsramPage;


//char PrintBuf[256];

uint8_t cnnt = 0;

char PassEntered[16];
uint8_t PassLen = 0;

char TcVal[4][4] =
{
	'7', '8', '9', 'F',
	'4', '5', '6', 'E',
	'1', '2', '3', 'D',
	'0', 'A', 'B', 'C'
};

char PassIs[] = { "EDB0C9" };


char ch[16];
bool PassOk = false;



extern volatile double MainVoltage;
extern bool AllPacksReceived;


bool PowerState = false;

char ReceivedChar;

char EpdPrint[128];


extern uint8_t NfcData[NFC_DECODED_MAX]; 
size_t NfcDataLen;


#define RX_BUFFER_SIZE      1024 /* Even numbers only */

/* RX Ring Buffer */
struct 
{
	char buffer[RX_BUFFER_SIZE];
	int startIndex;
	int stopIndex; 
	int counts; 
} rxBuffer = { .startIndex = 0, .stopIndex = 0 };


//Touch
bool TouchFound;
uint8_t TouchModel;
bool Touched;
uint16_t TouchX;
uint16_t TouchY;
uint8_t TouchArr[8];


//touch panel int
void IntWork()
{
	uint32_t intval = GPIO_IntGet();

	if (PowerState)
	{

		switch (intval)
		{

		case 1 << INT_PIN: //touch
		
			if (FT6336_ReadLocation(TouchArr))
			{
				if (!Touched)
				{
					Touched = true;
					TouchX = TouchArr[0];
					TouchY = (TouchArr[1] << 8) + TouchArr[2];
					
					
				}
			}
						
			break;

	
		default:
			break;
		}
	}
	
	GPIO_IntClear(intval);

}



void GPIO_EVEN_IRQHandler(void) 
{
	IntWork();
}



void GPIO_ODD_IRQHandler(void) 
{
	IntWork(); 
}




/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/
/******************************/



void dispw(uint8_t d)
{
	
	cnnt++;
	if (cnnt < 23)
	{
		
		FlashtoEpd[FlashEpdP++] = d;
		if (FlashEpdP > 255)
		{
			FlashEpdP = 0;
			while (SpiFlashReadStatus() & SPI_STATUS_BSY) ; //wait for ready 
			SpiFlashCmd(SPICMD_WREN);
			SpiFlashErasePage(FBp); //erase page

			while (SpiFlashReadStatus() & SPI_STATUS_BSY) ; //wait for ready 
			SpiFlashCmd(SPICMD_WREN);
			SpiFlashWritePage(FlashtoEpd, FBp);
			FBp += 0x100;
		}
	}
	if (cnnt > 49) cnnt = 0;

}




void ShowButtons()
{
	const uint16_t Ys = 88;
	uint8_t i, x, y;
	char ch[2] = { 0, 0 };	
	
	for (i = 0; i < 176; i += 44)
	{
		EPD_line(i, 0 + Ys, i, 175 + Ys, 1);
		EPD_line(0, i + Ys, 175, i + Ys, 1);
	}
	EPD_line(175, 0 + Ys, 175, 175 + Ys, 1);
	EPD_line(0, 175 + Ys, 175, 175 + Ys, 1);

	for (y = 0; y < 4; y++)
	{
		for (x = 0; x < 4; x++)
		{
			ch[0] = TcVal[y][x];
			EPD_Text(x * 44 + 16, y * 44 + 15 + Ys, ch, 1, 0);

		}
	}
		
	EPD_PartWriteFull();
}












void Shuffle(char * arr)
{
	uint8_t RandV;
	char bc, idx;
	for (uint8_t i = 0; i < 16; i++)
	{
		RandV = EfmGetRandom();
		idx = RandV & 0x04;
		bc = arr[i];
		arr[i] = arr[idx];
		arr[idx] = bc;
	}
}





int main()
{
	uint8_t i, j;
	
	uint16_t x, y;
	float mfX, mfY;
	float xScreen, yScreen;
	float xCent, yCent;
	
	void *user;
	Ecode_t res; 

	CHIP_Init();
	ClockInit();
	
#ifndef SHOWPACK
	
	while (true)
	{
		// Enter EM2 sleep, woken by IADC interrupt
		EMU_EnterEM2(true);
		if (MainVoltage > 14.0f) break;
	}
#endif	

	
	
	SpiFlashDispInit(); //flash and disp SPI init
	SpiFlashUDPD(); //Ultra-Deep Power-Down of Flash chip

	DMADRV_Init(); //init DMA
	
	SpiDispInit(); //touch panel and disp lines init

	UartInit();
	PsramInit(); //Psram ports init	
	
	NfcInit();
	
	

	
	
	POW_2V8(1); // enable 2.8 volt
	POW_I2C_T(1); // enable i2c level translator
	PowerState = true;

	Delay(1000);
	
	
	uint16_t Fs = SYSTEM_GetFlashSize();
	uint8_t rev = SYSTEM_GetDevinfoRev();
	uint16_t Pn = SYSTEM_GetPartNumber();

	uPrintf("\r\nReady  Pn: %u, Rev: %u, Flash: %u\r\n", Pn, rev, Fs);
	
	EPD_SetRotate(0);
	EPD_HW_Init(); //Electronic paper initialization	
	EPD_WhiteScreen_White();
	EPD_clear(1); //buf init
	EPD_DeepSleep(); //EPD_DeepSleep
	


		
//	GPIO_PinModeSet(UART_RX_PORT, UART_RX_PIN, gpioModePushPull, 0); //
//	
//	
//	
//	CMU_ClockEnable(cmuClock_ACMP0, true);
//	
//	// Initialize with default settings
//	ACMP_Init_TypeDef init = ACMP_INIT_DEFAULT;
//	init.biasProg = 3;
//	init.accuracy = acmpAccuracyHigh;
//	init.vrefDiv = 55;
//
//	ACMP_Init(ACMP0, &init);
//
//	// Allocate CDODD0 to ACMP0 to be able to use the input
//	GPIO->BBUSALLOC = GPIO_BBUSALLOC_BEVEN0_ACMP0;
//	
//	ACMP_ChannelSet(ACMP0, acmpInputVREFDIV1V25, acmpInputPB2);
//	
//	
//	ACMP_GPIOSetup(ACMP0, gpioPortB, 6, true, false);
//	
//	while (!(ACMP0->IF & ACMP_IF_ACMPRDY)) ;
//	
//
//	while (true)
//	{
//	         
//	}
//	
	
	
	
	
	TouchFound = true;
	TouchModel = FT6336_ReadId();
	if (TouchModel == FT6206_CHIPID) sprintf(EpdPrint, "FT6206 Touch");
	else if (TouchModel == FT6236_CHIPID) sprintf(EpdPrint, "FT6236 Touch");
	else if (TouchModel == FT6336_CHIPID) sprintf(EpdPrint, "FT6336 Touch");
	else 
	{
		TouchFound = false;
		sprintf(EpdPrint, "Touch Panel NG");
	}
	
	uPrintf("%s\r\n", EpdPrint);
	EPD_Text(0, 0, EpdPrint, 1, 0);
	
	SpiFlashExitSleep();
	SpiFlashExitSleep();
	
	size_t FlashSize = SpiFlashGetSize();
	
	if (FlashSize > 0 && (FlashSize & -FlashSize) == FlashSize) //check if pow of 2
	{
		sprintf(EpdPrint, "Flash %u KB", FlashSize);
		
	}
	else  sprintf(EpdPrint, "Flash not found");
	SpiFlashUDPD(); //Ultra-Deep Power-Down of Flash chip
	
	uPrintf("%s\r\n", EpdPrint);
	EPD_Text(0, 20, EpdPrint, 1, 0);
	
	
	
	PsramReset();
	PsramReadId(Psram.Buf);
	if (Psram.Buf[0] == 0x0D && Psram.Buf[1] == 0x5D)
	{
		Psram.Size = (Psram.Buf[2] & 0xF0) << 7;
		Psram.Found = true;
		sprintf(EpdPrint, "PSRAM %u KB", Psram.Size ? Psram.Size : 2048);
	}
	else 
	{
		Psram.Found = false;
		sprintf(EpdPrint, "PSRAM Not Found");
	}
	
	uPrintf("%s\r\n", EpdPrint);
	EPD_Text(0, 40, EpdPrint, 1, 0);
	
	
	EPD_putLogo(alzilogo176, 110, sizeof(alzilogo176));
	
	EPD_HW_Init_Fast();
	EPD_PartWriteFull();
	EPD_PartWriteFull();
	EPD_DeepSleep(); //EPD_DeepSleep

	Delay(100);
	
	EPD_HW_Init_Fast();
	
	EPD_clear(1); //buf init

	EPD_PartWriteFull();
	
	
	Shuffle((char*)TcVal);
	
	EPD_putLogo(alzilogo176, 22, sizeof(alzilogo176));
	ShowButtons();
	EPD_PartWriteFull();
	
	EPD_DeepSleep(); //EPD_DeepSlee
	

	if (Touched)
	{
		
	
	
	
		while (1)
		{
			if (Touched)
			{
				if (TouchY < 176)
				{
					TouchX = 175 - TouchX;
					TouchY = 265 - TouchY;
				
					xScreen = TouchX / 44.0f;
					yScreen = (TouchY - 88) / 44.0f;
				
					xCent = modff(xScreen, &mfX);
					yCent = modff(yScreen, &mfY);
					
					x = mfX; 
					y = mfY;
				
					if (((xCent > 0.2f && xCent < 0.8f) || x == 0 || x == 3) && 
						((yCent > 0.2f && yCent < 0.8f) || y == 0 || y == 3))
					{
				
						EPD_HW_Init_Fast();

						ch[0] = TcVal[y][x];
						ch[1] = 0;
				
						//				uPrintf("X: %03u Y: %03u  %u-%u  %.2f  %.2f\r\n", 
						//					175 - TouchX, 265 - TouchY, x, y, 
						//					(175 - TouchX) / 44.0f, (265 - TouchY - 88) / 44.0f);
							
						EPD_DrawFilledRectangle(x * 44 + 2, y * 44 + 2 + 88, x < 3 ? 40 : 39, y < 3 ? 40 : 39, 1);
						EPD_Text(x * 44 + 16, y * 44 + 15 + 88, ch, 0, 1);
				
						EPD_putLogo(alzilogo176, 22, sizeof(alzilogo176));
						EPD_PartWriteFull();
						EPD_clear(1); //buf init
						EPD_putLogo(alzilogo176, 22, sizeof(alzilogo176));
				
						uPrintf("%c", ch[0]);
						PassEntered[PassLen++] = ch[0];
						if (PassLen == sizeof(PassIs) - 1) 
						{
							for (i = 0; i < PassLen; i++)
							{
								if (PassIs[i] != PassEntered[i])
								{
									EPD_clear(1); //buf init
									EPD_PartWriteFull();
									EPD_line(0, 0, 175, 263, 1);
									EPD_line(175, 0, 0, 263, 1);
									EPD_WriteScreen_Buff_Fast();
							
									EPD_DeepSleep();
									uPrintf("\r\nPass err\r\n");
									while (true) ;
								}
							}
							uPrintf("\r\nPass OK\r\n");
							while (FT6336_ReadLocation(TouchArr))
							{
								Delay(10);
							}
							Touched = false;
							PassOk = true;
					
							break;
						}
				
						while (FT6336_ReadLocation(TouchArr))
						{
							Delay(10);
						}
						Touched = false;
				
						Shuffle((char*)TcVal);			
						ShowButtons();
						EPD_DeepSleep(); 			
					}
				
				}
			}
	 
			Touched = false;
		
		
		}
	}
	
	Delay(500);
	
//	POW_2V8(0); 
//	POW_I2C_T(0);
//	PowerState = false;
	
	//	POW_2V8(1); // enable 2.8 volt
	//	POW_I2C_T(1); // enable i2c level translator
	//	PowerState = true;
	//	Delay(1000);
	
	
	
	SpiFlashExitSleep();
	EPD_HW_Init_Fast(); //EPD init
	Epaper_Write_Command(0x24); //write RAM for black(0)/white (1)	
	
	FlashEpdP = 0;
	FBp = 0;	
	for (uint16_t j = 0; j < 5808; j++)
	{
		if (!FlashEpdP)
		{
			SpiFlashReadPage(FlashtoEpd, FBp);
			FBp += 0x100;
		}
		Epaper_Write_Data(FlashtoEpd[FlashEpdP]);
		FlashEpdP++;
		if (FlashEpdP == 256) FlashEpdP = 0;
	}

		
	
	SpiFlashUDPD(); //Ultra-Deep Power-Down of Flash chip
	EPD_Update_Fast();	
	while (DISP_BUSY) ;	
	EPD_DeepSleep(); //EPD_DeepSleep

//
//	POW_2V8(0); 
//	POW_I2C_T(0);
//	PowerState = false;
	
	
	


	while (true)
	{
		PsramPage = 0;
		while (true)
		{
			
			if (Touched && AllPacksReceived)
			{
				SpiFlashExitSleep();
		
			
				Touched = false;
				PicBase += 0x10000;
				if (PicBase > 0x50000) PicBase = 0;
					
			
				EPD_HW_Init_Fast(); //EPD init
				Epaper_Write_Command(0x24); //write RAM for black(0)/white (1)
						
				FlashEpdP = 0;
				FBp = PicBase;	
				for (uint16_t j = 0; j < 5808; j++)
				{
					if (!FlashEpdP)
					{
						SpiFlashReadPage(FlashtoEpd, FBp);
						FBp += 0x100;
					}
					Epaper_Write_Data(FlashtoEpd[FlashEpdP]);
					FlashEpdP++;
					if (FlashEpdP == 256) FlashEpdP = 0;
				}
						
						
				EPD_Update_Fast();	
				EPD_DeepSleep(); //EPD_DeepSleep
			
				Touched = false;

			}
			
			
			
			if (AllPacksReceived && PsramPage) break;
			else AllPacksReceived = false;
		
			NfcDataLen = NfcWork();
			if (NfcDataLen)
			{
				if ((NfcDataLen & 0x0FFF) > 2) 
				{
					
					//if ((NfcDataLen & 0x0FFF) > 200) PsramPage++;
					//uPrintf("Big: %u\r\n", NfcDataLen);
					if (NfcDataLen & 0x8000)
					{
						
						uPrintf("Response: > ");
						for (uint32_t i = 0; i < (NfcDataLen & 0x0FFF); i++) 
						{
							//if (!(i % 32)) uPrintf("\r\n");
							uPrintf("%02X ", NfcData[i]);
						}
						uPrintf("<\r\n\r\n");
					}
					else 
					{
						
						if (NfcDataLen > 200) 
						{
							TIMER1->CNT = 0;  //timeout reset
							
							AddDataPos = NfcData[4] << 1; //tail addr in 1-st page
							//uPrintf("%02u ", PsramPage);
							uPrintf(".");
							
							PsramDmaWrite(1 + PsramPage++, NfcData); //leave first page for pack tails
												
						}
						else if (NfcDataLen == 5 && NfcData[0] == 0x02 && PackLoaded) //if pack tail 2 bytes
						{
							PsramWrite(AddDataPos, NfcData + 1, 2); //tail write
							PackLoaded = false;
						}
					
						//uPrintf("Request: %u bytes", NfcDataLen & 0x0FFF);
						
//						for (uint32_t i = 0; i < (NfcDataLen); i++) 
//						{
//							if (!(i % 32)) uPrintf("\r\n");
//							uPrintf("%02X ", NfcData[i]);
//						}
						//uPrintf("\r\n");						
					}
				}

				NfcDecodeReset();
			}

			
		}

	
//		for (uint16_t pg = 0; pg < PsramPage; pg++)
//		{
//			PsramDmaRead(pg, NfcData);
//			for (uint32_t i = 0; i < 256; i++) 
//			{
//				if (!(i % 32)) uPrintf("\r\n");
//				uPrintf("%02X ", NfcData[i]);
//			}
//			uPrintf("\r\n");
//
//			PsramDmaRead(pg + 0x4000, NfcData);
//			for (uint32_t i = 0; i < 32; i++) 
//			{
//				if (!(i % 32)) uPrintf("\r\n");
//				uPrintf("%02X ", NfcData[i]);
//			}
//			uPrintf("\r\n");
//			
//		}		
		
		uPrintf("\r\nTotal: %u Big Packs\r\n", PsramPage);
		
		SpiFlashExitSleep();
	
		FlashEpdP = 0;
		FBp = PicBase;
						
		if (PassOk)
		{
						
			for (uint16_t j = 0; j < 60; j++)
			{
				PsramDmaRead(j + 1, Psram.Buf);
				while (Psram.Busy) ;
				
				for (uint16_t i = 6; i < 254; i++)
				{
					dispw(Psram.Buf[i]);
								
				}
							
				PsramRead(j << 1, Psram.Buf, 2);
				dispw(Psram.Buf[0]);
				dispw(Psram.Buf[1]);
			}
			
			EPD_HW_Init_Fast(); //EPD init
			Epaper_Write_Command(0x24); //write RAM for black(0)/white (1)
						
			FlashEpdP = 0;
			FBp = PicBase;	
			for (uint16_t j = 0; j < 5808; j++)
			{
				if (!FlashEpdP)
				{
					SpiFlashReadPage(FlashtoEpd, FBp);
					FBp += 0x100;
				}
				Epaper_Write_Data(FlashtoEpd[FlashEpdP]);
				FlashEpdP++;
				if (FlashEpdP == 256) FlashEpdP = 0;
			}
						
						
			EPD_Update_Fast();	
			EPD_DeepSleep(); //EPD_DeepSleep
			
		
		}		
		
		

		
		
	}
		


	
//	while (1)
//	{
//	
//			
//		
//		//					
//		#ifdef SHOWPACK
//														
//								if (VcdDataLen > 3)
//								{
//									uPrintf("Len %02u: > ", VcdDataLen);
//									for (uint32_t i = 0; i < VcdDataLen; i++) 
//									{
//										uPrintf("%02X ", VcdData[i]);
//									}
//									uPrintf("<\r\n");
//		//							while (true)
//		//							{
//		//	         
//		//							}
//								}
//								else
//								{
//									uPrintf("%u ", VcdDataLen);
//								}
//		#endif						
//								
//								if (VcdDataLen > 200) 
//								{
//									AddDataPos = VcdData[4] << 1; //tail addr in 1-st page
//									//uPrintf("%02u ", PsramPage);
//									uPrintf(".");
//									PsramDmaWrite(1 + PsramPage++, VcdData); //leave first page for pack tails
//					
//							
//									PackLoaded = true; //next should be tail
//								}
//								else if (VcdDataLen == 5 && VcdData[0] == 0x02 && PackLoaded) //if pack tail 2 bytes
//								{
//									PsramWrite(AddDataPos, VcdData + 1, 2); //tail write
//									PackLoaded = false;
//								}
//								else if(VcdDataLen > 40 && VcdDataLen < 100 && VcdData[11] == 0x72 && VcdData[12] == 0x75)
//								{
//									text test
//															
//															uPrintf("Len: %02u, ", VcdDataLen);
//																
//															for (uint8_t i = 0; i < VcdDataLen; i++) 
//															{
//																if (i < 32)
//																{
//																	uPrintf("%02X ", VcdData[i]);
//																}
//															}
//															if (i >= 32) uPrintf(">>>");
//															uPrintf("\r\n");
//															
//															
//															
//																PsramDmaWrite(PsramPage++, VcdData); //leave first page for pack tails
//																NfcFifo.TimeOut = true; //rx timeout
//
//																return;
//															
//	}
//						
//						
//							PrintPos = 0;
//							for (uint8_t i = 0; i < ((VcdDataLen > 32) ? 32 : VcdDataLen); i++) 
//							{
//								PrintPos += sprintf(PrintBuf + PrintPos, "%02X ", VcdData[i]);
//							}
//							sprintf(PrintBuf + PrintPos, "\r\n");
//							uPrintf(PrintBuf);
//					
//						
//						
//#ifndef SHOWPACK						
//												//check if end of pack
//	if (NfcData[0] == 0xE0 && NfcData[1] == 0x80 && NfcDataLen == 4) //end of data
//	{
//		uPrintf("<\r\n");
//	}
//#endif 
//
//					

		

//
//	//
	//				
	//				
	//				
	//				
	//				if (NfcFifo.TimeOut)
	//				{
	//					if (PsramPage > 1)
	//					{			
	//						uPrintf("Loaded %u pages\r\n", PsramPage);
							//PsramInit(); //Psram ports init	

						
						
							//						PsramDmaRead(1, Psram.Buf);
							//						
							//						for (uint8_t i = 0; i < 32; i++) 
							//						{
							//							uPrintf("%02X ", Psram.Buf[i]);
							//						}
							//						uPrintf("\r\n");
						

							//						
							//						cnnt = 0;
							//						
							//						SpiFlashExitSleep();
							//	
							//						FlashEpdP = 0;
							//						FBp = PicBase;
							//						
							//	
							//						
							//						for (uint16_t j = 0; j < 60; j++)
							//						{
							//							PsramDmaRead(j + 1, Psram.Buf);
							//							while (Psram.Busy) ;
							//				
							//							for (uint16_t i = 6; i < 254; i++)
							//							{
							//								dispw(Psram.Buf[i]);
							//								
							//							}
							//							
							//							PsramRead(j << 1, Psram.Buf, 2);
							//							dispw(Psram.Buf[0]);
							//							dispw(Psram.Buf[1]);
							//			
							//
							//						}
							//						
						
						
							//						POW_2V8(1); // enable 2.8 volt
							//						POW_I2C_T(1); // enable i2c level translator
							//						PowerState = true;
							//						Delay(1000);
						
	EPD_HW_Init_Fast(); //EPD init
	Epaper_Write_Command(0x24); //write RAM for black(0)/white (1)
						
	FlashEpdP = 0;
	FBp = PicBase;	
	for (uint16_t j = 0; j < 5808; j++)
	{
		if (!FlashEpdP)
		{
			SpiFlashReadPage(FlashtoEpd, FBp);
			FBp += 0x100;
		}
		Epaper_Write_Data(FlashtoEpd[FlashEpdP]);
		FlashEpdP++;
		if (FlashEpdP == 256) FlashEpdP = 0;
	}
						
						
	EPD_Update_Fast();	
	EPD_DeepSleep(); //EPD_DeepSleep

	//						POW_2V8(0); 
	//						POW_I2C_T(0);
	//						PowerState = false;
						
						
						
						
	//						NfcFifo.TimeOut = false; 
	//						PsramPage = 0;
						


		
		
		
	
		


	
	
	
	
	
	
	
	
	
	
	
	
	
	
	

	
	POW_2V8(1); // enable 2.8 volt
	POW_I2C_T(1); // enable i2c level translator
	PowerState = true;

	
	
	//start UART receive
	res = DMADRV_AllocateChannel(&channelRX, NULL);
	if (res == ECODE_OK)
	{
		res = DMADRV_PeripheralMemoryPingPong(channelRX,
			dmadrvPeripheralSignal_EUSART1_RXDATAV,
			rxBuffer.buffer,
			rxBuffer.buffer+(RX_BUFFER_SIZE >> 1),
			(void*)&(EUSART1->RXDATA),
			true,
			RX_BUFFER_SIZE >> 1,
			dmadrvDataSize1,
			NULL,
			NULL);
	}
	
	
	

		

	
	
	//	
	//	while (true)
	//	{
	//		
	//		//fifo
	//		rxBuffer.startIndex = LDMA->CH[channelRX].DST - (uint32_t)&rxBuffer.buffer[0];
	//		
	//		while (rxBuffer.stopIndex != rxBuffer.startIndex) 
	//		{
	//			rxBuffer.startIndex = LDMA->CH[channelRX].DST - (uint32_t)&rxBuffer.buffer[0];
	//			
	//			ReceivedChar = rxBuffer.buffer[rxBuffer.stopIndex++];
	//			if (rxBuffer.stopIndex == RX_BUFFER_SIZE) rxBuffer.stopIndex = 0;
	//			uPrintf("%c", ReceivedChar);
	//		}
	//		
	//		
	//	}
	

	
	
	
	Delay(60000);
	NVIC_SystemReset();
	
	
	
	

	char txbuf[128];
	while (1)
	{
		//		for (i = 0; i < 16; i++) 
		//		{
		//	    
		//			EPD_clear(1); //buf init
		//			sprintf(txbuf, "%02X  ", i);
		//			EPD_Text(i * 10, i * 15, txbuf, 1, 0);
		//	    
		//			EPD_Part_Write(0, 0, 0, 0);
		//			while (DISP_BUSY) ;	
		//			//EPD_WriteScreen_Buff_Fast();
		//			EPD_DeepSleep(); //EPD_DeepSlee
		//	    
		//			
		//			LEDT;
		//       
		//		}
		EPD_HW_Init(); //Electronic paper initialization	
		EPD_WhiteScreen_White();
		EPD_DeepSleep(); //EPD_DeepSlee
		Delay(15000);
 
	}
	
	
	
	return 0;
}






















