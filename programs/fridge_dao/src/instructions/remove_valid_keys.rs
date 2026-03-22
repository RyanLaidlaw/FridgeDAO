use anchor_lang::prelude::*;

use crate::{error, state};

#[derive(Accounts)]
pub struct RemoveKeys<'info> {
    #[account(mut)]
    pub remover: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,
}

pub fn remove_valid_keys(ctx: Context<RemoveKeys>, keys_to_remove: Vec<Pubkey>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.remover.key() == dao.authority, error::Error::InvalidAuthority);

    for k in &keys_to_remove {
        require!(dao.valid_member_keys.iter().any(|user| user.key == *k), error::Error::AddressDoesNotExist);
    }

    dao.valid_member_keys.retain(|x| !keys_to_remove.contains(&x.key));
    
    Ok(())
}