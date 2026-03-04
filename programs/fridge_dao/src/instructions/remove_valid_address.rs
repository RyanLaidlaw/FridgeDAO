use anchor_lang::prelude::*;

use crate::{error, state};

#[derive(Accounts)]
pub struct RemoveAddress<'info> {
    #[account(mut)]
    pub remover: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,
}

pub fn remove_valid_address(ctx: Context<RemoveAddress>, address_to_remove: Pubkey) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.remover.key() == dao.authority.key(), error::Error::InvalidAuthority);
    require!(dao.valid_member_addresses.contains(&address_to_remove), error::Error::AddressDoesNotExist);

    dao.valid_member_addresses.retain(|x| x != &address_to_remove);

    Ok(())
}