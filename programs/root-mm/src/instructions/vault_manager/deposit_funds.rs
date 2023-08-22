use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::state::*;

pub fn deposit_funds(ctx: Context<DepositFunds>, base_qty: u64, quote_qty: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    if base_qty > 0 {
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.base_token_user_ac.to_account_info(),
                    to: ctx.accounts.base_token_vault_ac.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            base_qty,
        )?;

        vault.base_asset_balance += base_qty;
    }

    if quote_qty > 0 {
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.quote_token_user_ac.to_account_info(),
                    to: ctx.accounts.quote_token_vault_ac.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            quote_qty,
        )?;

        vault.quote_asset_balance += quote_qty;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct DepositFunds<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: No constraint needed
    pub market: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            owner.key().as_ref(),
            market.key().as_ref()
        ],
        bump = vault.bump,
        has_one = market,
        has_one = owner
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub base_token_user_ac: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_token_user_ac: Account<'info, TokenAccount>,

    #[account(mut)]
    pub base_token_vault_ac: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_token_vault_ac: Account<'info, TokenAccount>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}
