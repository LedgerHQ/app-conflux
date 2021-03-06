#pragma once

#include <stdbool.h>  // bool

#include "common/buffer.h"

/**
 * Handler for SIGN_TX command. If successfully parse BIP32 path
 * and transaction, sign transaction and send APDU response.
 *
 * @see G_context.sign_tx.bip32_path, G_context.sign_tx.raw_transaction,
 * G_context.sign_tx.signature and G_context.sign_tx.v.
 *
 * @param[in,out] cdata
 *   Command data with BIP32 path and raw transaction serialized.
 * @param[in]       first
 *   Whether this is the first APDU chunk or not.
 */
void handler_sign_tx(buffer_t *cdata, bool first);
