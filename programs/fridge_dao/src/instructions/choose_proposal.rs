use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::{instructions::lib, state, error};

#[derive(Accounts)]
pub struct Choose<'info> {
    #[account(mut)]
    pub chooser: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        constraint = admin_token_account.owner == chooser.key() @ error::Error::InvalidTokenAccount,
        constraint = admin_token_account.mint == usdc_mint.key() @ error::Error::InvalidMint
    )]
    pub admin_token_account: InterfaceAccount<'info, TokenAccount>,
}

pub fn choose_proposal<'info>(ctx: Context<'_, '_, 'info, 'info, Choose<'info>>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.chooser.key() == dao.admin, error::Error::InvalidAuthority);

    let now = Clock::get()?.unix_timestamp;
    let voting_end = lib::get_voting_end(&dao)?;

    require!(now > voting_end, error::Error::VotingHasNotEnded);

    let balance = dao.vault_balance;

    lib::compute_winner(dao, ctx.remaining_accounts, balance)?;

    let now = Clock::get()?.unix_timestamp;
    dao.next_vote_allowed_at = now + dao.vote_cooldown as i64;

    Ok(())
}
