use anchor_lang::prelude::*;

use crate::{error, state};

pub fn get_voting_end(dao: &Account<state::FridgeDao>) -> Result<i64> {
    let voting_end = dao
        .next_vote_allowed_at
        .checked_add(dao.vote_period as i64)
        .ok_or(error::Error::MathOverflowOrUnderflow)?;

    Ok(voting_end)
}