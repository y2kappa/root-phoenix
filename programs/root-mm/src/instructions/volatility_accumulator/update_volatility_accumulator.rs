use anchor_lang::prelude::*;

use crate::state::*;

pub fn update_volatility_accumulator(
    ctx: Context<UpdateVolatilityAccumulator>,
    new_val: f64,
) -> Result<()> {
    let volatility_accumulator = &mut ctx.accounts.volatility_accumulator;

    volatility_accumulator.update_buffer(new_val);

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateVolatilityAccumulator<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            market.key().as_ref()
        ],
        bump = volatility_accumulator.bump,
        constraint = volatility_accumulator.creator == signer.key()
    )]
    pub volatility_accumulator: Account<'info, VolatilityAccumulator>,

    /// CHECK: No constraint needed
    pub market: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
