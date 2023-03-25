/***************************************************************************/ /**
 * @file
 * @brief CPC
 *******************************************************************************
 * # License
 * <b>Copyright 2019 Silicon Laboratories Inc. www.silabs.com</b>
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

#ifndef SLI_CPC_TYPE_H
#define SLI_CPC_TYPE_H

#include "sl_cpc.h"
#include "sl_slist.h"
#include "sl_cpc_config.h"
#include "sli_cpc_timer.h"

#if defined(SL_COMPONENT_CATALOG_PRESENT)
#include "sl_component_catalog.h"
#endif

#if defined(SL_CATALOG_KERNEL_PRESENT)
#include "cmsis_os2.h"
#include "sl_cmsis_os2_common.h"
#endif

#if defined(SL_CATALOG_CPC_SECONDARY_PRESENT)
#define SL_CPC_ON_POLL_PRESENT
#endif

#if (!defined(SL_CATALOG_CPC_SECURITY_PRESENT))
#define SECURITY_ENABLED    (0)
#else
#if (SL_CPC_SECURITY_ENABLED >= 1)
#define SECURITY_ENABLED    (1)
#else
#define SECURITY_ENABLED    (0)
#endif
#endif

#if (!defined(SL_CATALOG_CPC_PRIMARY_PRESENT) && !defined(SL_CATALOG_CPC_SECONDARY_PRESENT))
// This is required for unit testing (assumed that we are unit testing on a secondary)
#define SL_CPC_ON_POLL_PRESENT
#endif

typedef void (*sl_cpc_on_poll_t)(uint8_t endpoint_id, void *arg,
                                 void *poll_data, uint32_t poll_data_length,    // Rx buffer is freed once this on_poll function return
                                 void **reply_data, uint32_t *reply_data_lenght,
                                 void **on_write_complete_arg);

typedef void (*sl_cpc_on_final_t)(uint8_t endpoint_id, void *arg, void *answer, uint32_t answer_lenght);

typedef struct {
  void *on_fnct_arg;
#ifdef SL_CPC_ON_FINAL_PRESENT
  sl_cpc_on_final_t on_final;
#endif
#ifdef SL_CPC_ON_POLL_PRESENT
  sl_cpc_on_poll_t on_poll;
  void *data;   // Anwser
  uint32_t data_length;
#endif
} sl_cpc_poll_final_t;

typedef struct  {
  sl_slist_node_t node;
  sl_slist_node_t node_closing;
  uint8_t id;
  uint8_t flags;
  uint8_t seq;
  uint8_t ack;
  uint8_t configured_tx_window_size;
  uint8_t current_tx_window_space;
  uint8_t frames_count_re_transmit_queue;
  uint8_t packet_re_transmit_count;
  uint32_t re_transmit_timeout;
  uint64_t last_iframe_sent_timestamp;
  uint64_t smoothed_rtt;
  uint64_t rtt_variation;
  sli_cpc_timer_handle_t re_transmit_timer;
  sli_cpc_timer_handle_t close_timer;
  sl_cpc_endpoint_state_t state;
  sl_cpc_poll_final_t poll_final;
  sl_cpc_on_write_completed_t on_iframe_write_completed;
  sl_cpc_on_data_reception_t on_iframe_data_reception;
  void *on_iframe_data_reception_arg;
  sl_cpc_on_write_completed_t on_uframe_write_completed;
  sl_cpc_on_data_reception_t on_uframe_data_reception;
  void *on_uframe_data_reception_arg;
  sl_cpc_on_error_callback_t on_error;
  void *on_error_arg;
  sl_slist_node_t *iframe_receive_queue;
  sl_slist_node_t *uframe_receive_queue;
  sl_slist_node_t *re_transmit_queue;
  sl_slist_node_t *holding_list;
#if (SL_CPC_DEBUG_ENDPOINT_EVENT_COUNTERS == 1)
  sl_cpc_endpoint_debug_counters_t debug_counters;
#endif
#if defined(SL_CATALOG_KERNEL_PRESENT)
  __ALIGNED(4) uint8_t lock_cb[osMutexCbSize];
  osMutexId_t lock;
  osSemaphoreId_t receive_signal;
  bool read_aborted;
#endif
#if (SECURITY_ENABLED >= 1)
  uint32_t frame_counter_rx;
  uint32_t frame_counter_tx;
  bool encrypted;
  bool packets_held_for_security;
#endif
} sl_cpc_endpoint_t;

#endif
