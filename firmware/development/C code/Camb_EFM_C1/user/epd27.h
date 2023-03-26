#pragma once


#include "utils.h" 
#include <stdbool.h>
#include "alzlogo.h"



#define X_SIZE 176
#define Y_SIZE 264

#define MIRROR MIRROR_HORIZONTAL

typedef enum {
	MIRROR_NONE       = 0x00,
	MIRROR_HORIZONTAL = 0x01,
	MIRROR_VERTICAL   = 0x02,
	MIRROR_ORIGIN     = 0x03,
} MIRROR_IMAGE;





void EPD_init();
void EPD_cmd(uint8_t Cmd, uint8_t * Data, size_t Size);
void EPD_refresh();
void EPD_down();
void EPD_sendImageNormal(); //uint8_t * data1s, uint8_t * data2s);
void EPD_global2(); //uint8_t * data1s, uint8_t * data2s);
void EPD_cmd8(uint8_t Cmd); 
void EPD_setPoint(uint16_t x1, uint16_t y1, uint16_t colour);
void EPD_clear(uint8_t colour);
void EPD_reset();
void EPD_sendImageFast();
void EPD_putLogo(const uint8_t * logo, uint16_t y, size_t Len);
void EPD_SetRotate(uint16_t r);

void globalUpdate(const uint8_t *data1s, const uint8_t *data2s);

void fastUpdate(unsigned char* fastImgSet[], uint8_t fastImgSize, uint8_t numLoops);
void partialUpdate(const unsigned char* partialImgSet[], uint8_t partialImgConfig[5], long windowSize, uint8_t numLoops);
void Epd_LoadPic(); 
void EPD_WriteScreen_ALL_Fast(const unsigned char *datas);
void EPD_Update_Fast(void);
void EPD_Part_Update(void);
void EPD_WhiteScreen_ALL(const unsigned char *datas);
void EPD_WriteScreen_Buff_Fast(); 
void EPD_WriteScreen_Buff(); 

void EPD_Part_Write(unsigned int x_startA, unsigned int y_startA, unsigned int PART_COLUMN, unsigned int PART_LINE);

void EPD_setResolution(uint16_t x, uint16_t y);

void EPD_SetRAMValue_BaseMap(const unsigned char * datas); 



void EPD_HW_Init(void);
void EPD_HW_Init_Fast(void);
void EPD_HW_Init_GUI(void);

void Epaper_Write_Command(unsigned char cmd);
void Epaper_Write_Data(unsigned char data);
void EPD_DeepSleep(void);
void EPD_WhiteScreen_White(void);

void EPD_Update(void);

void EPD_PartWriteFull();


void SpiWriteArray(uint8_t * Data, size_t Len);
void SpiFillArray(uint8_t Data, size_t Len);

void EPD_line(uint16_t x0, uint16_t y0, uint16_t x1, uint16_t y1, uint16_t c);
void EPD_DrawRectangle(uint16_t x, uint16_t y, uint16_t w, uint16_t h, uint8_t c);
void EPD_DrawFilledRectangle(uint16_t x, uint16_t y, uint16_t w, uint16_t h, uint8_t c);
void EPD_DrawCircle(int16_t x0, int16_t y0, int16_t r, uint8_t c);
void EPD_DrawFilledCircle(int16_t x0, int16_t y0, int16_t r, uint8_t c);

void EPD_GotoXY(uint16_t x, uint16_t y);
void EPD_Text(uint16_t x0, uint16_t y0, char * text, uint16_t textColour, uint16_t backColour);

uint8_t EPD_getCharacter(uint8_t character, uint8_t index);


