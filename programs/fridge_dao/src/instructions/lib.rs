use anchor_lang::prelude::*;

use crate::{error, state};

pub fn get_voting_end(dao: &Account<state::FridgeDao>) -> Result<i64> {
    let voting_end = dao
        .next_vote_allowed_at
        .checked_add(dao.vote_period as i64)
        .ok_or(error::Error::MathOverflowOrUnderflow)?;

    Ok(voting_end)
}

fn _sqrt(n: u64) -> u64 {
    (n as f64).sqrt() as u64
}

pub fn compute_winner<'info>(dao: &mut state::FridgeDao, remaining_accounts: &'info [AccountInfo<'info>], vault_balance: u64) -> Result<()> {
    let mut max_score: u64 = 0;
    let mut winner: Option<(Pubkey, state::Proposal)> = None;
    let mut ties: Vec<Pubkey> = Vec::new();
    
    for proposal_info in remaining_accounts.iter() {
        let proposal: Account<state::Proposal> = Account::try_from(proposal_info)?;
        require!(dao.proposals.contains(&proposal.key()), error::Error::InvalidProposal);

        let mut score: u64 = 0;
        for voter in &proposal.voters {
            let user = dao.valid_member_keys.iter()
                .find(|user| user.key == voter.key())
                .ok_or(error::Error::InvalidMember)?;

            score = score.checked_add(_sqrt(user.balance)).ok_or(error::Error::MathOverflowOrUnderflow)?;
        }

        if score > max_score {
            max_score = score;
            ties.clear();
            winner = Some((proposal.key(), proposal.into_inner().clone()));
        } else if score == max_score {
            if let Some((ref w, _)) = winner {
                if !ties.contains(w) {
                    ties.push(*w);
                }
            }
            ties.push(proposal.key());
        }
    }

    if ties.len() > 0 {
        dao.tie = Some(state::Tie {
            score: max_score,
            winners: ties.clone(),
        });

        emit!(state::TieEvent {
            score: max_score,
            winners: ties,
        });
        return Ok(());
    }

    let (_, winner_proposal) = winner.ok_or(error::Error::NoWinner)?;

    let remaining = vault_balance.checked_sub(winner_proposal.price).ok_or(error::Error::MathOverflowOrUnderflow)?;

    for user in dao.valid_member_keys.iter_mut() {
        user.balance = user.balance
            .checked_mul(remaining)
            .ok_or(error::Error::MathOverflowOrUnderflow)?
            .checked_div(vault_balance)
            .ok_or(error::Error::MathOverflowOrUnderflow)?;
    }

    emit!(state::WinnerChosen {
        description: winner_proposal.description.clone(),
        price: winner_proposal.price
    });

    dao.recent_winner = Some(winner_proposal); 

    Ok(())
}
