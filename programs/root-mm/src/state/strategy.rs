use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Default, Clone)]
pub struct OrderQuotes {
    pub bid_price: u64,
    pub bid_size: u64,

    pub ask_price: u64,
    pub ask_size: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum TickSizeMultiple {
    One,       // 0.1
    OneHalf,   // 0.5
    Two,       // 0.01
    TwoHalf,   // 0.05
    Three,     // 0.001
    ThreeHalf, // 0.005
}

impl Default for TickSizeMultiple {
    fn default() -> Self {
        Self::Two
    }
}

pub trait Strategy {
    fn rebalance_quote(&self) -> OrderQuotes;
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Default, Clone)]
pub struct AvellanedaStoikovStrategy {
    pub market: Pubkey,

    pub price_balancer: u16,
    pub spread_optimizer: u16,
    pub inventory_skew_target_in_pct: u8,
    pub min_spread_bps: u16,

    pub default_size_in_quote_units: u64,

    pub tick_size_multiple: TickSizeMultiple,

    pub strictly_post_only: bool,
    pub strictly_local_price_discovery: bool,
    pub use_log_returns: bool,

    pub last_refresh_slot: u64,
    pub min_refresh_slot_frequency: u64,
}

impl AvellanedaStoikovStrategy {
    pub const LEN: usize = 8 + (32 * 1) + (2 * 3) + (8 * 3) + (1 * 5);
}

impl Strategy for AvellanedaStoikovStrategy {
    fn rebalance_quote(&self) -> OrderQuotes {
        // Implement the A-S strategy here.
        // We have a working strategy that runs on a Rust backend server
        // and is market making on Phoenix SOL-USDC market on mainnet.
        //
        // The current strategy includes:
        // - Volatility adjusted dynamic spreads
        // - Targeted rebalancing to minimize inventory skew
        // - User-defined risk management
        // - Works without any oracle dependency
        //
        // Strategy will be open-sourced during main-net launch

        // Returning dummy values
        OrderQuotes {
            bid_price: 25u64,
            bid_size: 100u64,
            ask_price: 28u64,
            ask_size: 79u64,
        }
    }
}
