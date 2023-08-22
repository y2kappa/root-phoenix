use anchor_lang::prelude::*;

use super::AvellanedaStoikovStrategy;

#[derive(Debug, Default)]
#[account]
pub struct Vault {
    pub bump: u8,
    pub owner: Pubkey,
    pub market: Pubkey,

    pub base_asset_balance: u64,
    pub quote_asset_balance: u64,

    pub strategy: AvellanedaStoikovStrategy,
}

impl Vault {
    pub const LEN: usize = 8 + (1 * 2) + (2 * 3) + (8 * 2) + (32 * 2);
}
