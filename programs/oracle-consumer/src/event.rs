use anchor_lang::prelude::*;

use crate::Price;

#[event]
pub struct SolUsdPrice {
    pub price: i64,
    pub confidence: u64,
    pub exponent: i32,
    pub publish_time: i64,
    pub lower_bound: i64,
    pub upper_bound: i64,
}
impl From<&Price> for SolUsdPrice {
    fn from(price: &Price) -> Self {
        Self {
            price: price.price,
            confidence: price.confidence,
            exponent: price.exponent,
            publish_time: price.publish_time,
            lower_bound: price.lower_bound,
            upper_bound: price.upper_bound,
        }
    }
}
