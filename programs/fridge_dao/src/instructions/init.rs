use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::error;
use crate::state;

#[derive(Accounts)]
pub struct InitDAO<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + state::FridgeDao::INIT_SPACE,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,

    #[account(
        seeds = [b"vault_authority", fridge_dao.key().as_ref()],
        bump
    )]
    pub vault_authority: SystemAccount<'info>,

    #[account(
        init,
        payer = authority,
        token::mint = usdc_mint,
        token::authority = vault_authority,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<InitDAO>, vote_period_len: u64, time_until_first_vote: u64) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    let clock = Clock::get()?;
    let next_vote_time = clock.unix_timestamp + time_until_first_vote as i64;

    dao.authority = ctx.accounts.authority.key();
    dao.vault = ctx.accounts.vault.key();
    dao.usdc_mint = ctx.accounts.usdc_mint.key();
    dao.proposal_count = 0;
    dao.bump = ctx.bumps.fridge_dao;
    dao.vault_bump = ctx.bumps.vault_authority;
    dao.vote_period = vote_period_len;
    dao.next_vote_allowed_at = next_vote_time;
    dao.vote_cooldown = 1_209_600; // two weeks (can be updated)
    dao.proposals = Vec::new();
    dao.valid_member_addresses = vec![ctx.accounts.authority.key()];

    Ok(())
}