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

pub fn remove_valid_addresses(ctx: Context<RemoveAddress>, addresses_to_remove: Vec<Pubkey>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.remover.key() == dao.authority, error::Error::InvalidAuthority);
    for addr in addresses_to_remove.iter() {
        require!(dao.valid_member_addresses.contains(addr), error::Error::AddressDoesNotExist);
    }

    dao.valid_member_addresses.retain(|x| !addresses_to_remove.contains(x));
    
    Ok(())
}