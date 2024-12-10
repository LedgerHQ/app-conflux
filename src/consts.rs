#![allow(dead_code)]

pub const ADDRRESS_BYTES_LEN: usize = 20;

pub const HASH_BYTES_LEN: usize = 32;

/**
 * Maximum length of MAJOR_VERSION || MINOR_VERSION || PATCH_VERSION.
 */
pub const APPVERSION_LEN: usize = 3;

/**
 * Maximum length of application name.
 */
pub const MAX_APPNAME_LEN: usize = 64;

/**
 * Maximum transaction length (bytes).
 */
pub const MAX_TRANSACTION_LEN: usize = 765;

/**
 * Maximum signature length (bytes).
 */
pub const MAX_DER_SIG_LEN: usize = 72;

/**
 * Exponent used to convert Drip to CFX unit (N CFX = N * 10^18 Drip).
 */
pub const EXPONENT_SMALLEST_UNIT: usize = 18;

/**
 * Well-known Conflux chain IDs.
 */
pub const CONFLUX_MAINNET_CHAINID: usize = 1029;

pub const CONFLUX_TESTNET_CHAINID: usize = 1;

/**
 * Application flags.
 */
pub const APP_FLAG_BLIND_SIGNING_ENABLED: u8 = 0x01;

pub const APP_FLAG_DETAILED_DISPLAY_ENABLED: u8 = 0x02;
