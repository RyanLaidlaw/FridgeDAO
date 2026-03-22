use anchor_lang::prelude::{borsh::de, *};
use anchor_spl::token_interface::{TokenAccount, TokenInterface, Mint, TransferChecked, transfer_checked};

use crate::{error, state};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,

    #[account(
        mut,
        constraint = vault.key() == fridge_dao.vault @ error::Error::InvalidVault
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_account.owner == depositor.key()
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.user_token_account.mint == dao.usdc_mint, error::Error::InvalidMint);
    require!(dao.valid_member_keys.iter().any(|user| user.key == ctx.accounts.depositor.key()), error::Error::InvalidMember);

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.depositor.to_account_info(),
        mint: ctx.accounts.usdc_mint.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );

    transfer_checked(cpi_ctx, amount, state::DECIMALS)?;

    let user = dao
        .valid_member_keys
        .iter_mut()
        .find(|user| user.key == ctx.accounts.depositor.key())
        .ok_or(error::Error::InvalidMember)?;

    user.balance = user
        .balance
        .checked_add(amount)
        .ok_or(error::Error::MathOverflowOrUnderflow)?;

    Ok(())
}