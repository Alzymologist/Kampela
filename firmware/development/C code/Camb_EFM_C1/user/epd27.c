
#include "epd27.h" 
//#include "main.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "utils.h"
#include "Terminal12x16e.h"
#include "AT25.h"




//2.7
#define BUFSIZE (X_SIZE >> 3) * Y_SIZE


uint16_t xSize = X_SIZE;
uint16_t ySize = Y_SIZE;

uint16_t BufSize = BUFSIZE;

uint8_t FrameBuffer[BUFSIZE];
uint16_t Rotate = 0;

typedef struct 
{
	uint16_t CurrentX;
	uint16_t CurrentY;
	bool Inverted;
	bool Initialized;
} EPD_t;

static EPD_t edisp;


void Epaper_Write_Command(unsigned char cmd)
{
	DISP_CS(1);
	DISP_CS(0);
	DISP_DC(0);
	USART_SpiTransfer(SPI_PERF, cmd);
	DISP_CS(1);
}

void Epaper_Write_Data(unsigned char data)
{
	DISP_DC(1);
	DISP_CS(1);
	DISP_CS(0);


	USART_SpiTransfer(SPI_PERF, data);
	DISP_CS(1);
}


void SpiWriteArray(uint8_t * Data, size_t Len)
{
	while (Len--)
	{
		USART_SpiTransfer(SPI_PERF, *Data++);
	}
}


void SpiFillArray(uint8_t Data, size_t Len)
{
	DISP_CS(0);
	DISP_DC(1);
	while (Len--)
	{
		USART_SpiTransfer(SPI_PERF, Data);
	}
	DISP_CS(1);
}

void EPD_cmd8(uint8_t Cmd)
{
	DISP_DC(0);
	DISP_CS(0); //CS 0
	USART_SpiTransfer(SPI_PERF, Cmd);
	DISP_CS(1); //CS 1
	DISP_CS(1); //CS 1 again for the sake of God
}


void EPD_cmd(uint8_t Cmd, uint8_t * Data, size_t Size)
{
	
	DISP_DC(0);
	DISP_CS(0);
	USART_SpiTransfer(SPI_PERF, Cmd);
	DISP_CS(1);
	DISP_DC(1);
	
	DISP_CS(0);
	SpiWriteArray(Data, Size);
	DISP_CS(1);
	DISP_CS(1); //CS 1 again for the sake of God
}



void EPD_reset()
{
	Delay(1);
	DISP_RES(0);
	Delay(5);
	DISP_RES(1);
	Delay(10);

	DISP_RES(0);
	Delay(5);
	DISP_CS(1);
	Delay(5);
}




/**********************/
/**********************/
/**********************/
/**********************/
/**********************/
// 200x200 EPD

