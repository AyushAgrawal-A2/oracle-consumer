use anchor_lang::prelude::*;

#[error_code]
pub enum OracleError {
    #[msg("Price out of range")]
    PriceOutOfRange,
    #[msg("Invalid exponent")]
    InvalidExponent,
    #[msg("Invalid confidence")]
    InvalidConfidence,
    #[msg("Overflow")]
    Overflow,
    #[msg("Underflow")]
    Underflow,
}
