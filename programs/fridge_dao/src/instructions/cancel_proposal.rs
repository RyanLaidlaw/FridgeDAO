use anchor_lang::prelude::*;

use crate::{error, state};

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
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
        seeds = [
            state::Proposal::SEED_PREFIX,
            fridge_dao.key().as_ref(),
            proposal_id.to_le_bytes().as_ref(),
        ],
        bump = proposal.bump,
        has_one = proposer,
        close = proposer
    )]
    pub proposal: Account<'info, state::Proposal>,

    /// CHECK: validated by `has_one`
    #[account(mut)]
    pub proposer: UncheckedAccount<'info>,
}

pub fn cancel_proposal(ctx: Context<Cancel>, proposal_id: u64) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;
    let proposal = &mut ctx.accounts.proposal;

    require!(ctx.accounts.canceller.key() == dao.authority, error::Error::InvalidAuthority);

    dao.proposals.retain(|x|  *x != proposal.key());

    Ok(())
}