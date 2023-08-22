use anchor_lang::prelude::*;

use instructions::*;

declare_id!("FYiqpVa5WM8mXEz1VQF3DzJRLUrkNS3Y86HfQnREMyt3");

pub mod constants;
pub mod instructions;
pub mod state;
pub mod errors;

#[program]
pub mod root_mm {

    use super::*;

    pub fn initialize_volatility_accumulator(
        ctx: Context<InitializeVolatilityAccumulator>,
    ) -> Result<()> {
        instructions::initialize_volatility_accumulator(ctx)
    }

    pub fn update_volatility_accumulator(
        ctx: Context<UpdateVolatilityAccumulator>,
        new_val: f64,
    ) -> Result<()> {
        instructions::update_volatility_accumulator(ctx, new_val)
    }

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        price_balancer: u16,
        spread_optimizer: u16,
        inventory_skew_target_in_pct: u8,
        min_spread_bps: u16,
        default_size_in_quote_units: u64,
        strictly_post_only: bool,
        strictly_local_price_discovery: bool,
        use_log_returns: bool
    ) -> Result<()> {
        instructions::initialize_vault(
            ctx,
            price_balancer,
            spread_optimizer,
            inventory_skew_target_in_pct,
            min_spread_bps,
            default_size_in_quote_units,
            strictly_post_only,
            strictly_local_price_discovery,
            use_log_returns
        )
    }

    pub fn deposit_funds(ctx: Context<DepositFunds>, base_qty: u64, quote_qty: u64) -> Result<()> {
        instructions::deposit_funds(ctx, base_qty, quote_qty)
    }

    pub fn withdraw_funds(
        ctx: Context<WithdrawFunds>,
        base_qty: u64,
        quote_qty: u64,
    ) -> Result<()> {
        instructions::withdraw_funds(ctx, base_qty, quote_qty)
    }

    pub fn refresh_quotes(ctx: Context<RefreshQuotes>) -> Result<()> {
        instructions::refresh_quotes(ctx)
    }
}
