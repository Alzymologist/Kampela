
#include "NfcDecoder.h"


/************************/

#define RESPONSE false

struct NfcFifo_t
{
	uint16_t Buf[DMA_FIFO_SIZE];  
	uint16_t Tail;
	uint16_t Head;
	uint16_t Size;
	uint16_t LastVal;
	bool TimeOut;
	NFCStates State;
};


volatile struct NfcFifo_t NfcFifo;
volatile RxStates State;


static uint8_t DecoderByteRx = 0;
volatile uint8_t BPos = 0;
volatile uint8_t Parity = 0;
volatile bool ErF;
volatile bool One;
volatile uint8_t BitInv;

volatile bool AllPacksReceived = false;

volatile uint8_t NfcData[256];
volatile uint32_t VcdDataLen = 0;
volatile unsigned int DmaChannelTim0 = 7;

const LDMA_TransferCfg_t periTransferRx = LDMA_TRANSFER_CFG_PERIPHERAL(ldmaPeripheralSignal_TIMER0_CC0);
const LDMA_Descriptor_t xfer[] = 
{
	LDMA_DESCRIPTOR_LINKREL_P2M_HALF(&(TIMER0->CC[0].ICF), NfcFifo.Buf, (DMA_FIFO_SIZE >> 1), 1),
	LDMA_DESCRIPTOR_LINKREL_P2M_HALF(&(TIMER0->CC[0].ICF), NfcFifo.Buf + (DMA_FIFO_SIZE >> 1), (DMA_FIFO_SIZE >> 1), -1)
};

GPCRC_Init_TypeDef init_c = GPCRC_INIT_DEFAULT;


TIMER_InitCC_TypeDef timerCCInit = 
{
//	.eventCtrl = timerEventEveryEdge,
//	.edge = timerEdgeBoth, 
	
	.eventCtrl = timerEventFalling,
	.edge = timerEdgeFalling, 
	
	.prsSel = timerPRSSELCh0,
	.cufoa = timerOutputActionNone,
	.cofoa = timerOutputActionNone,
	.cmoa = timerOutputActionNone,
	.mode = timerCCModeCapture,
	.filter = false,
	.prsInput = false,
	.coist = false,
	.outInvert = true,
};

TIMER_InitCC_TypeDef messageTimerCC = TIMER_INITCC_DEFAULT;

TIMER_Init_TypeDef timerInit =
{
	.enable = false,
	.debugRun = true,
	.prescale = timerPrescale1,
	.clkSel = timerClkSelHFPerClk,
	
//	.fallAction = timerInputActionReloadStart,
	.fallAction = timerInputActionStop,
	
	.riseAction = timerInputActionReloadStart,
	.mode = timerModeUp,
	.dmaClrAct = false,
	.quadModeX4 = false,
	.oneShot = false,
	.sync = false,
};

TIMER_Init_TypeDef messageTimer =
{
	.enable = true,
	.debugRun = true,
	.prescale = timerPrescale128,
	.clkSel = timerClkSelHFPerClk,
	.fallAction = timerInputActionNone,
	.riseAction = timerInputActionNone,
	.mode = timerModeUp,
	.dmaClrAct = false,
	.quadModeX4 = false,
	.oneShot = false,
	.sync = false,
};



/************************/

void NfcInit()
{
	CMU_ClockEnable(cmuClock_GPIO, true);
	CMU_ClockEnable(cmuClock_TIMER0, true); //main NFC Capture timer
	CMU_ClockEnable(cmuClock_TIMER1, true); //pack timeout timer
	
	GPIO_PinModeSet(NFC_PORT, NFC_PIN, gpioModeInputPullFilter, 0); //NFC Input
	
	GPIO->TIMERROUTE_SET[TIMER_NUM(TIMER0)].ROUTEEN = GPIO_TIMER_ROUTEEN_CC0PEN;
	GPIO->TIMERROUTE_SET[TIMER_NUM(TIMER0)].CC0ROUTE =
		(NFC_PORT << _GPIO_TIMER_CC0ROUTE_PORT_SHIFT) | (NFC_PIN << _GPIO_TIMER_CC0ROUTE_PIN_SHIFT);
	
	/* Select CC channel parameters */

	
	TIMER_InitCC(TIMER0, 0, &timerCCInit);

	/* Select timer parameters */  


	/* Enable overflow and CC0 interrupt */
	//TIMER_IntEnable(TIMER0, TIMER_IF_OF | TIMER_IF_CC0);
	//TIMER_IntEnable(TIMER0, TIMER_IF_OF);
	/* Enable TIMER0 interrupt vector in NVIC */
	//NVIC_EnableIRQ(TIMER0_IRQn);
	
	GPIO->TIMERROUTE_SET[TIMER_NUM(TIMER1)].ROUTEEN = GPIO_TIMER_ROUTEEN_CC0PEN;
	GPIO->TIMERROUTE_SET[TIMER_NUM(TIMER1)].CC0ROUTE =
		(NFC_PORT << _GPIO_TIMER_CC0ROUTE_PORT_SHIFT) | (NFC_PIN << _GPIO_TIMER_CC0ROUTE_PIN_SHIFT);

	//TIMER_InitCC(TIMER1, 0, &messageTimerCC);

	
	TIMER_IntEnable(TIMER1, TIMER_IF_OF);
	NVIC_EnableIRQ(TIMER1_IRQn);
	
	
	TIMER_Init(TIMER0, &timerInit);
	
	TIMER_Init(TIMER1, &messageTimer);
	
	
	
	
	
	LDMA_StartTransfer(DmaChannelTim0, (void*)&periTransferRx, (void*)&xfer);
	
	
	CMU_ClockEnable(cmuClock_GPCRC, true);

	init_c.crcPoly = 0x1021;
	init_c.initValue = 0x6363;
	init_c.reverseBits = false;
	GPCRC_Init(GPCRC, &init_c);
	NfcDecodeReset();
	
	
}


