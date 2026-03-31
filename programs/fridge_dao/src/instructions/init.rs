use anchor_lang::prelude::*;

use crate::{error, state};

#[derive(Accounts)]
pub struct InitDAO<'info> {
    #[account(mut)]
    pub dao_mint: Signer<'info>,

    #[account(
        init,
        payer = dao_mint,
        space = 8 + state::FridgeDao::INIT_SPACE,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,

    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<InitDAO>, vote_period_len: u64, time_until_first_vote: u64) -> Result<()> { // future: redeploy just dao, redeploy dao_mint as well
    let dao = &mut ctx.accounts.fridge_dao;

    let clock = Clock::get()?;
    let next_vote_time = clock.unix_timestamp + time_until_first_vote as i64;

    dao.mint_admin_program = ctx.accounts.dao_mint.key();
    dao.bump = ctx.bumps.fridge_dao;
    dao.proposal_count = 0;
    dao.vote_period = vote_period_len;
    dao.next_vote_allowed_at = next_vote_time;
    dao.vote_cooldown = 1_209_600; // two weeks (can be updated)
    dao.proposals = Vec::new();
    
    dao.valid_member_keys = Vec::new();

    Ok(())
}