//SSD1681
void EPD_HW_Init(void)
{
	EPD_reset();
	
	while (DISP_BUSY) ;
	EPD_cmd8(0x12); //SWRESET
	while (DISP_BUSY) ;
		
	
	//	Epaper_Write_Command(0x01); //Driver output control      
	//	Epaper_Write_Data(0xC7);
	//	Epaper_Write_Data(0x00);
	//	Epaper_Write_Data(0x01);
	//
	//	Epaper_Write_Command(0x11); //data entry mode       
	//	Epaper_Write_Data(0x01);
	//
	//	Epaper_Write_Command(0x44); //set Ram-X address start/end position   
	//	Epaper_Write_Data(0x00);
	//	//Epaper_Write_Data(0x18); //0x0C-->(18+1)*8=200
	//	Epaper_Write_Data(0x21); //0x0C-->(18+1)*8=200
	//	
	//	Epaper_Write_Command(0x45); //set Ram-Y address start/end position          
	//	Epaper_Write_Data(0xC7); //0xC7-->(199+1)=200
	//	Epaper_Write_Data(0x00);
	//	Epaper_Write_Data(0x00);
	//	Epaper_Write_Data(0x00); 
	//
	//	Epaper_Write_Command(0x3C); //BorderWavefrom
	//	Epaper_Write_Data(0x05); //05	
	//	  	
	//	Epaper_Write_Command(0x18); //Read built-in temperature sensor
	//	Epaper_Write_Data(0x80);	
	//
	//	Epaper_Write_Command(0x4E); // set RAM x address count to 0;
	//	Epaper_Write_Data(0x00);
	////	Epaper_Write_Command(0x4F); // set RAM y address count to 0X199;    
	////	Epaper_Write_Data(0xC7);
	//
	//	
	//	Epaper_Write_Command(0x00); // set RAM y address count to 0X199;    
	//	Epaper_Write_Data(0x00);
	//
	//	Epaper_Write_Data(0x00);
	//	while (DISP_BUSY) ;
	
	
}
void EPD_HW_Init_Fast(void)
{
	EPD_reset();

	while (DISP_BUSY) ;
	EPD_cmd8(0x12); //SWRESET
	while (DISP_BUSY) ; 
	
	
	 	
	Epaper_Write_Command(0x18); //Read built-in temperature sensor
	Epaper_Write_Data(0x80);	
	  	
	Epaper_Write_Command(0x22); // Load temperature value
	Epaper_Write_Data(0xB1);		
	Epaper_Write_Command(0x20);	
	while (DISP_BUSY) ;   

	Epaper_Write_Command(0x1A); // Write to temperature register
	Epaper_Write_Data(0x64);		
	Epaper_Write_Data(0x00);	
				  	
	Epaper_Write_Command(0x22); // Load temperature value
	Epaper_Write_Data(0x91);		
	Epaper_Write_Command(0x20);	
	while (DISP_BUSY) ; 
	

	//	Epaper_Write_Command(0x01); //Driver output control      
	//	Epaper_Write_Data(0xC7);
	//	Epaper_Write_Data(0x00);
	//	Epaper_Write_Data(0x01); //01
	//	
	//	
	//	Epaper_Write_Command(0x18); //Read built-in temperature sensor
	//	Epaper_Write_Data(0x80);	
	//	
	//	Epaper_Write_Command(0x22); // Load temperature value
	//	Epaper_Write_Data(0xB1);		
	//	Epaper_Write_Command(0x20);	
	//	
	//	while (DISP_BUSY) ;
	//	Epaper_Write_Command(0x1A); // Write to temperature register
	//	Epaper_Write_Data(0x64);		
	//	Epaper_Write_Data(0x00);	
	//				  	
	//	Epaper_Write_Command(0x22); // Load temperature value
	//	Epaper_Write_Data(0x91);		
	//	Epaper_Write_Command(0x20);	
	//	while (DISP_BUSY) ;
	//	

}

void EPD_DeepSleep(void)
{  	
	Epaper_Write_Command(0x10); //enter deep sleep
	Epaper_Write_Data(0x01); 
	Delay(100);
}

void EPD_WhiteScreen_White(void)

{
	Epaper_Write_Command(0x24); //write RAM for black(0)/white (1)
	SpiFillArray(0xFF, sizeof(FrameBuffer)); 
	
	Epaper_Write_Command(0x26); //write RAM for black(0)/white (1)
	SpiFillArray(0xFF, sizeof(FrameBuffer));
	
	EPD_Update();
}


void EPD_WhiteScreen_ALL(const unsigned char *datas)
{
	unsigned int i;	
	const uint8_t *p = datas;
	
	Epaper_Write_Command(0x24); //write RAM for black(0)/white (1)
	for (i = 0; i < sizeof(FrameBuffer); i++)
	{               
		Epaper_Write_Data(*datas);
		datas++;
	} 
	EPD_Update();	 
}

void EPD_WriteScreen_ALL_Fast(const unsigned char *datas)
{
	unsigned int i;   
	
	const uint8_t *p = datas;
	Epaper_Write_Command(0x24); //Write Black and White image to RAM
	for (i = 0; i < sizeof(FrameBuffer); i++)
	{               
		Epaper_Write_Data(*datas);
		datas++;
	}
	
//	Epaper_Write_Command(0x26); //Write Black and White image to RAM
//	for (i = 0; i < sizeof(FrameBuffer); i++)
//	{               
//		Epaper_Write_Data(*p);
//		p++;
//	}
	
	EPD_Update_Fast();	 
}


