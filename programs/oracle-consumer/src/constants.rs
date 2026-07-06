use anchor_lang::prelude::*;

#[constant]
pub const SOL_USD_FEED_HEX: &str =
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

#[constant]
pub const MAX_PRICE_AGE_SECS: u64 = 60;

#[constant]
pub const MAX_CONF_BPS: u64 = 100;

#[constant]
pub const EXPECTED_EXPONENT: i32 = -8;

#[constant]
pub const MIN_PRICE: i64 = 1_000_000;

#[constant]
pub const MAX_PRICE: i64 = 10_000_000_000_000;
