use anchor_lang::prelude::*;

use crate::{error, state};

#[derive(Accounts)]
pub struct DAOBlacklist<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>
}

pub fn blacklist_keys<'info>(ctx: Context<DAOBlacklist>, keys_to_remove: Vec<Pubkey>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.signer.key() == dao.mint_admin_program, error::Error::InvalidAuthority);

    dao.valid_member_keys.retain(|user| !keys_to_remove.contains(&user.key));
    
    Ok(())
}
