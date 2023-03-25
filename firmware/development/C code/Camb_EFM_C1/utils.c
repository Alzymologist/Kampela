


#include "utils.h"
#include <em_i2c.h>


#include <stdio.h>
#include "em_device.h"
#include "em_chip.h"
#include "em_i2c.h"
#include "em_cmu.h"
#include "em_emu.h"
#include "em_gpio.h"
#include "em_acmp.h"


#include "em_ldma.h"

#include "dmadrv.h" 
#include "config\dmadrv_config.h"


#define TX_BUFFER_SIZE		2048

// Declare init structs
IADC_Init_t init = IADC_INIT_DEFAULT;
IADC_AllConfigs_t initAllConfigs = IADC_ALLCONFIGS_DEFAULT;
IADC_InitSingle_t initSingle = IADC_INITSINGLE_DEFAULT;
IADC_SingleInput_t initSingleInput = IADC_SINGLEINPUT_DEFAULT;

volatile IADC_Result_t sample;
volatile double MainVoltage;

volatile uint32_t TickCount = 0;


char TxBuffer[TX_BUFFER_SIZE];
bool TxBusy;
unsigned int channelTX = 0;

char buff[256];
size_t Head = 0, Tail = 0, Bts = 0;

#define NORM(x) x &= (TX_BUFFER_SIZE - 1)

bool UartTxComplete = false;




void swap(uint16_t a, uint16_t b)
{
	uint16_t w = a;
	a = b;
	b = w;
}


void IADC_IRQHandler(void)
{
	static double coe = 0.00002110f; //individual calibration required
	
	// Read most recent single conversion result
	sample = IADC_readSingleResult(IADC0);
	// For single-ended the result range is 0 to +Vref, i.e., 20 bits for the
	// conversion value.
	MainVoltage = (double)sample.data * coe; //2.42 / 0xFFFFF;
	IADC_clearInt(IADC0, IADC_IF_SINGLEDONE);
}





void UARTTxTransferComplete(unsigned int channel, bool primary, void *user)
{
	UartTxComplete = true;

}



void ClockInit()
{
	
	
	EMU_DCDCInit_TypeDef dcdcInit = EMU_DCDCINIT_DEFAULT;
	CMU_HFXOInit_TypeDef hfxoInit = CMU_HFXOINIT_DEFAULT;
	CMU_LFXOInit_TypeDef lfxoInit = CMU_LFXOINIT_DEFAULT;
	CMU_DPLLInit_TypeDef dpllInit = CMU_DPLL_HFXO_TO_80MHZ;
	
	//EMU_DCDCInit(&dcdcInit);

	hfxoInit.mode = cmuHfxoOscMode_Crystal;
	lfxoInit.mode = cmuLfxoOscMode_Crystal;

	CMU_HFXOInit(&hfxoInit);
	CMU_LFXOInit(&lfxoInit);

	
	
//	bool dpllLock = false;
//	while (!dpllLock)
//	{
//		dpllLock = CMU_DPLLLock(&dpllInit);
//	}


	//CMU_ClockDivSet(cmuClock_CORE, 1);	

	//CMU_ClockSelectSet(cmuClock_SYSCLK, cmuSelect_HFRCODPLL);
	CMU_ClockSelectSet(cmuClock_SYSCLK, cmuSelect_HFXO);
	SysTick_Config(CMU_ClockFreqGet(cmuClock_CORE) / 1000);
	
	CMU_ClockEnable(cmuClock_SEMAILBOX, true); //TRNG

	
	
	
	
}



void SpiDispInit()
{
	
	CMU_ClockEnable(cmuClock_GPIO, true);
	//2V8
	GPIO_PinModeSet(POW_EN_PORT, POW_EN_PIN, gpioModePushPull, 0); //2V8 EN
	
	//I2C level translator Pow
	GPIO_PinModeSet(I2CP_PORT, I2CP_PIN, gpioModePushPull, 0); 
	

	//TOUCH
	GPIO_PinModeSet(INT_PORT, INT_PIN, gpioModeInput, 1); //INT 
	
	NVIC_EnableIRQ(GPIO_ODD_IRQn);
	//NVIC_EnableIRQ(GPIO_EVEN_IRQn);
	
	GPIO_IntConfig(INT_PORT, INT_PIN, false, true, true);

	
	NVIC_SetPriority(SysTick_IRQn, 0); 
	
	NVIC_SetPriority(GPIO_ODD_IRQn, 3); 
//	NVIC_SetPriority(GPIO_EVEN_IRQn, 1); 
	
	
	//touch I2C
	CMU_ClockEnable(cmuClock_I2C0, true);
	GPIO_PinModeSet(SCL_PORT, SCL_PIN, gpioModeWiredAndPullUp, 1);
	GPIO_PinModeSet(SDA_PORT, SDA_PIN, gpioModeWiredAndPullUp, 1); 
	
	
	GPIO->I2CROUTE[0].ROUTEEN = GPIO_I2C_ROUTEEN_SDAPEN | GPIO_I2C_ROUTEEN_SCLPEN;
	GPIO->I2CROUTE[0].SCLROUTE = 
		(SCL_PORT << _GPIO_I2C_SCLROUTE_PORT_SHIFT) | (SCL_PIN << _GPIO_I2C_SCLROUTE_PIN_SHIFT);

	GPIO->I2CROUTE[0].SDAROUTE = 
		(SDA_PORT << _GPIO_I2C_SDAROUTE_PORT_SHIFT) | (SDA_PIN << _GPIO_I2C_SDAROUTE_PIN_SHIFT);
	
	I2C_Init_TypeDef i2cInit = I2C_INIT_DEFAULT;
	//i2cInit.freq = I2C_FREQ_FAST_MAX;
	
	I2C_Init(I2C0, &i2cInit);
	//I2C_BusFreqSet(I2C0, 0, I2C_FREQ_FASTPLUS_MAX, i2cClockHLRAsymetric);
	
}

