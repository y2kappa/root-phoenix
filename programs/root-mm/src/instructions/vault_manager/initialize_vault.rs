use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::*;

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
    *ctx.accounts.vault = Vault {
        bump: *ctx.bumps.get("vault").unwrap(),
        owner: ctx.accounts.owner.key(),
        market: ctx.accounts.market.key(),

        base_asset_balance: 0u64,
        quote_asset_balance: 0u64,

        strategy: AvellanedaStoikovStrategy {
            market: ctx.accounts.market.key(),
            price_balancer,
            spread_optimizer,
            inventory_skew_target_in_pct,
            min_spread_bps,
            default_size_in_quote_units,
            tick_size_multiple: TickSizeMultiple::default(),
            strictly_post_only,
            strictly_local_price_discovery,
            use_log_returns,
            last_refresh_slot: 0u64,
            min_refresh_slot_frequency: 3u64,
        },
    };

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: No constraint needed
    pub market: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [
            owner.key().as_ref(),
            market.key().as_ref()
        ],
        bump,
        payer = owner,
        space = Vault::LEN
    )]
    pub vault: Account<'info, Vault>,

    pub token_a_mint: Account<'info, Mint>,

    pub token_b_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = owner,
        token::mint = token_a_mint,
        token::authority = vault
    )]
    pub token_a_vault_ac: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = owner,
        token::mint = token_b_mint,
        token::authority = vault
    )]
    pub token_b_vault_ac: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}
