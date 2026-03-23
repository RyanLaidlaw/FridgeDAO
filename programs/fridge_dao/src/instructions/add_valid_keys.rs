use anchor_lang::prelude::*;
use std::collections::HashSet;

use crate::{error, state};

#[derive(Accounts)]
pub struct AddKeys<'info> {
    #[account(mut)]
    pub adder: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,
}

pub fn add_valid_keys(ctx: Context<AddKeys>, new_keys: Vec<Pubkey>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(dao.valid_member_keys.len() + new_keys.len() <= state::MAX_MEMBERS, error::Error::MaxMembers);
    require!(ctx.accounts.adder.key() == dao.authority, error::Error::InvalidAuthority);

    let existing: HashSet<Pubkey> = dao.valid_member_keys.iter().map(|u| u.key).collect();
    let mut seen: HashSet<Pubkey> = HashSet::new();

    for k in &new_keys {
        require!(!existing.contains(k), error::Error::KeyAlreadyAdded);
        require!(seen.insert(*k), error::Error::DuplicateKeys);
    }

    dao.valid_member_keys.extend(new_keys.iter().map(|addr| state::User{key: *addr, balance: 0})); // TODO need their token account to get their balance

    Ok(())
}