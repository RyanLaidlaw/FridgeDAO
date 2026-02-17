use anchor_lang::prelude::*;

use crate::state;
use crate::error;

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct Vote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

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
        bump
    )]
    pub proposal: Account<'info, state::Proposal>,

    #[account(
        init,
        payer = voter,
        space = 8 + state::UserVote::INIT_SPACE,
        seeds = [
            state::UserVote::SEED_PREFIX,
            proposal.key().as_ref(),
            voter.key().as_ref(),
        ],
        bump,
    )]
    pub vote: Account<'info, state::UserVote>,

    pub system_program: Program<'info, System>,
}

pub fn vote(ctx: Context<Vote>, proposal_id: u64, vote_choice: state::VoteChoice) -> Result<()> {
    let vote = &mut ctx.accounts.vote;
    let proposal = &mut ctx.accounts.proposal;

    require!(proposal_id == proposal.identifier, error::Error::CouldNotFindProposal);

    let vote_power = 1000; // TODO add vote power logic

    vote.proposal = proposal.key();
    vote.voter = ctx.accounts.voter.key();
    vote.vote_choice = vote_choice;
    vote.vote_power = vote_power;
    vote.bump = ctx.bumps.vote;

    match vote_choice {
        state::VoteChoice::Yes => proposal.yes_votes += vote_power,
        state::VoteChoice::No => proposal.no_votes += vote_power,
    }

    Ok(())
}