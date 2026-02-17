use std::ops::Add;

use anchor_lang::prelude::*;

use crate::state;

#[derive(Accounts)]
pub struct Propose<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,

    #[account(
        init,
        payer = proposer,
        space = 8 + state::Proposal::INIT_SPACE,
        seeds = [
            state::Proposal::SEED_PREFIX,
            fridge_dao.key().as_ref(),
            fridge_dao.proposal_count.add(1).to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub proposal: Account<'info, state::Proposal>,

    pub system_program: Program<'info, System>,
}

pub fn propose(ctx: Context<Propose>, description: String) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let dao = &mut ctx.accounts.fridge_dao;
    
    dao.proposal_count += 1;

    proposal.identifier = dao.proposal_count;
    proposal.proposer = ctx.accounts.proposer.key();
    proposal.description = description;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.bump = ctx.bumps.proposal;

    Ok(())
}