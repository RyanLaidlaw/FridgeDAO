use anchor_lang::prelude::*;

pub const MAX_MEMBERS: usize = 20;
pub const MAX_PROPOSALS: usize = 15;
pub const DECIMALS: u8 = 6; // 6 decimals for USDC

#[event]
pub struct TieEvent {
    pub score: u64,
    pub winners: Vec<Pubkey>,
}

#[event]

pub struct WinnerChosen {
    pub description: String,
    pub price: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Tie {
    pub score: u64,
    #[max_len(MAX_PROPOSALS)]
    pub winners: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct UserWithBalance {
    pub key: Pubkey,
    pub balance: u64
}

#[account]
#[derive(InitSpace)]
pub struct FridgeDao {
    pub mint_admin_program: Pubkey,
    pub admin: Pubkey,
    pub vault_balance: u64,
    pub bump: u8,
    pub proposal_count: u64,
    pub vote_period: u64, // duration of voting period (seconds)
    pub vote_cooldown: u64, // time between voting periods (seconds)
    pub next_vote_allowed_at: i64, // absolute timestamp
    #[max_len(MAX_PROPOSALS)]
    pub proposals: Vec<Pubkey>,
    #[max_len(MAX_MEMBERS)]
    pub valid_member_keys: Vec<UserWithBalance>,
    pub tie: Option<Tie>,
    pub recent_winner: Option<Proposal>,
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
    pub bump: u8,
    pub price: u64,
}

impl Proposal {
    pub const SEED_PREFIX: &'static [u8; 11] = b"fridge_prop";
}

#[account]
#[derive(InitSpace)]
pub struct UserVote {
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub bump: u8,
}

impl UserVote {
    pub const SEED_PREFIX: &'static [u8] = b"vote";
}
