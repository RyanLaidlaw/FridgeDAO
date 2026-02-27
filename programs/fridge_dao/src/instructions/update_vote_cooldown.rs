use anchor_lang::prelude::*;

use crate::state;
use crate::error;

#[derive(Accounts)]
pub struct UpdateCooldown<'info> {
    #[account(mut)]
    pub updater: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,
}

pub fn update_vote_cooldown(ctx: Context<UpdateCooldown>, new_cooldown: u64) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;
    
    require!(ctx.accounts.updater.key() == dao.authority, error::Error::InvalidAuthority);
    require!(new_cooldown != dao.vote_cooldown, error::Error::IdenticalCooldown);

    dao.vote_cooldown = new_cooldown;

    Ok(())
}