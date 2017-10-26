/*
 * Copyright (c) Pavol Rusnak, SatoshiLabs
 *
 * Licensed under TREZOR License
 * see LICENSE file for details
 */

#ifndef __TREZORHAL_TOUCH_H__
#define __TREZORHAL_TOUCH_H__

#include <stdint.h>

#define TOUCH_START 0x00010000
#define TOUCH_MOVE  0x00020000
#define TOUCH_END   0x00040000

void touch_init(void);
uint32_t touch_read(void);
uint32_t touch_click(void);

#endif
