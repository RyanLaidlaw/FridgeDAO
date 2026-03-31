use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, TokenInterface, Mint};

use crate::{error, instructions::lib, state};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [state::DAOMint::SEED_PREFIX,],
        bump,
    )]
    pub dao_mint: Account<'info, state::DAOMint>,

    pub dao_program: Program<'info, fridge_dao::program::FridgeDao>,

    /// CHECK: This account is verified by the CPI to the fridge_dao program, where the seeds and bump are checked
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
        constraint = user_token_account.owner == depositor.key() @ error::Error::InvalidTokenAccount
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(ctx.accounts.dao_mint.valid_member_keys.iter().any(|user| user.key == ctx.accounts.depositor.key()), error::Error::InvalidMember);

    lib::transfer(
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.user_token_account.to_account_info(),
        ctx.accounts.dao_mint.to_account_info(),
        ctx.accounts.usdc_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        amount
    )?;

    let seeds = &[
        state::DAOMint::SEED_PREFIX,
        &[ctx.bumps.dao_mint][..],
    ];

    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.dao_program.to_account_info(),
        fridge_dao::cpi::accounts::Deposit {
            signer: ctx.accounts.dao_mint.to_account_info(),
            fridge_dao: ctx.accounts.fridge_dao.to_account_info(),
            user: ctx.accounts.depositor.to_account_info(),
        },
        signer_seeds
    );

    fridge_dao::cpi::deposit(cpi_ctx, amount)?;

    Ok(())
}