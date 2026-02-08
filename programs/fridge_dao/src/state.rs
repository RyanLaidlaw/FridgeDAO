use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FridgeDao {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub usdc_mint: Pubkey,
    pub proposal_count: u64,
}

impl FridgeDao {
    pub const SEED_PREFIX: &'static [u8; 6] = b"fridge";
}