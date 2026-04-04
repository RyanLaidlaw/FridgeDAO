use anchor_lang::prelude::*;
use std::collections::HashSet;
use anchor_spl::token_interface::{TokenAccount, TokenInterface, Mint, TransferChecked, transfer_checked};
use anchor_spl::associated_token::get_associated_token_address;

use crate::{error, state};

#[derive(Accounts)]
pub struct BlacklistKeys<'info> {
    #[account(mut)]
    pub adder: Signer<'info>,

    #[account(
        mut,
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

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn blacklist_keys<'info>(ctx: Context<'_, '_, '_, 'info , BlacklistKeys<'info>>, keys_to_blacklist: Vec<Pubkey>) -> Result<()> {
    require!(ctx.remaining_accounts.len() == keys_to_blacklist.len(), error::Error::MismatchingRemainingAccountsLength);

    let mut seen: HashSet<Pubkey> = HashSet::new();

    let seeds = &[
        state::DAOMint::SEED_PREFIX,
        &[ctx.bumps.dao_mint][..],
    ];
    let signer_seeds = &[&seeds[..]];

    for (i, k) in keys_to_blacklist.iter().enumerate() {
        require!(seen.insert(*k), error::Error::DuplicateKeys);

        require!(ctx.accounts.dao_mint.valid_member_keys.iter().any(|user| user.key == *k), error::Error::KeyDoesNotExist);
        
        let acct = &ctx.remaining_accounts[i];
        let token_acct = TokenAccount::try_deserialize(&mut acct.try_borrow_data()?.as_ref())?;

        require!(token_acct.owner == *k, error::Error::InvalidTokenAccountOwner);
        require!(token_acct.mint == ctx.accounts.dao_mint.usdc_mint, error::Error::InvalidMint);

        let amount: u64 = token_acct.amount;

        if amount > 0 {
            let expected_ata = get_associated_token_address(k, &ctx.accounts.dao_mint.usdc_mint);
            require!(acct.key() == expected_ata, error::Error::InvalidTokenAccount);

            let cpi_accounts = TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                to: acct.to_account_info(),
                authority: ctx.accounts.dao_mint.to_account_info(),
                mint: ctx.accounts.usdc_mint.to_account_info(),
            };

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );

            transfer_checked(cpi_ctx, amount, state::DECIMALS)?;
        }
    }

    ctx.accounts.dao_mint.valid_member_keys.retain(|user| !keys_to_blacklist.contains(&user.key));

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.dao_program.to_account_info(),
        fridge_dao::cpi::accounts::DAOBlacklist {
            signer: ctx.accounts.dao_mint.to_account_info(),
            fridge_dao: ctx.accounts.fridge_dao.to_account_info()
        },
        signer_seeds
    );

    fridge_dao::cpi::blacklist(cpi_ctx, keys_to_blacklist)?;

    Ok(())
}