void EPD_WriteScreen_Buff_Fast()
{
	unsigned int i;	
	Epaper_Write_Command(0x24); //write RAM for black(0)/white (1)
	for (i = 0; i < sizeof(FrameBuffer); i++)
	{               
		Epaper_Write_Data(FrameBuffer[i]);
	} 
	 
//	Epaper_Write_Command(0x26); //write RAM for black(0)/white (1)
//	for (i = 0; i < sizeof(FrameBuffer); i++)
//	{               
//		Epaper_Write_Data(FrameBuffer[i]);
//	} 
	
	//EPD_Part_Update();
	EPD_Update_Fast();	 
}

void EPD_WriteScreen_Buff()
{
	unsigned int i;	
	Epaper_Write_Command(0x24); //write RAM for black(0)/white (1)
	for (i = 0; i < sizeof(FrameBuffer); i++)
	{               
		Epaper_Write_Data(FrameBuffer[i]);
	} 
	 
//	Epaper_Write_Command(0x26); //write RAM for black(0)/white (1)
//	for (i = 0; i < sizeof(FrameBuffer); i++)
//	{               
//		Epaper_Write_Data(FrameBuffer[i]);
//	} 
	
	EPD_Update();	 
}


void EPD_Update(void)
{   
	Epaper_Write_Command(0x22); //Display Update Control
	Epaper_Write_Data(0xF7);   
	Epaper_Write_Command(0x20); //Activate Display Update Sequence
	while (DISP_BUSY) ;
}

void EPD_Update_Fast(void)
{   
	Epaper_Write_Command(0x22); //Display Update Control
	Epaper_Write_Data(0xC7); //C7
	Epaper_Write_Command(0x20); //Activate Display Update Sequence
	Delay(1);
	while (DISP_BUSY) ;  

}
/*When the electronic paper screen is updated, do not unplug the electronic paper to avoid damage to the screen*/
void EPD_Part_Update(void)
{
	Epaper_Write_Command(0x22); //Display Update Control
	Epaper_Write_Data(0xFF);   
	Epaper_Write_Command(0x20); //Activate Display Update Sequence
	Delay(1);
	while (DISP_BUSY) ;			
	
}


void EPD_putLogo(const uint8_t * logo, uint16_t y, size_t Len)
{
	for (uint16_t i = 0; i < Len; i++)
	{
		FrameBuffer[i + y * 22] = *logo++;
	}
}



/******************/
//part update

void EPD_Part_Write(unsigned int x_startA, unsigned int y_startA, unsigned int PART_COLUMN, unsigned int PART_LINE)
{

	uint16_t i;
	uint8_t xStart = x_startA;
	uint8_t xStop = PART_COLUMN - 1 + xStart;
	uint16_t BufSize = PART_COLUMN * PART_LINE;
	
	
	// Add hardware reset to prevent background color change
	EPD_reset();	
	//Lock the border to prevent flashing
	Epaper_Write_Command(0x3C); //BorderWavefrom,
	Epaper_Write_Data(0x80);	
	
	
	
	
	//	Epaper_Write_Command(0x44); // set RAM x address start/end, in page 35
	//	Epaper_Write_Data(0); // RAM x address start at 00h;
	//	Epaper_Write_Data(22); // RAM x address end at 0fh(15+1)*8->128 
	//	Epaper_Write_Command(0x45); // set RAM y address start/end, in page 35
	//	Epaper_Write_Data(0); // RAM y address start at 0127h;
	//	Epaper_Write_Data(0); // RAM y address start at 0127h;
	//	Epaper_Write_Data(0); // RAM y address end at 00h;
	//	Epaper_Write_Data(8); // ????=0	


	
	
	
	
	//	Epaper_Write_Command(0x01); //Driver output control      
	//	Epaper_Write_Data(0xC7);
	//	Epaper_Write_Data(0x00);
	//	Epaper_Write_Data(0x01); //01
	
	//	Epaper_Write_Command(0x44); // set RAM x address start/end, in page 35
	//	Epaper_Write_Data(xStart); //(x_startA); // RAM x address start at 00h;
	//	Epaper_Write_Data(xStop); //(x_end); // RAM x address end at 0fh(15+1)*8->128 
	//	
	//	Epaper_Write_Command(0x45); // set RAM y address start/end, in page 35
	//	Epaper_Write_Data(0);//(y_start2); // RAM y address start at 0127h;
	//	Epaper_Write_Data(0);//(y_start1); // RAM y address start at 0127h;
	//	
	//	Epaper_Write_Data(0);//(y_end2); // RAM y address end at 00h;
	//	Epaper_Write_Data(0);//(y_end1);    
	//
	//	Epaper_Write_Command(0x4E); // set RAM x address count to 0;
	//	Epaper_Write_Data(xStart); //x_startA); 
	//	
	//	Epaper_Write_Command(0x4F); // set RAM y address count to 0X127;    
	//	Epaper_Write_Data(199 - y_startA); //y_start2);
	//	Epaper_Write_Data(0);//y_start1);
	
	
	Epaper_Write_Command(0x24); //Write Black and White image to RAM
	//for (i = 0; i < PART_COLUMN*PART_LINE / 8; i++)
		
	DISP_CS(0);
	DISP_DC(1);

	SpiWriteArray(FrameBuffer, BUFSIZE);
	DISP_CS(1);
	
	
	EPD_Part_Update();

}



