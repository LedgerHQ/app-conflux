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

pub const CHARSET: [char; 32] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'r', 's', 't', 'u', 'v', 'w',
    'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
]; // chars excluding 'oilq'

// network prefix
pub const MAINNET_PREFIX: &str = "cfx";
pub const TESTNET_PREFIX: &str = "cfxtest";

pub const MAIN_NET_ID: u64 = 1029;
pub const TEST_NET_ID: u64 = 1;

// These two network_ids are reserved.
pub const RESERVED_NETWORK_IDS: [u64; 2] = [TEST_NET_ID, MAIN_NET_ID];