void TIMER0_IRQHandler(void)
{ 
	//uint16_t intFlags = TIMER_IntGet(TIMER0);
	TIMER_IntClear(TIMER0, TIMER_IF_OF | TIMER_IF_CC0);
}


void TIMER1_IRQHandler(void)
{ 
	//uint16_t intFlags = TIMER_IntGet(TIMER0);
	AllPacksReceived = true;
	TIMER_IntClear(TIMER1, TIMER_IF_OF | TIMER_IF_CC0);
}


bool GetBit(bool bit)
{
	ErF = false;
	Parity += bit;
	
	if (BPos ^ 0x08)
	{
		DecoderByteRx |= bit << BPos;
		BPos++;
	}
	else
	{
		if (Parity & 0x01)
		{
			NfcData[VcdDataLen++] = DecoderByteRx;  
			GPCRC->INPUTDATABYTE = DecoderByteRx;  //update CRC
			/****************/
			if (!(GPCRC->DATA)) ErF = true; //finish at crc ok
			/****************/
		}
		else ErF = true; //Error or pack end
		
		DecoderByteRx = 0;
		BPos = 0;
		Parity = 0;
		return ErF;
	}
	return false;
}

bool NfcDecode()
{
	
//	static uint16_t ii = 0;
	
//	while (1)
//	{
//		NfcFifo.Tail = ((LDMA->CH[DmaChannelTim0].DST - (uint32_t)&NfcFifo.Buf[0]) >> 1) &
//			(DMA_FIFO_SIZE - 1);
//		if (NfcFifo.Tail != NfcFifo.Head) //new data came
//		{
//
//			NfcFifo.LastVal = NfcFifo.Buf[NfcFifo.Head++]; //get tim val
//			NfcFifo.Head &= 0x0FFF;
//
//			uPrintf("%u ", NfcFifo.LastVal);
//			if (NfcFifo.LastVal > CMAX) uPrintf("\r\n\r\n");
//		}
//	}
//	
	NfcFifo.Tail = ((LDMA->CH[DmaChannelTim0].DST - (uint32_t)&NfcFifo.Buf[0]) >> 1) & 0x0FFF;

	
	if (NfcFifo.Tail != NfcFifo.Head) //new data came
	{
		NfcFifo.LastVal = NfcFifo.Buf[NfcFifo.Head++]; //get tim val
		NfcFifo.Head &= 0x0FFF;

//		
//		//Manchester decoder
//		if (RESPONSE)
//		{
//			if (NfcFifo.LastVal > MMIN && NfcFifo.LastVal < MMAX)
//			{
//				NfcDecodeReset();			
//				NfcFifo.State = Manchester;
//				return false;
//			}
//		
//			if (NfcFifo.State == Manchester)
//			{
//				switch (NfcFifo.LastVal)
//				{
//				case MLMIN ... MLMAX:
//					if (One) 
//					{
//						return true; //error
//					}
//					BitInv ^= 0x01;
//					return GetBit(BitInv);
//				
//				case MSMIN ... MSMAX:
//					if (One)
//					{
//						One = false;
//						return GetBit(BitInv);	
//					}
//					else
//					{
//						One = true;
//						return false;
//					}
//				
//				
//				
//				default:
//					return true; //not in range - error or pack end
//				}
//			
//				return false;
//			}
//		}
//		/**************************/
		
		//Miller gap filter
		if (NfcFifo.LastVal < AMIN)
		{
			return false;
		}
			
		//Miller decoder
		
		if (NfcFifo.State == Started) //receiving
		{
			switch (NfcFifo.LastVal)
			{
			case AMIN ... AMAX:
				if (State == State_X)
				{
					return GetBit(1);
				}
				else if (State == State_Z)
				{
					return GetBit(0);
				}
				
			case BMIN ... BMAX:
				if (State == State_X)
				{
					State = State_Z;
					if (GetBit(0)) return true;
					return GetBit(0);
				}
				else if (State == State_Z)
				{
					State = State_X;
					return GetBit(1);
				}
				
			case CMIN ... CMAX:
				if (GetBit(0)) return true;
				return GetBit(1);
				
			default:
				return true; //not in range - error or pack end
			}
			
		}
		else //waiting for a start
		{
			switch (NfcFifo.LastVal)
			{
			case AMIN ... AMAX:
				State = State_Z;
				NfcFifo.State = Started; //found pack start cond 
				NfcFifo.TimeOut = false; 
				return GetBit(0);
			case BMIN ... BMAX:
				State = State_X;
				NfcFifo.State = Started; //found pack start cond 
				NfcFifo.TimeOut = false; 
				return GetBit(1);
				
			default:
				return true; //not in range - error or pack end
			}
		}
		
		/***************************/
	}
	
	return false;
}

size_t NfcWork()
{
	if (NfcDecode()) // data could be received 
	{
		if (VcdDataLen > 2) //data length must be at least 1 byte + CRC
		{
			
			if (!GPCRC_DataRead(GPCRC)) //crc ok
			{
				return VcdDataLen | ((NfcFifo.State == Manchester) ? 0x8000 : 0);
			}
		}
		NfcDecodeReset();	
	}
	
	return 0;
}


void NfcDecodeReset()
{
	GPCRC_Start(GPCRC);
	
	DecoderByteRx = 0;
	BPos = 0;
	Parity = 0;			
	NfcFifo.State = Ready;
	VcdDataLen = 0; //RX Data Array position/size
	One = false;
	BitInv = 1;
	
}