use anchor_lang::prelude::*;

use crate::{state, error};

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct ResolveTie<'info> {
    #[account(mut)]
    pub resolver: Signer<'info>,

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

pub fn resolve_tie(ctx: Context<ResolveTie>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.resolver.key() == dao.admin.key(), error::Error::InvalidAuthority);

    let tie: state::Tie = dao.tie.clone().ok_or(error::Error::NoTie)?;
    let winner = &ctx.accounts.proposal;

    require!(tie.winners.contains(&winner.key()), error::Error::ProposalNotInTies);

    let remaining = dao.vault_balance.checked_sub(tie.score).ok_or(error::Error::MathOverflowOrUnderflow)?;
    let vault_balance = dao.vault_balance;

    for user in dao.valid_member_keys.iter_mut() {
        user.balance = user.balance
            .checked_mul(remaining)
            .ok_or(error::Error::MathOverflowOrUnderflow)?
            .checked_div(vault_balance)
            .ok_or(error::Error::MathOverflowOrUnderflow)?;
    }

    dao.recent_winner = Some(state::Proposal {
        identifier: winner.identifier,
        voters: winner.voters.clone(),
        proposer: winner.proposer,
        description: winner.description.clone(),
        price: winner.price,
    });

    dao.tie = None;

    let now = Clock::get()?.unix_timestamp;
    dao.next_vote_allowed_at = now + dao.vote_cooldown as i64;
    dao.proposals.retain(|x|  *x != winner.key());

    emit!(state::WinnerChosen {
        description: winner.description.clone(),
        price: winner.price
    });

    Ok(())
}