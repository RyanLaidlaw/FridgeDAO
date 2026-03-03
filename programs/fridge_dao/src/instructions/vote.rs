use anchor_lang::prelude::*;

use crate::instructions::lib;
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

pub fn vote(ctx: Context<Vote>, proposal_id: u64) -> Result<()> {
    let voter = &ctx.accounts.voter;
    let dao = &ctx.accounts.fridge_dao;
    let vote = &mut ctx.accounts.vote;
    let proposal = &mut ctx.accounts.proposal;

    require!(dao.valid_member_addresses.contains(voter.key), error::Error::InvalidMember);

    let now = Clock::get()?.unix_timestamp;
    let voting_end = lib::get_voting_end(&dao)?;

    // add check for verified wallet addresses
    require!(proposal_id == proposal.identifier, error::Error::CouldNotFindProposal);
    require!(!proposal.voters.contains(voter.key), error::Error::AlreadyVoted);
    require!(now >= dao.next_vote_allowed_at && now < voting_end, error::Error::NotInVotingPeriod);

    vote.proposal = proposal.key();
    vote.voter = ctx.accounts.voter.key();
    vote.bump = ctx.bumps.vote;

    proposal.voters.push(voter.key());

    Ok(())
}