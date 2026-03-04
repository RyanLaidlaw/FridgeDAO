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

pub fn add_valid_address(ctx: Context<AddAddress>, new_address: Pubkey) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.adder.key() == dao.authority.key(), error::Error::InvalidAuthority);
    require!(!dao.valid_member_addresses.contains(&new_address), error::Error::AddressAlreadyAdded);

    dao.valid_member_addresses.push(new_address);

    Ok(())
}