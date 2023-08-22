use anchor_lang::{prelude::*, solana_program::program::invoke};
use phoenix::{
    quantities::{BaseLots, Ticks, WrapperU64},
    state::{OrderPacket, Side},
};

use crate::{state::{Strategy, Vault}, errors::RootError};

pub fn refresh_quotes(ctx: Context<RefreshQuotes>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    let clock = Clock::get()?;
    let current_slot = clock.slot;

    require!((vault.strategy.last_refresh_slot + vault.strategy.min_refresh_slot_frequency) <= current_slot, RootError::RefreshedTooSoon);

    // Cancel all existing orders with free funds
    invoke(
        &phoenix::program::create_cancel_all_order_with_free_funds_instruction(
            &ctx.accounts.market.key(),
            &ctx.accounts.owner.key(),
        ),
        &[
            ctx.accounts.phoenix_program.to_account_info(),
            ctx.accounts.log_authority.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.market.to_account_info(),
        ],
    )?;

    let bid_order_packet: OrderPacket;
    let ask_order_packet: OrderPacket;

    let order_quotes = vault.strategy.rebalance_quote();

    if vault.strategy.strictly_post_only {
        bid_order_packet = OrderPacket::PostOnly {
            side: Side::Bid,
            price_in_ticks: Ticks::new(order_quotes.bid_price),
            num_base_lots: BaseLots::new(order_quotes.bid_size),
            client_order_id: 1,
            reject_post_only: true,
            use_only_deposited_funds: true,
            last_valid_slot: None,
            last_valid_unix_timestamp_in_seconds: None,
            fail_silently_on_insufficient_funds: false,
        };

        ask_order_packet = OrderPacket::PostOnly {
            side: Side::Ask,
            price_in_ticks: Ticks::new(order_quotes.ask_price),
            num_base_lots: BaseLots::new(order_quotes.ask_size),
            client_order_id: 2,
            reject_post_only: true,
            use_only_deposited_funds: true,
            last_valid_slot: None,
            last_valid_unix_timestamp_in_seconds: None,
            fail_silently_on_insufficient_funds: false,
        };
    } else {
        bid_order_packet = OrderPacket::Limit {
            side: Side::Bid,
            price_in_ticks: Ticks::new(order_quotes.bid_price),
            num_base_lots: BaseLots::new(order_quotes.bid_size),
            self_trade_behavior: phoenix::state::SelfTradeBehavior::CancelProvide,
            match_limit: None,
            client_order_id: 1,
            use_only_deposited_funds: true,
            last_valid_slot: None,
            last_valid_unix_timestamp_in_seconds: None,
            fail_silently_on_insufficient_funds: false,
        };

        ask_order_packet = OrderPacket::Limit {
            side: Side::Ask,
            price_in_ticks: Ticks::new(order_quotes.ask_price),
            num_base_lots: BaseLots::new(order_quotes.ask_size),
            self_trade_behavior: phoenix::state::SelfTradeBehavior::CancelProvide,
            match_limit: None,
            client_order_id: 2,
            use_only_deposited_funds: true,
            last_valid_slot: None,
            last_valid_unix_timestamp_in_seconds: None,
            fail_silently_on_insufficient_funds: false,
        }
    }

    // Post new orders to the book
    invoke(
        &phoenix::program::create_new_order_with_free_funds_instruction(
            &ctx.accounts.market.key(),
            &ctx.accounts.owner.key(),
            &bid_order_packet,
        ),
        &[
            ctx.accounts.phoenix_program.to_account_info(),
            ctx.accounts.log_authority.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.market.to_account_info(),
            ctx.accounts.owner_seat.to_account_info(),
        ],
    )?;

    invoke(
        &phoenix::program::create_new_order_with_free_funds_instruction(
            &ctx.accounts.market.key(),
            &ctx.accounts.owner.key(),
            &ask_order_packet,
        ),
        &[
            ctx.accounts.phoenix_program.to_account_info(),
            ctx.accounts.log_authority.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.market.to_account_info(),
            ctx.accounts.owner_seat.to_account_info(),
        ],
    )?;

    vault.strategy.last_refresh_slot = current_slot;

    Ok(())
}

#[derive(Accounts)]
pub struct RefreshQuotes<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: No constraints needed
    pub market: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            owner.key().as_ref(),
            market.key().as_ref()
        ],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    /// CHECK: No constraints needed
    pub owner_seat: UncheckedAccount<'info>,

    /// CHECK: No constraints needed
    pub log_authority: UncheckedAccount<'info>,

    #[
        account(
            address = phoenix::id()
        )
    ]
    /// CHECK: Program ID verified
    pub phoenix_program: UncheckedAccount<'info>,
}
