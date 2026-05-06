use anchor_lang::prelude::*;

use crate::{error, state};

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
            (fridge_dao.proposal_count + 1).to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub proposal: Account<'info, state::Proposal>,

    pub system_program: Program<'info, System>,
}

pub fn propose(ctx: Context<Propose>, description: String, price: u64) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;
    let proposal = &mut ctx.accounts.proposal;

    require!(dao.valid_member_keys.iter().any(|user| user.key == ctx.accounts.proposer.key()), error::Error::InvalidMember);
    require!(dao.proposal_count + 1 <= state::MAX_PROPOSALS as u64, error::Error::AtMaxProposals);

    dao.proposals.push(proposal.key());

    dao.proposal_count += 1;

    proposal.identifier = dao.proposal_count;
    proposal.price = price;
    proposal.voters = Vec::new();
    proposal.proposer = ctx.accounts.proposer.key();
    proposal.description = description;
    proposal.bump = ctx.bumps.proposal;

    Ok(())
}
