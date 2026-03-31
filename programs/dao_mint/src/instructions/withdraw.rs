use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, TokenInterface, Mint};

use crate::{error, instructions::lib, state};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,

    #[account(
        seeds = [state::DAOMint::SEED_PREFIX,],
        bump,
    )]
    pub dao_mint: Account<'info, state::DAOMint>,

    pub dao_program: Program<'info, fridge_dao::program::FridgeDao>,

    /// CHECK: This account is verified by the CPI to the fridge_dao program, where the seeds and bump are checked`
    #[account(mut)]
    pub fridge_dao: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = vault.key() == dao_mint.vault @ error::Error::InvalidVault
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        constraint = user_token_account.owner == withdrawer.key() @ error::Error::InvalidTokenAccount
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(ctx.accounts.dao_mint.valid_member_keys.iter().any(|user| user.key == ctx.accounts.withdrawer.key()), error::Error::InvalidMember);
    require!(ctx.accounts.user_token_account.mint == ctx.accounts.dao_mint.usdc_mint, error::Error::InvalidMint);

    let seeds = &[
        state::DAOMint::SEED_PREFIX,
        &[ctx.bumps.dao_mint][..],
    ];

    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.dao_program.to_account_info(),
        fridge_dao::cpi::accounts::Withdraw {
            signer: ctx.accounts.dao_mint.to_account_info(),
            fridge_dao: ctx.accounts.fridge_dao.to_account_info(),
            user: ctx.accounts.withdrawer.to_account_info(),
        },
        signer_seeds
    );

    fridge_dao::cpi::withdraw(cpi_ctx, amount)?;

    lib::transfer(
        ctx.accounts.user_token_account.to_account_info(), 
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.dao_mint.to_account_info(), 
        ctx.accounts.usdc_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        amount
    )?;

    Ok(())
}