void EPD_PartWriteFull()
{
	Epaper_Write_Command(0x24); //Write Black and White image to RAM

	DISP_CS(0);
	DISP_DC(1);
	SpiWriteArray(FrameBuffer, BUFSIZE);
	DISP_CS(1);
	
	
	EPD_Part_Update();
}




/*******************/
/*******************/
/*******************/
/*******************/
/*******************/
/*******************/




void EPD_clear(uint8_t colour)
{
	if (colour) colour = 0xFF;
	else
	{
		colour = 0;
	}
	memset(FrameBuffer, colour, sizeof(FrameBuffer));
	
}

/***** Text ******/

// 1.54
/*
void EPD_Text(uint16_t x0, uint16_t y0, char * text, uint16_t textColour, uint16_t backColour)
{
	uint8_t c;
	uint8_t line, line1, line2, line3;
	uint16_t x, y;
	uint8_t i, j, k;
	size_t StrLen = strlen(text);
	bool _f_fontSolid = true; 
	
	if (textColour == backColour)
	{
		_f_fontSolid = false;
		if (textColour) textColour = false;
		else textColour = true;
	}
	

	for (k = 0; k < StrLen; k++)
	{
		c = text[k] - ' '; 
		
		for (i = 0; i < 12; i++)
		{
			line = EPD_getCharacter(c, 2 * i);
			line1 = EPD_getCharacter(c, 2 * i + 1);

			for (j = 0; j < 8; j++)
			{
				if (bitRead(line, j))
				{
					EPD_setPoint(y0 + j, x0 + 12 * k + i, textColour);
				}
				else if (_f_fontSolid)
				{
					EPD_setPoint(y0 + j, x0 + 12 * k + i, backColour);
				}
				
				if (bitRead(line1, j))
				{
					EPD_setPoint(y0 + 8 + j, x0 + 12 * k + i, textColour);
				}
				else if (_f_fontSolid)
				{
					EPD_setPoint(y0 + 8 + j, x0 + 12 * k + i, backColour);
				}
			}
		}
	}

}
*/


void EPD_Text(uint16_t x0, uint16_t y0, char * text, uint16_t textColour, uint16_t backColour)
{
	uint8_t c;
	uint8_t line, line1, line2, line3;
	uint16_t x, y;
	uint8_t i, j, k;
	size_t StrLen = strlen(text);
	bool _f_fontSolid = true; 
	
	if (textColour == backColour)
	{
		_f_fontSolid = false;
		if (textColour) textColour = false;
		else textColour = true;
	}
	

	for (k = 0; k < StrLen; k++)
	{
		c = text[k] - ' '; 
		
		for (i = 0; i < 12; i++)
		{
			line = EPD_getCharacter(c, 2 * i);
			line1 = EPD_getCharacter(c, 2 * i + 1);

			for (j = 0; j < 8; j++)
			{
				if (bitRead(line, j))
				{
					EPD_setPoint(x0 + 12 * k + i, y0 + j, textColour);
				}
				else if (_f_fontSolid)
				{
					EPD_setPoint(x0 + 12 * k + i, y0 + j, backColour);
				}
				
				if (bitRead(line1, j))
				{
					EPD_setPoint(x0 + 12 * k + i, y0 + 8 + j, textColour);
				}
				else if (_f_fontSolid)
				{
					EPD_setPoint(x0 + 12 * k + i, y0 + 8 + j, backColour);
				}
			}
		}
	}


}


