use anchor_lang::prelude::*;

use crate::{instructions::lib, error, state};

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,

    #[account(mut)]
    pub proposal: Account<'info, state::Proposal>,
}

pub fn vote(ctx: Context<Vote>, proposal_id: u64) -> Result<()> {
    let voter = &ctx.accounts.voter;
    let dao = &ctx.accounts.fridge_dao;
    let proposal = &mut ctx.accounts.proposal;

    require!(dao.valid_member_keys.iter().any(|user| user.key == voter.key()), error::Error::InvalidMember);
    require!(proposal_id == proposal.identifier, error::Error::CouldNotFindProposal);
    require!(!proposal.voters.contains(&voter.key), error::Error::AlreadyVoted);

    let now = Clock::get()?.unix_timestamp;
    let voting_end = lib::get_voting_end(&dao)?;

    require!(lib::is_in_voting_period(now, dao.next_vote_allowed_at, voting_end), error::Error::NotInVotingPeriod);

    proposal.voters.push(voter.key());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_voting_outside_vote_period() {
        let vote_start: i64 = 1000;
        let voting_end: i64 = 2000;

        assert!(!lib::is_in_voting_period(999, vote_start, voting_end));
        assert!(!lib::is_in_voting_period(2000, vote_start, voting_end));
        assert!(!lib::is_in_voting_period(2001, vote_start, voting_end));
    }

    #[test]
    fn allows_voting_inside_vote_period() {
        let vote_start: i64 = 1000;
        let voting_end: i64 = 2000;

        assert!(lib::is_in_voting_period(1000, vote_start, voting_end));
        assert!(lib::is_in_voting_period(1500, vote_start, voting_end));
        assert!(lib::is_in_voting_period(1999, vote_start, voting_end));
    }
}