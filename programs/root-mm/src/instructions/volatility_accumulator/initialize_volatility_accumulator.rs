use anchor_lang::prelude::*;

use crate::{constants::VOLATILITY_BUFFER_SIZE, state::*};

pub fn initialize_volatility_accumulator(
    ctx: Context<InitializeVolatilityAccumulator>,
) -> Result<()> {
    *ctx.accounts.volatility_accumulator = VolatilityAccumulator {
        bump: *ctx.bumps.get("volatility_accumulator").unwrap(),
        creator: ctx.accounts.signer.key(),
        market: ctx.accounts.market.key(),
        buffer: [f64::default(); VOLATILITY_BUFFER_SIZE],
    };

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVolatilityAccumulator<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        seeds = [
            market.key().as_ref()
        ],
        bump,
        payer = signer,
        space = VolatilityAccumulator::LEN
    )]
    pub volatility_accumulator: Account<'info, VolatilityAccumulator>,

    /// CHECK: No constraint needed
    pub market: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