uint8_t EPD_getCharacter(uint8_t character, uint8_t index)
{
	return Terminal12x16e[character][index];
	//return Terminal6x8e[character][index];
}


void EPD_setResolution(uint16_t x, uint16_t y)
{
	if (!x)
	{
		xSize = X_SIZE;
		ySize = Y_SIZE;
		return;
	}
	xSize = x;
	ySize = y;
}


/*  Graphics */
/***** Point *****/
void EPD_SetRotate(uint16_t r)
{
	Rotate = r;
}

void EPD_setPoint(uint16_t x1, uint16_t y1, uint16_t colour)
{
	
	
	
	uint16_t X, Y;
	uint16_t Addr;
	uint8_t Rdata;
	uint16_t WidthByte = (X_SIZE % 8 == 0) ? (X_SIZE / 8) : (X_SIZE / 8 + 1);
	
	
	//	if (x1 > X_SIZE || y1 > Y_SIZE) {
	//
	//		return;
	//	}      
    

	switch (Rotate) {
	case 0:
		X = x1;
		Y = y1;  
		break;
	case 90:
		X = X_SIZE - y1 - 1;
		Y = x1;
		break;
	case 180:
		X = X_SIZE - x1 - 1;
		Y = Y_SIZE - y1 - 1;
		break;
	case 270:
		X = y1;
		Y = Y_SIZE - x1 - 1;
		break;
		
	default:
		return;
	}
    
	switch (MIRROR) {
	case MIRROR_NONE:
		break;
	case MIRROR_HORIZONTAL:
		X = X_SIZE - X - 1;
		break;
	case MIRROR_VERTICAL:
		Y = Y_SIZE - Y - 1;
		break;
	case MIRROR_ORIGIN:
		X = X_SIZE - X - 1;
		Y = Y_SIZE - Y - 1;
		break;
	default:
		return;
	}

	if (X > X_SIZE || Y > Y_SIZE) {
		return;
	}
    
	Addr = X / 8 + Y * WidthByte;
	Rdata = FrameBuffer[Addr];
	if (colour == 1)
		FrameBuffer[Addr] = Rdata & ~(0x80 >> (X % 8));
	else
		FrameBuffer[Addr] = Rdata | (0x80 >> (X % 8));
	
	
	
	
	
	
	
	
	
	//	if (x1 > xSize) x1 = xSize;
	//	if (y1 > ySize) y1 = ySize;
	//	uint8_t xBytes = xSize / 8;
	//	
	//	uint32_t z1 = (uint32_t)(x1>>3) * xBytes + y1;
	//
	//	
	//	// Basic colours
	//	if (colour == 1) //black
	//	{
	//		// physical black 00
	//		bitClear(FrameBuffer[z1], 7 - (y1 % 8));
	//	}
	//	else if(colour == 0)//white
	//	{
	//		// physical white 1
	//		bitSet(FrameBuffer[z1], 7 - (y1 % 8));
	//	}
}



/***** Line *****/
void EPD_line(uint16_t x0, uint16_t y0, uint16_t x1, uint16_t y1, uint16_t c)
{
	int16_t dx, dy, sx, sy, err, e2, i, tmp;

	dx = (x0 < x1) ? (x1 - x0) : (x0 - x1);
	dy = (y0 < y1) ? (y1 - y0) : (y0 - y1);
	sx = (x0 < x1) ? 1 : -1;
	sy = (y0 < y1) ? 1 : -1;
	err = ((dx > dy) ? dx : -dy) / 2;

	if (dx == 0) {
		if (y1 < y0) {
			tmp = y1;
			y1 = y0;
			y0 = tmp;
		}

		if (x1 < x0) {
			tmp = x1;
			x1 = x0;
			x0 = tmp;
		}

		/* Vertical line */
		for (i = y0; i <= y1; i++) {
			EPD_setPoint(x0, i, c);
		}

		/* Return from function */
		return;
	}

	if (dy == 0) {
		if (y1 < y0) {
			tmp = y1;
			y1 = y0;
			y0 = tmp;
		}

		if (x1 < x0) {
			tmp = x1;
			x1 = x0;
			x0 = tmp;
		}

		/* Horizontal line */
		for (i = x0; i <= x1; i++) {
			EPD_setPoint(i, y0, c);
		}

		/* Return from function */
		return;
	}

	while (1) {
		EPD_setPoint(x0, y0, c);
		if (x0 == x1 && y0 == y1) {
			break;
		}
		e2 = err;
		if (e2 > -dx) {
			err -= dy;
			x0 += sx;
		}
		if (e2 < dy) {
			err += dx;
			y0 += sy;
		}
	}	
}

