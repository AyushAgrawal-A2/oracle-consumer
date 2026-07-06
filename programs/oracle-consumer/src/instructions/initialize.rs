use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{Oracle, SolUsdPrice};

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub price_update: Account<'info, PriceUpdateV2>,
}

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    let sol_usd_oracle = Oracle::sol_usd()?;
    let sol_usd_price = sol_usd_oracle.get_price(&ctx.accounts.price_update)?;
    emit!(SolUsdPrice::from(&sol_usd_price));
    Ok(())
}
