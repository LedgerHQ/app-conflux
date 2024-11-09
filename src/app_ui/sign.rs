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
use crate::cfx_addr::{cfx_addr_encode, Network};
use crate::types::Transaction;
use crate::AppSW;

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{Field, MultiFieldReview},
};

#[cfg(any(target_os = "stax", target_os = "flex"))]
use crate::settings::Settings;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use include_gif::include_gif;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{Field, NbglGlyph, NbglReview};

use alloc::format;

/// Displays a transaction and returns true if user approved it.
///
/// This method can return [`AppSW::TxDisplayFail`] error if the coin name length is too long.
///
/// # Arguments
///
/// * `tx` - Transaction to be displayed for validation
pub fn ui_display_tx(tx: &Transaction) -> Result<bool, AppSW> {
    let value_str = tx.value.cfx_str().ok_or(AppSW::TxDisplayFail)?;
    let value_with_unit = format!("CFX {}", value_str);
    let network = Network::from_network_id(tx.chain_id);
    let to_str = cfx_addr_encode(&*tx.to, network).map_err(|_e| AppSW::AddrDisplayFail)?;
    let data_str = format!("0x{}", hex::encode(tx.data.clone()).to_uppercase());

    // Define transaction review fields
    let my_fields = [
        Field {
            name: "Amount",
            value: value_with_unit.as_str(),
        },
        Field {
            name: "To",
            value: to_str.as_str(),
        },
        Field {
            name: "Data",
            value: data_str.as_str(),
        },
    ];

    // Create transaction review
    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let my_review = MultiFieldReview::new(
            &my_fields,
            &["Review ", "Transaction"],
            Some(&EYE),
            "Approve",
            Some(&VALIDATE_14),
            "Reject",
            Some(&CROSSMARK),
        );

        Ok(my_review.show())
    }

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
        const CFX: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/cfx_64.gif", NBGL));
        // Create NBGL review. Maximum number of fields and string buffer length can be customised
        // with constant generic parameters of NbglReview. Default values are 32 and 1024 respectively.
        let review: NbglReview = NbglReview::new()
            .titles(
                "Review transaction\nto send CFX",
                "",
                "Sign transaction\nto send CFX",
            )
            .glyph(&CFX);

        // If first setting switch is disabled do not display the transaction data
        let settings: Settings = Default::default();
        if settings.get_element(0) == 0 {
            Ok(review.show(&my_fields[0..2]))
        } else {
            Ok(review.show(&my_fields))
        }
    }
}

pub fn ui_display_msg(msg: &[u8]) -> Result<bool, AppSW> {
    let msg_str = core::str::from_utf8(msg).map_err(|_| AppSW::InvalidData)?;

    // Define message review fields
    let my_fields = [Field {
        name: "Message",
        value: msg_str,
    }];

    // Create message review
    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let my_review = MultiFieldReview::new(
            &my_fields,
            &["Review ", "Message"],
            Some(&EYE),
            "Approve",
            Some(&VALIDATE_14),
            "Reject",
            Some(&CROSSMARK),
        );

        Ok(my_review.show())
    }

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
        const CFX: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/cfx_64.gif", NBGL));
        // Create NBGL review. Maximum number of fields and string buffer length can be customised
        // with constant generic parameters of NbglReview. Default values are 32 and 1024 respectively.
        let review: NbglReview = NbglReview::new()
            .titles(
                "Review transaction\nto send CFX",
                "",
                "Sign transaction\nto send CFX",
            )
            .glyph(&CFX);

        Ok(review.show(&my_fields))
    }
}