void EPD_DrawRectangle(uint16_t x, uint16_t y, uint16_t w, uint16_t h, uint8_t c) 
{
	/* Check input parameters */
	if (
		x >= X_SIZE ||
		y >= Y_SIZE) {
		/* Return error */
		return;
	}

	/* Check width and height */
	if ((x + w) >= X_SIZE) {
		w = X_SIZE - x;
	}
	if ((y + h) >= Y_SIZE) {
		h = Y_SIZE - y;
	}

	/* Draw 4 lines */
	EPD_line(x, y, x + w, y, c); /* Top line */
	EPD_line(x, y + h, x + w, y + h, c); /* Bottom line */
	EPD_line(x, y, x, y + h, c); /* Left line */
	EPD_line(x + w, y, x + w, y + h, c); /* Right line */
}

void EPD_DrawFilledRectangle(uint16_t x, uint16_t y, uint16_t w, uint16_t h, uint8_t c) 
{
	uint8_t i;

	/* Check input parameters */
	if (
		x >= X_SIZE ||
		y >= Y_SIZE) {
		/* Return error */
		return;
	}

	/* Check width and height */
	if ((x + w) >= X_SIZE) {
		w = X_SIZE - x;
	}
	if ((y + h) >= Y_SIZE) {
		h = Y_SIZE - y;
	}

	/* Draw lines */
	for (i = 0; i <= h; i++) {
		/* Draw lines */
		EPD_line(x, y + i, x + w, y + i, c);
	}
}


void EPD_DrawCircle(int16_t x0, int16_t y0, int16_t r, uint8_t c) 
{
	int16_t f = 1 - r;
	int16_t ddF_x = 1;
	int16_t ddF_y = -2 * r;
	int16_t x = 0;
	int16_t y = r;

	EPD_setPoint(x0, y0 + r, c);
	EPD_setPoint(x0, y0 - r, c);
	EPD_setPoint(x0 + r, y0, c);
	EPD_setPoint(x0 - r, y0, c);

	while (x < y) {
		if (f >= 0) {
			y--;
			ddF_y += 2;
			f += ddF_y;
		}
		x++;
		ddF_x += 2;
		f += ddF_x;

		EPD_setPoint(x0 + x, y0 + y, c);
		EPD_setPoint(x0 - x, y0 + y, c);
		EPD_setPoint(x0 + x, y0 - y, c);
		EPD_setPoint(x0 - x, y0 - y, c);

		EPD_setPoint(x0 + y, y0 + x, c);
		EPD_setPoint(x0 - y, y0 + x, c);
		EPD_setPoint(x0 + y, y0 - x, c);
		EPD_setPoint(x0 - y, y0 - x, c);
	}
}

void EPD_DrawFilledCircle(int16_t x0, int16_t y0, int16_t r, uint8_t c) 
{
	int16_t f = 1 - r;
	int16_t ddF_x = 1;
	int16_t ddF_y = -2 * r;
	int16_t x = 0;
	int16_t y = r;

	EPD_setPoint(x0, y0 + r, c);
	EPD_setPoint(x0, y0 - r, c);
	EPD_setPoint(x0 + r, y0, c);
	EPD_setPoint(x0 - r, y0, c);
	EPD_line(x0 - r, y0, x0 + r, y0, c);

	while (x < y) {
		if (f >= 0) {
			y--;
			ddF_y += 2;
			f += ddF_y;
		}
		x++;
		ddF_x += 2;
		f += ddF_x;

		EPD_line(x0 - x, y0 + y, x0 + x, y0 + y, c);
		EPD_line(x0 + x, y0 - y, x0 - x, y0 - y, c);

		EPD_line(x0 + y, y0 + x, x0 - y, y0 + x, c);
		EPD_line(x0 + y, y0 - x, x0 - y, y0 - x, c);
	}
}