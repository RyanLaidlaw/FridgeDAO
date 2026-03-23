use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, TokenInterface, Mint, TransferChecked, transfer_checked};
use anchor_spl::associated_token::get_associated_token_address;
use std::collections::HashSet;

use crate::{error, state};

// DEV: pass in users token accounts through ctx.remaining_accounts
#[derive(Accounts)]
pub struct RemoveKeys<'info> {
    #[account(mut)]
    pub remover: Signer<'info>,

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

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn remove_valid_keys<'info>(ctx: Context<'_, '_, 'info, 'info, RemoveKeys<'info>>, keys_to_remove: Vec<Pubkey>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.remover.key() == dao.authority, error::Error::InvalidAuthority);
    require!(ctx.remaining_accounts.len() == keys_to_remove.len(), error::Error::MismatchingRemainingAccountsLength);
    require!(ctx.accounts.vault.mint == dao.usdc_mint, error::Error::InvalidMint);

    let mut seen: HashSet<Pubkey> = HashSet::new();

    let dao_key = dao.key();

    let seeds = &[
        b"vault_authority",
        dao_key.as_ref(),
        &[dao.vault_bump],
    ];

    let signer = &[&seeds[..]];

    for (i, k) in keys_to_remove.iter().enumerate() {
        require!(seen.insert(*k), error::Error::DuplicateKeys);

        let user = dao.valid_member_keys.iter().find(|user| user.key == *k).ok_or(error::Error::KeyDoesNotExist)?;
        let amount = user.balance;

        if amount > 0 {
            let acct_info = &ctx.remaining_accounts[i];
            let user_token_account: InterfaceAccount<TokenAccount> = InterfaceAccount::try_from(acct_info)?;

            require!(user_token_account.owner == *k, error::Error::InvalidTokenAccountOwner);
            require!(user_token_account.mint == dao.usdc_mint, error::Error::InvalidMint);
            
            let expected_ata = get_associated_token_address(k, &dao.usdc_mint);
            require!(user_token_account.key() == expected_ata, error::Error::InvalidTokenAccount);

            let cpi_accounts = TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                to: user_token_account.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
                mint: ctx.accounts.usdc_mint.to_account_info(),
            };

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                signer,
            );

            transfer_checked(cpi_ctx, amount, state::DECIMALS)?;
        }
    }

    dao.valid_member_keys.retain(|user| !seen.contains(&user.key));
    
    Ok(())
}