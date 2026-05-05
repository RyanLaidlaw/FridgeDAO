use anchor_lang::prelude::*;

use crate::{error, state};

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub canceller: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,

    #[account(
        mut,
        has_one = proposer,
        close = proposer
    )]
    pub proposal: Account<'info, state::Proposal>,

    /// CHECK: validated by `has_one`
    #[account(mut)]
    pub proposer: UncheckedAccount<'info>,
}

pub fn cancel_proposal(ctx: Context<Cancel>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;
    let proposal = &mut ctx.accounts.proposal;

    let is_admin = ctx.accounts.canceller.key() == dao.admin;
    let is_proposer = ctx.accounts.canceller.key() == proposal.proposer;

    require!(is_admin || is_proposer, error::Error::InvalidAuthority);

    dao.proposals.retain(|x|  *x != proposal.key());
    dao.proposal_count -= 1;

    Ok(())
}
