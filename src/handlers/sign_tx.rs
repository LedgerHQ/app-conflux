/*****************************************************************************
 *   Ledger App Conflux Rust.
 *   (c) 2023 Conflux Foundation.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *****************************************************************************/
use crate::app_ui::sign::ui_display_tx;
use crate::consts::MAX_TRANSACTION_LEN;
use crate::crypto::decode_der_sig;
use crate::types::Transaction;
use crate::utils::Bip32Path;
use crate::AppSW;
use alloc::vec::Vec;
use ledger_device_sdk::ecc::{Secp256k1, SeedDerive};
use ledger_device_sdk::hash::{sha3::Keccak256, HashInit};
use ledger_device_sdk::io::Comm;
use rlp_decoder::decode;

#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::NbglHomeAndSettings;

pub struct TxContext {
    pub raw_tx: Vec<u8>,
    pub path: Bip32Path,
    pub review_finished: bool,
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    pub home: NbglHomeAndSettings,
}

// Implement constructor for TxInfo with default values
impl TxContext {
    // Constructor
    pub fn new() -> TxContext {
        TxContext {
            raw_tx: Vec::new(),
            path: Default::default(),
            review_finished: false,
            #[cfg(any(target_os = "stax", target_os = "flex"))]
            home: Default::default(),
        }
    }
    // Get review status
    #[allow(dead_code)]
    pub fn finished(&self) -> bool {
        self.review_finished
    }
    // Implement reset for TxInfo
    pub fn reset(&mut self) {
        self.raw_tx.clear();
        self.path = Default::default();
        self.review_finished = false;
    }
}

pub fn handler_sign_tx(
    comm: &mut Comm,
    chunk: u8,
    more: bool,
    ctx: &mut TxContext,
) -> Result<(), AppSW> {
    // Try to get data from comm
    let data = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;
    // First chunk, try to parse the path
    if chunk == 0 {
        // Reset transaction context
        ctx.reset();
        // This will propagate the error if the path is invalid
        ctx.path = data.try_into()?;
        Ok(())
    // Next chunks, append data to raw_tx and return or parse
    // the transaction if it is the last chunk.
    } else {
        if ctx.raw_tx.len() + data.len() > MAX_TRANSACTION_LEN {
            return Err(AppSW::TxWrongLength);
        }

        // Append data to raw_tx
        ctx.raw_tx.extend(data);

        // If we expect more chunks, return
        if more {
            ctx.review_finished = false;
            Ok(())
        // Otherwise, try to parse the transaction
        } else {
            // Try to deserialize the transaction
            let tx: Transaction =
                decode(ctx.raw_tx.as_slice()).map_err(|_| AppSW::TxParsingFail)?;
            // Display transaction. If user approves
            // the transaction, sign it. Otherwise,
            // return a "deny" status word.
            if ui_display_tx(&tx, ctx)? {
                ctx.review_finished = true;
                compute_signature_and_append(comm, ctx)
            } else {
                ctx.review_finished = true;
                Err(AppSW::Deny)
            }
        }
    }
}

fn compute_signature_and_append(comm: &mut Comm, ctx: &mut TxContext) -> Result<(), AppSW> {
    let mut keccak256 = Keccak256::new();
    let mut message_hash: [u8; 32] = [0u8; 32];

    let _ = keccak256.hash(&ctx.raw_tx, &mut message_hash);

    let (sig, siglen, parity) = Secp256k1::derive_from_path(ctx.path.as_ref())
        .deterministic_sign(&message_hash)
        .map_err(|_| AppSW::TxSignFail)?;

    let mut r: [u8; 32] = [0u8; 32];
    let mut s: [u8; 32] = [0u8; 32];

    decode_der_sig(&sig[..siglen as usize], &mut r, &mut s).map_err(|_| AppSW::TxSignFail)?;

    comm.append(&[parity as u8]);
    comm.append(&r);
    comm.append(&s);
    Ok(())
}
