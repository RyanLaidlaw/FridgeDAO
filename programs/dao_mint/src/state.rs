use anchor_lang::prelude::*;

pub const MAX_MEMBERS: usize = 20;
pub const DECIMALS: u8 = 6; // 6 decimals for USDC

#[account]
#[derive(InitSpace)]
pub struct UserWithTokenAcct {
    pub key: Pubkey,
    pub token_account: Pubkey, // only store the token accounts for users in the dao_mint, not in the dao itself
}

#[account]
#[derive(InitSpace)]
pub struct DAOMint {
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub usdc_mint: Pubkey,
    pub dao_program: Pubkey,
    pub mint_authority_bump: u8,
    pub bump: u8,
    #[max_len(MAX_MEMBERS)]
    pub valid_member_keys: Vec<UserWithTokenAcct>,
}

impl DAOMint {
    pub const SEED_PREFIX: &'static [u8; 11] = b"fridge_mint";
}