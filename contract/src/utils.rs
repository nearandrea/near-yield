use near_sdk::{Gas};

// Define the default message
pub const DEFAULT_MESSAGE: &str = "Hello";
pub const DEFAULT_OWNER: &str = "andreapn1709.testnet";
pub const REF_CONTRACT: &str = "ref-finance-101.testnet";
pub const REF_FARM_CONTRACT: &str = "boostfarm.ref-finance.testnet";
pub const DEPOSIT_YOCTO: u128 = 1;
pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

// Define some gas
pub const FT_CALL_35_TGAS: Gas = Gas(35_000_000_000_000);
pub const FT_CALL_45_TGAS: Gas = Gas(45_000_000_000_000);
pub const FT_CALL_60_TGAS: Gas = Gas(60_000_000_000_000);
pub const ZERO: u128 = 0;

//Define some special character
pub const POOL_SEPARATE: &str = "@";

// Type
pub type PoolId = String;
pub type FarmId = String;
pub type TokenId = String;
pub type Amount = u128;