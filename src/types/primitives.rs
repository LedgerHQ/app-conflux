use core::cmp::Ordering;
use core::ops::Deref;
use rlp_decoder::{Decodable, DecoderError, Rlp};
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

impl Decodable for U256 {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder().decode_value(|bytes| {
            if !bytes.is_empty() && bytes[0] == 0 {
                Err(DecoderError::RlpInvalidIndirection)
            } else if bytes.len() <= 32 {
                Ok(U256::from_big_endian(bytes))
            } else {
                Err(DecoderError::RlpIsTooBig)
            }
        })
    }
}

pub const HASH_LENGTH: usize = 32;
pub const ADDRESS_LENGTH: usize = 20;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct H256(pub [u8; HASH_LENGTH]);

impl Decodable for H256 {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder()
            .decode_value(|bytes| match bytes.len().cmp(&(HASH_LENGTH)) {
                Ordering::Less => Err(DecoderError::RlpIsTooShort),
                Ordering::Greater => Err(DecoderError::RlpIsTooBig),
                Ordering::Equal => {
                    let mut t = [0u8; HASH_LENGTH];
                    t.copy_from_slice(bytes);
                    Ok(H256(t))
                }
            })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Address(pub [u8; ADDRESS_LENGTH]);

impl Decodable for Address {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder()
            .decode_value(|bytes| match bytes.len().cmp(&(ADDRESS_LENGTH)) {
                Ordering::Less => Err(DecoderError::RlpIsTooShort),
                Ordering::Greater => Err(DecoderError::RlpIsTooBig),
                Ordering::Equal => {
                    let mut t = [0u8; ADDRESS_LENGTH];
                    t.copy_from_slice(bytes);
                    Ok(Address(t))
                }
            })
    }
}

impl Deref for Address {
    type Target = [u8; ADDRESS_LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
