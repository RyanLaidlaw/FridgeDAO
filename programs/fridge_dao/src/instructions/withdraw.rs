use anchor_lang::prelude::*;

use crate::{error, state};

#[derive(Accounts)]
pub struct Withdraw<'info> {
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

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(ctx.accounts.signer.key() == ctx.accounts.fridge_dao.mint_admin_program, error::Error::InvalidAuthority);
    require!(ctx.accounts.fridge_dao.valid_member_keys.iter().any(|user| user.key == ctx.accounts.user.key()), error::Error::InvalidMember);

    let user = ctx.accounts.fridge_dao
        .valid_member_keys
        .iter_mut()
        .find(|user| user.key == ctx.accounts.user.key())
        .ok_or(error::Error::InvalidMember)?;

    require!(amount <= user.balance, error::Error::InvalidAmount);

    user.balance = user
        .balance
        .checked_sub(amount)
        .ok_or(error::Error::MathOverflowOrUnderflow)?;

    Ok(())
}
