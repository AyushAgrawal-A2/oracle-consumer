pub mod constants;
pub mod error;
pub mod event;
pub mod instructions;
pub mod oracle;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use event::*;
pub use instructions::*;
pub use oracle::*;

declare_id!("GujZcMQix2ctpPHstodSJMDbmdw6zcPmhFEpQJAWFvUX");

#[program]
pub mod oracle_consumer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        handle_initialize(ctx)
    }
}