void UartInit()
{
	CMU_ClockEnable(cmuClock_GPIO, true);
	
	
	CMU_ClockEnable(cmuClock_EUSART1, true);

	
	EUSART_UartInit_TypeDef  init_UART = EUSART_UART_INIT_DEFAULT_HF;
	GPIO_PinModeSet(UART_TX_PORT, UART_TX_PIN, gpioModePushPull, 1); //UART TX
//	GPIO_PinModeSet(UART_RX_PORT, UART_RX_PIN, gpioModeInputPull, 1); //UART RX
	
	GPIO->EUSARTROUTE[UART_PERF_NO].TXROUTE =
		(UART_TX_PORT << _GPIO_EUSART_TXROUTE_PORT_SHIFT) | (UART_TX_PIN << _GPIO_EUSART_TXROUTE_PIN_SHIFT);
	
//	GPIO->EUSARTROUTE[UART_PERF_NO].RXROUTE =	
//		(UART_RX_PORT << _GPIO_EUSART_RXROUTE_PORT_SHIFT) | (UART_RX_PIN << _GPIO_EUSART_RXROUTE_PIN_SHIFT);

	GPIO->EUSARTROUTE[UART_PERF_NO].ROUTEEN = GPIO_EUSART_ROUTEEN_TXPEN;// | GPIO_EUSART_ROUTEEN_RXPEN;
	
	init_UART.baudrate = 115200;
	//init_UART.baudrate = 460800;
	EUSART_UartInitHf(UART_PERF, &init_UART);
	
	NVIC_SetPriority(EUSART1_TX_IRQn, 1); 

}


bool TxCmpl()
{
	CORE_DECLARE_IRQ_STATE;
	
	CORE_ENTER_CRITICAL();
	Tail += Bts;
	NORM(Tail);
	TxBusy = false;
	DMADRV_FreeChannel(channelTX);
	CORE_EXIT_CRITICAL();
	
	UartFifoSend();
	
	return false;
	
}

//DMA TX

void UartFifoSend()
{	
	CORE_DECLARE_IRQ_STATE;
	
	CORE_ENTER_CRITICAL();
	if (Head == Tail || TxBusy) 
	{
		CORE_EXIT_CRITICAL();
		return;
	}
	
	static size_t Top;
	TxBusy = true;
	
	NORM(Head);
	NORM(Tail);
	Bts = Head - Tail;
	NORM(Bts);

	Top = TX_BUFFER_SIZE - Tail;

	
	if (Bts > Top) //if DMA should turn
	{
		Bts = Top;
	}
	NORM(Bts);
	CORE_EXIT_CRITICAL();

	
	Ecode_t res;


	res = DMADRV_AllocateChannel(&channelTX, NULL);
	
	if (res == ECODE_OK)
	{
		res = DMADRV_MemoryPeripheral(channelTX,
			dmadrvPeripheralSignal_EUSART1_TXBL,
			(void*)&(EUSART1->TXDATA),
			TxBuffer + Tail,
			true,
			Bts,
			dmadrvDataSize1,
			TxCmpl,
			NULL);
	}


}

