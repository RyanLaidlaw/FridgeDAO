use anchor_lang::prelude::*;

use crate::{state, error};

#[derive(Accounts)]
pub struct ResolveTie<'info> {
    #[account(mut)]
    pub resolver: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,
}

pub fn resolve_tie(ctx: Context<ResolveTie>, winner: Pubkey) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.resolver.key() == dao.admin.key(), error::Error::InvalidAuthority);

    let tie: state::Tie = dao.tie.clone().ok_or(error::Error::NoTie)?;
    require!(tie.winners.contains(&winner), error::Error::ProposalNotInTies);

    let remaining = dao.vault_balance.checked_sub(tie.score).ok_or(error::Error::MathOverflowOrUnderflow)?;
    let vault_balance = dao.vault_balance;

    for user in dao.valid_member_keys.iter_mut() {
        user.balance = user.balance
            .checked_mul(remaining)
            .ok_or(error::Error::MathOverflowOrUnderflow)?
            .checked_div(vault_balance)
            .ok_or(error::Error::MathOverflowOrUnderflow)?;
    }

    dao.recent_winner = winner;
    dao.tie = None;

    let now = Clock::get()?.unix_timestamp;
    dao.next_vote_allowed_at = now + dao.vote_cooldown as i64;

    emit!(state::WinnerChosen {proposal: winner});
    Ok(())
}