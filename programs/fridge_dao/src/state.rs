use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FridgeDao {
    pub authority: Pubkey,
    pub vault: Pubkey,
    pub usdc_mint: Pubkey,
    pub proposal_count: u64,
    pub bump: u8,
    pub vault_bump: u8,
}

impl FridgeDao {
    pub const SEED_PREFIX: &'static [u8; 6] = b"fridge";
}

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub identifier: u64,
    #[max_len(35)]
    pub voters: Vec<Pubkey>,
    pub proposer: Pubkey,
    #[max_len(50)]
    pub description: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub bump: u8
}

impl Proposal {
    pub const SEED_PREFIX: &'static [u8; 11] = b"fridge_prop";
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum VoteChoice {
    Yes, 
    No,
}

#[account]
#[derive(InitSpace)]
pub struct UserVote {
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub vote_choice: VoteChoice,
    pub vote_power: u64,
    pub bump: u8,
}

impl UserVote {
    pub const SEED_PREFIX: &'static [u8] = b"vote";
}