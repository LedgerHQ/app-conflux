use super::consts::{
    MAINNET_PREFIX, MAIN_NET_ID, RESERVED_NETWORK_IDS, TESTNET_PREFIX, TEST_NET_ID,
};
use alloc::{format, string::String};
use core::fmt;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Network {
    /// Main network.
    Main,
    /// Test network.
    Test,
    /// Specific Network Id.
    Id(u64),
}

impl Network {
    pub fn prefix(&self) -> Result<String, EncodingError> {
        match self {
            Network::Main => Ok(MAINNET_PREFIX.into()),
            Network::Test => Ok(TESTNET_PREFIX.into()),
            Network::Id(network_id) => {
                if RESERVED_NETWORK_IDS.contains(network_id) {
                    Err(EncodingError::NetworkId(*network_id))
                } else {
                    Ok(format!("net{}", network_id))
                }
            }
        }
    }

    pub fn from_network_id(network_id: u64) -> Self {
        match network_id {
            MAIN_NET_ID => Self::Main,
            TEST_NET_ID => Self::Test,
            _ => Self::Id(network_id),
        }
    }
}

/// Error concerning encoding of cfx_base32_addr.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EncodingError {
    Length(usize),
    NetworkId(u64),
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Length(length) => {
                write!(f, "invalid length ({})", length)
            }
            Self::NetworkId(network_id) => {
                write!(f, "invalid network_id (reserved: {})", network_id)
            }
        }
    }
}
