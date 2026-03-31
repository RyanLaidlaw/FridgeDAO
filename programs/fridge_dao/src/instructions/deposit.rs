use anchor_lang::prelude::*;

use crate::{error, state};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,

    /// CHECK: Pubkey is used only for user lookup in fridge_dao. No data is read from the account
    pub user: UncheckedAccount<'info>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.signer.key() == dao.mint_admin_program, error::Error::InvalidAuthority);

    let user = dao
        .valid_member_keys
        .iter_mut()
        .find(|user| user.key == ctx.accounts.user.key())
        .ok_or(error::Error::InvalidMember)?;

    user.balance = user
        .balance
        .checked_add(amount)
        .ok_or(error::Error::MathOverflowOrUnderflow)?;

    Ok(())
}
