/***************************************************************************//**
 * @file
 * @brief CLI io functions.
 *    All cli input and output will go through these functions.
 * @version x.y.z
 *******************************************************************************
 * # License
 * <b>Copyright 2018 Silicon Laboratories Inc. www.silabs.com</b>
 *******************************************************************************
 *
 * SPDX-License-Identifier: Zlib
 *
 * The licensor of this software is Silicon Laboratories Inc.
 *
 * This software is provided 'as-is', without any express or implied
 * warranty. In no event will the authors be held liable for any damages
 * arising from the use of this software.
 *
 * Permission is granted to anyone to use this software for any purpose,
 * including commercial applications, and to alter it and redistribute it
 * freely, subject to the following restrictions:
 *
 * 1. The origin of this software must not be misrepresented; you must not
 *    claim that you wrote the original software. If you use this software
 *    in a product, an acknowledgment in the product documentation would be
 *    appreciated but is not required.
 * 2. Altered source versions must be plainly marked as such, and must not be
 *    misrepresented as being the original software.
 * 3. This notice may not be removed or altered from any source distribution.
 *
 ******************************************************************************/

#ifndef SLI_CLI_IO_H
#define SLI_CLI_IO_H

#ifdef __cplusplus
extern "C" {
#endif

/***************************************************************************//**
 * @brief
 *   Get a character from the standard I/O.
 *
 * @return
 *   Returns the input character or EOF if no character is available.
 ******************************************************************************/
int sli_cli_io_getchar(void);

/***************************************************************************//**
 * @brief
 *   Put a character to the standard I/O.
 *
 * @param[in] ch
 *   The character that will be output.
 *
 * @return
 *   Returns the character or EOF if an error occurs.
 ******************************************************************************/
int sli_cli_io_putchar(int ch);

/***************************************************************************//**
 * @brief
 *   Print a string to the standard I./O using printf compatible parameters.
 *
 * @param[in] format
 *   A pointer to the string that will be output.
 *
 * @return
 *   Returns 0.
 ******************************************************************************/
int sli_cli_io_printf(const char *format, ...);

#ifdef __cplusplus
}
#endif

#endif // SLI_CLI_IO_H
