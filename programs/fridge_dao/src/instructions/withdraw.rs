use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, TokenInterface, Mint, TransferChecked, transfer_checked};

use crate::{error, state};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,

    #[account(
        seeds = [b"vault_authority", fridge_dao.key().as_ref()],
        bump = fridge_dao.vault_bump,
    )]
    pub vault_authority: SystemAccount<'info>,

    #[account(
        mut,
        constraint = vault.key() == fridge_dao.vault @ error::Error::InvalidVault
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_account.owner == withdrawer.key()
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.user_token_account.mint == dao.usdc_mint, error::Error::InvalidMint);
    require!(dao.valid_member_keys.iter().any(|user| user.key == ctx.accounts.withdrawer.key()), error::Error::InvalidMember);

    let dao_key = dao.key();
    let seeds = &[
        b"vault_authority",
        dao_key.as_ref(),
        &[dao.vault_bump],
    ];

    let pda_signer = &[&seeds[..]];

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.vault_authority.to_account_info(),
        mint: ctx.accounts.usdc_mint.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        pda_signer
    );

    transfer_checked(cpi_ctx, amount, state::DECIMALS)?;

    let user = dao
        .valid_member_keys
        .iter_mut()
        .find(|user| user.key == ctx.accounts.withdrawer.key())
        .ok_or(error::Error::InvalidMember)?;

    user.balance = user
        .balance
        .checked_sub(amount)
        .ok_or(error::Error::MathOverflowOrUnderflow)?;

    Ok(())
}