void uPrintf(const char *fmt, ...)
{
	CORE_DECLARE_IRQ_STATE;
	uint8_t *p = (uint8_t *)buff;
	static size_t bts;
	static size_t Free;
	
	va_list args;
	va_start(args, fmt);
	bts = vsnprintf(buff, sizeof(buff), fmt, args);
	va_end(args);

	CORE_ENTER_CRITICAL();
	NORM(Head);
	NORM(Tail);
	
	Free = Tail - Head - 1;
	
	NORM(Free);
	CORE_EXIT_CRITICAL();
	
	while (bts > Free)
	{

		while (TxBusy)
		{
	         
		}
		CORE_ENTER_CRITICAL();
		NORM(Head);
		NORM(Tail);
	
		Free = Tail - Head - 1;
		NORM(Free);
		CORE_EXIT_CRITICAL();
		
	}
	
	CORE_ENTER_CRITICAL();
	while (bts--)
	{
		TxBuffer[Head++] = *p++;
		NORM(Head);
		if (Head == Tail)
		{
			Head--;
			NORM(Head);
		}
	}
	CORE_EXIT_CRITICAL();
	UartFifoSend();
}



void Delay(uint32_t delayInTicks)
{
	uint32_t curTicks;
	curTicks = TickCount;
	while ((TickCount - curTicks) < delayInTicks) ;
} 


void SysTick_Handler(void)
{
	TickCount++; 
}


void AdcInit()
{
	
	// Enable IADC clock
	CMU_ClockEnable(cmuClock_IADC0, true);

	// Reset IADC to reset configuration in case it has been modified
	IADC_reset(IADC0);

	// Configure IADC clock source for use while in EM2
	CMU_ClockSelectSet(cmuClock_IADCCLK, cmuSelect_FSRCO);

	// Modify init structs and initialize
	init.warmup = iadcWarmupKeepWarm;

	// Set the HFSCLK prescale value here
	init.srcClkPrescale = IADC_calcSrcClkPrescale(IADC0, CLK_SRC_ADC_FREQ, 0);

	// Configuration 0 is used by both scan and single conversions by default
	// Use internal bandgap (supply voltage in mV) as reference
	initAllConfigs.configs[0].reference = iadcCfgReferenceInt1V2;
	initAllConfigs.configs[0].vRef = 1210;
	initAllConfigs.configs[0].analogGain = iadcCfgAnalogGain0P5x;

	// Divides CLK_SRC_ADC to set the CLK_ADC frequency for desired sample rate
	initAllConfigs.configs[0].adcClkPrescale = IADC_calcAdcClkPrescale(IADC0,
		CLK_ADC_FREQ,
		0,
		iadcCfgModeNormal,
		init.srcClkPrescale);

	// Set oversampling rate to 32x; digital averaging to 16x
	// resolution formula res = 11 + log2(oversampling * digital averaging)
	// in this case res = 11 + log2(32 * 16) = 20
	initAllConfigs.configs[0].osrHighSpeed = iadcCfgOsrHighSpeed32x;
	initAllConfigs.configs[0].digAvg = iadcDigitalAverage16;

	// Single initialization
	initSingle.dataValidLevel = iadcFifoCfgDvl1;

	// Set conversions to run continuously
	initSingle.triggerAction = iadcTriggerActionContinuous;

	// Set alignment to right justified with 20 bits for data field
	initSingle.alignment = iadcAlignRight20;

	// Configure Input sources for single ended conversion
	initSingleInput.posInput = IADC_INPUT_0_PORT_PIN;
	initSingleInput.negInput = iadcNegInputGnd;

	// Initialize IADC
	// Note oversampling and digital averaging will affect the offset correction
	// This is taken care of in the IADC_init() function in the emlib
	IADC_init(IADC0, &init, &initAllConfigs);

	// Initialize Scan
	IADC_initSingle(IADC0, &initSingle, &initSingleInput);

	// Allocate the analog bus for ADC0 inputs
	GPIO->IADC_INPUT_0_BUS |= IADC_INPUT_0_BUSALLOC;

	// Enable interrupts on data valid level
	IADC_enableInt(IADC0, IADC_IEN_SINGLEDONE);

	// Enable ADC interrupts
	NVIC_ClearPendingIRQ(IADC_IRQn);
	NVIC_EnableIRQ(IADC_IRQn);
	
	IADC_command(IADC0, iadcCmdStartSingle);
	
	EMU->CTRL_SET = EMU_CTRL_EM2DBGEN;
	
	
	
	
	
	
	

}



//Get random data from hardware TRNG.

uint32_t EfmGetRandom()
{
	static uint32_t r;

	SE_Command_t *se_cmd = &cmd_ctx.command;
	
	
	cmd_ctx.command.command = SLI_SE_COMMAND_TRNG_GET_RANDOM; 
	cmd_ctx.command.data_in = NULL; 
	cmd_ctx.command.data_out = NULL;
	cmd_ctx.command.num_parameters = 0;


	SE_DataTransfer_t data_out = SE_DATATRANSFER_DEFAULT(&r, 4);
	SE_addDataOutput(se_cmd, &data_out);
	SE_addParameter(se_cmd, 1);
		

	SE_executeCommand(se_cmd);
	SE_readCommandResponse();

	
	return r;
}




