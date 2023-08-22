use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::state::*;

pub fn withdraw_funds(ctx: Context<WithdrawFunds>, base_qty: u64, quote_qty: u64) -> Result<()> {
    let owner = ctx.accounts.owner.key();
    let market = ctx.accounts.market.key();

    let vault_bump = ctx.accounts.vault.bump;

    let vault_seeds = &[owner.as_ref(), market.as_ref(), &[vault_bump]];
    let vault_signer_seeds = &[&vault_seeds[..]];

    let vault = &mut ctx.accounts.vault;

    if base_qty > 0 {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.base_token_vault_ac.to_account_info(),
                    to: ctx.accounts.base_token_user_ac.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
                vault_signer_seeds,
            ),
            base_qty,
        )?;

        vault.base_asset_balance += base_qty;
    }

    if quote_qty > 0 {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.quote_token_vault_ac.to_account_info(),
                    to: ctx.accounts.quote_token_user_ac.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
                vault_signer_seeds,
            ),
            quote_qty,
        )?;

        vault.quote_asset_balance += quote_qty;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
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
