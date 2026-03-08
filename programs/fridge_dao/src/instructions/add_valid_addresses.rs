use anchor_lang::prelude::*;

use crate::{error, state};

#[derive(Accounts)]
pub struct AddAddress<'info> {
    #[account(mut)]
    pub adder: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,
}

pub fn add_valid_addresses(ctx: Context<AddAddress>, new_addresses: Vec<Pubkey>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    let mut seen = std::collections::HashSet::new();

    require!(dao.valid_member_addresses.len() + new_addresses.len() <= state::MAX_MEMBERS, error::Error::MaxMembers);

    require!(ctx.accounts.adder.key() == dao.authority, error::Error::InvalidAuthority);
    for addr in new_addresses.iter() {
        require!(!dao.valid_member_addresses.contains(addr), error::Error::AddressAlreadyAdded);    
        require!(seen.insert(addr), error::Error::DuplicateAddress);
    }

    dao.valid_member_addresses.extend(new_addresses);

    Ok(())
}