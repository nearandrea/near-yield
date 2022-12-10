use near_sdk::{Gas};
use near_sdk::json_types::U128;

// Define the default message
pub const DEFAULT_MESSAGE: &str = "Hello";
pub const DEFAULT_OWNER: &str = "andreapn1709.testnet";
pub const REF_CONTRACT: &str = "ref-finance-101.testnet";
pub const REF_FARM_CONTRACT: &str = "boostfarm.ref-finance.testnet";
pub const DEPOSIT_YOCTO: u128 = 1;
pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

pub const FT_CALL_GAS: Gas = Gas(35_000_000_000_000);
pub const FT_CALL_WITHDRAW_TGAS: Gas = Gas(60_000_000_000_000);
pub const ZERO: u128 = 0;

// Type
pub type PoolId = String;
pub type FarmId = String;
pub type TokenId = String;
pub type Amount = u128;