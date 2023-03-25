
#include "FT6336.h"
#include <em_chip.h>
#include "utils.h"
 
I2C_TransferSeq_TypeDef i2cTransfert;
static uint8_t i2c_read_data[8];
static uint8_t i2c_write_data[2];

extern volatile uint32_t TickCount;

uint8_t FT6336_ReadId()
{
	i2cTransfert.addr = FT6X36_ADDR << 1; // Chip address << 1
	i2cTransfert.flags = I2C_FLAG_WRITE_READ;
	i2c_write_data[0] = 0xA3;
		
	i2cTransfert.buf[0].data   = i2c_write_data;
	i2cTransfert.buf[0].len    = 1;
		
	i2cTransfert.buf[1].data = i2c_read_data;
	i2cTransfert.buf[1].len  = 1;
		
	// Start I2C read   
	int _status = I2C_TransferInit(I2C0, &i2cTransfert);
	
	uint32_t curTicks;
	curTicks = TickCount;
	
	while (_status == i2cTransferInProgress) 
	{
		_status = I2C_Transfer(I2C0);
		if ((TickCount - curTicks) > 10) break;
	}
	return i2c_read_data[0];
}


uint8_t FT6336_ReadLocation(uint8_t * loc)
{
	i2cTransfert.addr = FT6X36_ADDR << 1; // Chip address << 1
	i2cTransfert.flags = I2C_FLAG_WRITE_READ;
	i2c_write_data[0] = FT6X36_REG_NUM_TOUCHES;
		
	i2cTransfert.buf[0].data   = i2c_write_data;
	i2cTransfert.buf[0].len    = 1;
		
	i2cTransfert.buf[1].data = i2c_read_data;
	i2cTransfert.buf[1].len  = 5;
		
	// Start I2C read   
	
	uint32_t curTicks;
	curTicks = TickCount;
	int _status = I2C_TransferInit(I2C0, &i2cTransfert);
	
	while (_status == i2cTransferInProgress) 
	{
		_status = I2C_Transfer(I2C0);
		if ((TickCount - curTicks) > 10) break;	
	}
	*loc++ = i2c_read_data[2];
	*loc++ = i2c_read_data[3];
	*loc = i2c_read_data[4];
	
	return i2c_read_data[0];
}