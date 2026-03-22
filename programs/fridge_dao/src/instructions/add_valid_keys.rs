use anchor_lang::prelude::*;

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

    let mut seen = std::collections::HashSet::new();

    require!(dao.valid_member_keys.len() + new_keys.len() <= state::MAX_MEMBERS, error::Error::MaxMembers);
    require!(ctx.accounts.adder.key() == dao.authority, error::Error::InvalidAuthority);

    for k in &new_keys {
        require!(!dao.valid_member_keys.iter().any(|user| user.key == *k), error::Error::AddressAlreadyAdded);    
        require!(seen.insert(k), error::Error::DuplicateAddress);
    }

    dao.valid_member_keys.extend(new_keys.iter().map(|addr| state::User{key: *addr, balance: 0}));

    Ok(())
}