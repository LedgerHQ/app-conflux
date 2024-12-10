pub const CHARSET_SIZE: usize = 32;

pub const RESERVED_BITS_MASK: u8 = 0xf8;

// Because we use a different CHARSET than BCH, it's OK that we disregard all of
// the BITCOIN type bits.
//
// // pub const TYPE_MASK: u8 = 0x78;
// // pub const TYPE_BITCOIN_P2PKH: u8 = 0x00;
// // pub const TYPE_BITCOIN_P2SH: u8 = 0x08;
//
// In Conflux we have so far only one type of account key format. So we try to
// use the 4 type bits differently. In the future we may use them in some
// special transaction scenarios. e.g. A payment code, an address linked to
// off-chain or cross-chain mechanism.

pub const SIZE_MASK: u8 = 0x07;
pub const SIZE_160: u8 = 0x00;

// In Conflux we only have 160 bits hash size, however we keep these unused
// sizes for unit test and compatibility.
pub const SIZE_192: u8 = 0x01;
pub const SIZE_224: u8 = 0x02;
pub const SIZE_256: u8 = 0x03;
pub const SIZE_320: u8 = 0x04;
pub const SIZE_384: u8 = 0x05;
pub const SIZE_448: u8 = 0x06;
pub const SIZE_512: u8 = 0x07;

pub const BASE32_CHARS: &str = "abcdefghijklmnopqrstuvwxyz0123456789";
pub const EXCLUDE_CHARS: [char; 4] = ['o', 'i', 'l', 'q'];
pub const CHARSET: [char; 32] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'r', 's', 't', 'u', 'v', 'w',
    'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
]; // chars excluding 'oilq'

// network prefix
pub const MAINNET_PREFIX: &str = "cfx";
pub const TESTNET_PREFIX: &str = "cfxtest";
pub const NETWORK_ID_PREFIX: &str = "net";

// address types
pub const ADDRESS_TYPE_BUILTIN: &str = "builtin";
pub const ADDRESS_TYPE_CONTRACT: &str = "contract";
pub const ADDRESS_TYPE_NULL: &str = "null";
pub const ADDRESS_TYPE_UNKNOWN: &str = "unknown";
pub const ADDRESS_TYPE_USER: &str = "user";

pub const MAIN_NET_ID: u64 = 1029;
pub const TEST_NET_ID: u64 = 1;

// These two network_ids are reserved.
pub const RESERVED_NETWORK_IDS: [u64; 2] = [TEST_NET_ID, MAIN_NET_ID];
