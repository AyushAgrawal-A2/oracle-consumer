use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, FeedId, PriceUpdateV2};

use crate::{
    OracleError, EXPECTED_EXPONENT, MAX_CONF_BPS, MAX_PRICE, MAX_PRICE_AGE_SECS, MIN_PRICE,
    SOL_USD_FEED_HEX,
};

pub struct Oracle {
    pub feed_id: FeedId,
    pub max_age_secs: u64,
    pub max_conf_bps: u64,
    pub expected_exponent: i32,
    pub min_price: i64,
    pub max_price: i64,
}
impl Oracle {
    pub fn sol_usd() -> Result<Self> {
        Ok(Self {
            feed_id: get_feed_id_from_hex(SOL_USD_FEED_HEX)?,
            max_age_secs: MAX_PRICE_AGE_SECS,
            max_conf_bps: MAX_CONF_BPS,
            expected_exponent: EXPECTED_EXPONENT,
            min_price: MIN_PRICE,
            max_price: MAX_PRICE,
        })
    }

    pub fn get_price(&self, price_update: &Account<PriceUpdateV2>) -> Result<Price> {
        let price = price_update.get_price_no_older_than(
            &Clock::get()?,
            self.max_age_secs,
            &self.feed_id,
        )?;

        require!(
            price.price >= self.min_price && price.price <= self.max_price,
            OracleError::PriceOutOfRange
        );

        require!(
            price.exponent == self.expected_exponent,
            OracleError::InvalidExponent
        );

        let conf_scaled = (price.conf as u128)
            .checked_mul(10_000)
            .ok_or(OracleError::Overflow)?;
        let price_scaled = (price.price as u128)
            .checked_mul(self.max_conf_bps as u128)
            .ok_or(OracleError::Overflow)?;
        require!(conf_scaled <= price_scaled, OracleError::InvalidConfidence);

        Ok(Price {
            price: price.price,
            confidence: price.conf,
            exponent: price.exponent,
            publish_time: price.publish_time,
            lower_bound: price
                .price
                .checked_sub(price.conf as i64)
                .ok_or(OracleError::Underflow)?,
            upper_bound: price
                .price
                .checked_add(price.conf as i64)
                .ok_or(OracleError::Overflow)?,
        })
    }
}

pub struct Price {
    pub price: i64,
    pub confidence: u64,
    pub exponent: i32,
    pub publish_time: i64,
    pub lower_bound: i64,
    pub upper_bound: i64,
}
