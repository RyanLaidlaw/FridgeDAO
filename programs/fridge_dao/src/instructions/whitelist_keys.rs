use anchor_lang::prelude::*;

use crate::{error, state};

#[derive(Accounts)]
pub struct DAOWhitelist<'info> {
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,
}

pub fn whitelist_keys(ctx: Context<DAOWhitelist>, new_keys: Vec<Pubkey>, token_account_amounts: Vec<u64>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.signer.key() == dao.mint_admin_program, error::Error::InvalidAuthority);

    dao.valid_member_keys.extend(new_keys
        .iter()
        .zip(token_account_amounts.iter())
        .map(|(addr, amount)| state::UserWithBalance{key: *addr, balance: *amount}));

    Ok(())
}
